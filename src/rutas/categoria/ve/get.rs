//! src/rutas/categoria/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una categoria

use crate::layout;
use crate::domain::categoria::{
    Categoria, 
    CategoriaError,
    obtiene,
};
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
    button .form-submit #sublista type="button" { "Marcas" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
