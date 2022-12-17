//! domain/fabrica/fabrica.rs
//! autor: fyaniquez
//! fecha: 03-03-2022

use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Serialize, sqlx::FromRow)]
pub struct Fabrica {
    pub id: Option<i64>,
    pub nombre: String,
    pub pais_id: i64,
}

// errores considerados para lista de fabricas
#[derive(thiserror::Error)]
pub enum FabricaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
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
            FabricaError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

