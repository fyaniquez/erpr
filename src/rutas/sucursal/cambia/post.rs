//! src/rutas/sucursal/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de sucursal

use crate::domain::sucursal::Nombre;
use crate::domain::sucursal::{Sucursal, SucursalError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    catalogo_id: i64,
    empresa_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Sucursal {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        Ok( Self{ 
            id: None, 
            nombre: String::from(nombre.as_ref()), 
            catalogo_id: form_data.catalogo_id,
            empresa_id: form_data.empresa_id,
        })
    }
}

// extrae datos del sucursal del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de sucursal",
    skip(form, pool),
    fields( 
        sucursal_nombre = %form.nombre,
    )
)]
#[post("/sucursal/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, SucursalError> {

    let (id,) = path.into_inner();

    let sucursal = form.0.try_into().map_err(SucursalError::Validacion)?;

    sucursal_actualiza(&pool, &sucursal, id)
        .await
        .context("Error al actualizar sucursal en la BD")?;

    let url_ver =  format!("/sucursal/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un sucursal en la base de datos
#[tracing::instrument(name = "modifica sucursal", skip(sucursal, pool))]
pub async fn sucursal_actualiza(
    pool: &PgPool,
    sucursal: &Sucursal,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"UPDATE sucursales 
        SET nombre=$2, catalogo_id=$3, empresa_id=$4 
        WHERE id=$1"#,
        id,
        &sucursal.nombre,
        &sucursal.catalogo_id,
        &sucursal.empresa_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
