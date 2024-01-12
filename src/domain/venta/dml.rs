//! src/domain/venta/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para Ventas

use crate::domain::login::Estado;
use crate::layout::lista::Paginado;
use crate::domain::venta::{
    Venta,
    VentaVe,
};
use sqlx::{PgPool, Transaction};
use chrono::Utc;

const SELECT: &str = 
    r#"SELECT id, fecha, subtotal, descuento, cliente_id,
        puesto_id, usuario_id, medio_id, estado
    FROM ventas WHERE puesto_id=$1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista ventas", skip(pool))]
pub async fn lista(
    pool: &PgPool, 
    puesto_id: i64
) -> Result<Vec<Venta>, sqlx::Error> {
    let filas: Vec<Venta> = sqlx::query_as(SELECT)
        .bind(puesto_id)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista ventas", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado,
    puesto_id: i64,
) -> Result<(Vec<Venta>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Venta> = sqlx::query_as(qry.as_ref())
        .bind(puesto_id)
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(puesto_id)
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un venta de la base de datos
#[tracing::instrument(name = "ve venta", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Venta, sqlx::Error> {
    let fila: Venta = sqlx::query_as(
            r#"SELECT id, fecha, subtotal, descuento, cliente_id,
                puesto_id, usuario_id, medio_id, estado
            FROM ventas WHERE id=$1"#
        ).bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}

// obtiene un venta de la base de datos
#[tracing::instrument(name = "ve venta", skip(pool))]
pub async fn obtiene_ve(
    pool: &PgPool, id: i64
) -> Result<VentaVe, sqlx::Error> {
    let fila: VentaVe = sqlx::query_as(
            r#"SELECT v.id, v.fecha, v.subtotal, v.descuento, 
                c.nombre as cliente, p.nombre as puesto, 
                u.nombre as usuario, m.nombre as medio, v.estado
            FROM ventas v, clientes c, puestos p,
                usuarios u, medios m
            WHERE v.id=$1 AND v.cliente_id = c.id 
                AND v.puesto_id = p.id AND v.usuario_id = u.id
                AND v.medio_id = m.id"#
        ).bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}

// inserta un venta en la base de datos
#[tracing::instrument(name = "Inserta venta", skip(venta, tx))]
pub async fn inserta(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    venta: &Venta,
    estado: &Estado,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO ventas 
        (fecha, subtotal, descuento, cliente_id, puesto_id, 
        usuario_id, medio_id, estado)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id"#,
    )
    .bind(Utc::now().naive_utc())
    .bind(venta.subtotal)
    .bind(venta.descuento)
    .bind(venta.cliente_id)
    .bind(estado.puesto_id)
    .bind(estado.usuario_id)
    .bind(venta.medio_id)
    .bind("Pagado".to_string())
    .fetch_one(tx)
    .await?;
    Ok(id)
}
