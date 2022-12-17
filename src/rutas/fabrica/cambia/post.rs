//! src/rutas/fabrica/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de fabrica

use crate::domain::fabrica::{Fabrica, FabricaError};
use crate::domain::fabrica::Nombre;
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Fabrica {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        Ok( Self{ 
            id: None, 
            nombre: String::from(nombre.as_ref()), 
            pais_id: 0,
        })
    }
}

// extrae datos del fabrica del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de fabrica",
    skip(form, pool),
    fields( 
        fabrica_nombre = %form.nombre,
    )
)]
#[post("/fabrica/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, FabricaError> {
    let (id,) = path.into_inner();
    let fabrica = form.0.try_into().map_err(FabricaError::Validacion)?;
    fabrica_actualiza(&pool, &fabrica, id)
        .await
        .context("Error al actualizar fabrica en la BD")?;
    let url_ver =  format!("/fabrica/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un fabrica en la base de datos
#[tracing::instrument(name = "modifica fabrica", skip(fabrica, pool))]
pub async fn fabrica_actualiza(
    pool: &PgPool,
    fabrica: &Fabrica,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE fabricas SET nombre=$1 WHERE id=$2",
        &fabrica.nombre,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
