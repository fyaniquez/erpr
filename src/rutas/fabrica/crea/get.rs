//! src/rutas/fabrica/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de fabrica

use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::{html, Markup};
use crate::layout;

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub pais: i64
}

#[get("/fabrica")]
pub async fn muestra(
    query: web::Query<QueryData>, 
) -> AwResult<Markup> {
    layout::form::crea(
        "Fábrica", 
        format!("/pais/{}/fabricas", query.pais).as_ref(), 
        "form.css", Some("fabrica/crea.js"), contenido(query.pais))
}

fn contenido(pais_id: i64) -> Markup { html! {
    form method="POST" action="/fabrica" {
        input type="hidden" name="pais_id" value=(pais_id);
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre categoría";
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
