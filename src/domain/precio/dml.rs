//! src/domain/precio/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::precio::Precio;
use crate::layout::lista::Paginado;
use sqlx::PgPool;

const SELECT: &str = 
    r#"SELECT pre.id, pro.nombre, pre.producto_id, 
    pre.precio, pre.catalogo_id
    FROM precios pre 
    INNER JOIN productos pro ON pre.producto_id = pro.id
    WHERE pre.catalogo_id=$1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista precios", skip(pool))]
pub async fn lista(pool: &PgPool, catalogo_id: i64) -> Result<Vec<Precio>, sqlx::Error> {
    let filas: Vec<Precio> = sqlx::query_as(SELECT)
        .bind(catalogo_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista precios", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
    catalogo_id: i64,
) -> Result<(Vec<Precio>, i32), sqlx::Error> {

    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Precio> = sqlx::query_as(qry.as_ref())
        .bind(catalogo_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(catalogo_id)
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un precio de la base de datos
#[tracing::instrument(name = "ve precio", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) -> Result<Precio, sqlx::Error> {
    let fila: Precio =
        sqlx::query_as(
        r#"SELECT pre.id, pro.nombre, pre.producto_id, 
        pre.precio, pre.catalogo_id
        FROM precios pre INNER JOIN productos pro 
        ON pre.producto_id = pro.id
        WHERE pre.id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
