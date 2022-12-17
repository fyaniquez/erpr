//! domain/catgoria/catalogo
//! autor: fyaniquez
//! fecha: 03-03-2022

use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use chrono::NaiveDateTime;

#[derive(Serialize, sqlx::FromRow)]
pub struct Catalogo {
    pub id: Option<i64>,
    pub nombre: String,
    pub propietario: i64,
    pub empresa_id: i64,
    pub fecha: NaiveDateTime,
    pub activo: bool,
}

// errores considerados para lista de catalogos
#[derive(thiserror::Error)]
pub enum CatalogoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
}

impl std::fmt::Debug for CatalogoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CatalogoError {
    fn status_code(&self) -> StatusCode {
        match self {
            CatalogoError::Validacion(_) => StatusCode::BAD_REQUEST,
            CatalogoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CatalogoError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

