//! src/domain/distribuidora/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::layout::lista::Paginado;
use crate::domain::distribuidora::Distribuidora;
use sqlx::PgPool;

const SELECT: &str = r#"SELECT id, empresa_id, nombre, descripcion,
        documento, preventa, activa FROM distribuidoras"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista distribuidoras", skip(pool))]
pub async fn lista(
    pool: &PgPool
) -> Result<Vec<Distribuidora>, sqlx::Error> { 
    let filas: Vec<Distribuidora> = sqlx::query_as(SELECT)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista distribuidoras", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado
) -> Result<(Vec<Distribuidora>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Distribuidora> = sqlx::query_as(qry.as_ref())
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene distribuidora de la base de datos
#[tracing::instrument(name = "ve distribuidora", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) 
-> Result<Distribuidora, sqlx::Error> {
    let fila: Distribuidora =
        sqlx::query_as(
            r#"SELECT id, empresa_id, nombre, descripcion,
            documento, preventa, activa
            FROM distribuidoras WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
