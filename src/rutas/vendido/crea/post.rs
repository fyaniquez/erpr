//! src/rutas/vendido/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea vendido

use crate::domain::vendido::Nuevo;
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    venta_id: i64,
    producto_id: i64,
    cantidad: i64,
    precio: i64,
    descuento: i64,
    total: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        // todo simplificar las siguientes 2 lineas en una sola
        Ok( Self{ 
            producto_id: form_data.producto_id, 
            venta_id: form_data.venta_id,
            cantidad: form_data.cantidad,
            precio: form_data.precio,
            descuento: form_data.descuento,
            total: form_data.total,
        })
    }
}

// extrae datos del vendido del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de vendido",
    skip(form, pool),
)]
#[post("/vendido")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, VendidoError> {
    //TODO añadir validacion de existencia de venta_id
    let vendido = form.0.try_into().map_err(VendidoError::Validacion)?;
    let id = vendido_inserta(&pool, &vendido)
        .await
        .context("Error al insertar vendido en la BD")?;
    let url_ver =  format!("/vendido/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de vendidos
#[derive(thiserror::Error)]
pub enum VendidoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for VendidoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for VendidoError {
    fn status_code(&self) -> StatusCode {
        match self {
            VendidoError::Validacion(_) => StatusCode::BAD_REQUEST,
            VendidoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un vendido en la base de datos
#[tracing::instrument(name = "Inserta vendido", skip(vendido_nuevo, pool))]
pub async fn vendido_inserta(
    pool: &PgPool,
    vendido_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO vendidos 
        (producto_id, venta_id, cantidad, precio, descuento, total) 
        VALUES ($1, $2, $3, $4 $5, $6) RETURNING id"#
    )
    .bind(vendido_nuevo.producto_id)
    .bind(vendido_nuevo.venta_id)
    .bind(vendido_nuevo.cantidad)
    .bind(vendido_nuevo.precio)
    .bind(vendido_nuevo.descuento)
    .bind(vendido_nuevo.total)
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
