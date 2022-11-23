//! src/rutas/public/home/get.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: muestra pagina incial a visitante (usuario sin autorización)

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/")]
pub async fn home() -> AwResult<Markup> {
    layout::form::crea("Don coco", "home.css", None, contenido())
}

/// contenido de la página
fn contenido() -> Markup { html! {
    .main {
        section {
        }
    }
}}
