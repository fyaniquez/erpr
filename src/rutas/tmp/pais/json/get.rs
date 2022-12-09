//! src/rutas/pais/json/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un pais en formato json

use crate::modelo::pais::{Pais, PaisError};
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use anyhow::Context;

#[tracing::instrument(name="pais json", skip(pool))]
#[get("/pais/{id}.json")]
pub async fn pais_json(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PaisError> {
    let (id,) = path.into_inner();
    let pais = obtiene(&pool, id).await
        .context("Error al leer pais")?;
    Ok(HttpResponse::Ok().body(serde_json::to_string(&pais).unwrap()))
}

pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Pais, sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre, descripcion FROM paiss WHERE id=$1";
    let fila: Pais = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
