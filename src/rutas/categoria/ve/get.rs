//! src/rutas/categoria/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una categoria

use crate::layout;
use crate::modelo::categoria::{Categoria, CategoriaError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve categoria", skip(pool))]
#[get("/categoria/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaError> {
    let (id,) = path.into_inner();
    let categoria = obtiene(&pool, id).await
        .context("Error al leer categoria")?;
    let pagina = layout::form::crea(
        "Categoría", 
        format!("/capitulo/{}/categorias", categoria.capitulo_id).as_ref(), 
        "form.css", Some("categoria/ve.js"), contenido(categoria));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(categoria: Categoria) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(categoria.nombre)}
    button .form-submit #hijos type="button" { "Marcas" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
// obtiene un categoria de la base de datos
#[tracing::instrument(name = "ve categoria", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Categoria, sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre, capitulo_id FROM categorias WHERE id=$1";
    let fila: Categoria = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
