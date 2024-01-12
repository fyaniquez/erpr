//! src/rutas/compra/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea compra

use serde_json;
use crate::layout::ErrMsg;
use crate::domain::login::{
    get_estado,
};
use crate::domain::compra::{
    Compra, 
    CompraError, 
    inserta as compra_inserta,
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
#[post("/compra")]
pub async fn procesa(
    form: web::Json<JsonData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CompraError> { 

    // obtiene el estado del usuario en el servidor web
    let estado = get_estado();

    let mut tx = pool.begin()
        .await
        .context(serde_json::to_string(&ErrMsg {
            codigo: 401,
            mensaje:"Error al iniciar transacción".to_string(),
        }).expect("falla en serialización"))?;
        
    let compra_id = compra_inserta(&mut tx, &form.compra, &estado)
        .await
        .context("Error al insertar compra en la BD")?;

    let _fff = inserta_mul(&mut tx, compra_id, &form.comprados)
        .await
        .context(serde_json::to_string(&ErrMsg {
            codigo: 401,
            mensaje:"Error al insertar comprado en la DB".to_string(),
        }).expect("falla en serialización"))?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().json(compra_id))

}
