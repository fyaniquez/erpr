//! src/rutas/venta/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea venta

use crate::domain::venta::{
    VentaError, 
    Nuevo,
};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    puesto_id: i64,
    usuario_id: i64,
    cliente_id: i64,
    descuento: i32,
    total: i32,
    medio_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        // todo simplificar las siguientes 2 lineas en una sola
        Ok( Self { 
            descuento: form_data.descuento,
            total: form_data.total,
            cliente_id: form_data.cliente_id,
            puesto_id: form_data.puesto_id,
            usuario_id: form_data.usuario_id,
            medio_id: form_data.medio_id,
        })
    }
}

// extrae datos del venta del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de venta",
    skip(form, pool),
)]
#[post("/venta")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, VentaError> {
    //TODO añadir validacion de existencia de capitulo_id
    let venta = form.0.try_into().map_err(VentaError::Validacion)?;
    let id = venta_inserta(&pool, &venta)
        .await
        .context("Error al insertar venta en la BD")?;
    let url_ver =  format!("/venta/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un venta en la base de datos
#[tracing::instrument(name = "Inserta venta", skip(venta_nuevo, pool))]
pub async fn venta_inserta(
    pool: &PgPool,
    venta_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO ventas 
        (total, descuento, cliente_id, puesto_id, usuario_id, medio_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id"#,
    )
    .bind(venta_nuevo.total)
    .bind(venta_nuevo.descuento)
    .bind(venta_nuevo.cliente_id)
    .bind(venta_nuevo.puesto_id)
    .bind(venta_nuevo.usuario_id)
    .bind(venta_nuevo.medio_id)
    .fetch_one(pool)
    .await?;
    Ok(id)
}
