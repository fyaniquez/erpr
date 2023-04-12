//! src/rutas/fabrica/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de fabrica

use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub pais: i64
}

#[get("/pais/{id}/fabrica")]
pub async fn muestra(
    path: web::Path<(i64,)>, 
    pool: web::Data<PgPool>,
) -> AwResult<Markup> {
    let (id,) = path.into_inner();
    let url = format!("/pais/{}/fabrica", id); 
    layout::form::crea(
        "Fábrica", &url, 
        "form.css", Some("fabrica/crea.js"), contenido(&url))
}

fn contenido(url: &str) -> Markup { html! {
    form method="POST" action=(url) {
        .form-fila {
            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre fabrica";
        }
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
