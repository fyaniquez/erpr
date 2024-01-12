//! src/rutas/vendido/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una vendido

use crate::layout;
use crate::domain::vendido::{
    VendidoVe, 
    VendidoError,
    obtiene_ve,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve vendido", skip(pool))]
#[get("/vendido/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, VendidoError> {
    let (id,) = path.into_inner();
    let vendido = obtiene_ve(&pool, id).await
        .context("Error al leer vendido")?;
    let pagina = layout::form::crea(
        "Categoría", 
        format!("/venta/{}/vendidos", vendido.venta_id).as_ref(), 
        "ve.css", Some("vendido/ve.js"), contenido(vendido));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(vendido: VendidoVe) -> Markup { html! {
    .ve-label { strong { "Nombre: " }}
    .ve-label #producto {(vendido.producto)}
    .ve-label #cantidad {(vendido.cantidad)}
    .ve-label #precio {(vendido.precio)}
    .ve-label #descuento {(vendido.descuento)}
    .ve-label #subtotal {(vendido.subtotal)}
    button .form-submit #sublista type="button" { "Marcas" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
