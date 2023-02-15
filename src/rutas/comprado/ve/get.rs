//! src/rutas/comprado/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una comprado

use crate::layout;
use crate::domain::comprado::{
    CompradoVe, 
    CompradoError,
    obtiene_ve,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve comprado", skip(pool))]
#[get("/comprado/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CompradoError> {
    let (id,) = path.into_inner();
    let comprado = obtiene_ve(&pool, id).await
        .context("Error al leer comprado")?;
    let pagina = layout::form::crea(
        "Categoría", 
        format!("/compra/{}/comprados", comprado.compra_id).as_ref(), 
        "form.css", Some("comprado/ve.js"), contenido(comprado));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(comprado: CompradoVe) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #producto {(comprado.producto)}
    .form-field #cantidad {(comprado.cantidad)}
    .form-field #costo {(comprado.costo)}
    .form-field #descuento {(comprado.descuento)}
    .form-field #total {(comprado.total)}
    button .form-submit #sublista type="button" { "Marcas" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}
