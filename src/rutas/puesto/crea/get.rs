//! src/rutas/puesto/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de puesto

use crate::domain::puesto::PuestoError;
use crate::layout;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub sucursal: i64,
}

#[get("/puesto")]
pub async fn muestra(query: web::Query<QueryData>) -> Result<HttpResponse, PuestoError> {
    let pagina = layout::form::crea(
        "Puesto",
        format!("/sucursal/{}/puestos", query.sucursal).as_ref(),
        "form.css",
        Some("puesto/crea.js"),
        contenido(query.sucursal),
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(sucursal_id: i64) -> Markup {
    html! {
        form method="POST" action="/puesto" {

            input type="hidden" name="sucursal_id" value=(sucursal_id);

            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre puesto";

            label for="sigla" {"Sigla:" }
            input type="text" name="sigla" id="sigla" required
                placeholder="Sigla puesto";

            label for="descripcion" {"Descripción:" }
            input type="text" name="descripcion" id="descripcion" required
                placeholder="Descripción puesto";

            button #crea .form-submit type="submit" { "Crear" }
            button #cancela .form-submit type="button" { "Cancelar" }
        }
    }
}
