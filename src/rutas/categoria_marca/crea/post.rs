//! src/rutas/categoria_marca/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea categoria

use crate::domain::categoria_marca::{
    CategoriaMarca,
    inserta as categoria_marca_inserta,
};
use actix_web::http::{StatusCode, header};
use actix_web::{post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    marca_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for CategoriaMarca {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        Ok( Self{ 
            categoria_id: 1, 
            marca_id: form_data.marca_id
        } )
    }
}

// extrae datos del categoria del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de categoria",
    skip(form, pool),
)]
#[post("/categoria/{id}/categoria_marca")]
pub async fn procesa(
    form: web::Form<FormData>, 
    path: web::Path<(i64,)>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CategoriaMarcaError> {
    //TODO añadir validacion de existencia de categoria_id
    let mut categoria_marca: CategoriaMarca = form.0.try_into()
        .map_err(CategoriaMarcaError::Validacion)?;
    let (categoria_id,) = path.into_inner();
    categoria_marca.categoria_id = categoria_id;
    let _cn = categoria_marca_inserta(&pool, &categoria_marca)
        .await
        .context("Error al insertar relacion categoria marca en la BD")?;
    let url_ver =  format!(
        "/categoria_marca/{}/{}", 
        categoria_id, categoria_marca.marca_id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de categorias
#[derive(thiserror::Error)]
pub enum CategoriaMarcaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for CategoriaMarcaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CategoriaMarcaError {
    fn status_code(&self) -> StatusCode {
        match self {
            CategoriaMarcaError::Validacion(_) => StatusCode::BAD_REQUEST,
            CategoriaMarcaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
