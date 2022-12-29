//! src/rutas/precio/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de precio

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::precio::{
    Precio, PrecioError, 
    obtiene as precio_obtiene,
};
use anyhow::Context;

#[tracing::instrument(name="Cambia precio", skip(pool))]
#[get("/precio/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, PrecioError> {

    let (id,) = path.into_inner();

    let precio = precio_obtiene(&pool, id).await
        .context("Error al leer precio")?;

    let pagina = layout::form::crea(
        "Precio", 
        format!("/catalogo/{}/precios", precio.catalogo_id).as_ref(), 
        "form.css", Some("precio/cambia.js"), 
        contenido(&precio)
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(precio: &Precio) 
-> Markup { html! {
    form method="POST" action={"/precio/"(precio.id.unwrap())} {

        input type="hidden" name="catalogo_id" value=(precio.catalogo_id);
        input type="hidden" name="producto_id" value=(precio.producto_id);

        label for="producto" {"Producto:" }
        .form-field #precio {(precio.nombre)}

        label for="precio" {"Precio:" }
        input type="text" name="precio" id="precio" required
            placeholder="precio p/venta" value=(precio.precio);

        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
