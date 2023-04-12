//! src/rutas/catalogo/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de catalogo

use crate::domain::catalogo::{obtiene as catalogo_obtiene, Catalogo, CatalogoError};
use crate::layout;
use actix_web::{get, web, HttpResponse};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

#[tracing::instrument(name = "Cambia catalogo", skip(pool))]
#[get("/catalogo/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, CatalogoError> {
    let (id,) = path.into_inner();

    let catalogo = catalogo_obtiene(&pool, id)
        .await
        .context("Error al leer catálogo")?;

    let pagina = layout::form::crea(
        "Catálogo",
        format!("/sucursal/{}/catalogos", catalogo.sucursal_id).as_ref(),
        "form.css",
        Some("catalogo/cambia.js"),
        contenido(&catalogo),
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(catalogo: &Catalogo) -> Markup {
    html! {
        form method="POST" action={"/catalogo/"(catalogo.id.unwrap())} {

            input type="hidden" name="sucursal_id" value=(catalogo.sucursal_id);
            .form-fila {
                label for="nombre" {"Nombre:" }
                input type="text" name="nombre" id="nombre" required
                    placeholder="Nombre catálogo" value=(catalogo.nombre);
            }

            button .form-submit #graba type="submit" { "Graba" }
            button .form-submit #cancela type="button" { "Cancela" }
        }
    }
}
