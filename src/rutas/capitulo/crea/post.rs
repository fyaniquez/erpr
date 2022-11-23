//! src/rutas/capitulo/alta/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de capitulo

use crate::domain::{
    CapituloNombre, 
    CapituloDescripcion,
    CapituloNuevo,
};
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    descripcion: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for CapituloNuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = CapituloNombre::parse(form_data.nombre)?;
        let descripcion = CapituloDescripcion::parse(form_data.descripcion)?;
        Ok( Self{ nombre, descripcion, })
    }
}

// extrae datos del capitulo del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de capitulo",
    skip(form, pool),
    fields( 
        capitulo_descripcion = %form.descripcion,
        capitulo_nombre = %form.nombre,
    )
)]
#[post("/capitulo")]
pub async fn capitulo_crea(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CapituloError> {
    let capitulo = form.0.try_into().map_err(CapituloError::Validacion)?;
    let capitulo_id = capitulo_inserta(&pool, &capitulo)
        .await
        .context("Error al insertar capitulo en la BD")?;
    Ok(HttpResponse::Ok().finish())
}

// errores considerados para alta de capitulos
#[derive(thiserror::Error)]
pub enum CapituloError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for CapituloError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CapituloError {
    fn status_code(&self) -> StatusCode {
        match self {
            CapituloError::Validacion(_) => StatusCode::BAD_REQUEST,
            CapituloError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un capitulo en la base de datos
#[tracing::instrument(name = "Inserta capitulo", skip(capitulo_nuevo, pool))]
pub async fn capitulo_inserta(
    pool: &PgPool,
    capitulo_nuevo: &CapituloNuevo,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query!(
r#"INSERT INTO capitulos (nombre, descripcion)
   VALUES ($1, $2)
   RETURNING capitulo_id"#,
        capitulo_nuevo.nombre.as_ref(),
        capitulo_nuevo.descripcion.as_ref(),
    )
    .fetch_one(pool)
    .await?;
    Ok(row.capitulo_id)
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Causa:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
