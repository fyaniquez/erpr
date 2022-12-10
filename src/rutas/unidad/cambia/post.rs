//! src/rutas/unidad/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de unidad

use crate::domain::{
    UnidadNombre, 
    UnidadSigla,
};
use crate::modelo::unidad::{Unidad, UnidadError};
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
impl TryFrom<FormData> for Unidad {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = UnidadNombre::parse(form_data.nombre)?;
        let sigla = UnidadSigla::parse(form_data.sigla)?;
        Ok( Self{ 
            id:None, 
            nombre: String::from(nombre.as_ref()), 
            sigla: String::from(sigla.as_ref()), })
    }
}

// extrae datos del unidad del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de unidad",
    skip(form, pool),
    fields( 
        unidad_sigla = %form.sigla,
        unidad_nombre = %form.nombre,
    )
)]
#[post("/unidad/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, UnidadError> {
    let (id,) = path.into_inner();
    let unidad = form.0.try_into().map_err(UnidadError::Validacion)?;
    unidad_actualiza(&pool, &unidad, id)
        .await
        .context("Error al actualizar unidad en la BD")?;
    let url_ver =  format!("/unidad/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un unidad en la base de datos
#[tracing::instrument(name = "modifica unidad", skip(unidad, pool))]
pub async fn unidad_actualiza(
    pool: &PgPool,
    unidad: &Unidad,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE unidades SET nombre=$1, sigla=$2 WHERE id=$3",
        &unidad.nombre,
        &unidad.sigla,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
