//! src/rutas/precio/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de precio

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use crate::domain::catalogo::{
    Catalogo, 
    obtiene as catalogo_obtiene,
};
use crate::domain::producto::{
    Producto,
    obtiene as producto_obtiene,
};
use crate::domain::precio::PrecioError;
use anyhow::Context;

#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct QueryData {
    pub catalogo: i64,
    pub producto: i64,
}

#[tracing::instrument(name="lista de productos s/precio", skip(pool))]
#[get("/precio")]
pub async fn muestra(
    query: web::Query<QueryData>, 
    pool: web::Data<sqlx::PgPool>, 
) -> Result<HttpResponse, PrecioError> {

    let catalogo = catalogo_obtiene(&pool, query.catalogo)
        .await
        .context("Error al leer catalogo")?;

    let producto = producto_obtiene(&pool, query.producto)
        .await
        .context("Error al leer producto")?;

    let pagina = layout::form::crea(
        "Precio", 
        format!("/catalogo/{}/precios", query.catalogo).as_ref(), 
        "form.css", Some("precio/crea.js"), 
        contenido(&catalogo, &producto)
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
 
}

fn contenido(catalogo: &Catalogo, producto: &Producto) -> Markup { html! {
    form method="POST" action="/precio" {
        input type="hidden" name="catalogo_id" value=(catalogo.id.unwrap());
        input type="hidden" name="producto_id" value=(producto.id.unwrap());

        label for="nombre" {"Nombre:" }
        .form-field {(producto.nombre)}

        label for="precio" {"Precio:" }
        input type="number" name="precio" id="precio" required
            placeholder="Precio de venta";

        label for="costo" {"Costo:" }
        input type="number" name="costo" id="costo" required
            placeholder="Costo de compra";

        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
