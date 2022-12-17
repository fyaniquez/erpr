//! src/rutas/empresa/ve/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un empresa

use crate::layout;
use crate::domain::empresa::{Empresa, EmpresaError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

// controlador
#[tracing::instrument(name="Ve empresa", skip(pool))]
#[get("/empresa/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, EmpresaError> {
    let (id,) = path.into_inner();
    let empresa = obtiene(&pool, id).await
        .context("Error al leer empresa")?;

    let pagina = layout::form::crea(
        "Empresa", "/empresas", 
        "form.css", Some("empresa/ve.js"), contenido(empresa));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(empresa: Empresa) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(empresa.nombre)}
    .form-label {"NIT:" }
    .form-field {(empresa.nit)}
    .form-label {"Activa:" }
    .form-field {@if empresa.activa {"Sí"} @else {"No"}}
    button .form-submit #sublista type="button" { "Catálogos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
// obtiene un empresa de la base de datos
#[tracing::instrument(name = "ve empresa", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Empresa, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, nit, activa FROM empresas WHERE id=$1";
    let fila: Empresa = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
