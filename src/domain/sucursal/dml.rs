//! src/domain/sucursal/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::sucursal::Sucursal;
use crate::layout::lista::Paginado;
use sqlx::PgPool;

const SELECT: &str = r#"SELECT id, nombre, empresa_id
    FROM sucursales WHERE empresa_id=$1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista sucursales", skip(pool))]
pub async fn lista(pool: &PgPool, empresa_id: i64) -> Result<Vec<Sucursal>, sqlx::Error> {
    let filas: Vec<Sucursal> = sqlx::query_as(SELECT)
        .bind(empresa_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista sucursales paginada", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
    empresa_id: i64,
) -> Result<(Vec<Sucursal>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Sucursal> = sqlx::query_as(qry.as_ref())
        .bind(empresa_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(empresa_id)
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un sucursal de la base de datos
#[tracing::instrument(name = "ve sucursal", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) 
-> Result<Sucursal, sqlx::Error> {
    let fila: Sucursal =
        sqlx::query_as(
        r#"SELECT id, nombre, empresa_id
        FROM sucursales WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
