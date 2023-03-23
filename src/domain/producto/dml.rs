//! src/domain/producto/dml
//! author: fyaniquez
//! date: 30/10/2022
//! instrucciones dml para apitulo

use crate::layout::lista::Paginado;
use crate::domain::producto::{
    Nuevo,
    Producto,
    ProductoVe,
};
use sqlx::PgPool;

const SELECT: &str = 
    r#"SELECT id, nombre, caracteristicas, categoria_id,
        marca_id, unidad_id, fabrica_id, contenido, cantidad,
        fraccionable, barras, activo
    FROM productos"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista productos", skip(pool))]
pub async fn lista(pool: &PgPool) -> Result<Vec<Producto>, sqlx::Error> {
    let filas: Vec<Producto> = sqlx::query_as(SELECT)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(name = "Lista productos", skip(pool))]
pub async fn lista_paginada(
    pool: &PgPool, 
    paginado: &Paginado
) -> Result<(Vec<Producto>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Producto> = sqlx::query_as(qry.as_ref())
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// lista los productos sin precios en el catalogo proporcionado
const SELECT_SP: &str = 
    r#"SELECT pro.id, pro.nombre, pro.caracteristicas, pro.categoria_id,
        pro.marca_id, pro.unidad_id, pro.fabrica_id, pro.contenido, 
        pro.cantidad, pro.fraccionable, pro.barras, pro.activo
    FROM productos AS pro
    WHERE pro.id NOT IN
        (SELECT pre.producto_id FROM precios AS pre
        WHERE pre.catalogo_id = $1)"#;

// obtiene una lista de objetos
#[tracing::instrument(name = "Lista productos sin precio", skip(pool))]
pub async fn lista_sin_precio(pool: &PgPool, catalogo_id: i64) 
-> Result<Vec<Producto>, sqlx::Error> {
    let filas: Vec<Producto> = sqlx::query_as(SELECT_SP)
        .bind(catalogo_id)
        .fetch_all(pool).await?;
    Ok(filas)
}

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(
    name = "Lista paginada productos sin precio", skip(pool))]
pub async fn lista_paginada_sin_precio(
    pool: &PgPool, 
    paginado: &Paginado,
    catalogo_id: i64,
) -> Result<(Vec<Producto>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT_SP);
    let filas: Vec<Producto> = sqlx::query_as(qry.as_ref())
        .bind(catalogo_id)
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT_SP);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(catalogo_id)
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// lista los productos sin precios en el catalogo proporcionado
const SELECT_SI: &str = 
    r#"SELECT pro.id, pro.nombre, pro.caracteristicas, pro.categoria_id,
        pro.marca_id, pro.unidad_id, pro.fabrica_id, pro.contenido, 
        pro.cantidad, pro.fraccionable, pro.barras, pro.activo
    FROM productos AS pro
    WHERE pro.id NOT IN
        (SELECT inv.producto_id FROM inventariados AS inv
        WHERE inv.inventario_id = $1)"#;

// obtiene una pagina de la tabla de objetos
#[tracing::instrument(
    name = "Lista paginada productos sin inventariado", skip(pool))]
pub async fn lista_paginada_sin_inventariado(
    pool: &PgPool, 
    paginado: &Paginado,
    inventario_id: i64,
) -> Result<(Vec<Producto>, i32), sqlx::Error> {
    let qry = paginado.get_qry(SELECT_SI);
    let filas: Vec<Producto> = sqlx::query_as(qry.as_ref())
        .bind(inventario_id)
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT_SI);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(inventario_id)
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// obtiene un producto de la base de datos
#[tracing::instrument(name = "ve producto ve", skip(pool))]
pub async fn obtiene_ve(
    pool: &PgPool, id: i64
) -> Result<ProductoVe, sqlx::Error> {
    const SELECT: &str = 
        r#"SELECT p.id, p.nombre, p.caracteristicas, cap.nombre as capitulo,
            cat.nombre as categoria, mar.nombre as marca, 
            uni.nombre as unidad, fab.nombre as fabrica, p.barras,
            p.contenido, p.cantidad, 
            CASE WHEN p.fraccionable THEN 'Si' ELSE 'No' END as fraccionable,  
            CASE WHEN p.activo THEN 'Si' ELSE 'No' END as activo
        FROM productos p 
        INNER JOIN categorias as cat ON p.categoria_id = cat.id
        INNER JOIN marcas as mar ON p.marca_id = mar.id
        INNER JOIN unidades as uni ON p.unidad_id = uni.id
        INNER JOIN fabricas as fab ON p.fabrica_id = fab.id
        INNER JOIN capitulos as cap ON cat.capitulo_id = cap.id
        WHERE p.id=$1"#;
    let fila: ProductoVe = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}

// obtiene un producto de la base de datos
#[tracing::instrument(name = "ve producto", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Producto, sqlx::Error> {
    const SELECT: &str = 
        r#"SELECT id, nombre, caracteristicas, 
            categoria_id, marca_id, unidad_id, fabrica_id, barras,
            contenido, cantidad, fraccionable, activo
        FROM productos
        WHERE id=$1"#;
    let fila: Producto = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
// inserta un producto en la base de datos
#[tracing::instrument(
    name = "Inserta producto", 
    skip(producto_nuevo, pool)
)]
pub async fn inserta(
    pool: &PgPool,
    producto_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO productos 
        (caracteristicas, categoria_id, marca_id, unidad_id,
        fabrica_id, contenido, cantidad, fraccionable, barras, activo) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
        RETURNING id"#,
    )
    .bind(producto_nuevo.caracteristicas.as_ref())
    .bind(producto_nuevo.categoria_id)
    .bind(producto_nuevo.marca_id)
    .bind(producto_nuevo.unidad_id)
    .bind(producto_nuevo.fabrica_id)
    .bind(producto_nuevo.contenido.as_ref())
    .bind(producto_nuevo.cantidad)
    .bind(producto_nuevo.fraccionable)
    .bind(producto_nuevo.barras.as_ref())
    .bind(true)
    .fetch_one(pool)
    .await?;

    Ok(id)
}


// inserta un producto en la base de datos
#[tracing::instrument(name = "modifica producto", skip(producto, pool))]
pub async fn actualiza(
    pool: &PgPool,
    producto: &Producto,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        r#"UPDATE productos SET nombre=$2, contenido=$3, caracteristicas=$4, 
        barras=$5, cantidad=$6, categoria_id=$7, fraccionable=$8, fabrica_id=$9, 
        marca_id=$10, unidad_id=$11 WHERE id=$1"#
    )
    .bind(id)
    .bind(&producto.nombre)
    .bind(&producto.contenido)
    .bind(&producto.caracteristicas)
    .bind(producto.barras.as_ref())
    .bind(producto.cantidad)
    .bind(producto.categoria_id)
    .bind(producto.fraccionable)
    .bind(producto.fabrica_id)
    .bind(producto.marca_id)
    .bind(producto.unidad_id)
    .execute(pool)
    .await?;

    Ok(())
}
