//! src/rutas/catalogo/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una catalogo

use crate::layout;
use crate::domain::catalogo::{Catalogo, CatalogoError, obtiene};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve catalogo", skip(pool))]
#[get("/catalogo/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CatalogoError> {
    let (id,) = path.into_inner();
    let catalogo = obtiene(&pool, id).await
        .context("Error al leer catalogo")?;
    let pagina = layout::form::crea(
        "Catálogo", 
        format!("/sucursal/{}/catalogos", catalogo.sucursal_id).as_ref(), 
        "form.css", Some("catalogo/ve.js"), contenido(catalogo));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(catalogo: Catalogo) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(catalogo.nombre)}
    button .form-submit #sublista type="button" { "Precios" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
