//! domain/comprado/comprado
//! autor: fyaniquez
//! fecha: 03-03-2022

use serde::{Serialize, Deserialize};
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Comprado {
    pub id: Option<i64>,
    pub producto_id: i64,
    pub compra_id: Option<i64>,
    pub cantidad: i32,
    pub costo: i32,
    pub descuento: i32,
    pub total: i32,
    pub vencimiento: Option<NaiveDateTime>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CompradoVe {
    pub id: i64,
    pub compra_id: i64,
    pub producto: String,
    pub cantidad: i32,
    pub costo: i32,
    pub descuento: i32,
    pub total: i32,
    pub vencimiento: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct Comprados {
    pub producto_ids: Vec<i64>,
    pub cantidads: Vec<i32>,
    pub costos: Vec<i32>,
    pub descuentos: Vec<i32>,
    pub totals: Vec<i32>,
    pub vencimientos: Vec<Option<String>>,
}
// errores considerados para lista de comprados
#[derive(thiserror::Error)]
pub enum CompradoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
}

impl std::fmt::Debug for CompradoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CompradoError {
    fn status_code(&self) -> StatusCode {
        match self {
            CompradoError::Validacion(_) => StatusCode::BAD_REQUEST,
            CompradoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CompradoError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

