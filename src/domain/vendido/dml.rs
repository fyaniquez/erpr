//! src/domain/vendido/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::vendido::{
    Vendido,
    VendidoVe,
    Vendidos,
};
use crate::layout::lista::Paginado;
use sqlx::{PgPool, Transaction};

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

// inserta un item vendido en la base de datos
#[tracing::instrument(name = "Inserta item vendido", 
skip(vendido, venta_id, tx))]
pub async fn inserta(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    venta_id: i64,
    vendido: &Vendido,
) -> Result<i64, sqlx::Error> {

    let (id,) = sqlx::query_as(
    r#"INSERT INTO vendidos 
        (producto_id, venta_id, cantidad, precio, descuento, total)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id"#,
    )
    .bind(vendido.producto_id)
    .bind(venta_id)
    .bind(vendido.cantidad)
    .bind(vendido.precio)
    .bind(vendido.descuento)
    .bind(vendido.total)
    .fetch_one(tx)
    .await?;
    Ok(id)
}

#[tracing::instrument(name = "Inserta items vendidos", 
skip(vendidos, venta_id, tx))]
pub async fn inserta_mul(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    venta_id: i64,
    vendidos: &Vendidos,
) -> Result<i64, sqlx::Error> {

    let venta_ids: Vec<i64> = (0..vendidos.precios.len())
        .map(|_| venta_id).collect();

    sqlx::query( 
    r#"INSERT INTO vendidos 
        (producto_id, venta_id, cantidad, precio, descuento, total)
        SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::integer[], 
        $4::integer[], $5::integer[], $6::integer[])
        "#)
    .bind(&vendidos.producto_ids[..])
    .bind(&venta_ids[..])
    .bind(&vendidos.cantidads[..])
    .bind(&vendidos.precios[..])
    .bind(&vendidos.descuentos[..])
    .bind(&vendidos.totals[..])
    .execute(tx)
    .await?;
    Ok(1)
}
