//! src/rutas/fabrica/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de fabrica

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::fabrica::{Fabrica, FabricaError};
use anyhow::Context;

#[tracing::instrument(name="Cambia fabrica", skip(pool))]
#[get("/fabrica/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, FabricaError> {
    let (id,) = path.into_inner();
    let fabrica = obtiene(&pool, id).await
        .context("Error al leer categoría")?;
    let pagina = layout::form::crea(
        "Fabrica", 
        format!("/pais/{}/fabricas", fabrica.pais_id).as_ref(), 
        "form.css", Some("fabrica/cambia.js"), contenido(&fabrica));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(fabrica: &Fabrica) -> Markup { html! {
    form method="POST" action={"/fabrica/"(fabrica.id.unwrap())} {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre capítulo" value=(fabrica.nombre);
        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
// modelo
// obtiene un fabrica de la base de datos
#[tracing::instrument(name = "ve fabrica", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Fabrica, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, pais_id FROM fabricas WHERE id=$1";
    let fila: Fabrica = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
