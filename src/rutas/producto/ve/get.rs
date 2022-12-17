//! src/rutas/producto/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una producto

use crate::layout;
use crate::domain::producto::{ProductoError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve producto", skip(pool))]
#[get("/producto/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ProductoError> {
    let (id,) = path.into_inner();
    let producto = obtiene(&pool, id).await
        .context("Error al leer producto")?;

    let pagina = layout::form::crea(
        "Producto", "/productos", 
        "form.css", Some("producto/ve.js"), contenido(producto));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(producto: ProductoVe) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(producto.nombre)}
    .form-label {"Características:" }
    .form-field {(producto.caracteristicas)}
    .form-label {"Capítulo:" }
    .form-field {(producto.capitulo)}
    .form-label {"Categoría:" }
    .form-field {(producto.categoria)}
    .form-label {"Marca:" }
    .form-field {(producto.marca)}
    .form-label {"Unidad:" }
    .form-field {(producto.unidad)}
    .form-label {"Fábrica:" }
    .form-field {(producto.fabrica)}
    .form-label {"Cod.Barras:" }
    .form-field {(producto.barras)}
    .form-label {"Contenido:" }
    .form-field {(producto.contenido)}
    .form-label {"Cantidad:" }
    .form-field {(producto.cantidad)}
    .form-label {"Fraccionable:" }
    .form-field {(producto.fraccionable)}
    .form-label {"Activo:" }
    .form-field {(producto.activo)}
    button .form-submit #sublista type="button" { "Costos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct ProductoVe {
    pub id: i64,
    pub nombre: String,
    pub caracteristicas: String,
    pub capitulo: String,
    pub categoria: String,
    pub marca: String,
    pub unidad: String,
    pub fabrica: String,
    pub contenido: String,
    pub cantidad: i32,
    pub fraccionable: String,
    pub barras: String,
    pub activo: String,
}

// obtiene un producto de la base de datos
#[tracing::instrument(name = "ve producto", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<ProductoVe, sqlx::Error> {
    const SELECT: &str = 
        r#"SELECT p.id, p.nombre, p.caracteristicas, cap.nombre as capitulo,
            cat.nombre as categoria, mar.nombre as marca, 
            uni.nombre as unidad, fab.nombre as fabrica, p.barras,
            p.contenido, p.cantidad, 
            CASE WHEN p.fraccionable THEN 'Si' ELSE 'No' END as fraccionable,  
            CASE WHEN p.activo THEN 'Si' ELSE 'No' END as activo
        FROM productos p 
        INNER JOIN categorias as cat ON p.categoria_id = cat.id
        INNER JOIN marcas as mar ON p.marca_id = mar.id
        INNER JOIN unidades as uni ON p.unidad_id = uni.id
        INNER JOIN fabricas as fab ON p.fabrica_id = fab.id
        INNER JOIN capitulos as cap ON cat.capitulo_id = cap.id
        WHERE p.id=$1"#;
    let fila: ProductoVe = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
