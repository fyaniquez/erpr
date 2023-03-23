//! src/domain/pais/categoria_marca.rs
//! autor: fyaniquez
//! fecha: 03-03-2022

use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Serialize, sqlx::FromRow)]
pub struct CategoriaMarca {
    pub categoria_id: i64,
    pub marca_id: i64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CategoriaMarcaNombres {
    pub categoria_id: i64,
    pub categoria_nombre: String,
    pub marca_id: i64,
    pub marca_nombre: String,
}

// errores considerados para lista de categorias
#[derive(thiserror::Error)]
pub enum CategoriaMarcaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
}

impl ResponseError for CategoriaMarcaError {
    fn status_code(&self) -> StatusCode {
        match self {
            CategoriaMarcaError::Validacion(_) => StatusCode::BAD_REQUEST,
            CategoriaMarcaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CategoriaMarcaError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Debug for CategoriaMarcaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
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

