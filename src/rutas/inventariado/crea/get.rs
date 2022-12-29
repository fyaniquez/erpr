//! src/rutas/inventariado/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de inventariado

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use crate::domain::inventario::{
    Inventario, 
    obtiene as inventario_obtiene,
};
use crate::domain::producto::{
    Producto,
    obtiene as producto_obtiene,
};
use crate::domain::inventariado::InventariadoError;
use anyhow::Context;

#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct QueryData {
    pub inventario: i64,
    pub producto: i64,
}

#[tracing::instrument(name="lista de productos s/inventariado", skip(pool))]
#[get("/inventariado")]
pub async fn muestra(
    query: web::Query<QueryData>, 
    pool: web::Data<sqlx::PgPool>, 
) -> Result<HttpResponse, InventariadoError> {

    let inventario = inventario_obtiene(&pool, query.inventario)
        .await
        .context("Error al leer inventario")?;

    let producto = producto_obtiene(&pool, query.producto)
        .await
        .context("Error al leer producto")?;

    let pagina = layout::form::crea(
        "Inventariado", 
        format!("/inventario/{}/inventariados", query.inventario).as_ref(), 
        "form.css", Some("inventariado/crea.js"), 
        contenido(&inventario, &producto)
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
 
}

fn contenido(inventario: &Inventario, producto: &Producto) -> Markup { html! {
    form method="POST" action="/inventariado" {
        input type="hidden" name="inventario_id" 
            value=(inventario.id.unwrap());
        input type="hidden" name="producto_id" value=(producto.id.unwrap());

        label for="nombre" {"Nombre:" }
        .form-field {(producto.nombre)}

        label for="cantidad" {"Cantidad:" }
        input type="number" name="cantidad" id="cantidad" required
            placeholder="Cantidad en existencia";

        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
