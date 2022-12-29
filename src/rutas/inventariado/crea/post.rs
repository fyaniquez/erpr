//! src/rutas/inventariado/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea inventariado

use crate::domain::inventariado::Nuevo;
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    producto_id: i64,
    cantidad: i32,
    inventario_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        Ok( Self{ 
            producto_id: form_data.producto_id,
            cantidad: form_data.cantidad,
            inventario_id: form_data.inventario_id,
        })
    }
}

// extrae datos del inventariado del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de inventariado",
    skip(form, pool),
)]
#[post("/inventariado")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, InventariadoError> {
    //TODO añadir validacion de existencia de capitulo_id
    let inventariado = form.0.try_into().map_err(InventariadoError::Validacion)?;
    let id = inventariado_inserta(&pool, &inventariado)
        .await
        .context("Error al insertar inventariado en la BD")?;
    let url_ver =  format!("/inventariado/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de inventariados
#[derive(thiserror::Error)]
pub enum InventariadoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for InventariadoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for InventariadoError {
    fn status_code(&self) -> StatusCode {
        match self {
            InventariadoError::Validacion(_) => StatusCode::BAD_REQUEST,
            InventariadoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un inventariado en la base de datos
#[tracing::instrument(name = "Inserta inventariado", skip(inventariado_nuevo, pool))]
pub async fn inventariado_inserta(
    pool: &PgPool,
    inventariado_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO inventariados 
    (producto_id, cantidad, inventario_id) 
    VALUES ($1, $2, $3) RETURNING id"#)
    .bind(inventariado_nuevo.producto_id)
    .bind(inventariado_nuevo.cantidad)
    .bind(inventariado_nuevo.inventario_id)
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
