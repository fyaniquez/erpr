//! src/rutas/categoria_marca/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una categoria_marca

use crate::layout;
use crate::domain::categoria_marca::{
    CategoriaMarcaNombres, 
    CategoriaMarcaError,
    obtiene,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve categoria_marca", skip(pool))]
#[get("/categoria_marca/{categoria_id}/{marca_id}")]
pub async fn muestra(
    path: web::Path<(i64, i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaMarcaError> {
    let (categoria_id, marca_id,) = path.into_inner();
    let categoria_marca = obtiene(&pool, categoria_id, marca_id).await
        .context("Error al leer categoria_marca")?;
    let titulo = format!("Categoría: {} - Marca", categoria_marca.categoria_nombre);
    let pagina = layout::form::crea(
        &titulo,
        format!(
            "/categoria/{}/categorias_marcas", 
            categoria_marca.categoria_id).as_ref(), 
        "form.css", Some("categoria_marca/ve.js"), contenido(categoria_marca));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(categoria_marca: CategoriaMarcaNombres) -> Markup { html! {
    .form-label {"Categoria: " (categoria_marca.categoria_nombre)}
    .form-label {"Marca: " (categoria_marca.marca_nombre)}
    button .form-submit #sublista type="button" { "Productos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
