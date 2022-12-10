//! src/rutas/categoria/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea categoria

use crate::domain::CategoriaNombre;
use crate::domain::CategoriaNuevo;
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    capitulo_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for CategoriaNuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = CategoriaNombre::parse(form_data.nombre)?;
        // todo simplificar las siguientes 2 lineas en una sola
        let capitulo_id = form_data.capitulo_id;
        Ok( Self{ nombre, capitulo_id})
    }
}

// extrae datos del categoria del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de categoria",
    skip(form, pool),
    fields( 
        categoria_nombre = %form.nombre,
    )
)]
#[post("/categoria")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CategoriaError> {
    //TODO añadir validacion de existencia de capitulo_id
    let categoria = form.0.try_into().map_err(CategoriaError::Validacion)?;
    let id = categoria_inserta(&pool, &categoria)
        .await
        .context("Error al insertar categoria en la BD")?;
    let url_ver =  format!("/categoria/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de categorias
#[derive(thiserror::Error)]
pub enum CategoriaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for CategoriaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CategoriaError {
    fn status_code(&self) -> StatusCode {
        match self {
            CategoriaError::Validacion(_) => StatusCode::BAD_REQUEST,
            CategoriaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un categoria en la base de datos
#[tracing::instrument(name = "Inserta categoria", skip(categoria_nuevo, pool))]
pub async fn categoria_inserta(
    pool: &PgPool,
    categoria_nuevo: &CategoriaNuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
"INSERT INTO categorias (nombre, capitulo_id) VALUES ($1, $2) RETURNING id",
    )
    .bind(categoria_nuevo.nombre.as_ref())
    .bind(categoria_nuevo.capitulo_id)
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
