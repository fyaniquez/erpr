//! src/rutas/catalogo/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea catalogo

use crate::domain::catalogo::{
    Nombre,
    Nuevo,
    inserta as catalogo_inserta,
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
            sucursal_id: 1,
        })
    }
}

// extrae datos del catalogo del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de catalogo",
    skip(form, pool),
    fields( 
        catalogo_nombre = %form.nombre,
    )
)]
#[post("/catalogo")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CatalogoError> {

    //TODO añadir validacion de existencia de capitulo_id
    let mut catalogo: Nuevo = form.0.try_into().map_err(CatalogoError::Validacion)?;

    // TOMAR la sucursal del estado
    catalogo.sucursal_id = 1;

    let id = catalogo_inserta(&pool, &catalogo)
        .await
        .context("Error al insertar catalogo en la BD")?;

    let url_ver =  format!("/catalogo/{}", id);

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de catalogos
#[derive(thiserror::Error)]
pub enum CatalogoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
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
