//! src/domain/fabrica/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::layout::lista::Paginado;
use crate::domain::fabrica::{
    Fabrica,
    Nuevo,
};
use sqlx::PgPool;

const SELECT: &str = r#"SELECT id, nombre, pais_id 
    FROM fabricas WHERE pais_id = $1"#;
const SELECT_JSON: &str = r#"SELECT id, nombre, pais_id 
    FROM fabricas WHERE pais_id=$1
    ORDER BY nombre"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista fabricas", skip(pool))]
pub async fn lista(pool: &PgPool, pais_id: i64) 
-> Result<Vec<Fabrica>, sqlx::Error> {
    let filas: Vec<Fabrica> = sqlx::query_as(SELECT_JSON)
        .bind(pais_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista fabricas", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado,
    pais_id: i64,
) -> Result<(Vec<Fabrica>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Fabrica> = sqlx::query_as(qry.as_ref())
        .bind(pais_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(pais_id)
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}
// inserta un fabrica en la base de datos
#[tracing::instrument(name = "Inserta fabrica", skip(nuevo, pool))]
pub async fn inserta(
    pool: &PgPool,
    nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
"INSERT INTO fabricas (nombre, pais_id) VALUES ($1, $2) RETURNING id",
    )
    .bind(nuevo.nombre.as_ref())
    .bind(nuevo.pais_id)
    .fetch_one(pool)
    .await?;
    Ok(id)
}
// obtiene un pais de la base de datos
#[tracing::instrument(name = "ve fabrica", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Fabrica, sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre, pais_id FROM fabricas WHERE id=$1";
    let fila: Fabrica = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
