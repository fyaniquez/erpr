//! src/rutas/sucursal/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea sucursal

use crate::domain::sucursal::Nombre;
use crate::domain::sucursal::Nuevo;
use actix_web::http::StatusCode;
use actix_web::{http::header, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    empresa_id: i64,
    catalogo_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        Ok( Self{ 
            nombre, 
            empresa_id: form_data.empresa_id,
            catalogo_id: form_data.catalogo_id,
        })
    }
}

// extrae datos del sucursal del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de sucursal",
    skip(form, pool),
    fields( 
        sucursal_nombre = %form.nombre,
    )
)]
#[post("/sucursal")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, SucursalError> {
    //TODO añadir validacion de existencia de empresa_id
    let sucursal = form.0.try_into().map_err(SucursalError::Validacion)?;
    let id = sucursal_inserta(&pool, &sucursal)
        .await
        .context("Error al insertar sucursal en la BD")?;
    let url_ver =  format!("/sucursal/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de sucursales
#[derive(thiserror::Error)]
pub enum SucursalError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for SucursalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SucursalError {
    fn status_code(&self) -> StatusCode {
        match self {
            SucursalError::Validacion(_) => StatusCode::BAD_REQUEST,
            SucursalError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un sucursal en la base de datos
#[tracing::instrument(name = "Inserta sucursal", skip(sucursal_nuevo, pool))]
pub async fn sucursal_inserta(
    pool: &PgPool,
    sucursal_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO sucursales 
        (nombre, empresa_id, catalogo_id) 
        VALUES ($1, $2, $3) 
        RETURNING id"#,
    )
    .bind(sucursal_nuevo.nombre.as_ref())
    .bind(sucursal_nuevo.empresa_id)
    .bind(sucursal_nuevo.catalogo_id)
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
