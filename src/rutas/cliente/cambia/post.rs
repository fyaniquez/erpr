//! src/rutas/cliente/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de cliente

use crate::domain::cliente::{
    Nombre, 
    Documento,
};
use crate::domain::cliente::{Cliente, ClienteError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    documento: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Cliente {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let documento = Documento::parse(form_data.documento)?;
        Ok( Self{ 
            id:None, 
            nombre: String::from(nombre.as_ref()), 
            documento: String::from(documento.as_ref()), })
    }
}

// extrae datos del cliente del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de cliente",
    skip(form, pool),
    fields( 
        cliente_documento = %form.documento,
        cliente_nombre = %form.nombre,
    )
)]
#[post("/cliente/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, ClienteError> {
    let (id,) = path.into_inner();
    let cliente = form.0.try_into().map_err(ClienteError::Validacion)?;
    cliente_actualiza(&pool, &cliente, id)
        .await
        .context("Error al actualizar cliente en la BD")?;
    let url_ver =  format!("/cliente/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un cliente en la base de datos
#[tracing::instrument(name = "modifica cliente", skip(cliente, pool))]
pub async fn cliente_actualiza(
    pool: &PgPool,
    cliente: &Cliente,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE clientes SET nombre=$2, documento=$3 WHERE id=$1",
        id,
        &cliente.nombre,
        &cliente.documento,
    )
    .execute(pool)
    .await?;
    Ok(())
}
