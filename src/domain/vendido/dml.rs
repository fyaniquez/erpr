//! src/domain/vendido/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::vendido::{
    Vendido,
    VendidoVe,
};
use crate::layout::lista::Paginado;
use sqlx::PgPool;

const SELECT: &str = r#"SELECT id, cantidad, precio, 
    total, producto_id venta_id 
    FROM vendidos WHERE venta_id=$1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista vendidos", skip(pool))]
pub async fn lista(pool: &PgPool, venta_id: i64) 
-> Result<Vec<Vendido>, sqlx::Error> {
    let filas: Vec<Vendido> = sqlx::query_as(SELECT)
        .bind(venta_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista vendidos", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
    venta_id: i64,
) -> Result<(Vec<VendidoVe>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<VendidoVe> = sqlx::query_as(qry.as_ref())
        .bind(venta_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(venta_id)
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}
// obtiene un vendido de la base de datos
#[tracing::instrument(name = "ve vendido", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) 
-> Result<Vendido, sqlx::Error> {
    let fila: Vendido =
        sqlx::query_as(
            r#"SELECT id, cantidad, precio, total, descuento,
            producto_id, venta_id
            FROM vendidos 
            WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}

// obtiene un vendido de la base de datos
#[tracing::instrument(name = "ve vendido ve", skip(pool))]
pub async fn obtiene_ve(pool: &PgPool, id: i64) 
-> Result<VendidoVe, sqlx::Error> {
    let fila: VendidoVe =
        sqlx::query_as(
            r#"SELECT v.id, v.cantidad, v.precio, v.descuento, v.total,
            p.nombre as producto, venta_id
            FROM vendidos v INNER JOIN porductos p
            ON v.producto_id = p.id
            WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
