//! src/rutas/marca/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de marca

use crate::domain::MarcaNombre;
use crate::modelo::marca::{Marca, MarcaError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Marca {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = MarcaNombre::parse(form_data.nombre)?;
        Ok( Self{ 
            id:None, 
            nombre: String::from(nombre.as_ref()), })
    }
}

// extrae datos del marca del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de marca",
    skip(form, pool),
    fields( 
        marca_nombre = %form.nombre,
    )
)]
#[post("/marca/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, MarcaError> {
    let (id,) = path.into_inner();
    let marca = form.0.try_into().map_err(MarcaError::Validacion)?;
    marca_actualiza(&pool, &marca, id)
        .await
        .context("Error al actualizar marca en la BD")?;
    let url_ver =  format!("/marca/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un marca en la base de datos
#[tracing::instrument(name = "modifica marca", skip(marca, pool))]
pub async fn marca_actualiza(
    pool: &PgPool,
    marca: &Marca,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE marcas SET nombre=$1 WHERE id=$2",
        &marca.nombre,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
