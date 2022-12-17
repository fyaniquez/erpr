//! src/rutas/categoria/borra/delete.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: borra un categoria

use crate::domain::categoria::CategoriaError;
use actix_web::{delete, web, HttpResponse};
use sqlx::PgPool;
use anyhow::Context;

#[tracing::instrument(name="borra categoria", skip(pool))]
#[delete("/categoria/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaError> {
    let (id,) = path.into_inner();
    let _row = borra(&pool, id).await
        .context("Error al borrar categoria")?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn borra(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    const DELETE: &str = "DELETE FROM categorias WHERE id=$1";
    let _fila = sqlx::query(DELETE.as_ref())
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
