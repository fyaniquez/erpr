//! src/rutas/precio/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea precio

use crate::domain::precio::Nuevo;
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    producto_id: i64,
    precio: i32,
    catalogo_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        Ok( Self{ 
            producto_id: form_data.producto_id,
            precio: form_data.precio,
            catalogo_id: form_data.catalogo_id,
        })
    }
}

// extrae datos del precio del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de precio",
    skip(form, pool),
)]
#[post("/precio")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, PrecioError> {
    //TODO añadir validacion de existencia de capitulo_id
    let precio = form.0.try_into().map_err(PrecioError::Validacion)?;
    let id = precio_inserta(&pool, &precio)
        .await
        .context("Error al insertar precio en la BD")?;
    let url_ver =  format!("/precio/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de precios
#[derive(thiserror::Error)]
pub enum PrecioError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for PrecioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PrecioError {
    fn status_code(&self) -> StatusCode {
        match self {
            PrecioError::Validacion(_) => StatusCode::BAD_REQUEST,
            PrecioError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un precio en la base de datos
#[tracing::instrument(name = "Inserta precio", skip(precio_nuevo, pool))]
pub async fn precio_inserta(
    pool: &PgPool,
    precio_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO precios 
    (producto_id, precio, catalogo_id) 
    VALUES ($1, $2, $3) RETURNING id"#)
    .bind(precio_nuevo.producto_id)
    .bind(precio_nuevo.precio)
    .bind(precio_nuevo.catalogo_id)
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
