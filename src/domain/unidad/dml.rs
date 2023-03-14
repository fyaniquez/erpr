//! src/domain/unidad/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::layout::lista::Paginado;
use crate::domain::unidad::{
    Unidad,
    Nuevo,
};
use sqlx::PgPool;

const SELECT: &str = "SELECT id, nombre, sigla FROM unidades";
const SELECT_JSON: &str = "SELECT id, nombre, sigla FROM unidades ORDER BY nombre";

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista unidades", skip(pool))]
pub async fn lista(pool: &PgPool) -> Result<Vec<Unidad>, sqlx::Error> {
    let filas: Vec<Unidad> = sqlx::query_as(SELECT_JSON)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista unidades", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado
) -> Result<(Vec<Unidad>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Unidad> = sqlx::query_as(qry.as_ref())
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// inserta un unidad en la base de datos
#[tracing::instrument(name = "Inserta unidad", skip(unidad_nuevo, pool))]
pub async fn inserta(
    pool: &PgPool,
    unidad_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO unidades (nombre, sigla) 
        VALUES ($1, $2) RETURNING id"#,
    )
    .bind(unidad_nuevo.nombre.as_ref())
    .bind(unidad_nuevo.sigla.as_ref())
    .fetch_one(pool)
    .await?;
    Ok(id)
}
