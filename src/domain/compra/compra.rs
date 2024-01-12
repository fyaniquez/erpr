//! domain/compra/compra
//! autor: fyaniquez
//! fecha: 03-03-2022

use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Compra {
    pub id: Option<i64>,
    pub fecha: Option<NaiveDateTime>,
    pub total: i32,
    pub descuento: i32,
    pub distribuidora_id: Option<i64>,
    pub sucursal_id: Option<i64>,
    pub usuario_id: Option<i64>,
    pub medio_id: i64,
    pub documento: String,
    pub observaciones: String,
    pub estado: Option<String>,
}

// errores considerados para lista de compras
#[derive(thiserror::Error)]
pub enum CompraError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
}

// modelo
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct CompraVe {
    pub id: i64,
    pub fecha: NaiveDateTime,
    pub total: i32,
    pub descuento: i32,
    pub distribuidora: String,
    pub sucursal: String,
    pub usuario: String,
    pub medio: String,
    pub documento: String,
    pub observaciones: String,
}

impl std::fmt::Debug for CompraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CompraError {
    fn status_code(&self) -> StatusCode {
        match self {
            CompraError::Validacion(_) => StatusCode::BAD_REQUEST,
            CompraError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CompraError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
