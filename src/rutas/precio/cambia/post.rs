//! src/rutas/precio/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de precio

use crate::domain::precio::{Precio, PrecioError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    precio: f32,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Precio {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        Ok( Self{ 
            id: None, 
            nombre: String::from(""),
            producto_id: 0,
            precio: (form_data.precio * 100.0) as i32,
            catalogo_id: 0,
        })
    }
}

// extrae datos del precio del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de precio",
    skip(form, pool),
)]
#[post("/precio/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, PrecioError> {
    let (id,) = path.into_inner();
    let precio = form.0.try_into().map_err(PrecioError::Validacion)?;
    precio_actualiza(&pool, &precio, id)
        .await
        .context("Error al actualizar precio en la BD")?;
    let url_ver =  format!("/precio/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un precio en la base de datos
#[tracing::instrument(name = "modifica precio", skip(precio, pool))]
pub async fn precio_actualiza(
    pool: &PgPool,
    precio: &Precio,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"UPDATE precios 
        SET precio=$2
        WHERE id=$1"#,
        id,
        precio.precio,
    )
    .execute(pool)
    .await?;
    Ok(())
}
