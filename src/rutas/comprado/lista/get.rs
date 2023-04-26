//! src/rutas/comprado/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de comprados

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::domain::venta::{Venta, obtiene};
use crate::domain::comprado::{
    CompradoVe,
    lista_paginada,
};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de comprados", skip(pool))]
#[get("/venta/{id}/comprados")]
pub async fn muestra(
    path: web::Path<(i64,)>, 
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CompradoError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto

    let (venta_id,) = path.into_inner();

    let venta = obtiene(&pool, venta_id).await
        .context("Error al leer venta")?;

    let (filas, total_filas) = lista_paginada(&pool, &paginado, venta_id)
        .await
        .context("Error al leer comprados de la BD")?;
    paginado.total_filas = Some(total_filas);

    let pagina = lista::crea(
        "Comprados",
        "/ventas",
        "lista.css",
        Some("comprado/lista.js"),
        &paginado,
        contenido(filas, &venta),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<CompradoVe>, venta: &Venta) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Comprados en: "(venta.puesto_id.unwrap()) }
            .lista {
                .lista-cabecera {
                    span .nombre {"Producto"}
                    span .nombre {"Cantidad"}
                    span .nombre {"Precio"}
                    span .nombre {"Total"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id)} {
                        span .nombre-largo {(fila.producto)}
                        span .nombre {(fila.cantidad)}
                        span .nombre {(fila.costo)}
                        span .nombre {(fila.total)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de comprados
#[derive(thiserror::Error)]
pub enum CompradoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for CompradoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CompradoError {
    fn status_code(&self) -> StatusCode {
        match self {
            CompradoError::Validacion(_) => StatusCode::BAD_REQUEST,
            CompradoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
