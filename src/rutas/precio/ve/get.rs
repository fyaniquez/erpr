//! src/rutas/precio/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una precio

use crate::layout;
use crate::domain::precio::{Precio, PrecioError, obtiene};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador json
#[tracing::instrument(name="Ve precio json", skip(pool))]
#[get("/precio/{id}.{ext}")]
pub async fn muestra_json(
    path: web::Path<(i64, String)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PrecioError> {
    let (id, _ext) = path.into_inner();
    let precio = obtiene(&pool, id).await
        .context("Error al leer precio")?;

    // a json
    let obj_json = serde_json::to_string(&precio)
        .map_err(|err| PrecioError::Validacion(err.to_string()))
        .unwrap();

    // al browser
    Ok(HttpResponse::Ok().body(obj_json))
}

// controlador
#[tracing::instrument(name="Ve precio", skip(pool))]
#[get("/precio/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PrecioError> {
    let (id,) = path.into_inner();
    let precio = obtiene(&pool, id).await
        .context("Error al leer precio")?;
    let pagina = layout::form::crea(
        "Precio", 
        format!("/catalogo/{}/precios", precio.catalogo_id).as_ref(), 
        "ve.css", Some("precio/ve.js"), contenido(precio));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(precio: Precio) -> Markup { html! {
    @let prc = precio.precio as f32 / 100.0;
    .ve-label { strong { "Nombre: " } (precio.nombre) }
    .ve-label { strong { "Precio: " } (prc) }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
