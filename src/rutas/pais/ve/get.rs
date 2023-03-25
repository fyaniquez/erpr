//! src/rutas/pais/ve/get.rs
//! author: fyaniquez
//! date: 09/12/2022
//! purpose: muestra un pais

use crate::layout;
use crate::domain::pais::{Pais, PaisError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve pais", skip(pool))]
#[get("/pais/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PaisError> {
    let (id,) = path.into_inner();
    let pais = obtiene(&pool, id).await
        .context("Error al leer paises de la BD")?;

    let pagina = layout::form::crea(
        "País", "/paises", 
        "ve.css", Some("pais/ve.js"), 
        contenido(pais));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(pais: Pais) -> Markup { html! {
    .ve-label { strong {"Nombre: " } (pais.nombre)}
    .ve-label { strong {"Sigla: " } (pais.sigla)}
    button .form-submit #sublista type="button" { "Fábricas" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
// obtiene un pais de la base de datos
#[tracing::instrument(name = "ve pais", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Pais, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, sigla FROM paises WHERE id=$1";
    let fila: Pais = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
