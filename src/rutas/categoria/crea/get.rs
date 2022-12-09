//! src/rutas/categoria/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de categoria

use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::{html, Markup};
use crate::layout;

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub capitulo: i64
}

#[get("/categoria")]
pub async fn categoria_crea_form(
    query: web::Query<QueryData>, 
) -> AwResult<Markup> {
    layout::form::crea(
        "Capítulo", 
        format!("/capitulo/{}/categorias", query.capitulo).as_ref(), 
        "form.css", Some("categoria/crea.js"), contenido(query.capitulo))
}

fn contenido(capitulo_id: i64) -> Markup { html! {
    form method="POST" action="/categoria" {
        input type="hidden" name="capitulo_id" value=(capitulo_id);
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre categoría";
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
