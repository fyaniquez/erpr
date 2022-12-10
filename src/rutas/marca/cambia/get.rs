//! src/rutas/marca/cambia/get.rs
//! author: fyaniquez
//! date: 09/12/2022
//! purpose: muestra el formulario de modificacion de marca

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::modelo::marca::{Marca, MarcaError};
use anyhow::Context;

#[tracing::instrument(name="Cambia marca", skip(pool))]
#[get("/marca/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, MarcaError> {
    let (id,) = path.into_inner();
    let marca = obtiene(&pool, id).await
        .context("Error al leer marcas de la BD")?;
    let pagina = layout::form::crea(
        "País", "/marcas", "form.css", 
        Some("/marca/cambia.js"), contenido(&marca));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(marca: &Marca) -> Markup { html! {
    form method="POST" action={"/marca/"(marca.id.unwrap())} {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre país" value=(marca.nombre);
        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
// modelo
// obtiene un marca de la base de datos
#[tracing::instrument(name = "ve marca", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Marca, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre FROM marcas WHERE id=$1";
    let fila: Marca = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
