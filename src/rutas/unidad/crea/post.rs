//! src/rutas/unidad/crea/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de unidad

use crate::domain::unidad::{
    Nombre, 
    Sigla,
    Nuevo,
    inserta as unidad_inserta,
};
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    sigla: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let sigla = Sigla::parse(form_data.sigla)?;
        Ok( Self{ nombre, sigla, })
    }
}

// extrae datos del unidad del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de unidad",
    skip(form, pool),
    fields( 
        unidad_sigla = %form.sigla,
        unidad_nombre = %form.nombre,
    )
)]
#[post("/unidad")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<sqlx::PgPool>
) -> Result<HttpResponse, UnidadError> {
    let unidad = form.0.try_into().map_err(UnidadError::Validacion)?;
    let id = unidad_inserta(&pool, &unidad)
        .await
        .context("Error al insertar unidad en la BD")?;
    let url_ver =  format!("/unidad/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de unidads
#[derive(thiserror::Error)]
pub enum UnidadError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for UnidadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for UnidadError {
    fn status_code(&self) -> StatusCode {
        match self {
            UnidadError::Validacion(_) => StatusCode::BAD_REQUEST,
            UnidadError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
