//! src/rutas/inventariado/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una inventariado

use crate::layout;
use crate::domain::inventariado::{Inventariado, InventariadoError, obtiene};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve inventariado", skip(pool))]
#[get("/inventariado/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, InventariadoError> {
    let (id,) = path.into_inner();
    let inventariado = obtiene(&pool, id).await
        .context("Error al leer inventariado")?;
    let pagina = layout::form::crea(
        "Inventariado", 
        format!("/inventario/{}/inventariados", inventariado.inventario_id).as_ref(), 
        "ve.css", Some("inventariado/ve.js"), contenido(inventariado));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(inventariado: Inventariado) -> Markup { html! {
    @let vencimiento = inventariado.vencimiento
        .map(|date| date.format("%d/%m/%Y").to_string())
        .unwrap_or_default();
    @let cantidad = inventariado.cantidad as f32 / 100.0;
    .ve-label { strong { "Nombre: " } (inventariado.nombre)}
    .ve-label { strong { "Cantidad: "} (cantidad)}
    .ve-label { strong { "Vencimiento: "} (vencimiento)}
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
