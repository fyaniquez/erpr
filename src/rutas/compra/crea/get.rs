//! src/rutas/compra/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de compra
//!
use crate::domain::medio::{
    lista as medio_lista, 
    Medio
};
use crate::domain::distribuidora::{
    obtiene as distribuidora_obtiene, 
    Distribuidora
};

use crate::layout;
use actix_web::Result as AwResult;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;

#[tracing::instrument(name = "formulario crea compra", skip(pool))]
#[get("/distribuidora/{id}/compra")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> AwResult<Markup> {
    let sucursal_id: i64 = 1;
    let usuario_id: i64 = 1;
    let catalogo_id: i64 = 1;

    let (distribuidora_id,) = path.into_inner();
    let distribuidora = distribuidora_obtiene(&pool, distribuidora_id)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    let medios = medio_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    layout::form::crea(
        "Compra",
        &format!("/distribuidora/{}/compras", distribuidora_id),
        "maestro-detalle.css",
        Some("compra/crea.js"),
        contenido(medios, sucursal_id, usuario_id, &distribuidora),
    )
}

fn contenido(
    medios: Vec<Medio>,
    sucursal_id: i64,
    usuario_id: i64,
    distribuidora: &Distribuidora,
) -> Markup {
    html! {
        form method="POST" action="/compra" {

            input #sucursal_id type="hidden" 
                name="sucursal_id" value=(sucursal_id);
            input #usuario_id type="hidden" 
                name="usuario_id" value=(usuario_id);
            input #distribuidora_id type="hidden" 
                name="distribuidora_id" value=(distribuidora.id.unwrap());

            .busqueda-box #busqueda-box {
                .busqueda_titulo #busqueda_titulo {};
                .busqueda #busqueda {};
            }
            .nombre-largo {(distribuidora.nombre)}

            (formulario_detalle())

            .maestro-box {
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

                label for="documento" {"Factura:" }
                input type="text"
                    id="documento" required placeholder="Factura/Recibo";

                label for="observaciones" {"Observaciones:" }
                input type="text"
                    id="observaciones" required 
                    placeholder="Datos adicionales respecto a la compra";

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
                    input .det-corto type="text" id="costo"
                        placeholder="costo" required;
                    input .det-corto type="text" id="cantidad"
                        placeholder="cantidad" required;
                    input .det-corto type="text" id="descuento_detalle"
                        placeholder="descuento" required;
                    input .det-corto type="text" id="total_detalle"
                        placeholder="total" required;
                    input .det-corto type="date" id="vencimiento"
                        placeholder="vencimiento" required;
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
                div { "Ven." }
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
                div { }
                div { img src="/img/waste-24.png"; }
            }

            #totales {
                div { }
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
