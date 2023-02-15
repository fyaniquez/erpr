//! src/rutas/venta/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea venta

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

    let mut tx = pool.begin()
        .await
        .context("Error al iniciar transacción")?;
        
    let venta_id = venta_inserta(&mut tx, &form.venta)
        .await
        .context("Error al insertar venta en la BD")?;

    let fff = inserta_mul(&mut tx, venta_id, &form.vendidos)
        .await
        .context("Error al insertar vendido en la BD")?;

    tx.commit().await?;

    //let respuesta = serde_json::json!( {
        //"status": "exito", "venta_id": venta_id
    //});
    //let r = {
        //estado: "exito",
        //venta_id: venta_id,
    //};

    Ok(HttpResponse::Ok()
   //     .content_type(ContentType::json())
        .json(venta_id))

}
