//! src/rutas/cliente/crea/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de cliente

use crate::domain::cliente::{
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
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let documento = Documento::parse(form_data.documento)?;
        Ok( Self{ nombre, documento, })
    }
}

// extrae datos del cliente del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de cliente",
    skip(form, pool),
    fields( 
        cliente_documento = %form.documento,
        cliente_nombre = %form.nombre,
    )
)]
#[post("/cliente")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, ClienteError> {

    let cliente = form.0.try_into().map_err(ClienteError::Validacion)?;

    let id = cliente_inserta(&pool, &cliente)
        .await
        .context("Error al insertar cliente en la BD")?;

    let url_ver =  format!("/cliente/{}", id);

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de clientes
#[derive(thiserror::Error)]
pub enum ClienteError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for ClienteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ClienteError {
    fn status_code(&self) -> StatusCode {
        match self {
            ClienteError::Validacion(_) => StatusCode::BAD_REQUEST,
            ClienteError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un cliente en la base de datos
#[tracing::instrument(name = "Inserta cliente", skip(cliente_nuevo, pool))]
pub async fn cliente_inserta(
    pool: &PgPool,
    cliente_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {

    let (id,) = sqlx::query_as(
        r#"INSERT INTO clientes (nombre, documento) 
        VALUES ($1, $2) RETURNING id"#,
    )
    .bind(cliente_nuevo.nombre.as_ref())
    .bind(cliente_nuevo.documento.as_ref())
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
