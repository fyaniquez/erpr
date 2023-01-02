//! src/rutas/venta/ve/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra una venta

use crate::layout;
use crate::domain::venta::{VentaError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;
use chrono::NaiveDateTime;

// controlador
#[tracing::instrument(name="Ve venta", skip(pool))]
#[get("/venta/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, VentaError> {
    let (id,) = path.into_inner();
    let venta = obtiene(&pool, id).await
        .context("Error al leer venta")?;

    let pagina = layout::form::crea(
        "Venta", "/ventas", 
        "form.css", Some("venta/ve.js"), contenido(venta));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(venta: VentaVe) -> Markup { html! {
    @let fecha = venta.fecha.format("%d-%m-%Y %H:%M").to_string();
    @let total_parcial = venta.total - venta.descuento;
    .form-label {"Fecha:" }
    .form-field {(fecha)}
    .form-label {"Pto.Vta.:" }
    .form-field {(venta.puesto)}
    .form-label {"Cajero:" }
    .form-field {(venta.usuario)}
    .form-label {"Cliente:" }
    .form-field {(venta.cliente)}
//    (items())
    .form-label {"Total:" }
    .form-field {(total_parcial)}
    .form-label {"Descuento:" }
    .form-field {(venta.descuento)}
    .form-label {"Total a pagar:" }
    .form-field {(venta.total)}
    button .form-submit #graba type="button" { "Graba" }
    button .form-submit #cancela type="button" { "Cancela" }
}}

// modelo
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct VentaVe {
    pub id: i64,
    pub fecha: NaiveDateTime,
    pub total: i32,
    pub descuento: i32,
    pub puesto: String,
    pub usuario: String,
    pub cliente: String,
    pub medio: String,
}

// obtiene un venta de la base de datos
#[tracing::instrument(name = "ve venta", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<VentaVe, sqlx::Error> {
    const SELECT: &str = 
        r#"SELECT p.id, p.nombre, p.caracteristicas, cap.nombre as capitulo,
            cat.nombre as categoria, mar.nombre as marca, 
            uni.nombre as unidad, fab.nombre as fabrica, p.barras,
            p.contenido, p.cantidad, 
            CASE WHEN p.fraccionable THEN 'Si' ELSE 'No' END as fraccionable,  
            CASE WHEN p.activo THEN 'Si' ELSE 'No' END as activo
        FROM ventas p 
        INNER JOIN categorias as cat ON p.categoria_id = cat.id
        INNER JOIN marcas as mar ON p.marca_id = mar.id
        INNER JOIN unidades as uni ON p.unidad_id = uni.id
        INNER JOIN fabricas as fab ON p.fabrica_id = fab.id
        INNER JOIN capitulos as cap ON cat.capitulo_id = cap.id
        WHERE p.id=$1"#;
    let fila: VentaVe = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
