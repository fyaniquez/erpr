//! domain/vendido/vendido
//! autor: fyaniquez
//! fecha: 03-03-2022

use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Serialize, sqlx::FromRow)]
pub struct Vendido {
    pub id: Option<i64>,
    pub producto_id: i64,
    pub venta_id: i64,
    pub cantidad: i32,
    pub precio: i32,
    pub descuento: i32,
    pub total: i32,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct VendidoVe {
    pub id: i64,
    pub venta_id: i64,
    pub producto: String,
    pub cantidad: i32,
    pub precio: i32,
    pub descuento: i32,
    pub total: i32,
}
// errores considerados para lista de vendidos
#[derive(thiserror::Error)]
pub enum VendidoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
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
            VendidoError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
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

