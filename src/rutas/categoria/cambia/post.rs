//! src/rutas/categoria/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de categoria

use crate::domain::CategoriaNombre;
use crate::modelo::categoria::{Categoria, CategoriaError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Categoria {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = CategoriaNombre::parse(form_data.nombre)?;
        Ok( Self{ 
            id: None, 
            nombre: String::from(nombre.as_ref()), 
            capitulo_id: 0,
        })
    }
}

// extrae datos del categoria del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de categoria",
    skip(form, pool),
    fields( 
        categoria_nombre = %form.nombre,
    )
)]
#[post("/categoria/{id}")]
pub async fn categoria_cambia(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CategoriaError> {
    let (id,) = path.into_inner();
    let categoria = form.0.try_into().map_err(CategoriaError::Validacion)?;
    categoria_actualiza(&pool, &categoria, id)
        .await
        .context("Error al actualizar categoria en la BD")?;
    let url_ver =  format!("/categoria/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un categoria en la base de datos
#[tracing::instrument(name = "modifica categoria", skip(categoria, pool))]
pub async fn categoria_actualiza(
    pool: &PgPool,
    categoria: &Categoria,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE categorias SET nombre=$1 WHERE id=$2",
        &categoria.nombre,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
