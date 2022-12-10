//! src/rutas/pais/ve/get.rs
//! author: fyaniquez
//! date: 09/12/2022
//! purpose: muestra un pais

use crate::layout;
use crate::modelo::pais::{Pais, PaisError};
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

const OBJETO: &str = "pais";
const OBJETOS: &str = "paises";
const LOCAL: &str = "país";
const LOCAL_MAYUSCULA: &str = "País";

// controlador
#[tracing::instrument(name="Ve pais", skip(pool))]
#[get("/pais/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PaisError> {
    let (id,) = path.into_inner();
    let pais = obtiene(&pool, id).await
        .context(format!("Error al leer {}", LOCAL))?;

    let pagina = layout::form::crea(
        LOCAL_MAYUSCULA, &format!("/{}", OBJETOS), 
        "form.css", Some(&format!("{}/ve.js", OBJETO)), 
        contenido(pais));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(pais: Pais) -> Markup { html! {
    .form-label {"Nombre:" }
    .form-field #nombre {(pais.nombre)}
    .form-label {"Sigla:" }
    .form-field #sigla {(pais.sigla)}
    button .form-submit #hijos type="button" { "Fábricas" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

// modelo
// obtiene un pais de la base de datos
#[tracing::instrument(name = "ve pais", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Pais, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, sigla FROM paises WHERE id=$1";
    let fila: Pais = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
