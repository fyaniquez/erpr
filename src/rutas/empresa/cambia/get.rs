//! src/rutas/empresa/cambia/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de miodificacion de empresa

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::empresa::{Empresa, EmpresaError};
use anyhow::Context;

#[tracing::instrument(name="Cambia empresa", skip(pool))]
#[get("/empresa/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, EmpresaError> {
    let (id,) = path.into_inner();
    let empresa = obtiene(&pool, id).await
        .context("Error al leer empresa")?;
    let pagina = layout::form::crea(
        "Empresa", "/empresas", "form.css", 
        Some("/empresa/cambia.js"), contenido(&empresa));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(empresa: &Empresa) -> Markup { html! {
    form method="POST" action={"/empresa/"(empresa.id.unwrap())} {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre capítulo" value=(empresa.nombre);

        label for="NIT" {"NIT:" }
        input type="text" name="nit" id="nit" required
            placeholder="# Iden. tributaria" value=(empresa.nit);

        fieldset {
            legend { "¿Está activo?"}
            input type="radio" name="activa" id="activo_si" value="true"
                checked[empresa.activa];
            label for="activo_si" {"Si"}
            input type="radio" name="activa" id="activo_no" value="false"
                checked[!empresa.activa];
            label for="activo_no" {"No"}
            }

   button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
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
