//! src/rutas/medio/crea/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de medio

use crate::domain::medio::{
    Nombre, 
    Sigla,
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
    sigla: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let sigla = Sigla::parse(form_data.sigla)?;
        Ok( Self{ nombre, sigla, })
    }
}

// extrae datos del medio del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de medio",
    skip(form, pool),
    fields( 
        medio_sigla = %form.sigla,
        medio_nombre = %form.nombre,
    )
)]
#[post("/medio")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, MedioError> {
    let medio = form.0.try_into().map_err(MedioError::Validacion)?;
    let id = medio_inserta(&pool, &medio)
        .await
        .context("Error al insertar medio en la BD")?;
    let url_ver =  format!("/medio/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de medios
#[derive(thiserror::Error)]
pub enum MedioError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for MedioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for MedioError {
    fn status_code(&self) -> StatusCode {
        match self {
            MedioError::Validacion(_) => StatusCode::BAD_REQUEST,
            MedioError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un medio en la base de datos
#[tracing::instrument(name = "Inserta medio", skip(medio_nuevo, pool))]
pub async fn medio_inserta(
    pool: &PgPool,
    medio_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO medios (nombre, sigla) 
        VALUES ($1, $2) RETURNING id"#,
    )
    .bind(medio_nuevo.nombre.as_ref())
    .bind(medio_nuevo.sigla.as_ref())
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
