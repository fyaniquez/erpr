//! domain/venta/venta
//! autor: fyaniquez
//! fecha: 03-03-2022

use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Venta {
    pub id: Option<i64>,
    pub fecha: Option<NaiveDateTime>,
    pub total: i32,
    pub descuento: i32,
    pub cliente_id: i64,
    pub puesto_id: Option<i64>,
    pub usuario_id: Option<i64>,
    pub medio_id: i64,
    pub estado: Option<String>,
}

// errores considerados para lista de ventas
#[derive(thiserror::Error)]
pub enum VentaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
}

// modelo
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct VentaVe {
    pub id: i64,
    pub fecha: NaiveDateTime,
    pub total: i32,
    pub descuento: i32,
    pub puesto: String,
    pub usuario: String,
    pub cliente: String,
    pub medio: String,
}

impl std::fmt::Debug for VentaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for VentaError {
    fn status_code(&self) -> StatusCode {
        match self {
            VentaError::Validacion(_) => StatusCode::BAD_REQUEST,
            VentaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VentaError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
