//! src/rutas/inventario/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea inventario

use crate::domain::inventario::{
    Nombre,
    Nuevo,
    inserta as inventario_inserta,
};
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use chrono::Utc;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        Ok( Self{ 
            nombre, 
            sucursal_id: 0,
            fecha: Utc::now().naive_utc(),
            estado: String::from(""),
        })
    }
}

// extrae datos del inventario del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de inventario",
    skip(form, pool),
    fields( 
        inventario_nombre = %form.nombre,
    )
)]
#[post("/inventario")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, InventarioError> {

    //TODO añadir validacion de existencia de empresa_id
    let mut inventario: Nuevo = form.0.try_into().map_err(InventarioError::Validacion)?;
    //TODO obtener la sucursal del estado
    inventario.sucursal_id = 1;
    let id = inventario_inserta(&pool, &inventario)
        .await
        .context("Error al insertar inventario en la BD")?;

    let url_ver =  format!("/inventario/{}", id);

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de inventarios
#[derive(thiserror::Error)]
pub enum InventarioError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for InventarioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for InventarioError {
    fn status_code(&self) -> StatusCode {
        match self {
            InventarioError::Validacion(_) => StatusCode::BAD_REQUEST,
            InventarioError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
