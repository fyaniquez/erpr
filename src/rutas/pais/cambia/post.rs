//! src/rutas/pais/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de pais

use crate::domain::pais::{
    Nombre, 
    Sigla,
    actualiza,
};
use crate::domain::pais::{Pais, PaisError};
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
impl TryFrom<FormData> for Pais {
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

// extrae datos del pais del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de pais",
    skip(form, pool),
    fields( 
        pais_sigla = %form.sigla,
        pais_nombre = %form.nombre,
    )
)]
#[post("/pais/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, PaisError> {
    let (id,) = path.into_inner();
    let pais = form.0.try_into().map_err(PaisError::Validacion)?;
    actualiza(&pool, &pais, id)
        .await
        .context("Error al actualizar pais en la BD")?;
    let url_ver =  format!("/pais/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

