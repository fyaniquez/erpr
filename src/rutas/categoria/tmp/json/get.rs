//! src/rutas/categoria/json/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un categoria en formato json

use crate::modelo::categoria::{Categoria, CategoriaError};
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use anyhow::Context;

#[tracing::instrument(name="categoria json", skip(pool))]
#[get("/categoria/{id}.json")]
pub async fn categoria_json(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaError> {
    let (id,) = path.into_inner();
    let categoria = obtiene(&pool, id).await
        .context("Error al leer categoria")?;
    Ok(HttpResponse::Ok().body(serde_json::to_string(&categoria).unwrap()))
}

pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Categoria, sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre, descripcion FROM categorias WHERE id=$1";
    let fila: Categoria = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
