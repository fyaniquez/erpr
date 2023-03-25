//! src/rutas/cliente/ve/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un cliente

use crate::layout;
use crate::domain::cliente::{
    Cliente, 
    ClienteError,
    obtiene,
    obtiene_documento
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve cliente", skip(pool))]
#[get("/cliente/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClienteError> {
    let (id,) = path.into_inner();
    let cliente = obtiene(&pool, id).await
        .context("Error al leer cliente")?;

    let pagina = layout::form::crea(
        "Cliente", "/clientes", 
        "ve.css", Some("cliente/ve.js"), contenido(cliente));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// controlador json
#[tracing::instrument(name="Ve cliente json", skip(pool))]
#[get("/cliente/{id}.{ext}")]
pub async fn muestra_json(
    path: web::Path<(i64, String)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClienteError> {
    let (id, _ext) = path.into_inner();
    let cliente = obtiene(&pool, id).await
        .context("Error al leer cliente")?;

   // a json
    let obj_json = serde_json::to_string(&cliente)
        .map_err(|err| ClienteError::Validacion(err.to_string()))
        .unwrap();

    // al browser
    Ok(HttpResponse::Ok().body(obj_json))
}

// controlador json por documento
#[tracing::instrument(name="Ve cliente por documento json", skip(pool))]
#[get("/cliente/{tipo}_{id}.{ext}")]
pub async fn muestra_documento_json(
    path: web::Path<(String, String, String)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClienteError> {
    let (_tipo, documento, _ext) = path.into_inner();
    let cliente = obtiene_documento(&pool, documento).await
        .context("Error al leer cliente por documento")?;

   // a json
    let obj_json = serde_json::to_string(&cliente)
        .map_err(|err| ClienteError::Validacion(err.to_string()))
        .unwrap();

    // al browser
    Ok(HttpResponse::Ok().body(obj_json))
}

// vista
fn contenido(cliente: Cliente) -> Markup { html! {
    .ve-label { strong {"Nombre: " } (cliente.nombre)}
    .ve-label { strong {"Documento: " } (cliente.documento)}
    button .form-submit #sublista type="button" { "Datos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

