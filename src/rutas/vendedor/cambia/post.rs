//! src/rutas/vendedor/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de vendedor

use crate::domain::vendedor::{
    Nombre, 
    Cargo,
};
use crate::domain::vendedor::{Vendedor, VendedorError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    cargo: String,
    contactos: String,
    distribuidora_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Vendedor {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let cargo = Cargo::parse(form_data.cargo)?;
        Ok( Self{ 
            id:None, 
            nombre: String::from(nombre.as_ref()), 
            cargo: String::from(cargo.as_ref()), 
            contactos: form_data.contactos,
            distribuidora_id: form_data.distribuidora_id,
        })
    }
}

// extrae datos del vendedor del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de vendedor",
    skip(form, pool),
    fields( 
        vendedor_cargo = %form.cargo,
        vendedor_nombre = %form.nombre,
    )
)]
#[post("/vendedor/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, VendedorError> {
    let (id,) = path.into_inner();
    let vendedor = form.0.try_into().map_err(VendedorError::Validacion)?;
    vendedor_actualiza(&pool, &vendedor, id)
        .await
        .context("Error al actualizar vendedor en la BD")?;
    let url_ver =  format!("/vendedor/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un vendedor en la base de datos
#[tracing::instrument(name = "modifica vendedor", skip(vendedor, pool))]
pub async fn vendedor_actualiza(
    pool: &PgPool,
    vendedor: &Vendedor,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"UPDATE vendedores SET nombre=$2, cargo=$3,
        contactos=$4 WHERE id=$1"#,
        id,
        &vendedor.nombre,
        &vendedor.cargo,
        &vendedor.contactos,
    )
    .execute(pool)
    .await?;
    Ok(())
}
