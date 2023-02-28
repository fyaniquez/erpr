//! src/rutas/categoria/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea categoria

use crate::domain::categoria::{
    Nombre,
    Nuevo,
    inserta as categoria_inserta,
};
use actix_web::http::{StatusCode, header};
use actix_web::{post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

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
        let capitulo_id = 1;
        Ok( Self{ nombre, capitulo_id})
    }
}

// extrae datos del categoria del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de categoria",
    skip(form, pool),
    fields( 
        categoria_nombre = %form.nombre,
    )
)]
#[post("/capitulo/{id}/categoria")]
pub async fn procesa(
    form: web::Form<FormData>, 
    path: web::Path<(i64,)>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CategoriaError> {
    //TODO añadir validacion de existencia de capitulo_id
    let mut categoria: Nuevo = form.0.try_into()
        .map_err(CategoriaError::Validacion)?;
    let (capitulo_id,) = path.into_inner();
    categoria.capitulo_id = capitulo_id;
    let id = categoria_inserta(&pool, &categoria)
        .await
        .context("Error al insertar categoria en la BD")?;
    let url_ver =  format!("/categoria/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de categorias
#[derive(thiserror::Error)]
pub enum CategoriaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for CategoriaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CategoriaError {
    fn status_code(&self) -> StatusCode {
        match self {
            CategoriaError::Validacion(_) => StatusCode::BAD_REQUEST,
            CategoriaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
