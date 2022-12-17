//! src/rutas/producto/crea/post.rs
//! author: fyaniquez
//! date: 06/12/2022
//! purpose: procesa el formulario crea producto

use crate::domain::producto::{
    ProductoError, Nuevo,
    Contenido, Nombre, Caracteristicas,
};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    contenido: String,
    caracteristicas: String,
    activo: bool,
    barras: String,
    cantidad: i32,
    categoria_id: i64,
    fraccionable: bool,
    fabrica_id: i64,
    marca_id: i64,
    unidad_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Nuevo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        // todo simplificar las siguientes 2 lineas en una sola
        let contenido = Contenido::parse(form_data.contenido)?;
        let caracteristicas = Caracteristicas::parse(form_data.caracteristicas)?;
        Ok( Self { 
            nombre,
            contenido,
            caracteristicas,
            activo: form_data.activo,
            barras: Some(form_data.barras),
            cantidad: form_data.cantidad,
            categoria_id: form_data.categoria_id,
            fraccionable: form_data.fraccionable,
            fabrica_id: form_data.fabrica_id,
            marca_id: form_data.marca_id,
            unidad_id: form_data.unidad_id,
        })
    }
}

// extrae datos del producto del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Alta de producto",
    skip(form, pool),
    fields( 
        producto_nombre = %form.nombre,
    )
)]
#[post("/producto")]
pub async fn procesa(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, ProductoError> {
    //TODO añadir validacion de existencia de capitulo_id
    let producto = form.0.try_into().map_err(ProductoError::Validacion)?;
    let id = producto_inserta(&pool, &producto)
        .await
        .context("Error al insertar producto en la BD")?;
    let url_ver =  format!("/producto/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un producto en la base de datos
#[tracing::instrument(name = "Inserta producto", skip(producto_nuevo, pool))]
pub async fn producto_inserta(
    pool: &PgPool,
    producto_nuevo: &Nuevo,
) -> Result<i64, sqlx::Error> {
    let (id,) = sqlx::query_as(
    r#"INSERT INTO productos 
        (nombre, caraceristicas, categoria_id, marca_id, unidad_id,
        fabrica_id, contenido, cantidad, fraccionable, barras, activo) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
        RETURNING id"#,
    )
    .bind(producto_nuevo.nombre.as_ref())
    .bind(producto_nuevo.caracteristicas.as_ref())
    .bind(producto_nuevo.categoria_id)
    .bind(producto_nuevo.marca_id)
    .bind(producto_nuevo.unidad_id)
    .bind(producto_nuevo.fabrica_id)
    .bind(producto_nuevo.contenido.as_ref())
    .bind(producto_nuevo.cantidad)
    .bind(producto_nuevo.fraccionable)
    .bind(producto_nuevo.barras.as_ref())
    .bind(producto_nuevo.activo)
    .fetch_one(pool)
    .await?;
    Ok(id)
}
