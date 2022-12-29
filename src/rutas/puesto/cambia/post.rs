//! src/rutas/puesto/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de puesto

use crate::domain::puesto::{
    Nombre,
    Sigla,
    Descripcion,
    Puesto, 
    PuestoError
};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    sigla: String,
    descripcion: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Puesto {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let sigla = Sigla::parse(form_data.sigla)?;
        let descripcion = Descripcion::parse(form_data.descripcion)?;
        Ok( Self { 
            id: None, 
            sucursal_id: 0,
            nombre: String::from(nombre.as_ref()),
            sigla: String::from(sigla.as_ref()),
            descripcion: String::from(descripcion.as_ref()),
            activo: false,
        })
    }
}

// extrae datos del puesto del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de puesto",
    skip(form, pool),
    fields( 
        puesto_nombre = %form.nombre,
        puesto_sigla = %form.sigla,
        puesto_descripcion = %form.descripcion,
    )
)]
#[post("/puesto/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, PuestoError> {

    let (id,) = path.into_inner();

    let puesto = form.0.try_into()
        .map_err(PuestoError::Validacion)?;

    puesto_actualiza(&pool, &puesto, id)
        .await
        .context("Error al actualizar puesto en la BD")?;

    let url_ver =  format!("/puesto/{}", id);

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un puesto en la base de datos
#[tracing::instrument(name = "modifica puesto", skip(puesto, pool))]
pub async fn puesto_actualiza(
    pool: &PgPool,
    puesto: &Puesto,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"UPDATE puestos 
        SET nombre=$2, sigla=$3, descripcion=$4
        WHERE id=$1"#,
        id,
        &puesto.nombre,
        &puesto.sigla,
        &puesto.descripcion,
    )
    .execute(pool)
    .await?;
    Ok(())
}
