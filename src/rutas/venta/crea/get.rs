//! src/rutas/venta/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de venta
//!
use crate::domain::medio::{lista as medio_lista, Medio};

use crate::layout;
use actix_web::Result as AwResult;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup, PreEscaped};
use sqlx::PgPool;

#[tracing::instrument(name = "formulario crea venta", skip(pool))]
#[get("/venta")]
pub async fn muestra(pool: web::Data<PgPool>) -> AwResult<Markup> {
    let puesto_id: i64 = 1;
    let usuario_id: i64 = 1;
    let catalogo_id: i64 = 2;

    let medios = medio_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    layout::form::crea(
        "Venta",
        &format!("/puest/{}/ventas", puesto_id),
        "maedet.css",
        Some("venta/crea.js"),
        contenido(medios, puesto_id, usuario_id, catalogo_id),
    )
}

fn contenido(
    medios: Vec<Medio>,
    puesto_id: i64,
    usuario_id: i64,
    catalogo_id: i64,
) -> Markup { html! {
    .busqueda-box #busqueda-box {
        .busqueda_titulo #busqueda_titulo {};
        .busqueda #busqueda {};
    }

    table .form-tabla { colgroup { col; col; col; col; col; col; col; }
        (formulario_detalle())
        (formulario_maestro(medios))
    }

    (formulario_cliente())

    .bar-cmd {
        button .btn .accion #agrega_item type="button" { "Graba venta" }
        button .btn .peligro #borra_item type="button" { "Cancela venta" }
    }
} }

fn formulario_detalle() -> Markup { html! {
    tr .form-tabla-cabecera {
        th {"id"} th {"Producto"} th {"Prc."} 
        th {"Ctd."} th {"Dsc."} th {"Tot."} th;
    }
    tr .form-tabla-fila {
        td { input #frm_id type="text"; }
        td { input #frm_nombre type="text"; }
        td { span #frm_prc; }
        td { input #frm_cantidad type="text" required; }
        td { input #frm_descuento type="text" required; }
        td { input #frm_total type="text" required; }
        td { .cmd { #frm_cmd .btn-min .accion { (PreEscaped("&#x2714")) } } }
    }
}}

fn formulario_maestro(medios: Vec<Medio>) -> Markup { html! {
    tr .form-tabla-fila {
        td .tot-label colspan="5" { "Subtotal" }
        td #mas_subotal .tot-monto;
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="5" { "Descuento" }
        td { input #mas_descuento type="text" required; }
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="2" { "Tipo Pago" }
        td {
            select #medio name="medio" {
                @for medio in medios.into_iter() {
                    option value=(medio.id.unwrap()) 
                        selected[medio.nombre == "efectivo"] {(medio.nombre)}
                }
            }
        }
        td;
        td .tot-label { "Total" }
        td #mas_total .tot-monto;
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="5" { "Pago" }
        td { input #mas_pago type="text" required; }
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="5" { "Cambio" }
        td #mas_cambio .tot-monto;
        td;
    }
} }

fn formulario_cliente() -> Markup { html! {
    fieldset { legend { "Cliente" }
        table {
            tr {
                td { 
                    input #cli_id type="text" name="cliente_id" required
                        placeholder="Nro.Cliente"; 
                }
                td { 
                    input #cli_documento type="text" required
                        placeholder="NIT/CI/CEX";
                }
                td rowspan="2" { .cmd {
                    button .btn .accion .centrado .disabled type="button" {
                        "Crea cliente"
                    }
                } }
            }
            tr {
                td rowspan="2" {
                    input #cli_nombre type="text" required
                        placeholder="Apellidos, Nombres / Razón Social";
                }
                td;
            }
        }
    }
} }
