//! src/domain/catalogo/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::catalogo::Catalogo;
use crate::layout::lista::Paginado;
use sqlx::PgPool;

const SELECT: &str = 
    r#"SELECT id, nombre, sucursal_id, fecha, activo
    FROM catalogos WHERE sucursal_id=$1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista catalogos", skip(pool))]
pub async fn lista(pool: &PgPool, sucursal_id: i64) 
-> Result<Vec<Catalogo>, sqlx::Error> {
    let filas: Vec<Catalogo> = sqlx::query_as(SELECT)
        .bind(sucursal_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista catalogos", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
    sucursal_id: i64,
) -> Result<(Vec<Catalogo>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Catalogo> = sqlx::query_as(qry.as_ref())
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

// obtiene un catalogo de la base de datos
#[tracing::instrument(name = "ve catalogo", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) -> Result<Catalogo, sqlx::Error> {
    let fila: Catalogo =
        sqlx::query_as(
        r#"SELECT id, nombre, sucursal_id, fecha, activo
        FROM catalogos WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
