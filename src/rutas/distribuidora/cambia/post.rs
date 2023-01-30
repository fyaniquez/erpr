//! src/rutas/distribuidora/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de distribuidora

use crate::domain::distribuidora::{
    Nombre, 
    Nit,
};
use crate::domain::distribuidora::{Distribuidora, DistribuidoraError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    nit: String,
    activa: bool,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Distribuidora {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let nit = Nit::parse(form_data.nit)?;
        Ok( Self{ 
            id:None, 
            nombre: String::from(nombre.as_ref()), 
            nit: String::from(nit.as_ref()), 
            activa: form_data.activa,
        })
    }
}

// extrae datos del distribuidora del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de distribuidora",
    skip(form, pool),
    fields( 
        distribuidora_nombre = %form.nombre,
        distribuidora_nit = %form.nit,
        distribuidora_activa = %form.activa,
    )
)]
#[post("/distribuidora/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, DistribuidoraError> {
    let (id,) = path.into_inner();
    let distribuidora = form.0.try_into().map_err(DistribuidoraError::Validacion)?;
    distribuidora_actualiza(&pool, &distribuidora, id)
        .await
        .context("Error al actualizar distribuidora en la BD")?;
    let url_ver =  format!("/distribuidora/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un distribuidora en la base de datos
#[tracing::instrument(name = "modifica distribuidora", skip(distribuidora, pool))]
pub async fn distribuidora_actualiza(
    pool: &PgPool,
    distribuidora: &Distribuidora,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE distribuidoras SET nombre=$1, nit=$2, activa=$3 WHERE id=$4",
        &distribuidora.nombre,
        &distribuidora.nit,
        distribuidora.activa,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
