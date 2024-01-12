//! src/rutas/venta/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una venta

use crate::layout;
use crate::domain::venta::{
    VentaError,
    VentaVe,
    obtiene_ve,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve venta", skip(pool))]
#[get("/venta/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, VentaError> {
    let (id,) = path.into_inner();
    let venta = obtiene_ve(&pool, id).await
        .context("Error al leer venta")?;

    let pagina = layout::form::crea(
        "Venta", "/venta", 
        "ve.css", Some("venta/ve.js"), contenido(venta));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(venta: VentaVe) -> Markup { html! {
    @let fecha = venta.fecha.format("%d-%m-%Y %H:%M").to_string();
    @let des = venta.descuento as f32 / 100.0;
    @let subtot = venta.subtotal as f32 / 100.0;
    @let total = subtot - des;
    .ve-label { strong { "Fecha: " } (fecha)}
    .ve-label { strong { "Pto.Vta.: " } (venta.puesto)}
    .ve-label { strong { "Cajero: " } (venta.usuario)}
    .ve-label { strong { "Cliente: " } (venta.cliente)}
    .ve-label { strong { "SubTotal: " } (subtot)}
    .ve-label { strong { "Descuento: " } (des)}
    .ve-label { strong { "Total a pagar: " } (total)}
    button .form-submit #graba type="button" { "Graba" }
    button .form-submit #cancela type="button" { "Cancela" }
}}

