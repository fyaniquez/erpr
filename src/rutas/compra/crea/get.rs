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
    lista as distribuidora_lista, 
    Distribuidora
};

use crate::layout;
use actix_web::Result as AwResult;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup, PreEscaped};
use sqlx::PgPool;

#[tracing::instrument(name = "formulario crea compra", skip(pool))]
#[get("/compra")]
pub async fn muestra(
    pool: web::Data<PgPool>,
) -> AwResult<Markup> {

    let distribuidoras = distribuidora_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    let medios = medio_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    layout::form::crea(
        "Compra",
        "/compras",
        "compra/crea.css",
        Some("compra/crea.js"),
        contenido(medios, distribuidoras,),
    )
}

fn contenido(
    medios: Vec<Medio>,
    distribuidoras: Vec<Distribuidora>,
) -> Markup { html! {

    table .form-tabla #form_tabla { 
        colgroup { col; col; col; col; col; col; col; }
        (formulario_detalle())
        (formulario_maestro(medios, distribuidoras))
    }

    .bar-cmd {
        button .btn .accion #btn_guarda type="button" { "Graba compra" }
        button .btn .peligro #btn_cancela type="button" { "Cancela compra" }
    }
}}
fn formulario_detalle() -> Markup { html! {
    tr .form-tabla-cabecera {
        th {"id"} th {"Producto"} th {"Costo"} 
        th {"Ctd."} th {"Dsc."} th {"Tot."} th {"Vcmto"} th;
    }
    tr .form-tabla-fila #form_tabla_fila {
        td { input #det_id type="text"; }
        td { 
            input #det_nombre type="text" list="det_nombre_datalist"; 
            datalist #det_nombre_datalist;
        }
        td { input #det_costo type="text" value="0" required; }
        td { input #det_cantidad type="text" value="1" required; }
        td { input #det_descuento type="text" value="0"; }
        td { span #det_total { "0" } }
        td { input #det_vencimiento type="date"; }
        td { .cmd { button #det_agrega .btn-min .accion {
            (PreEscaped("&#x2714"))}}}
    }
}}

fn formulario_maestro(
    medios: Vec<Medio>, 
    distribuidoras: Vec<Distribuidora>) 
-> Markup { html! {
    tr .form-tabla-fila {
        td .tot-label colspan="5" { "Subtotal" }
        td #mas_subtotal .tot-monto {"0"}
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="2" { "Distribuidora" }
        td colspan="2" {
            select #mas_distribuidora {
                @for distribuidora in distribuidoras.into_iter() {
                    option value=(distribuidora.id.unwrap()) {
                        (distribuidora.nombre)
                    }
                }
            }
        }
        td;
        td .tot-label { "Descuento" }
        td { input #mas_descuento type="text" value="0"; }
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="2" { "Tipo Pago" }
        td colspan="2" {
            select #mas_medio {
                @for medio in medios.into_iter() {
                    option value=(medio.id.unwrap()) 
                        selected[medio.nombre == "efectivo"] {
                            (medio.nombre)
                        }
                }
            }
        }
        td;
        td .tot-label { "Total" }
        td #mas_total .tot-monto { "0" };
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="2" { "Factura/Recibo" }
        td colspan="2" {
            input type="text" #mas_documento required 
                placeholder="Factura/Recibo";
        }
        td;
        td .tot-label { "Pago" }
        td { input #mas_pago type="text" required; }
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="2" { "Observaciones" }
        td colspan="2" {
            input type="text" id="mas_observaciones" required 
                placeholder="Datos adicionales respecto a la compra";
        }
        td;
        td .tot-label { "Cambio" }
        td #mas_cambio .tot-monto;
        td;
    }
} }

