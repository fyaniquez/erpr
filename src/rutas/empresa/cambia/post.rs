//! src/rutas/empresa/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de empresa

use crate::domain::empresa::{
    Nombre, 
    Nit,
};
use crate::domain::empresa::{Empresa, EmpresaError};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    nombre: String,
    nit: String,
    activa: bool,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Empresa {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let nombre = Nombre::parse(form_data.nombre)?;
        let nit = Nit::parse(form_data.nit)?;
        Ok( Self{ 
            id:None, 
            nombre: String::from(nombre.as_ref()), 
            nit: String::from(nit.as_ref()), 
            activa: form_data.activa,
        })
    }
}

// extrae datos del empresa del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de empresa",
    skip(form, pool),
    fields( 
        empresa_nombre = %form.nombre,
        empresa_nit = %form.nit,
        empresa_activa = %form.activa,
    )
)]
#[post("/empresa/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> Result<HttpResponse, EmpresaError> {
    let (id,) = path.into_inner();
    let empresa = form.0.try_into().map_err(EmpresaError::Validacion)?;
    empresa_actualiza(&pool, &empresa, id)
        .await
        .context("Error al actualizar empresa en la BD")?;
    let url_ver =  format!("/empresa/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

// inserta un empresa en la base de datos
#[tracing::instrument(name = "modifica empresa", skip(empresa, pool))]
pub async fn empresa_actualiza(
    pool: &PgPool,
    empresa: &Empresa,
    id: i64,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        "UPDATE empresas SET nombre=$1, nit=$2, activa=$3 WHERE id=$4",
        &empresa.nombre,
        &empresa.nit,
        empresa.activa,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
