//! src/rutas/fabrica/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea fabrica

use crate::domain::fabrica::Nombre;
use crate::domain::fabrica::Nuevo;
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    pais_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        // todo simplificar las siguientes 2 lineas en una sola
        let pais_id = form_data.pais_id;
        Ok( Self{ nombre, pais_id})
    }
}

// extrae datos del fabrica del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de fabrica",
    skip(form, pool),
    fields( 
        fabrica_nombre = %form.nombre,
    )
)]
#[post("/fabrica")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, FabricaError> {
    //TODO añadir validacion de existencia de pais_id
    let fabrica = form.0.try_into().map_err(FabricaError::Validacion)?;
    let id = fabrica_inserta(&pool, &fabrica)
        .await
        .context("Error al insertar fabrica en la BD")?;
    let url_ver =  format!("/fabrica/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de fabricas
#[derive(thiserror::Error)]
pub enum FabricaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for FabricaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for FabricaError {
    fn status_code(&self) -> StatusCode {
        match self {
            FabricaError::Validacion(_) => StatusCode::BAD_REQUEST,
            FabricaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un fabrica en la base de datos
#[tracing::instrument(name = "Inserta fabrica", skip(nuevo, pool))]
pub async fn fabrica_inserta(
    pool: &PgPool,
    nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
"INSERT INTO fabricas (nombre, pais_id) VALUES ($1, $2) RETURNING id",
    )
    .bind(nuevo.nombre.as_ref())
    .bind(nuevo.pais_id)
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
