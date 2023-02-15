//! src/rutas/distribuidora/ve/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un distribuidora

use crate::layout;
use crate::domain::distribuidora::{
    Distribuidora, 
    DistribuidoraError,
    obtiene,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve distribuidora", skip(pool))]
#[get("/distribuidora/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, DistribuidoraError> {
    let (id,) = path.into_inner();
    let distribuidora = obtiene(&pool, id).await
        .context("Error al leer distribuidora")?;

    let pagina = layout::form::crea(
        "Distribuidora", "/distribuidoras", 
        "form.css", Some("distribuidora/ve.js"), contenido(distribuidora));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(distribuidora: Distribuidora) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(distribuidora.nombre)}
    .form-label {"NIT:" }
    .form-field {(distribuidora.documento)}
    .form-label {"Descripción:" }
    .form-field {(distribuidora.descripcion)}
    .form-label {"Preventa:" }
    .form-field {(distribuidora.preventa)}
    .form-label {"Activa:" }
    .form-field {@if distribuidora.activa {"Sí"} @else {"No"}}
    button .form-submit #sublista type="button" { "Contactos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
