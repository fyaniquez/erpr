//! src/rutas/capitulo/alta/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de alta de capitulo

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/capitulo")]
pub async fn capitulo_crea_form() -> AwResult<Markup> {
    layout::form::crea("Crear capitulo", "form.css", None, contenido())
}

fn contenido() -> Markup { html! {
    .form-box {
        form-titulo { "CAPITULO" }
        form method="POST" action="/capitulo" {
            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre capítulo";
            label for="descripcion" {"Descripción:" }
            textarea name="descripcion" id="descripcion" required
                placeholder="Descripción" rows="3" { " " }
            button .form-submit type="submit" { "Crear" }
        }
    }
}}
