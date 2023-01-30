//! src/domain/cliente/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::layout::lista::Paginado;
use crate::domain::cliente::Cliente;
use sqlx::PgPool;

const SELECT: &str = "SELECT id, nombre, documento FROM clientes";

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista clientes", skip(pool))]
pub async fn lista(pool: &PgPool) -> Result<Vec<Cliente>, sqlx::Error> {
    let filas: Vec<Cliente> = sqlx::query_as(SELECT)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista clientes", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado
) -> Result<(Vec<Cliente>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Cliente> = sqlx::query_as(qry.as_ref())
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un cliente de la base de datos
#[tracing::instrument(name = "ve cliente", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) 
-> Result<Cliente, sqlx::Error> {
    let fila: Cliente =
        sqlx::query_as(
            r#"SELECT id, nombre, documento
            FROM clientes WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}

// obtiene un cliente por documento
#[tracing::instrument(name = "ve cliente", skip(pool))]
pub async fn obtiene_documento(pool: &PgPool, documento: String) 
-> Result<Cliente, sqlx::Error> {
    let fila: Cliente =
        sqlx::query_as(
            r#"SELECT id, nombre, documento
            FROM clientes WHERE documento=$1"#)
            .bind(documento)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
