//! src/rutas/usuario/alta/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de usuario

use crate::domain::usuario::{
    Nombre, 
    Apellido, 
    Email, 
    Nuevo,
    Documento,
};
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    apellido: String,
    email: String,
    documento: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let email = Email::parse(form_data.email)?;
        let nombre = Nombre::parse(form_data.nombre)?;
        let apellido = Apellido::parse(form_data.apellido)?;
        let documento = Documento::parse(form_data.documento)?;
        Ok( Self{ email, nombre, apellido, documento })
    }
}

// extrae datos del usuario del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de usuario",
    skip(form, pool),
    fields( 
        usuario_email = %form.email,
        usuario_nombre = %form.nombre,
        usuario_documento = %form.documento,
    )
)]
#[post("/usuario/alta")]
pub async fn usuario_alta(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, UsuarioError> {
    let usuario = form.0.try_into().map_err(UsuarioError::Validacion)?;
    let _usuario_id = usuario_inserta(&pool, &usuario)
        .await
        .context("Error al insertar usuario en la BD")?;
    Ok(HttpResponse::Ok().finish())
}

// errores considerados para alta de usuarios
#[derive(thiserror::Error)]
pub enum UsuarioError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for UsuarioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for UsuarioError {
    fn status_code(&self) -> StatusCode {
        match self {
            UsuarioError::Validacion(_) => StatusCode::BAD_REQUEST,
            UsuarioError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// inserta un usuario en la base de datos
#[tracing::instrument(name = "Inserta usuario", skip(usuario_nuevo, pool))]
pub async fn usuario_inserta(
    pool: &PgPool,
    usuario_nuevo: &Nuevo,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query!(
r#"INSERT INTO usuarios (email, nombre, documento, estado)
   VALUES ($1, $2, $3, $4)
   RETURNING usuario_id"#,
        usuario_nuevo.email.as_ref(),
        usuario_nuevo.nombres(),
        usuario_nuevo.documento.as_ref(),
        "pendiente",
    )
    .fetch_one(pool)
    .await?;
    Ok(row.usuario_id)
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
