//! src/domain/marca/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::layout::lista::Paginado;
use crate::domain::marca::Marca;
use sqlx::PgPool;

const SELECT: &str = "SELECT id, nombre FROM marcas";

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista marcas", skip(pool))]
pub async fn lista(pool: &PgPool) -> Result<Vec<Marca>, sqlx::Error> {
    let filas: Vec<Marca> = sqlx::query_as(SELECT)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista marcas", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado
) -> Result<(Vec<Marca>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Marca> = sqlx::query_as(qry.as_ref())
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

