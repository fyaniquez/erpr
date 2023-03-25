//! src/rutas/compra/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una compra

use crate::layout;
use crate::domain::compra::{
    CompraError,
    CompraVe,
    obtiene_ve,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve compra", skip(pool))]
#[get("/compra/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CompraError> {
    let (id,) = path.into_inner();
    let compra = obtiene_ve(&pool, id).await
        .context("Error al leer compra")?;

    let sucursal_id = 1;
    let pagina = layout::form::crea(
        "Compra", &format!("/sucursal/{}/compras", sucursal_id), 
        "ve.css", Some("compra/ve.js"), contenido(compra));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(compra: CompraVe) -> Markup { html! {
    @let fecha = compra.fecha.format("%d-%m-%Y %H:%M").to_string();
    @let total_parcial = compra.total - compra.descuento;
    .ve-label { strong {"Fecha: " } (fecha)}
    .ve-label { strong {"Pto.Vta.: " } (compra.sucursal)}
    .ve-label { strong {"Cajero: " } (compra.usuario)}
    .ve-label { strong {"Cliente: " } (compra.cliente)}
    .ve-label { strong {"Total: " } (total_parcial)}
    .ve-label { strong {"Descuento: " } (compra.descuento)}
    .ve-label { strong {"Total a pagar: " } (compra.total)}
    button .form-submit #graba type="button" { "Graba" }
    button .form-submit #cancela type="button" { "Cancela" }
}}

