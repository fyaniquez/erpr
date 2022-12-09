//! src/rutas/categoria/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el formulario de modificacion de categoria

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::modelo::categoria::{Categoria, CategoriaError};
use anyhow::Context;

#[tracing::instrument(name="Cambia categoria", skip(pool))]
#[get("/categoria/{id}/cambia")]
pub async fn categoria_cambia_form(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, CategoriaError> {
    let (id,) = path.into_inner();
    let categoria = obtiene(&pool, id).await
        .context("Error al leer categoría")?;
    let pagina = layout::form::crea(
        "Categoría", 
        format!("/capitulo/{}/categorias", categoria.capitulo_id).as_ref(), 
        "form.css", Some("categoria/cambia.js"), contenido(&categoria));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(categoria: &Categoria) -> Markup { html! {
    form method="POST" action={"/categoria/"(categoria.id.unwrap())} {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre capítulo" value=(categoria.nombre);
        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
// modelo
// obtiene un categoria de la base de datos
#[tracing::instrument(name = "ve categoria", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Categoria, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, capitulo_id FROM categorias WHERE id=$1";
    let fila: Categoria = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
