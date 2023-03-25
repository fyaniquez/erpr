//! src/rutas/marca/ve/get.rs
//! author: fyaniquez
//! date: 09/12/2022
//! purpose: muestra un marca

use crate::layout;
use crate::domain::marca::{Marca, MarcaError};
use actix_web::{get, web, HttpResponse};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Ve marca", skip(pool))]
#[get("/marca/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, MarcaError> {
    let (id,) = path.into_inner();
    let marca = obtiene(&pool, id)
        .await
        .context("Error al leer marca de la BD")?;

    let pagina = layout::form::crea(
        "Marca",
        &format!("/{}", "marcas"),
        "ve.css",
        Some("marca/ve.js"),
        contenido(marca),
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(marca: Marca) -> Markup {
    html! {
        .ve-label { strong {"Nombre: " } (marca.nombre)}
        button .form-submit #sublista type="button" { "Fábricas" }
        button .form-submit #cambia type="button" { "Cambiar" }
        button .form-submit #borra type="button" { "Borrar" }
    }
}

// modelo
// obtiene un marca de la base de datos
#[tracing::instrument(name = "ve marca", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) -> Result<Marca, sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre FROM marcas WHERE id=$1";
    let fila: Marca = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
