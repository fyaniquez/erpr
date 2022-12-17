//! src/rutas/catalogo/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de catalogo

use crate::domain::catalogo::Nombre;
use crate::domain::catalogo::{Catalogo, CatalogoError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use chrono::NaiveDateTime;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    propietario: i64,
    empresa_id: i64,
    fecha: NaiveDateTime,
    activo: bool,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Catalogo {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        Ok( Self{ 
            id: None, 
            nombre: String::from(nombre.as_ref()), 
            propietario: form_data.propietario,
            empresa_id: form_data.empresa_id,
            fecha: form_data.fecha,
            activo: form_data.activo,
        })
    }
}

// extrae datos del catalogo del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de catalogo",
    skip(form, pool),
    fields( 
        catalogo_nombre = %form.nombre,
    )
)]
#[post("/catalogo/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, CatalogoError> {
    let (id,) = path.into_inner();
    let catalogo = form.0.try_into().map_err(CatalogoError::Validacion)?;
    catalogo_actualiza(&pool, &catalogo, id)
        .await
        .context("Error al actualizar catalogo en la BD")?;
    let url_ver =  format!("/catalogo/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un catalogo en la base de datos
#[tracing::instrument(name = "modifica catalogo", skip(catalogo, pool))]
pub async fn catalogo_actualiza(
    pool: &PgPool,
    catalogo: &Catalogo,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"UPDATE catalogos 
        SET nombre=$2, propietario=$3, empresa_id=$4, fecha=$5, activo=$6
        WHERE id=$1"#,
        id,
        &catalogo.nombre,
        catalogo.propietario,
        catalogo.empresa_id,
        catalogo.fecha,
        catalogo.activo
    )
    .execute(pool)
    .await?;
    Ok(())
}
