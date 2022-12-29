//! src/rutas/inventario/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de inventario

use crate::domain::inventario::{obtiene as inventario_obtiene, Inventario, InventarioError};
use crate::layout;
use actix_web::{get, web, HttpResponse};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

#[tracing::instrument(name = "Cambia inventario", skip(pool))]
#[get("/inventario/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, InventarioError> {
    let (id,) = path.into_inner();

    let inventario = inventario_obtiene(&pool, id)
        .await
        .context("Error al leer inventario")?;

    let pagina = layout::form::crea(
        "Inventario",
        format!("/sucursal/{}/inventarios", inventario.sucursal_id).as_ref(),
        "form.css",
        Some("inventario/cambia.js"),
        contenido(&inventario),
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(inventario: &Inventario) -> Markup {
    html! {
        form method="POST" action={"/inventario/"(inventario.id.unwrap())} {

            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre inventario" value=(inventario.nombre);

            button .form-submit #graba type="submit" { "Graba" }
            button .form-submit #cancela type="button" { "Cancela" }
        }
    }
}
