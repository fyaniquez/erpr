//! src/rutas/comprado/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de comprado

use actix_web::Result as AwResult;
use actix_web::{get, web};
use maud::{html, Markup};
use crate::layout;

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub venta: i64
}

#[get("/comprado")]
pub async fn muestra(
    query: web::Query<QueryData>, 
) -> AwResult<Markup> {
    layout::form::crea(
        "Comprados", 
        format!("/venta/{}/comprados", query.venta).as_ref(), 
        "form.css", Some("comprado/crea.js"), contenido(query.venta))
}

fn contenido(venta_id: i64) -> Markup { html! {
    form method="POST" action="/comprado" {
        input type="hidden" name="venta_id" value=(venta_id);

        .form-fila {
            label for="producto_id" {"Producto:" }
            input type="text" name="producto_id" id="producto_id" required
                placeholder="Producto";
        }
        .form-fila {
            label for="cantidad" {"Cantidad:" }
            input type="text" name="cantidad" id="cantidad" required
                placeholder="Cantidad";
        }
        .form-fila {
            label for="precio" {"Precio:" }
            input type="text" name="precio" id="precio" required
                placeholder="Precio";
        }
        .form-fila {
            label for="vencimiento" {"Vencimiento:" }
            input type="text" name="vencimiento" id="vencimiento" required
                placeholder="Vencimiento";
        }
        .form-fila {
            label for="total" {"SubTotal:" }
            input type="text" name="subtotal" id="subtotal" required
                placeholder="Sub Total";
        }
        .form-fila {
            label for="total" {"Descuento:" }
            input type="text" name="descuento" id="descuento" required
                placeholder="Descuento";
        }
        .form-fila {
            label for="total" {"Total:" }
            input type="text" name="total" id="total" required
                placeholder="Total";
        }

        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
