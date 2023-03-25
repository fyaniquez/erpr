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

    let puesto_id = 1;
    let pagina = layout::form::crea(
        "Venta", &format!("/puesto/{}/ventas", puesto_id), 
        "ve.css", Some("venta/ve.js"), contenido(venta));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(venta: VentaVe) -> Markup { html! {
    @let fecha = venta.fecha.format("%d-%m-%Y %H:%M").to_string();
    @let total_parcial = venta.total - venta.descuento;
    .ve-label { strong { "Fecha: " } (fecha)}
    .ve-label { strong { "Pto.Vta.: " } (venta.puesto)}
    .ve-label { strong { "Cajero: " } (venta.usuario)}
    .ve-label { strong { "Cliente: " } (venta.cliente)}
    .ve-label { strong { "Total: " } (total_parcial)}
    .ve-label { strong { "Descuento: " } (venta.descuento)}
    .ve-label { strong { "Total a pagar: " } (venta.total)}
    button .form-submit #graba type="button" { "Graba" }
    button .form-submit #cancela type="button" { "Cancela" }
}}

