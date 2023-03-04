//! src/rutas/producto/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de producto

use crate::domain::producto::{
    Producto, ProductoError,
    Contenido, Caracteristicas,
};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
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
impl TryFrom<FormData> for Producto {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let contenido = Contenido::parse(form_data.contenido)?;
        let caracteristicas = Caracteristicas::parse(form_data.caracteristicas)?;
        Ok( Self{ 
            id: None, 
            nombre: "-.-".to_string(),
            contenido: String::from(contenido.as_ref()), 
            caracteristicas: String::from(caracteristicas.as_ref()), 
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
    name = "Actualización de producto",
    skip(form, pool),
)]
#[post("/producto/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, ProductoError> {
    let (id,) = path.into_inner();
    let producto = form.0.try_into().map_err(ProductoError::Validacion)?;
    producto_actualiza(&pool, &producto, id)
        .await
        .context("Error al actualizar producto en la BD")?;
    let url_ver =  format!("/producto/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un producto en la base de datos
#[tracing::instrument(name = "modifica producto", skip(producto, pool))]
pub async fn producto_actualiza(
    pool: &PgPool,
    producto: &Producto,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE productos SET nombre=$1 WHERE id=$2",
        &producto.nombre,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
