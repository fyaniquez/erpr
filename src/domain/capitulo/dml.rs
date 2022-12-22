//! src/domain/capitulo/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::layout::lista::Paginado;
use crate::domain::capitulo::Capitulo;
use sqlx::PgPool;

const SELECT: &str = "SELECT id, nombre, descripcion FROM capitulos";

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista capitulos", skip(pool))]
pub async fn lista(pool: &PgPool) -> Result<Vec<Capitulo>, sqlx::Error> {
    let filas: Vec<Capitulo> = sqlx::query_as(SELECT)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista capitulos", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado
) -> Result<(Vec<Capitulo>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Capitulo> = sqlx::query_as(qry.as_ref())
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un capitulo de la base de datos
#[tracing::instrument(name = "ve capitulo", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) 
-> Result<Capitulo, sqlx::Error> {
    let fila: Capitulo =
        sqlx::query_as(
            r#"SELECT id, nombre, descripcion
            FROM capitulos WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
