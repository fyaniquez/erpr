//! src/rutas/inventario/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de inventario

use crate::domain::inventario::InventarioError;
use crate::layout;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
#[derive(serde::Deserialize)]
pub struct QueryData {
    pub sucursal: i64,
}

#[get("/inventario")]
pub async fn muestra(query: web::Query<QueryData>) -> Result<HttpResponse, InventarioError> {
    let pagina = layout::form::crea(
        "Inventario",
        format!("/sucursal/{}/inventarios", query.sucursal).as_ref(),
        "form.css",
        Some("inventario/crea.js"),
        contenido(query.sucursal),
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(sucursal_id: i64) -> Markup {
    html! {
        form method="POST" action="/inventario" {

            input type="hidden" name="sucursal_id" value=(sucursal_id);
            .form-fila {
                label for="nombre" {"Nombre:" }
                input type="text" name="nombre" id="nombre" required
                    placeholder="Nombre inventario";
            }
            button #crea .form-submit type="submit" { "Crear" }
            button #cancela .form-submit type="button" { "Cancelar" }
        }
    }
}
