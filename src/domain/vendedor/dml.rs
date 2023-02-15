//! src/domain/vendedor/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::layout::lista::Paginado;
use crate::domain::vendedor::Vendedor;
use sqlx::PgPool;

const SELECT: &str = r#"SELECT id, distribuidora_id, nombre, cargo, contactos FROM vendedores
    WHERE distribuidora_id = $1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista vendedors", skip(pool))]
pub async fn lista(
    pool: &PgPool, 
    distribuidora_id: i64
) -> Result<Vec<Vendedor>, sqlx::Error> {
    let filas: Vec<Vendedor> = sqlx::query_as(SELECT)
        .bind(distribuidora_id)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista vendedors", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado,
    distribuidora_id: i64,
) -> Result<(Vec<Vendedor>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Vendedor> = sqlx::query_as(qry.as_ref())
        .bind(distribuidora_id)
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(distribuidora_id)
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un vendedor de la base de datos
#[tracing::instrument(name = "ve vendedor", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) 
-> Result<Vendedor, sqlx::Error> {
    let fila: Vendedor =
        sqlx::query_as(
            r#"SELECT id, distribuidora_id, nombre, cargo, contactos
            FROM vendedores WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
