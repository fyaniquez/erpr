//! src/rutas/capitulo/cambia/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de miodificacion de capitulo

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::capitulo::{Capitulo, CapituloError};
use anyhow::Context;

#[tracing::instrument(name="Cambia capitulo", skip(pool))]
#[get("/capitulo/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, CapituloError> {
    let (id,) = path.into_inner();
    let capitulo = obtiene(&pool, id).await
        .context("Error al leer capítulo")?;
    let pagina = layout::form::crea(
        "Capítulo", "/capitulos", "form.css", 
        Some("/capitulo/cambia.js"), contenido(&capitulo));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(capitulo: &Capitulo) -> Markup { html! {
    form method="POST" action={"/capitulo/"(capitulo.id.unwrap())} {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre capítulo" value=(capitulo.nombre);
        label for="descripcion" {"Descripción:" }
        textarea name="descripcion" id="descripcion" required
            placeholder="Descripción" rows="3" {(capitulo.descripcion)}
        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
// modelo
// obtiene un capitulo de la base de datos
#[tracing::instrument(name = "ve capitulo", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Capitulo, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, descripcion FROM capitulos WHERE id=$1";
    let fila: Capitulo = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
