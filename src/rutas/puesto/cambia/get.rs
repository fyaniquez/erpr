//! src/rutas/puesto/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de puesto

use crate::domain::puesto::{obtiene as puesto_obtiene, Puesto, PuestoError};
use crate::layout;
use actix_web::{get, web, HttpResponse};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

#[tracing::instrument(name = "Cambia puesto", skip(pool))]
#[get("/puesto/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, PuestoError> {
    let (id,) = path.into_inner();

    let puesto = puesto_obtiene(&pool, id)
        .await
        .context("Error al leer catálogo")?;

    let pagina = layout::form::crea(
        "Puesto",
        format!("/sucursal/{}/puestos", puesto.sucursal_id).as_ref(),
        "form.css",
        Some("puesto/cambia.js"),
        contenido(&puesto),
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(puesto: &Puesto) -> Markup {
    html! {
        form method="POST" action={"/puesto/"(puesto.id.unwrap())} {

            input type="hidden" name="sucursal_id" value=(puesto.sucursal_id);

            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre puesto" value=(puesto.nombre);

            label for="sigla" {"Sigla:" }
            input type="text" name="sigla" id="sigla" required
                placeholder="Sigla puesto" value=(puesto.sigla);

            label for="descripcion" {"Descripción:" }
            input type="text" name="descripcion" id="descripcion" required
                placeholder="Descripción" value=(puesto.descripcion);

            button .form-submit #graba type="submit" { "Graba" }
            button .form-submit #cancela type="button" { "Cancela" }
        }
    }
}
