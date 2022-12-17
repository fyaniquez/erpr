//! src/rutas/catalogo/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de catalogo

use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::{html, Markup};
use crate::layout;
use crate::domain::empresa::{
    Empresa, lista as empresa_lista,
};
use anyhow::Context;

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub empresa: i64
}

#[get("/catalogo")]
pub async fn muestra(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<QueryData>, 
) -> AwResult<Markup> {

    let empresas = empresa_lista(&pool)
        .await
        .context("Error al leer empresas")
        .unwrap();

    layout::form::crea(
        "Catálogo", 
        format!("/empresa/{}/catalogos", query.empresa).as_ref(), 
        "form.css", Some("catalogo/crea.js"), 
        contenido(query.empresa, empresas))
}

fn contenido(empresa_id: i64, propietarios: Vec<Empresa>) 
-> Markup { html! {
    form method="POST" action="/catalogo" {

        input type="hidden" name="empresa_id" value=(empresa_id);

        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre catálogo";

        label for="propietario" {"Propietario:" }
        select #propietario name="propietario" {
            @for propietario in propietarios.into_iter() {
                 option value=(propietario.id.unwrap())
                 {(propietario.nombre)}
            }
        }

        label for="nombre" {"Fecha:" }
        input type="date" name="fecha" id="fecha" required
            placeholder="Fecha vigencia";

        fieldset {
            legend { "¿Está activo?"}
            input type="radio" name="activo" id="activo_si" 
                value="true" checked;
            label for="activo_si" {"Si"}
            input type="radio" name="activo" id="activo_no" 
                value="false";
            label for="activo_no" {"No"}
       }

        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
