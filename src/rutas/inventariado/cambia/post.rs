//! src/rutas/inventariado/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de inventariado

use crate::domain::inventariado::{
    Inventariado, 
    InventariadoError,
    actualiza as inventariado_actualiza,
};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    cantidad: f32,
    vencimiento: chrono::NaiveDate,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Inventariado {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        Ok( Self{ 
            id: None, 
            nombre: String::from(""),
            producto_id: 0,
            vencimiento: Some(form_data.vencimiento),
            cantidad: (form_data.cantidad * 100.0) as i32,
            inventario_id: 0,
        })
    }
}

// extrae datos del inventariado del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de inventariado",
    skip(form, pool),
)]
#[post("/inventariado/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, InventariadoError> {
    let (id,) = path.into_inner();
    let inventariado = 
        form.0.try_into().map_err(InventariadoError::Validacion)?;
    inventariado_actualiza(&pool, &inventariado, id)
        .await
        .context("Error al actualizar inventariado en la BD")?;
    let url_ver =  format!("/inventariado/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}


