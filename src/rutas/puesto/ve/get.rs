//! src/rutas/puesto/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una puesto

use crate::layout;
use crate::domain::puesto::{Puesto, PuestoError, obtiene};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve puesto", skip(pool))]
#[get("/puesto/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PuestoError> {

    let (id,) = path.into_inner();

    let puesto = obtiene(&pool, id).await
        .context("Error al leer puesto")?;

    let pagina = layout::form::crea(
        "Puesto", 
        format!("/sucursal/{}/puestos", puesto.sucursal_id).as_ref(), 
        "ve.css", Some("puesto/ve.js"), contenido(puesto));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(puesto: Puesto) -> Markup { html! {
    .ve-label { strong {"Nombre:" } (puesto.nombre)}
    .ve-label { strong {"Sigla:" } (puesto.sigla)}
    .ve-label { strong {"Descripción:" } (puesto.descripcion)}
    button .form-submit #sublista type="button" { "Ventas" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
