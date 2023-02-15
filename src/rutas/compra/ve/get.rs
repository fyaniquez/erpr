//! src/rutas/compra/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una compra

use crate::layout;
use crate::domain::compra::{
    CompraError,
    CompraVe,
    obtiene_ve,
};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve compra", skip(pool))]
#[get("/compra/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CompraError> {
    let (id,) = path.into_inner();
    let compra = obtiene_ve(&pool, id).await
        .context("Error al leer compra")?;

    let sucursal_id = 1;
    let pagina = layout::form::crea(
        "Compra", &format!("/sucursal/{}/compras", sucursal_id), 
        "form.css", Some("compra/ve.js"), contenido(compra));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(compra: CompraVe) -> Markup { html! {
    @let fecha = compra.fecha.format("%d-%m-%Y %H:%M").to_string();
    @let total_parcial = compra.total - compra.descuento;
    .form-label {"Fecha:" }
    .form-field {(fecha)}
    .form-label {"Pto.Vta.:" }
    .form-field {(compra.sucursal)}
    .form-label {"Cajero:" }
    .form-field {(compra.usuario)}
    .form-label {"Cliente:" }
    .form-field {(compra.cliente)}
//    (items())
    .form-label {"Total:" }
    .form-field {(total_parcial)}
    .form-label {"Descuento:" }
    .form-field {(compra.descuento)}
    .form-label {"Total a pagar:" }
    .form-field {(compra.total)}
    button .form-submit #graba type="button" { "Graba" }
    button .form-submit #cancela type="button" { "Cancela" }
}}

