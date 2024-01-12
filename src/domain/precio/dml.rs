//! src/domain/precio/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo
use crate::domain::login::Estado;
use crate::domain::precio::{
    Precio,
    Nuevo,
};
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
    estado: &Estado,
) -> Result<(Vec<Precio>, i32), sqlx::Error> {

    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Precio> = sqlx::query_as(qry.as_ref())
        .bind(estado.catalogo_id)
        .fetch_all(pool)
        .await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(estado.catalogo_id)
        .fetch_one(pool)
        .await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un precio de la base de datos
// por id del producto y del catalogo activo
#[tracing::instrument(name = "ve precio por catalogo", skip(pool))]
pub async fn obtiene_prod(
    pool: &PgPool, 
    catalogo_id: i64, 
    producto_id: i64
) -> Result<Precio, sqlx::Error> {
    let fila: Precio =
        sqlx::query_as(
        r#"SELECT pre.id, pro.nombre, pre.producto_id, 
        pre.precio, pre.catalogo_id
        FROM precios pre INNER JOIN productos pro 
        ON pre.producto_id = pro.id
        WHERE pre.catalogo_id = $1 AND pre.producto_id=$2"#)
            .bind(catalogo_id)
            .bind(producto_id)
            .fetch_one(pool)
            .await?;
    Ok(fila)
}
//
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
//
// inserta un precio en la base de datos
#[tracing::instrument(name = "Inserta precio", skip(precio_nuevo, pool))]
pub async fn inserta(
    pool: &PgPool,
    precio_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO precios 
    (producto_id, precio, catalogo_id) 
    VALUES ($1, $2, $3) RETURNING id"#)
    .bind(precio_nuevo.producto_id)
    .bind(precio_nuevo.precio)
    .bind(precio_nuevo.catalogo_id)
    .fetch_one(pool)
    .await?;
    Ok(id)
}
