//! src/rutas/distribuidora/crea/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de alta de distribuidora

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/distribuidora")]
pub async fn muestra() -> AwResult<Markup> {
    layout::form::crea(
        "Distribuidora", "/distribuidoras", "form.css", 
        Some("distribuidora/crea.js"), contenido())
}

fn contenido() -> Markup { html! {
    form method="POST" action="/distribuidora" {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre distribuidora";

        label for="nit" {"NIT:" }
        input type="text" name="nit" id="nit" required
            placeholder="# Iden. Tributaria";

        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
