//! src/rutas/capitulo/ve/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un capitulo

use crate::layout;
use crate::modelo::capitulo::{Capitulo, CapituloError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve capitulo", skip(pool))]
#[get("/capitulo/{id}")]
pub async fn capitulo_ve(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CapituloError> {
    let (id,) = path.into_inner();
    let capitulo = obtiene(&pool, id).await
        .context("Error al leer capitulo")?;

    let pagina = layout::form::crea(
        "Capítulo", "/capitulos", 
        "form.css", Some("capitulo/ve.js"), contenido(capitulo));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(capitulo: Capitulo) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(capitulo.nombre)}
    .form-label {"Descripción:" }
    .form-field #descripcion {(capitulo.descripcion)}
    button .form-submit #hijos type="button" { "Categorias" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
// obtiene un capitulo de la base de datos
#[tracing::instrument(name = "ve capitulo", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Capitulo, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, descripcion FROM capitulos WHERE id=$1";
    let fila: Capitulo = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
