//! src/rutas/precio/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una precio

use crate::layout;
use crate::layout::ErrMsg;
use crate::domain::precio::{Precio, PrecioError, obtiene, obtiene_prod};
use actix_web::{get, web, HttpResponse, Responder};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// obtiene el precio de un producto para el catalogo en uso json
#[tracing::instrument(name="Ve precio json", skip(pool))]
#[get("/precio/{id}.json")]
pub async fn muestra_json(
    path: web::Path<i64>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let producto_id = path.into_inner();
    let catalogo_id = 2;

    match obtiene_prod(&pool, catalogo_id, producto_id).await {
        Ok(producto) => HttpResponse::Ok().json(producto),
        Err(err) => match err {
            sqlx::Error::Database(db_err) => 
                HttpResponse::InternalServerError().json( ErrMsg {
                    codigo: 500,
                    mensaje: format!("Error en la BD: {}", db_err) ,
                }),
            sqlx::Error::RowNotFound => 
                HttpResponse::NotFound().json( ErrMsg {
                    codigo: 404,
                    mensaje: format!("No existe producto: {}", producto_id)
                }),
            _ => HttpResponse::Conflict().json( ErrMsg {
                    codigo: 500,
                    mensaje: format!("Error: {}", err)
            }),
        }
    }
}

// controlador
#[tracing::instrument(name="Ve precio", skip(pool))]
#[get("/precio/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PrecioError> {
    let (id,) = path.into_inner();
    let precio = obtiene(&pool, id).await
        .context("Error al leer precio")?;
    let pagina = layout::form::crea(
        "Precio", 
        format!("/catalogo/{}/precios", precio.catalogo_id).as_ref(), 
        "ve.css", Some("precio/ve.js"), contenido(precio));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(precio: Precio) -> Markup { html! {
    @let prc = precio.precio as f32 / 100.0;
    .ve-label { strong { "Nombre: " } (precio.nombre) }
    .ve-label { strong { "Precio: " } (prc) }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
