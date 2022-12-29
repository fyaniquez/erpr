//! src/rutas/inventariado/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de inventariado

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::inventariado::{
    Inventariado, InventariadoError, 
    obtiene as inventariado_obtiene,
};
use anyhow::Context;

#[tracing::instrument(name="Cambia inventariado", skip(pool))]
#[get("/inventariado/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, InventariadoError> {

    let (id,) = path.into_inner();

    let inventariado = inventariado_obtiene(&pool, id).await
        .context("Error al leer inventariado")?;

    let pagina = layout::form::crea(
        "Inventariado", 
        format!("/inventario/{}/inventariados", inventariado.inventario_id).as_ref(), 
        "form.css", Some("inventariado/cambia.js"), 
        contenido(&inventariado)
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(inventariado: &Inventariado) 
-> Markup { html! {
    form method="POST" action={"/inventariado/"(inventariado.id.unwrap())} {

        input type="hidden" name="inventario_id" 
            value=(inventariado.inventario_id);
        input type="hidden" name="producto_id" 
            value=(inventariado.producto_id);

        label for="producto" {"Producto:" }
        .form-field #inventariado {(inventariado.nombre)}

        label for="cantidad" {"Cantidad:" }
        input type="text" name="cantidad" id="cantidad"
            placeholder="cantidad en existencia" 
            value=(inventariado.cantidad);

        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
