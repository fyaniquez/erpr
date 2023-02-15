//! src/rutas/distribuidora/cambia/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de miodificacion de distribuidora

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::distribuidora::{Distribuidora, DistribuidoraError};
use anyhow::Context;

#[tracing::instrument(name="Cambia distribuidora", skip(pool))]
#[get("/distribuidora/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, DistribuidoraError> {
    let (id,) = path.into_inner();
    let distribuidora = obtiene(&pool, id).await
        .context("Error al leer distribuidora")?;
    let pagina = layout::form::crea(
        "Distribuidora", "/distribuidoras", "form.css", 
        Some("/distribuidora/cambia.js"), contenido(&distribuidora));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(distribuidora: &Distribuidora) -> Markup { html! {
    form method="POST" action={"/distribuidora/"(distribuidora.id.unwrap())} {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre capítulo" value=(distribuidora.nombre);

        label for="NIT" {"NIT:" }
        input type="text" name="documento" id="documento" required
            placeholder="# Iden. tributaria" value=(distribuidora.documento);

        fieldset {
            legend { "¿Está activo?"}
            input type="radio" name="activa" id="activo_si" value="true"
                checked[distribuidora.activa];
            label for="activo_si" {"Si"}
            input type="radio" name="activa" id="activo_no" value="false"
                checked[!distribuidora.activa];
            label for="activo_no" {"No"}
            }

   button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
// modelo
// obtiene un distribuidora de la base de datos
#[tracing::instrument(name = "ve distribuidora", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Distribuidora, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, documento, activa FROM distribuidoras WHERE id=$1";
    let fila: Distribuidora = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
