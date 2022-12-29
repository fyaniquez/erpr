//! src/rutas/puesto/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea puesto

use crate::domain::puesto::Nombre;
use crate::domain::puesto::Nuevo;
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

// extrae datos del puesto del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de puesto",
    skip(form, pool),
    fields( 
        puesto_nombre = %form.nombre,
    )
)]
#[post("/puesto")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, PuestoError> {

    //TODO añadir validacion de existencia de empresa_id
    let puesto = form.0.try_into().map_err(PuestoError::Validacion)?;
    let id = puesto_inserta(&pool, &puesto)
        .await
        .context("Error al insertar puesto en la BD")?;

    let url_ver =  format!("/puesto/{}", id);

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de puestos
#[derive(thiserror::Error)]
pub enum PuestoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for PuestoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PuestoError {
    fn status_code(&self) -> StatusCode {
        match self {
            PuestoError::Validacion(_) => StatusCode::BAD_REQUEST,
            PuestoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un puesto en la base de datos
#[tracing::instrument(name = "Inserta puesto", skip(puesto_nuevo, pool))]
pub async fn puesto_inserta(
    pool: &PgPool,
    puesto_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO puestos 
        (nombre, sucursal_id) 
        VALUES ($1, $2) 
        RETURNING id"#,
    )
    .bind(puesto_nuevo.nombre.as_ref())
    .bind(puesto_nuevo.sucursal_id)
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
