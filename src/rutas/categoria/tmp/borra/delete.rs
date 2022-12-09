//! src/rutas/categoria/borra/delete.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: borra un categoria

use crate::modelo::categoria::CategoriaError;
use actix_web::{http::header, delete, web, HttpResponse};
use sqlx::PgPool;
use anyhow::Context;

#[tracing::instrument(name="Ve categoria", skip(pool))]
#[delete("/categoria/{id}")]
pub async fn categoria_borra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaError> {
    let (id,) = path.into_inner();
    let _row = borra(&pool, id).await
        .context("Error al borrar categoria")?;
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, "/categorias")).finish()
    )
}

pub async fn borra(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    const DELETE: &str = "DELETE FROM categorias WHERE id=$1";
    let _fila = sqlx::query(DELETE.as_ref())
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
