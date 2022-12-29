//! src/rutas/medio/crea/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de alta de medio

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/medio")]
pub async fn muestra() -> AwResult<Markup> {
    layout::form::crea(
        "Medio", "/medios", "form.css", 
        Some("medio/crea.js"), contenido())
}

fn contenido() -> Markup { html! {
    form method="POST" action="/medio" {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre país";
        label for="sigla" {"Sigla:" }
        input type="text" name="sigla" id="sigla" required
            placeholder="Sigla país";
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
