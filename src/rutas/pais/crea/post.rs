//! src/rutas/pais/crea/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de pais

use crate::domain::pais::{
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

// extrae datos del pais del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de pais",
    skip(form, pool),
    fields( 
        pais_sigla = %form.sigla,
        pais_nombre = %form.nombre,
    )
)]
#[post("/pais")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, PaisError> {
    let pais = form.0.try_into().map_err(PaisError::Validacion)?;
    let id = pais_inserta(&pool, &pais)
        .await
        .context("Error al insertar pais en la BD")?;
    let url_ver =  format!("/pais/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de paiss
#[derive(thiserror::Error)]
pub enum PaisError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for PaisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PaisError {
    fn status_code(&self) -> StatusCode {
        match self {
            PaisError::Validacion(_) => StatusCode::BAD_REQUEST,
            PaisError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un pais en la base de datos
#[tracing::instrument(name = "Inserta pais", skip(pais_nuevo, pool))]
pub async fn pais_inserta(
    pool: &PgPool,
    pais_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO paises (nombre, sigla) 
        VALUES ($1, $2) RETURNING id"#,
    )
    .bind(pais_nuevo.nombre.as_ref())
    .bind(pais_nuevo.sigla.as_ref())
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
