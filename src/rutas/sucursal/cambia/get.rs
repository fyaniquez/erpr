//! src/rutas/sucursal/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de sucursal

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::sucursal::{
    Sucursal, 
    SucursalError,
    obtiene,
};
use crate::domain::catalogo::{
    Catalogo,
    lista as catalogo_lista,
};
use anyhow::Context;

#[tracing::instrument(name="Cambia sucursal", skip(pool))]
#[get("/sucursal/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, SucursalError> {

    let (id,) = path.into_inner();

    let sucursal = obtiene(&pool, id).await
        .context("Error al leer sucursal")?;

    let catalogos = catalogo_lista(&pool, sucursal.empresa_id)
        .await
        .context("Error al leer catálogos")
        .unwrap();

    let pagina = layout::form::crea(
        "Sucursal", 
        format!("/empresa/{}/sucursales", sucursal.empresa_id).as_ref(), 
        "form.css", Some("sucursal/cambia.js"), 
        contenido(&sucursal, catalogos));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(sucursal: &Sucursal, catalogos: Vec<Catalogo>) 
-> Markup { html! {
    form method="POST" action={"/sucursal/"(sucursal.id.unwrap())} {

        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre sucursal" value=(sucursal.nombre);

        label for="catalogo_id" {"Catálogo:" }
        select #catalogo_id name="catalogo_id" {
            @for catalogo in catalogos.into_iter() {
                option value=(catalogo.id.unwrap())
                selected[catalogo.id == Some(sucursal.catalogo_id)]
                    {(catalogo.nombre)}
            }
        }

        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
