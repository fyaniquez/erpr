//! src/rutas/venta/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea venta
use crate::domain::login::{
    get_estado,
};
use crate::domain::venta::{
    Venta, 
    VentaError, 
    inserta as venta_inserta,
};
use crate::domain::vendido::{
    Vendidos,
    inserta_mul,
};
use actix_web::{post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct JsonData {
    venta: Venta,
    vendidos: Vendidos,
}

// extrae datos del venta del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de venta",
    skip(form, pool),
)]
#[post("/venta")]
pub async fn procesa(
    form: web::Json<JsonData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, VentaError> { 

    let estado = get_estado();

    let mut tx = pool.begin()
        .await
        .context("Error al iniciar transacción")?;

    let venta_id = venta_inserta(&mut tx, &form.venta, &estado)
        .await
        .context("Error al insertar venta en la BD")?;

    inserta_mul(&mut tx, venta_id, &form.vendidos)
        .await
        .context("Error al insertar vendido en la BD")?;

    tx.commit().await?;

    Ok(HttpResponse::Ok()
        .json(venta_id))

}
