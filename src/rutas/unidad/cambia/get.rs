//! src/rutas/unidad/cambia/get.rs
//! author: fyaniquez
//! date: 09/12/2022
//! purpose: muestra el formulario de modificacion de unidad

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::unidad::{Unidad, UnidadError};
use anyhow::Context;

    const ERROR_QRY: &str = "Error al leer unidades de la BD";
    const LOCAL_MAYUSCULA: &str = "Unidad";
    const OBJETOS_URL: &str = "/unidades";
    const OBJETO_JS: &str = "/unidad/cambia.js";

#[tracing::instrument(name="Cambia unidad", skip(pool))]
#[get("/unidad/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, UnidadError> {
    let (id,) = path.into_inner();
    let unidad = obtiene(&pool, id).await
        .context(ERROR_QRY)?;
    let pagina = layout::form::crea(
        LOCAL_MAYUSCULA, OBJETOS_URL, "form.css", 
        Some(OBJETO_JS), contenido(&unidad));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(unidad: &Unidad) -> Markup { html! {
    form method="POST" action={"/unidad/"(unidad.id.unwrap())} {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre país" value=(unidad.nombre);
        label for="sigla" {"Sigla:" }
        input type="text" name="sigla" id="sigla" required
            placeholder="Sigla país" value=(unidad.sigla);
        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
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
