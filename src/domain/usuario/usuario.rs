//! domain/usuario/usuario,rs
//! autor: fyaniquez
//! fecha: 03-03-2022

use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: i64,
    pub nombre: String,
    pub documento: String,
    pub email: String,
    pub password_digest: String,
    pub activo: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
// errores considerados para lista de capitulos
#[derive(thiserror::Error)]
pub enum UsuarioError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for UsuarioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for UsuarioError {
    fn status_code(&self) -> StatusCode {
        match self {
            UsuarioError::Validacion(_) => StatusCode::BAD_REQUEST,
            UsuarioError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
