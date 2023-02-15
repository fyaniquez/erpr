//! src/domain/comprado/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::comprado::{
    Comprado,
    CompradoVe,
    Comprados,
};
use crate::layout::lista::Paginado;
use sqlx::{PgPool, Transaction};

const SELECT: &str = r#"SELECT id, cantidad, costo, 
    descuento, total, vencimiento, producto_id, compra_id 
    FROM comprados WHERE compra_id=$1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista comprados", skip(pool))]
pub async fn lista(pool: &PgPool, compra_id: i64) 
-> Result<Vec<Comprado>, sqlx::Error> {
    let filas: Vec<Comprado> = sqlx::query_as(SELECT)
        .bind(compra_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista comprados", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
    compra_id: i64,
) -> Result<(Vec<CompradoVe>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<CompradoVe> = sqlx::query_as(qry.as_ref())
        .bind(compra_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(compra_id)
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}
// obtiene un comprado de la base de datos
#[tracing::instrument(name = "ve comprado", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) 
-> Result<Comprado, sqlx::Error> {
    let fila: Comprado =
        sqlx::query_as(
            r#"SELECT id, cantidad, costo, total, descuento,
            vencimiento, producto_id, compra_id
            FROM comprados 
            WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}

// obtiene un comprado de la base de datos
#[tracing::instrument(name = "ve comprado ve", skip(pool))]
pub async fn obtiene_ve(pool: &PgPool, id: i64) 
-> Result<CompradoVe, sqlx::Error> {
    let fila: CompradoVe =
        sqlx::query_as(
            r#"SELECT v.id, v.cantidad, v.costo, v.descuento, v.total,
            v.vencimiento, p.nombre as producto, compra_id
            FROM comprados v INNER JOIN porductos p
            ON v.producto_id = p.id
            WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}

// inserta un item comprado en la base de datos
#[tracing::instrument(name = "Inserta item comprado", 
skip(comprado, compra_id, tx))]
pub async fn inserta(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    compra_id: i64,
    comprado: &Comprado,
) -> Result<i64, sqlx::Error> {

    let (id,) = sqlx::query_as(
    r#"INSERT INTO comprados 
        (producto_id, compra_id, cantidad, costo, 
        descuento, total, vencimiento)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id"#,
    )
    .bind(comprado.producto_id)
    .bind(compra_id)
    .bind(comprado.cantidad)
    .bind(comprado.costo)
    .bind(comprado.descuento)
    .bind(comprado.total)
    .bind(comprado.vencimiento)
    .fetch_one(tx)
    .await?;
    Ok(id)
}

#[tracing::instrument(name = "Inserta items comprados", 
skip(comprados, compra_id, tx))]
pub async fn inserta_mul(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    compra_id: i64,
    comprados: &Comprados,
) -> Result<i64, sqlx::Error> {

    let compra_ids: Vec<i64> = (0..comprados.costos.len())
        .map(|_| compra_id).collect();

    sqlx::query( 
    r#"INSERT INTO comprados 
        (producto_id, compra_id, cantidad, costo, 
        descuento, total, vencimiento)
        SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::integer[], 
        $4::integer[], $5::integer[], $6::integer[], $7::timestamp[])
        "#)
    .bind(&comprados.producto_ids[..])
    .bind(&compra_ids[..])
    .bind(&comprados.cantidads[..])
    .bind(&comprados.costos[..])
    .bind(&comprados.descuentos[..])
    .bind(&comprados.totals[..])
    .bind(&comprados.vencimientos[..])
    .execute(tx)
    .await?;
    Ok(1)
}
