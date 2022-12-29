//! domain/catgoria/puesto
//! autor: fyaniquez
//! fecha: 03-03-2022

use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Serialize, sqlx::FromRow)]
pub struct Puesto {
    pub id: Option<i64>,
    pub nombre: String,
    pub sigla: String,
    pub descripcion: String,
    pub sucursal_id: i64,
    pub activo: bool,
}

// errores considerados para lista de puestos
#[derive(thiserror::Error)]
pub enum PuestoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
}

impl std::fmt::Debug for PuestoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PuestoError {
    fn status_code(&self) -> StatusCode {
        match self {
            PuestoError::Validacion(_) => StatusCode::BAD_REQUEST,
            PuestoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PuestoError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

