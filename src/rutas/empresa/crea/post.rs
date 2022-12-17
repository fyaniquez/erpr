//! src/rutas/empresa/crea/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de empresa

use crate::domain::empresa::{
    Nombre, 
    Nit,
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
    nit: String,
    activa: bool,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let nit = Nit::parse(form_data.nit)?;
        Ok( Self{ 
            nombre, 
            nit, 
            activa: form_data.activa,
        })
    }
}

// extrae datos del empresa del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de empresa",
    skip(form, pool),
    fields( 
        empresa_nombre = %form.nombre,
        empresa_nit = %form.nit,
        empresa_activa = %form.activa,
    )
)]
#[post("/empresa")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, EmpresaError> {
    let empresa = form.0.try_into().map_err(EmpresaError::Validacion)?;
    let id = empresa_inserta(&pool, &empresa)
        .await
        .context("Error al insertar empresa en la BD")?;
    let url_ver =  format!("/empresa/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de empresas
#[derive(thiserror::Error)]
pub enum EmpresaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for EmpresaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for EmpresaError {
    fn status_code(&self) -> StatusCode {
        match self {
            EmpresaError::Validacion(_) => StatusCode::BAD_REQUEST,
            EmpresaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un empresa en la base de datos
#[tracing::instrument(name = "Inserta empresa", skip(empresa_nuevo, pool))]
pub async fn empresa_inserta(
    pool: &PgPool,
    empresa_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO empresas (nombre, nit, activa) 
        VALUES ($1, $2, $3) RETURNING id"#,
    )
    .bind(empresa_nuevo.nombre.as_ref())
    .bind(empresa_nuevo.nit.as_ref())
    .bind(empresa_nuevo.activa)
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
