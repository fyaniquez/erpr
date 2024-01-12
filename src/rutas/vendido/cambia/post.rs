//! src/rutas/vendido/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de vendido

use crate::domain::vendido::{Vendido, VendidoError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    venta_id: i64,
    producto_id: i64,
    cantidad: i32,
    precio: i32,
    subtotal: i32,
    descuento: i32,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Vendido {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        Ok( Self{ 
            id: None, 
            venta_id: Some(form_data.venta_id),
            producto_id: form_data.producto_id,
            cantidad: form_data.cantidad,
            precio: form_data.precio,
            subtotal: form_data.subtotal,
            descuento: form_data.descuento,
        })
    }
}

// extrae datos del vendido del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de vendido",
    skip(form, pool),
)]
#[post("/vendido/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, VendidoError> {
    let (id,) = path.into_inner();
    let vendido = form.0.try_into().map_err(VendidoError::Validacion)?;
    vendido_actualiza(&pool, &vendido, id)
        .await
        .context("Error al actualizar vendido en la BD")?;
    let url_ver =  format!("/vendido/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un vendido en la base de datos
#[tracing::instrument(name = "modifica vendido", skip(vendido, pool))]
pub async fn vendido_actualiza(
    pool: &PgPool,
    vendido: &Vendido,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        r#"UPDATE vendidos 
        SET cantidad=$2, precio=$3, descuento=$4, subtotal=$5
        WHERE id=$1"#)
    .bind(id)
    .bind(&vendido.cantidad)
    .bind(&vendido.precio)
    .bind(&vendido.descuento)
    .bind(&vendido.subtotal)
    .execute(pool)
    .await?;
    Ok(())
}
