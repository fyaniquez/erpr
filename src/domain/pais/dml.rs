//! src/domain/pais/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::domain::pais::{
    Pais,
    Nuevo,
};
use crate::layout::lista::Paginado;
use sqlx::PgPool;

const SELECT: &str = 
    r#"SELECT id, nombre, sigla FROM paises"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista paises", skip(pool))]
pub async fn lista(pool: &PgPool) 
-> Result<Vec<Pais>, sqlx::Error> {
    let lista_ordenada = SELECT.to_owned() + " ORDER BY nombre";
    let filas: Vec<Pais> = sqlx::query_as(lista_ordenada.as_str())
        .fetch_all(pool)
        .await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista paises", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool,
    paginado: &Paginado,
) -> Result<(Vec<Pais>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Pais> = sqlx::query_as(qry.as_ref())
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un pais de la base de datos
#[tracing::instrument(name = "ve pais", skip(pool))]
pub async fn obtiene(pool: &PgPool, id: i64) -> Result<Pais, sqlx::Error> {
    let fila: Pais =
        sqlx::query_as(
        r#"SELECT id, nombre, sigla FROM paises WHERE id=$1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}

// inserta un pais en la base de datos
#[tracing::instrument(name = "Inserta pais", skip(pais_nuevo, pool))]
pub async fn inserta(
    pool: &PgPool,
    pais_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO paises (nombre, sigla) VALUES ($1) RETURNING id"#)
    .bind(pais_nuevo.nombre.as_ref())
    .bind(pais_nuevo.sigla.as_ref())
    .fetch_one(pool)
    .await?;
    Ok(id)
}

// inserta un pais en la base de datos
#[tracing::instrument(name = "modifica pais", skip(pais, pool))]
pub async fn actualiza(
    pool: &PgPool,
    pais: &Pais,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE paises SET nombre=$1, sigla=$2 WHERE id=$3",
        &pais.nombre,
        &pais.sigla,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
