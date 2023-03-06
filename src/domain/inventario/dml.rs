//! src/domain/inventario/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::inventario::{
    Inventario,
    Nuevo,
};
use crate::layout::lista::Paginado;
use sqlx::PgPool;

const SELECT: &str = 
    r#"SELECT id, nombre, sucursal_id, fecha, estado 
    FROM inventarios 
    WHERE sucursal_id=$1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista inventarios", skip(pool))]
pub async fn lista(pool: &PgPool, sucursal_id: i64) 
-> Result<Vec<Inventario>, sqlx::Error> {
    let filas: Vec<Inventario> = sqlx::query_as(SELECT)
        .bind(sucursal_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista inventarios", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
    sucursal_id: i64,
) -> Result<(Vec<Inventario>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Inventario> = sqlx::query_as(qry.as_ref())
        .bind(sucursal_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(sucursal_id)
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un inventario de la base de datos
#[tracing::instrument(name = "ve inventario", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) 
-> Result<Inventario, sqlx::Error> {
    let fila: Inventario =
        sqlx::query_as(
        r#"SELECT id, nombre, sucursal_id, fecha, estado
        FROM inventarios WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}

// inserta un inventario en la base de datos
#[tracing::instrument(name = "Inserta inventario", skip(inventario_nuevo, pool))]
pub async fn inserta(
    pool: &PgPool,
    inventario_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
        r#"INSERT INTO inventarios 
        (nombre, sucursal_id) 
        VALUES ($1, $2) 
        RETURNING id"#,
    )
    .bind(inventario_nuevo.nombre.as_ref())
    .bind(inventario_nuevo.sucursal_id)
    .fetch_one(pool)
    .await?;
    Ok(id)
}
