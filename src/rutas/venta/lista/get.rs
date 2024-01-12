//! src/rutas/venta/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de ventas

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::domain::puesto::{Puesto, obtiene};
use crate::domain::venta::{Venta, lista_paginada};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de ventas", skip(pool))]
#[get("/puesto/{id}/ventas")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, VentaError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto

    let (puesto_id,) = path.into_inner();

    let puesto = obtiene(&pool, puesto_id).await
        .context("Error al leer punto de venta")?;

    if paginado.orden.is_empty() {
        paginado.orden = "fecha".to_string();
    }

    let (filas, total_filas) = lista_paginada(&pool, &paginado, puesto_id)
        .await
        .context("Error al leer ventas de la BD")?;
    paginado.total_filas = Some(total_filas);

    let pagina = lista::crea(
        "Ventas",
        "/",
        "lista.css",
        Some("venta/lista.js"),
        &paginado,
        contenido(filas, &puesto),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<Venta>, puesto: &Puesto) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Ventas en "(puesto.nombre) }
            .lista {
                .lista-cabecera {
                    span .nombre {"Fecha"}
                    span .nombre {"Monto"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id.unwrap())} {
                        span .nombre {(fila.fecha.unwrap().format("%d-%m-%Y %H:%M").to_string())}
                        span .nombre {(fila.subtotal)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de ventas
#[derive(thiserror::Error)]
pub enum VentaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for VentaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for VentaError {
    fn status_code(&self) -> StatusCode {
        match self {
            VentaError::Validacion(_) => StatusCode::BAD_REQUEST,
            VentaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Causa:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

