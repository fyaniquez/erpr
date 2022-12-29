//! src/rutas/medio/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de medio

use crate::domain::medio::{
    Nombre, 
    Sigla,
};
use crate::domain::medio::{Medio, MedioError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    sigla: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Medio {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let sigla = Sigla::parse(form_data.sigla)?;
        Ok( Self{ 
            id:None, 
            nombre: String::from(nombre.as_ref()), 
            sigla: String::from(sigla.as_ref()), })
    }
}

// extrae datos del medio del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de medio",
    skip(form, pool),
    fields( 
        medio_sigla = %form.sigla,
        medio_nombre = %form.nombre,
    )
)]
#[post("/medio/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, MedioError> {
    let (id,) = path.into_inner();
    let medio = form.0.try_into().map_err(MedioError::Validacion)?;
    medio_actualiza(&pool, &medio, id)
        .await
        .context("Error al actualizar medio en la BD")?;
    let url_ver =  format!("/medio/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un medio en la base de datos
#[tracing::instrument(name = "modifica medio", skip(medio, pool))]
pub async fn medio_actualiza(
    pool: &PgPool,
    medio: &Medio,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE medios SET nombre=$1, sigla=$2 WHERE id=$3",
        &medio.nombre,
        &medio.sigla,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
