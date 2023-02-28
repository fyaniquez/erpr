//! src/domain/categoria/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::categoria::{Categoria, Nuevo};
use crate::layout::lista::Paginado;
use sqlx::PgPool;

const SELECT: &str = r#"SELECT id, nombre, capitulo_id 
    FROM categorias WHERE capitulo_id=$1"#;

const SELECT_JSON: &str = r#"SELECT id, nombre, capitulo_id 
    FROM categorias WHERE capitulo_id=$1
    ORDER BY nombre"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista categorias json", skip(pool))]
pub async fn lista(pool: &PgPool, capitulo_id: i64) 
-> Result<Vec<Categoria>, sqlx::Error> {
    let filas: Vec<Categoria> = sqlx::query_as(SELECT_JSON)
        .bind(capitulo_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista categorias", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
    capitulo_id: i64,
) -> Result<(Vec<Categoria>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Categoria> = sqlx::query_as(qry.as_ref())
        .bind(capitulo_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(capitulo_id)
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un categoria de la base de datos
#[tracing::instrument(name = "ve categoria", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) 
-> Result<Categoria, sqlx::Error> {
    let fila: Categoria =
        sqlx::query_as(
            r#"SELECT id, nombre, capitulo_id 
            FROM categorias 
            WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}

// inserta un categoria en la base de datos
#[tracing::instrument(name = "Inserta categoria", skip(categoria_nuevo, pool))]
pub async fn inserta(
    pool: &PgPool,
    categoria_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
"INSERT INTO categorias (nombre, capitulo_id) VALUES ($1, $2) RETURNING id",
    )
    .bind(categoria_nuevo.nombre.as_ref())
    .bind(categoria_nuevo.capitulo_id)
    .fetch_one(pool)
    .await?;
    Ok(id)
}

