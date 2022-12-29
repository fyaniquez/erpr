//! src/rutas/cliente/ve/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un cliente

use crate::layout;
use crate::domain::cliente::{Cliente, ClienteError};
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
        "form.css", Some("cliente/ve.js"), contenido(cliente));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(cliente: Cliente) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(cliente.nombre)}
    .form-label {"Documento:" }
    .form-field #documento {(cliente.documento)}
    button .form-submit #sublista type="button" { "Datos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
// obtiene un cliente de la base de datos
#[tracing::instrument(name = "ve cliente", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Cliente, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, documento FROM clientes WHERE id=$1";
    let fila: Cliente = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
