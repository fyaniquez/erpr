//! src/rutas/cliente/crea/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de alta de cliente

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/cliente")]
pub async fn muestra() -> AwResult<Markup> {
    layout::form::crea(
        "Cliente", "/clientes", "form.css", 
        Some("cliente/crea.js"), contenido())
}

fn contenido() -> Markup { html! {
    form method="POST" action="/cliente" {
        .form-fila {
            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre cliente";
        }
        .form-fila {
            label for="documento" {"Documento:" }
            input type="text" name="documento" id="documento" required
                placeholder="Documento";
        }
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
