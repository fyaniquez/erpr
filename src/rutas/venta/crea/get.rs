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

    //let medios: Vec<Medio> = Vec::new();
    // TODO: no esta controlando errores de servidor
    let medios = medio_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    layout::form::crea(
        "Venta",
        "/ventas",
        "venta/crea.css",
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
    .modal #busqueda_box {
        div {
            .busqueda-titulo #busqueda_titulo {};
            span .busqueda-close #busqueda_close { (PreEscaped("&times;")) }
        }
    }

    table .form-tabla #form_tabla { 
        colgroup { col; col; col; col; col; col; col; }
        (formulario_detalle())
        (formulario_maestro(medios))
    }

    (formulario_cliente())

    .bar-cmd {
        button .btn .accion #btn_guarda type="button" { "Graba venta" }
        button .btn .peligro #btn_cancela type="button" { "Cancela venta" }
    }
} }

fn formulario_detalle() -> Markup { html! {
    tr .form-tabla-cabecera {
        th {"id"} th {"Producto"} th {"Prc."} 
        th {"Ctd."} th {"Dsc."} th {"Tot."} th;
    }
    tr .form-tabla-fila #form_tabla_fila {
        td { input #det_id type="text"; }
        td { input #det_nombre type="text"; }
        td { span #det_precio { "0" } }
        td { input #det_cantidad type="text" value="1" required; }
        td { input #det_descuento type="text" value="0"; }
        td { span #det_total { "0" } }
        td { .cmd { button #det_agrega .btn-min .accion {
            (PreEscaped("&#x2714"))}}}
    }
}}

fn formulario_maestro(medios: Vec<Medio>) -> Markup { html! {
    tr .form-tabla-fila {
        td .tot-label colspan="5" { "Subtotal" }
        td #mas_subtotal .tot-monto {"0"}
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="5" { "Descuento" }
        td { input #mas_descuento type="text" value="0"; }
        td;
    }
    tr .form-tabla-fila {
        td .tot-label colspan="2" { "Tipo Pago" }
        td {
            select #mas_medio {
                @for medio in medios.into_iter() {
                    option value=(medio.id.unwrap()) 
                        selected[medio.nombre == "efectivo"] {(medio.nombre)}
                }
            }
        }
        td;
        td .tot-label { "Total" }
        td #mas_total .tot-monto { "0" };
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
                    input #cli_documento type="text" required
                        placeholder="NIT/CI/CEX";
                }
                td { 
                    input #cli_id type="text" name="cliente_id" required
                        placeholder="Nro.Cliente"; 
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
