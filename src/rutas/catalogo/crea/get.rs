//! src/rutas/catalogo/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de catalogo

use crate::domain::catalogo::CatalogoError;
use crate::layout;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub sucursal: i64,
}

#[get("/catalogo")]
pub async fn muestra(query: web::Query<QueryData>) -> Result<HttpResponse, CatalogoError> {
    let pagina = layout::form::crea(
        "Catálogo",
        format!("/sucursal/{}/catalogos", query.sucursal).as_ref(),
        "form.css",
        Some("catalogo/crea.js"),
        contenido(query.sucursal),
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(sucursal_id: i64) -> Markup {
    html! {
        form method="POST" action="/catalogo" {

            input type="hidden" name="sucursal_id" value=(sucursal_id);

            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre catálogo";

            button #crea .form-submit type="submit" { "Crear" }
            button #cancela .form-submit type="button" { "Cancelar" }
        }
    }
}
