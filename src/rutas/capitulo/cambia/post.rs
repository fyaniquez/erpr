//! src/rutas/capitulo/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de capitulo

use crate::domain::{
    CapituloNombre, 
    CapituloDescripcion,
};
use crate::modelo::capitulo::{Capitulo, CapituloError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    descripcion: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Capitulo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = CapituloNombre::parse(form_data.nombre)?;
        let descripcion = CapituloDescripcion::parse(form_data.descripcion)?;
        Ok( Self{ 
            id:None, 
            nombre: String::from(nombre.as_ref()), 
            descripcion: String::from(descripcion.as_ref()), })
    }
}

// extrae datos del capitulo del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de capitulo",
    skip(form, pool),
    fields( 
        capitulo_descripcion = %form.descripcion,
        capitulo_nombre = %form.nombre,
    )
)]
#[post("/capitulo/{id}")]
pub async fn capitulo_cambia(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CapituloError> {
    let (id,) = path.into_inner();
    let capitulo = form.0.try_into().map_err(CapituloError::Validacion)?;
    capitulo_actualiza(&pool, &capitulo, id)
        .await
        .context("Error al actualizar capitulo en la BD")?;
    let url_ver =  format!("/capitulo/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un capitulo en la base de datos
#[tracing::instrument(name = "modifica capitulo", skip(capitulo, pool))]
pub async fn capitulo_actualiza(
    pool: &PgPool,
    capitulo: &Capitulo,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE capitulos SET nombre=$1, descripcion=$2 WHERE id=$3",
        &capitulo.nombre,
        &capitulo.descripcion,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
