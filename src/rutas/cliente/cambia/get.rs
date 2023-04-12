//! src/rutas/cliente/cambia/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de miodificacion de cliente

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::cliente::{Cliente, ClienteError};
use anyhow::Context;

#[tracing::instrument(name="Cambia cliente", skip(pool))]
#[get("/cliente/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, ClienteError> {

    let (id,) = path.into_inner();

    let cliente = obtiene(&pool, id).await
        .context("Error al leer cliente")?;

    let pagina = layout::form::crea(
        "Cliente", "/clientes", "form.css", 
        Some("/cliente/cambia.js"), contenido(&cliente));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(cliente: &Cliente) -> Markup { html! {
    form method="POST" action={"/cliente/"(cliente.id.unwrap())} {
        .form-fila {
            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre capítulo" value=(cliente.nombre);
        }
        .form-fila {
            label for="documento" {"Documento:" }
            input type="text" name="documento" id="documento" required
                placeholder="Documento" value=(cliente.documento);
        }
        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
// modelo
// obtiene un cliente de la base de datos
#[tracing::instrument(name = "ve cliente", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Cliente, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, documento FROM clientes WHERE id=$1";
    let fila: Cliente = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
