//! src/rutas/fabrica/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una fabrica

use crate::layout;
use crate::domain::fabrica::{Fabrica, FabricaError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve fabrica", skip(pool))]
#[get("/fabrica/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, FabricaError> {
    let (id,) = path.into_inner();
    let fabrica = obtiene(&pool, id).await
        .context("Error al leer fábrica")?;
    let pagina = layout::form::crea(
        "Fábrica", 
        format!("/pais/{}/fabricas", fabrica.pais_id).as_ref(), 
        "form.css", Some("fabrica/ve.js"), contenido(fabrica));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(fabrica: Fabrica) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(fabrica.nombre)}
    button .form-submit #sublista type="button" { "Productos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
// obtiene un fabrica de la base de datos
#[tracing::instrument(name = "ve fabrica", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Fabrica, sqlx::Error> {
    let fila: Fabrica = sqlx::query_as(
        "SELECT id, nombre, pais_id FROM fabricas WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
