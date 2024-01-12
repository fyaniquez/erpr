//! src/domain/compra/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::login::Estado;
use crate::layout::lista::Paginado;
use crate::domain::compra::{
    Compra,
    CompraVe,
};
use sqlx::{PgPool, Transaction};
use chrono::Utc;

const SELECT: &str = 
    r#"SELECT id, fecha, total, descuento, documento, observaciones,
        distribuidora_id, sucursal_id, usuario_id, medio_id, estado
    FROM compras WHERE sucursal_id=$1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista compras", skip(pool))]
pub async fn lista(
    pool: &PgPool, 
    sucursal_id: i64
) -> Result<Vec<Compra>, sqlx::Error> {
    let filas: Vec<Compra> = sqlx::query_as(SELECT)
        .bind(sucursal_id)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista compras", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado,
    sucursal_id: i64,
) -> Result<(Vec<Compra>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Compra> = sqlx::query_as(qry.as_ref())
        .bind(sucursal_id)
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(sucursal_id)
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un compra de la base de datos
#[tracing::instrument(name = "ve compra", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Compra, sqlx::Error> {
    let fila: Compra = sqlx::query_as(
            r#"SELECT id, fecha, total, descuento, documento, observaciones,
                cliente_id, sucursal_id, usuario_id, medio_id, estado
            FROM compras WHERE id=$1"#
        ).bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}

// obtiene un compra de la base de datos
#[tracing::instrument(name = "ve compra", skip(pool))]
pub async fn obtiene_ve(
    pool: &PgPool, id: i64
) -> Result<CompraVe, sqlx::Error> {
    let fila: CompraVe = sqlx::query_as(
            r#"SELECT v.id, v.fecha, v.total, v.descuento, 
                d.nombre as distribuidora, p.nombre as sucursal, 
                u.nombre as usuario, m.nombre as medio, 
                v.documento, v.observaciones, v.estado
            FROM compras v, distribuidoras d, sucursales p,
                usuarios u, medios m
            WHERE v.id=$1 AND v.distribuidora_id = d.id 
                AND v.sucursal_id = p.id AND v.usuario_id = u.id
                AND v.medio_id = m.id"#
        ).bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}

// inserta un compra en la base de datos
#[tracing::instrument(name = "Inserta compra", skip(compra, tx))]
pub async fn inserta(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    compra: &Compra,
    estado: &Estado,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO compras 
        (fecha, total, descuento, documento, observaciones,
        distribuidora_id, sucursal_id, usuario_id, medio_id, estado)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id"#,
    )
    .bind(Utc::now().naive_utc())
    .bind(compra.total)
    .bind(compra.descuento)
    .bind(&compra.documento)
    .bind(&compra.observaciones)
    .bind(compra.distribuidora_id)
    .bind(estado.sucursal_id)
    .bind(estado.usuario_id)
    .bind(compra.medio_id)
    .bind("Pagado".to_string())
    .fetch_one(tx)
    .await?;
    Ok(id)
}
