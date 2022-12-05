//! src/rutas/capitulo/borra/delete.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: borra un capitulo

use crate::modelo::capitulo::CapituloError;
use actix_web::{http::header, delete, web, HttpResponse};
use sqlx::PgPool;
use anyhow::Context;

#[tracing::instrument(name="Ve capitulo", skip(pool))]
#[delete("/capitulo/{id}")]
pub async fn capitulo_borra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CapituloError> {
    let (id,) = path.into_inner();
    let _row = borra(&pool, id).await
        .context("Error al borrar capitulo")?;
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, "/capitulos")).finish()
    )
}

pub async fn borra(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    const DELETE: &str = "DELETE FROM capitulos WHERE id=$1";
    let _fila = sqlx::query(DELETE.as_ref())
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
