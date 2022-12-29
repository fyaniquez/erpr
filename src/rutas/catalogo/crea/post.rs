//! src/rutas/catalogo/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea catalogo

use crate::domain::catalogo::Nombre;
use crate::domain::catalogo::Nuevo;
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
            activo: false,
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
    let catalogo = form.0.try_into().map_err(CatalogoError::Validacion)?;

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

// inserta un catalogo en la base de datos
#[tracing::instrument(name = "Inserta catalogo", skip(catalogo_nuevo, pool))]
pub async fn catalogo_inserta(
    pool: &PgPool,
    catalogo_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO catalogos 
    (nombre, sucursal_id, fecha, activo) 
    VALUES ($1, $2, $3, $4, $5) RETURNING id"#)
    .bind(catalogo_nuevo.nombre.as_ref())
    .bind(catalogo_nuevo.sucursal_id)
    .bind(catalogo_nuevo.fecha)
    .bind(catalogo_nuevo.activo)
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
