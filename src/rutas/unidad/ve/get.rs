//! src/rutas/unidad/ve/get.rs
//! author: fyaniquez
//! date: 09/12/2022
//! purpose: muestra un unidad

use crate::layout;
use crate::domain::unidad::{Unidad, UnidadError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

const OBJETO: &str = "unidad";
const OBJETOS: &str = "unidades";
const LOCAL: &str = "unidad";
const LOCAL_MAYUSCULA: &str = "Unidad";

// controlador
#[tracing::instrument(name="Ve unidad", skip(pool))]
#[get("/unidad/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UnidadError> {
    let (id,) = path.into_inner();
    let unidad = obtiene(&pool, id).await
        .context(format!("Error al leer {}", LOCAL))?;

    let pagina = layout::form::crea(
        LOCAL_MAYUSCULA, &format!("/{}", OBJETOS), 
        "ve.css", Some(&format!("{}/ve.js", OBJETO)), 
        contenido(unidad));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(unidad: Unidad) -> Markup { html! {
    .ve-label {strong{"Nombre: " } (unidad.nombre)}
    .ve-label {strong{"Sigla: " } (unidad.sigla)}
    button .form-submit #sublista type="button" { "Productos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
// obtiene un unidad de la base de datos
#[tracing::instrument(name = "ve unidad", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Unidad, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, sigla FROM unidades WHERE id=$1";
    let fila: Unidad = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
