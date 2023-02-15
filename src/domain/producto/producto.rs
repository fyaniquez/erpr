//! domain/catgoria/producto
//! autor: fyaniquez
//! fecha: 03-03-2022

use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Producto {
    pub id: Option<i64>,
    pub nombre: String,
    pub caracteristicas: String,
    pub categoria_id: i64,
    pub marca_id: i64,
    pub unidad_id: i64,
    pub fabrica_id: i64,
    pub contenido: String,
    pub cantidad: i32,
    pub fraccionable: bool,
    pub barras: Option<String>,
    pub activo: bool,
}

// errores considerados para lista de productos
#[derive(thiserror::Error)]
pub enum ProductoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
    #[error(transparent)]
    Lookups(#[from] sqlx::Error),
}

impl std::fmt::Debug for ProductoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ProductoError {
    fn status_code(&self) -> StatusCode {
        match self {
            ProductoError::Validacion(_) => StatusCode::BAD_REQUEST,
            ProductoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ProductoError::Lookups(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

// modelo
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct ProductoVe {
    pub id: i64,
    pub nombre: String,
    pub caracteristicas: String,
    pub capitulo: String,
    pub categoria: String,
    pub marca: String,
    pub unidad: String,
    pub fabrica: String,
    pub contenido: String,
    pub cantidad: i32,
    pub fraccionable: String,
    pub barras: String,
    pub activo: String,
}
