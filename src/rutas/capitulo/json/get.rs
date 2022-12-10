//! src/rutas/capitulo/json/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un capitulo en formato json

use crate::modelo::capitulo::{Capitulo, CapituloError};
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use anyhow::Context;

#[tracing::instrument(name="capitulo json", skip(pool))]
#[get("/capitulo/{id}.json")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CapituloError> {
    let (id,) = path.into_inner();
    let capitulo = obtiene(&pool, id).await
        .context("Error al leer capitulo")?;
    Ok(HttpResponse::Ok().body(serde_json::to_string(&capitulo).unwrap()))
}

pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Capitulo, sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre, descripcion FROM capitulos WHERE id=$1";
    let fila: Capitulo = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
