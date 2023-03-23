//! src/rutas/categoria_marca/borra/delete.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: borra un categoria_marca

use crate::domain::categoria_marca::{
    CategoriaMarcaError,
    borra,
};
use actix_web::{delete, web, HttpResponse};
use sqlx::PgPool;
use anyhow::Context;

#[tracing::instrument(name="borra categoria_marca", skip(pool))]
#[delete("/categoria_marca/{categoria_id}/{marca_id}")]
pub async fn procesa(
    path: web::Path<(i64, i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaMarcaError> {
    let (categoria_id, marca_id,) = path.into_inner();
    let _row = borra(&pool, categoria_id, marca_id).await
        .context("Error al borrar relación categoria_marca")?;
    Ok(HttpResponse::Ok().finish())
}


