//! src/rutas/marca/crea/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de marca

use crate::domain::marca::{
    Nombre, 
    Nuevo,
};
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        Ok( Self{ nombre })
    }
}

// extrae datos del marca del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de marca",
    skip(form, pool),
    fields( 
        marca_nombre = %form.nombre,
    )
)]
#[post("/marca")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, MarcaError> {
    let marca = form.0.try_into().map_err(MarcaError::Validacion)?;
    let id = marca_inserta(&pool, &marca)
        .await
        .context("Error al insertar marca en la BD")?;
    let url_ver =  format!("/marca/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de marcas
#[derive(thiserror::Error)]
pub enum MarcaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for MarcaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for MarcaError {
    fn status_code(&self) -> StatusCode {
        match self {
            MarcaError::Validacion(_) => StatusCode::BAD_REQUEST,
            MarcaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un marca en la base de datos
#[tracing::instrument(name = "Inserta marca", skip(marca_nuevo, pool))]
pub async fn marca_inserta(
    pool: &PgPool,
    marca_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO marcas (nombre) 
        VALUES ($1) RETURNING id"#,
    )
    .bind(marca_nuevo.nombre.as_ref())
    .fetch_one(pool)
    .await?;
    Ok(id)
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
