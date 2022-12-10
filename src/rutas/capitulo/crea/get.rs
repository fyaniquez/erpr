//! src/rutas/capitulo/crea/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de alta de capitulo

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/capitulo")]
pub async fn muestra() -> AwResult<Markup> {
    layout::form::crea(
        "Capítulo", "/capitulos", "form.css", 
        Some("capitulo/crea.js"), contenido())
}

fn contenido() -> Markup { html! {
    form method="POST" action="/capitulo" {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre capítulo";
        label for="descripcion" {"Descripción:" }
        textarea name="descripcion" id="descripcion" required
            placeholder="Descripción" rows="3" { " " }
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
