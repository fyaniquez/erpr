//! src/rutas/marca/crea/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de alta de marca

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/marca")]
pub async fn muestra() -> AwResult<Markup> {
    layout::form::crea(
        "Marca", "/marcas", "form.css", 
        Some("marca/crea.js"), contenido())
}

fn contenido() -> Markup { html! {
    form method="POST" action="/marca" {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre país";
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
