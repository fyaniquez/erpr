//! src/rutas/inventario/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una inventario

use crate::layout;
use crate::domain::inventario::{
    Inventario, 
    InventarioError,
    obtiene,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve inventario", skip(pool))]
#[get("/inventario/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, InventarioError> {

    let (id,) = path.into_inner();

    let inventario = obtiene(&pool, id).await
        .context("Error al leer inventario")?;

    let pagina = layout::form::crea(
        "Inventario", 
        "/inventarios",
        "ve.css", Some("inventario/ve.js"), contenido(inventario));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(inventario: Inventario) -> Markup { html! {
    .ve-label { strong {"Nombre: " } (inventario.nombre)}
    button .form-submit #sublista type="button" { "Productos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
