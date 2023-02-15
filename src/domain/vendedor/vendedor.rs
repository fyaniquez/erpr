//! domain/vendedor/vendedor
//! autor: fyaniquez
//! fecha: 03-03-2022

use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Serialize, sqlx::FromRow)]
pub struct Vendedor {
    pub id: Option<i64>,
    pub distribuidora_id: i64,
    pub cargo: String,
    pub nombre: String,
    pub contactos: String,
}

// errores considerados para lista de vendedors
#[derive(thiserror::Error)]
pub enum VendedorError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
}

impl std::fmt::Debug for VendedorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for VendedorError {
    fn status_code(&self) -> StatusCode {
        match self {
            VendedorError::Validacion(_) => StatusCode::BAD_REQUEST,
            VendedorError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
            VendedorError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

