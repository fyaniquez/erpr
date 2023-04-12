//! src/rutas/vendedor/crea/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de alta de vendedor

use actix_web::{web, get, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use crate::domain::vendedor::{
    VendedorError,
};

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub distribuidora: i64
}

#[get("/vendedor")]
pub async fn muestra(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<QueryData>,
) -> Result<HttpResponse, VendedorError> {

    let pagina = layout::form::crea(
        "Vendedor", 
        &format!("/distribuidora/{}/vendedores", query.distribuidora), 
        "form.css", 
        Some("vendedor/crea.js"), contenido(query.distribuidora));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))

}

fn contenido(distribuidora_id: i64) -> Markup { html! {
    @let url = format!("/distribuidora/{}/vendedor", distribuidora_id);
    form method="POST" action=(url) {
        .form-fila {
            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre vendedor";
        }
        .form-fila {
            label for="cargo" {"Cargo:" }
            select #cargo name="cargo" {
                option value="Preventista" selected {"Preventista"}
                option value="Camionero" {"Camionero"}
                option value="Supervisor" {"Supervisor"}
            }
        }
        .form-fila {
            label for="contactos" {"Contactos:" }
            input type="text" name="contactos" id="contactos" required
                placeholder="Contactos";
        }
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
