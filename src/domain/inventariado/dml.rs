//! src/domain/inventariado/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::inventariado::Inventariado;
use crate::layout::lista::Paginado;
use sqlx::PgPool;

const SELECT: &str = 
    r#"SELECT inv.id, pro.nombre, inv.producto_id, 
        inv.cantidad, inv.inventario_id
    FROM inventariados inv INNER JOIN productos pro 
    ON inv.producto_id = pro.id
    WHERE inv.inventario_id=$1"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista inventariados", skip(pool))]
pub async fn lista(pool: &PgPool, inventario_id: i64) -> Result<Vec<Inventariado>, sqlx::Error> {
    let filas: Vec<Inventariado> = sqlx::query_as(SELECT)
        .bind(inventario_id)
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista inventariados", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
    inventario_id: i64,
) -> Result<(Vec<Inventariado>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Inventariado> = sqlx::query_as(qry.as_ref())
        .bind(inventario_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(inventario_id)
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un inventariado de la base de datos
#[tracing::instrument(name = "ve inventariado", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) -> Result<Inventariado, sqlx::Error> {
    let fila: Inventariado =
        sqlx::query_as(
        r#"SELECT inv.id, pro.nombre, inv.producto_id, 
            inv.cantidad, inv.inventario_id
        FROM inventariados inv INNER JOIN productos pro 
        ON inv.producto_id = pro.id
        WHERE inv.id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
