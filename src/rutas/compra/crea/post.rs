//! src/rutas/compra/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea compra

use crate::domain::compra::{
    Compra, 
    CompraError, 
    inserta as compra_inserta,
};
use crate::domain::distribuidora::{
    Distribuidora,
    obtiene as distribuidora_obtiene,
};
use crate::domain::comprado::{
    Comprados,
    inserta_mul,
};
use actix_web::{post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct JsonData {
    compra: Compra,
    comprados: Comprados,
}

// extrae datos del compra del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de compra",
    skip(form, pool),
)]
#[post("/distribuidora/{id}/compra")]
pub async fn procesa(
    mut form: web::Json<JsonData>, 
    path: web::Path<(i64,)>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CompraError> { 

    let (distribuidora_id,) = path.into_inner();
    let distribuidora = distribuidora_obtiene(&pool, distribuidora_id)
        .await
        .context("Error al leer distribuidora");

    form.compra.distribuidora_id = Some(distribuidora_id);
    form.compra.sucursal_id = Some(1);
    form.compra.usuario_id = Some(1);

    let mut tx = pool.begin()
        .await
        .context("Error al iniciar transacción")?;
        
    let compra_id = compra_inserta(&mut tx, &form.compra)
        .await
        .context("Error al insertar compra en la BD")?;

    let _fff = inserta_mul(&mut tx, compra_id, &form.comprados)
        .await
        .context("Error al insertar vendido en la BD")?;

    tx.commit().await?;

    //let respuesta = serde_json::json!( {
        //"status": "exito", "compra_id": compra_id
    //});
    //let r = {
        //estado: "exito",
        //compra_id: compra_id,
    //};

    Ok(HttpResponse::Ok()
   //     .content_type(ContentType::json())
        .json(compra_id))

}
