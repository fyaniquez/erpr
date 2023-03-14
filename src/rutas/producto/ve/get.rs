//! src/rutas/producto/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una producto

use crate::layout;
use crate::domain::producto::{
    obtiene,
    obtiene_ve,
    ProductoVe,
    ProductoError
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador json
#[tracing::instrument(name="Ve producto json", skip(pool))]
#[get("/producto/{id}.{ext}")]
pub async fn muestra_json(
    path: web::Path<(i64, String)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ProductoError> {

    let (id, _ext) = path.into_inner();

    let producto = obtiene(&pool, id).await
        .context("Error al leer producto")?;

    let obj_json = serde_json::to_string(&producto)
        .map_err(|err| ProductoError::Validacion(err.to_string()))
        .unwrap();

    Ok(HttpResponse::Ok().body(obj_json))
}

// controlador
#[tracing::instrument(name="Ve producto", skip(pool))]
#[get("/producto/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ProductoError> {
    let (id,) = path.into_inner();
    let producto = obtiene_ve(&pool, id).await
        .context("Error al leer producto")?;

    let pagina = layout::form::crea(
        "Producto", "/productos", 
        "ve.css", Some("producto/ve.js"), contenido(producto));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(producto: ProductoVe) -> Markup { html! {
    .ve-label {strong {"Nombre: " } (producto.nombre)}
    .ve-label {strong {"Características: " } (producto.caracteristicas)}
    .ve-label {strong {"Capítulo: "} (producto.capitulo) }
    .ve-label {strong {"Categoría: "} (producto.categoria)}
    .ve-label {strong {"Marca: "} (producto.marca)}
    .ve-label {strong {"Unidad: " } (producto.unidad)}
    .ve-label {strong {"Fábrica: " } (producto.fabrica)}
    .ve-label {strong {"Cod.Barras: " } (producto.barras)}
    .ve-label {strong {"Contenido: " } (producto.contenido)}
    .ve-label {strong {"Cantidad: " } (producto.cantidad)}
    .ve-label {strong {"Fraccionable: " } (producto.fraccionable)}
    .ve-label {strong {"Activo: " } (producto.activo)}
    button .form-submit #sublista type="button" { "Costos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

