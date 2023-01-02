//! src/domain/venta/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::layout::lista::Paginado;
use crate::domain::venta::Venta;
use sqlx::PgPool;

const SELECT: &str = 
    r#"SELECT id, fecha, total, descuento, cliente_id,
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
            r#"SELECT id, fecha, total, descuento, cliente_id,
                puesto_id, usuario_id, medio_id, estado
            FROM ventas WHERE id=$1"#
        ).bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
