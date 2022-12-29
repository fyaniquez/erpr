//! src/rutas/inventario/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de inventario

use crate::domain::inventario::Nombre;
use crate::domain::inventario::{Inventario, InventarioError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use chrono::Utc;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Inventario {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        Ok( Self { 
            id: None, 
            nombre: String::from(nombre.as_ref()), 
            sucursal_id: 0,
            fecha: Utc::now().naive_utc(),
            estado: String::from(""),
        })
    }
}

// extrae datos del inventario del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de inventario",
    skip(form, pool),
    fields( 
        inventario_nombre = %form.nombre,
    )
)]
#[post("/inventario/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, InventarioError> {

    let (id,) = path.into_inner();

    let inventario = form.0.try_into()
        .map_err(InventarioError::Validacion)?;

    inventario_actualiza(&pool, &inventario, id)
        .await
        .context("Error al actualizar inventario en la BD")?;

    let url_ver =  format!("/inventario/{}", id);

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un inventario en la base de datos
#[tracing::instrument(name = "modifica inventario", skip(inventario, pool))]
pub async fn inventario_actualiza(
    pool: &PgPool,
    inventario: &Inventario,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        r#"UPDATE inventarios 
        SET nombre=$2
        WHERE id=$1"#)
        .bind(id)
        .bind(&inventario.nombre)
    
    .execute(pool)
    .await?;
    Ok(())
}
