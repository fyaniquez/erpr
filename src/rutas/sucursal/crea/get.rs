//! src/rutas/sucursal/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de sucursal

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use crate::domain::sucursal::{
    SucursalError,
};
use crate::domain::catalogo::{
    Catalogo,
    lista as catalogo_lista,
};
use anyhow::Context;

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub empresa: i64
}

#[get("/sucursal")]
pub async fn muestra(
    pool: web::Data<sqlx::PgPool>, 
    query: web::Query<QueryData>, 
) -> Result<HttpResponse, SucursalError> {

    let catalogos = catalogo_lista(&pool, query.empresa)
        .await
        .context("Error al leer catálogos")
        .unwrap();

    let pagina = layout::form::crea(
        "Sucursal", 
        format!("/empresa/{}/sucursales", query.empresa).as_ref(), 
        "form.css", Some("sucursal/crea.js"), 
        contenido(query.empresa, catalogos));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(empresa_id: i64, catalogos: Vec<Catalogo>) -> Markup { html! {
    form method="POST" action="/sucursal" {

        input type="hidden" name="empresa_id" value=(empresa_id);
        .form-fila {
            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre sucursal";
        }
        .form-fila {
            label for="catalogo_id" {"Catálogo:" }
            select #catalogo_id name="catalogo_id" {
                @for catalogo in catalogos.into_iter() {
                    option value=(catalogo.id.unwrap());
                }
            }
        }

        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
