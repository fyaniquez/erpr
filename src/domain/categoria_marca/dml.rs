//! src/domain/categorias_marca/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para categoria marca

use crate::domain::categoria_marca::{
    CategoriaMarcaNombres,
    CategoriaMarca,
};
use crate::layout::lista::Paginado;
use sqlx::PgPool;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista categorias_marcas json", skip(pool))]
pub async fn lista(pool: &PgPool, categoria_id: i64) 
-> Result<Vec<CategoriaMarcaNombres>, sqlx::Error> {
    const SELECT: &str = r#"SELECT categoria_id, c.nombre as categoria_nombre,
        marca_id, m.nombre as marca_nombre
        FROM categorias_marcas, categorias c, marcas m 
        WHERE categoria_id = c.id and marca_id = m.id and categoria_id=$1"#;
    let filas: Vec<CategoriaMarcaNombres> = sqlx::query_as(SELECT)
        .bind(categoria_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista categorias_marcas", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
    categoria_id: i64,
) -> Result<(Vec<CategoriaMarcaNombres>, i32), sqlx::Error> {
    const SELECT: &str = r#"SELECT categoria_id, c.nombre as categoria_nombre, 
        marca_id, m.nombre as marca_nombre
        FROM categorias_marcas, categorias c, marcas m 
        WHERE categoria_id = c.id and marca_id = m.id and categoria_id=$1"#;
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<CategoriaMarcaNombres> = sqlx::query_as(qry.as_ref())
        .bind(categoria_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(categoria_id)
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un categorias_marca de la base de datos
#[tracing::instrument(name = "ve categorias_marca", skip(pool))]
pub async fn obtiene(pool: &PgPool, categoria_id: i64, marca_id:i64) 
-> Result<CategoriaMarcaNombres, sqlx::Error> {
    const SEL: &str = r#"SELECT categoria_id, c.nomre, marca_id, m.nombre
        FROM categorias_marcas, categorias c, marcas m
        WHERE categoria_id=$1 AND categoria_id = c.id 
        AND marca_id = m.id AND marca_id=$2"#;
    let fila: CategoriaMarcaNombres = sqlx::query_as(SEL.as_ref())
        .bind(categoria_id)
        .bind(marca_id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}

// inserta un categorias_marca en la base de datos
#[tracing::instrument(
    name = "Inserta categorias_marca", skip(categorias_marcas, pool))]
pub async fn inserta(
    pool: &PgPool,
    categorias_marcas: &CategoriaMarca,
) -> Result<CategoriaMarca, sqlx::Error> {
    const INSERT: &str = r#"INSERT INTO categorias_marcas (categoria_id, marca_id) 
        VALUES ($1, $2) RETURNING categoria_id, marca_id"#;
    let categoria_marca: CategoriaMarca = sqlx::query_as(INSERT.as_ref())
        .bind(categorias_marcas.categoria_id)
        .bind(categorias_marcas.marca_id)
        .fetch_one(pool)
        .await?;
    Ok(categoria_marca)
}

pub async fn borra(pool: &PgPool, categoria_id: i64, marca_id: i64) 
-> Result<(), sqlx::Error> {
    const DELETE: &str = r#"DELETE FROM categorias_marcas 
        WHERE categoria_id=$1 AND marca_id=$2"#;
    let _fila = sqlx::query(DELETE.as_ref())
        .bind(categoria_id)
        .bind(marca_id)
        .execute(pool)
        .await?;
    Ok(())
}
