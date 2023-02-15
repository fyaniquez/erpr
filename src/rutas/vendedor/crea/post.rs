//! src/rutas/vendedor/crea/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de vendedor

use crate::domain::vendedor::{
    Nombre, 
    Cargo,
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
    cargo: String,
    contactos: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let cargo = Cargo::parse(form_data.cargo)?;
        Ok( Self{ 
            nombre, 
            cargo, 
            contactos: form_data.contactos,
        })
    }
}

// extrae datos del vendedor del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de vendedor",
    skip(form, pool),
    fields( 
        vendedor_cargo = %form.cargo,
        vendedor_nombre = %form.nombre,
    )
)]
#[post("/distribuidora/{distribuidora}/vendedor")]
pub async fn procesa(
    form: web::Form<FormData>, 
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, VendedorError> {

    let (distribuidora,) = path.into_inner();

    let vendedor = form.0.try_into().map_err(VendedorError::Validacion)?;

    let id = vendedor_inserta(&pool, &vendedor, distribuidora)
        .await
        .context("Error al insertar vendedor en la BD")?;

    let url_ver =  format!("/vendedor/{}", id);

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// errores considerados para alta de vendedores
#[derive(thiserror::Error)]
pub enum VendedorError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for VendedorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for VendedorError {
    fn status_code(&self) -> StatusCode {
        match self {
            VendedorError::Validacion(_) => StatusCode::BAD_REQUEST,
            VendedorError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un vendedor en la base de datos
#[tracing::instrument(name = "Inserta vendedor", skip(vendedor_nuevo, pool))]
pub async fn vendedor_inserta(
    pool: &PgPool,
    vendedor_nuevo: &Nuevo,
    distribuidora_id: i64,
) -> Result<i64, sqlx::Error> {

    let (id,) = sqlx::query_as(
        r#"INSERT INTO vendedores (distribuidora_id, cargo, 
        nombre, contactos) 
        VALUES ($1, $2, $3, $4) RETURNING id"#,
    )
    .bind(distribuidora_id)
    .bind(vendedor_nuevo.cargo.as_ref())
    .bind(vendedor_nuevo.nombre.as_ref())
    .bind(&vendedor_nuevo.contactos)
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
