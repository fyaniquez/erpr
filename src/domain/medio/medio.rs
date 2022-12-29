//! src/domain/medio/medio.rs
//! autor: fyaniquez
//! fecha: 03-03-2022

use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Serialize, sqlx::FromRow)]
pub struct Medio {
    pub id: Option<i64>,
    pub nombre: String,
    pub sigla: String,
}

// errores considerados para lista de capitulos
#[derive(thiserror::Error)]
pub enum MedioError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for MedioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for MedioError {
    fn status_code(&self) -> StatusCode {
        match self {
            MedioError::Validacion(_) => StatusCode::BAD_REQUEST,
            MedioError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

