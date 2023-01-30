//! src/rutas/venta/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de venta
//!
use crate::domain::medio::{lista as medio_lista, Medio};

use crate::layout;
use actix_web::Result as AwResult;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;

#[tracing::instrument(name = "formulario crea venta", skip(pool))]
#[get("/venta")]
pub async fn muestra(pool: web::Data<PgPool>) -> AwResult<Markup> {
    let puesto_id: i64 = 1;
    let usuario_id: i64 = 1;
    let catalogo_id: i64 = 1;

    let medios = medio_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    layout::form::crea(
        "Venta",
        &format!("/puest/{}/ventas", puesto_id),
        "maestro-detalle.css",
        Some("venta/crea.js"),
        contenido(medios, puesto_id, usuario_id, catalogo_id),
    )
}

fn contenido(
    medios: Vec<Medio>,
    puesto_id: i64,
    usuario_id: i64,
    catalogo_id: i64,
) -> Markup {
    html! {
        form method="POST" action="/venta" {

            input #puesto_id type="hidden" 
                name="puesto_id" value=(puesto_id);
            input #usuario_id type="hidden" 
                name="usuario_id" value=(usuario_id);
            input #catalogo_id type="hidden" 
                name="catalogo_id" value=(catalogo_id);
            .busqueda-box #busqueda-box {
                .busqueda_titulo #busqueda_titulo {};
                .busqueda #busqueda {};
            }

            (formulario_detalle())

            .maestro-box {
                label for="cliente_id" {"Código:" }
                input type="text" name="cliente_id"
                    id="cliente_id" required placeholder="cliente";

                label for="documento" {"NIT/CI/CEX:" }
                input type="text"
                    id="documento" required placeholder="NIT/CI/CEX";

                label for="nombre" {"Nombres:" }
                input type="text"
                    id="nombre" required placeholder="Nombres/Razón Social";

                label for="subtotal" {"Sub Total:" }
                input type="text"
                    id="subtotal" required placeholder="Suma de items";

                label for="descuento" {"Descuento:" }
                input type="text" name="descuento" id="descuento"
                    required placeholder="Descuento";

                label for="total" {"Total:" }
                input type="text" name="total" id="total"
                    required placeholder="Total a pagar";

                label for="pago" {"Pago:" }
                input type="text" id="pago";

                label for="cambio" {"Cambio:" }
                input type="text" id="cambio";

                select #medio name="medio" {
                    @for medio in medios.into_iter() {
                        option value=(medio.id.unwrap())
                        selected[medio.nombre == "efectivo"]
                            {(medio.nombre)}
                    }
                }
            }

            button #crea .form-submit type="button" { "Crear" }
            button #cancela .form-submit type="button" { "Cancelar" }
        }
    }
}

fn formulario_detalle() -> Markup {
    html! {
        .det-box #detalle {
            .det-fila {
                .det-item {
                    input .det-corto type="text" id="producto_id"
                        placeholder="id prod.";
                    input .det-largo #producto type="text"
                        placeholder="nombre producto";
                    img .det-btn #"borra"
                        src="/img/waste-24.png" alt="Agrega";
                }
                .det-item {
                    .lit-corto #precio {}
                    input .det-corto type="text" id="cantidad"
                        placeholder="cantidad" required;
                    input .det-corto type="text" id="descuento"
                        placeholder="descuento" required;
                    input .det-corto type="text" id="total"
                        placeholder="total" required;
                }
            }
            .det-cell {
                button .btn #agrega_item type="button" {
                    img src="/img/si.png" alt="Agrega";
                }
            }
            .det-cell {
                button .btn #borra_item type="button" {
                    img src="/img/no.png" alt="Cancela";
                }
            }
        }
        (tabla_detalle())
    }
}

fn tabla_detalle() -> Markup {
    html! {
        .det-tabla #det_tabla {
            div {
                div { "id" }
                div { "Producto" }
                div { "Prc." }
                div { "Ctd." }
                div { "Dsc." }
                div { "Tot." }
                div { img src="/img/gear.png"; }
            }

            #det_fila {
                div { }
                div { }
                div { }
                div { }
                div { }
                div { }
                div { img src="/img/waste-24.png"; }
            }

            #totales {
                div { }
                div { "Totales" }
                div { }
                div { }
                #t_descuento { }
                #t_total { }
                div { }
            }
        }
    }
}
