//! src/rutas/venta/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de venta
//!
use crate::domain::cliente::{
    lista as cliente_lista, 
    Cliente
};
use crate::domain::medio::{
    lista as medio_lista, 
    Medio
};

use crate::layout;
use actix_web::Result as AwResult;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;

#[tracing::instrument(name = "formulario crea venta", skip(pool))]
#[get("/venta")]
pub async fn muestra(pool: web::Data<PgPool>) -> AwResult<Markup> {

    let clientes = cliente_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    let medios = medio_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    layout::form::crea(
        "Venta",
        "/ventas",
        "form.css",
        Some("venta/crea.js"),
        contenido( clientes, medios),
    )
}

fn contenido(
    clientes: Vec<Cliente>,
    medios: Vec<Medio>,
) -> Markup { html! {
    form method="POST" action="/venta" {

        label for="cliente_id" {"Cliente:" }
        input type="text" name="cliente_id" id="cliente_id" required
            placeholder="cliente";

        label for="medio" {"Medio de pago:" }
        select #medio name="medio" {
            @for medio in medios.into_iter() {
                option value=(medio.id.unwrap())
                selected[medio.nombre == "efectivo"] 
                    {(medio.nombre)}
            }
        }

        label for="total" {"Total:" }
        input type="text" name="total" id="total"
            required placeholder="Total a pagar";

        label for="descuento" {"Descuento:" }
        input type="text" name="descuento" id="descuento"
            required placeholder="Descuento";

        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
