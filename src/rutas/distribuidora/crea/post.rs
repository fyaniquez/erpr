//! src/rutas/distribuidora/crea/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de distribuidora

use crate::domain::distribuidora::{
    Nombre, 
    Documento,
    Nuevo,
};
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    documento: String,
    descripcion: String,
    preventa: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let documento = Documento::parse(form_data.documento)?;
        Ok( Self{ 
            nombre, 
            documento, 
            descripcion: form_data.descripcion,
            preventa: form_data.preventa,
        })
    }
}

// extrae datos del distribuidora del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de distribuidora",
    skip(form, pool),
    fields( 
        distribuidora_nombre = %form.nombre,
        distribuidora_documento = %form.documento,
    )
)]
#[post("/distribuidora")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, DistribuidoraError> {

    let distribuidora: Nuevo = form.0.try_into()
        .map_err(DistribuidoraError::Validacion)?;

// TODO leer la empresa del estado del site
    let empresa_id = 1;

    let id = distribuidora_inserta(&pool, &distribuidora, empresa_id)
        .await
        .context("Error al insertar distribuidora en la BD")?;

    let url_ver =  format!("/distribuidora/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de distribuidoras
#[derive(thiserror::Error)]
pub enum DistribuidoraError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for DistribuidoraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for DistribuidoraError {
    fn status_code(&self) -> StatusCode {
        match self {
            DistribuidoraError::Validacion(_) => StatusCode::BAD_REQUEST,
            DistribuidoraError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un distribuidora en la base de datos
#[tracing::instrument(name = "Inserta distribuidora", skip(distribuidora_nuevo, pool))]
pub async fn distribuidora_inserta(
    pool: &PgPool,
    distribuidora_nuevo: &Nuevo,
    empresa_id: i64,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO distribuidoras (empresa_id, nombre, documento, 
        descripcion, preventa, activa) 
        VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"#,
    )
    .bind(empresa_id)
    .bind(distribuidora_nuevo.nombre.as_ref())
    .bind(distribuidora_nuevo.documento.as_ref())
    .bind(&distribuidora_nuevo.descripcion)
    .bind(&distribuidora_nuevo.preventa)
    .bind(true)
    .fetch_one(pool)
    .await?;
    Ok(id)
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
