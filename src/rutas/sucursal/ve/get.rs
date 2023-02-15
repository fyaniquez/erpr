//! src/rutas/sucursal/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una sucursal

use crate::layout;
use crate::domain::sucursal::{
    Sucursal, 
    SucursalError,
    obtiene,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve sucursal", skip(pool))]
#[get("/sucursal/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SucursalError> {

    let (id,) = path.into_inner();

    let sucursal = obtiene(&pool, id).await
        .context("Error al leer sucursal")?;

    let pagina = layout::form::crea(
        "Sucursal", 
        format!("/empresa/{}/sucursales", sucursal.empresa_id).as_ref(), 
        "form.css", Some("sucursal/ve.js"), contenido(sucursal));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(sucursal: Sucursal) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(sucursal.nombre)}
    button .form-submit #sublista type="button" { "Marcas" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
