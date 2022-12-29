//! src/rutas/medio/ve/get.rs
//! author: fyaniquez
//! date: 09/12/2022
//! purpose: muestra un medio

use crate::layout;
use crate::domain::medio::{Medio, MedioError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve medio", skip(pool))]
#[get("/medio/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, MedioError> {
    let (id,) = path.into_inner();
    let medio = obtiene(&pool, id).await
        .context("Error al leer medios de la BD")?;

    let pagina = layout::form::crea(
        "País", "/medios", 
        "form.css", Some("medio/ve.js"), 
        contenido(medio));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(medio: Medio) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(medio.nombre)}
    .form-label {"Sigla:" }
    .form-field #sigla {(medio.sigla)}
    button .form-submit #sublista type="button" { "Fábricas" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
// obtiene un medio de la base de datos
#[tracing::instrument(name = "ve medio", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Medio, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, sigla FROM medios WHERE id=$1";
    let fila: Medio = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
