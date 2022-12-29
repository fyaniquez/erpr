//! src/rutas/medio/cambia/get.rs
//! author: fyaniquez
//! date: 09/12/2022
//! purpose: muestra el formulario de modificacion de medio

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::medio::{Medio, MedioError};
use anyhow::Context;

    const ERROR_QRY: &str = "Error al leer medios de la BD";
    const LOCAL_MAYUSCULA: &str = "Medio";
    const OBJETOS_URL: &str = "/medios";
    const OBJETO_JS: &str = "/medio/cambia.js";

#[tracing::instrument(name="Cambia medio", skip(pool))]
#[get("/medio/{id}/cambia")]
pub async fn muestra(

    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, MedioError> {
    let (id,) = path.into_inner();
    let medio = obtiene(&pool, id).await
        .context(ERROR_QRY)?;
    let pagina = layout::form::crea(
        LOCAL_MAYUSCULA, OBJETOS_URL, "form.css", 
        Some(OBJETO_JS), contenido(&medio));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(medio: &Medio) -> Markup { html! {
    form method="POST" action={"/medio/"(medio.id.unwrap())} {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre medio" value=(medio.nombre);
        label for="sigla" {"Sigla:" }
        input type="text" name="sigla" id="sigla" required
            placeholder="Sigla medio" value=(medio.sigla);
        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
// modelo
// obtiene un medio de la base de datos
#[tracing::instrument(name = "ve medio", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Medio, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, sigla FROM medios WHERE id=$1";
    let fila: Medio = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
