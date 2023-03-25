//! src/rutas/vendedor/ve/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un vendedor

use crate::layout;
use crate::domain::vendedor::{
    Vendedor, 
    VendedorError,
    obtiene,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve vendedor", skip(pool))]
#[get("/vendedor/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, VendedorError> {

    let (id,) = path.into_inner();

    let vendedor = obtiene(&pool, id).await
        .context("Error al leer vendedor")?;

    let pagina = layout::form::crea(
        "Vendedor", 
        &format!("/distribuidora/{}/vendedores", vendedor.distribuidora_id), 
        "ve.css", Some("vendedor/ve.js"), contenido(vendedor));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(vendedor: Vendedor) -> Markup { html! {
    .ve-label { strong { "Nombre:" } (vendedor.nombre)}
    .ve-label { strong { "Cargo:" } (vendedor.cargo)}
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

