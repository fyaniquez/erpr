//! src/rutas/catalogo/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de catalogos

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::domain::sucursal::{Sucursal, obtiene};
use crate::domain::catalogo::{Catalogo, lista_paginada};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de catalogos", skip(pool))]
#[get("/catalogos")]
pub async fn muestra(
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CatalogoError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    //
    let sucursal_id = 1;

    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }

    let (filas, total_filas) = lista_paginada(&pool, &paginado, sucursal_id)
        .await
        .context("Error al leer catalogos de la BD")?;

    paginado.total_filas = Some(total_filas);

    let sucursal = obtiene(&pool, sucursal_id).await
        .context("Error al leer sucursal")?;
    
    let pagina = lista::crea(
        "Catalogos",
        "/sucursales",
        "lista.css",
        Some("catalogo/lista.js"),
        &paginado,
        contenido(filas, &sucursal),
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<Catalogo>, sucursal: &Sucursal) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Catalogos en: "(sucursal.nombre) }
            .lista {
                .lista-cabecera {
                    span .nombre {"Nombre"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id.unwrap())} {
                        span .nombre-largo {(fila.nombre)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de catalogos
#[derive(thiserror::Error)]
pub enum CatalogoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for CatalogoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CatalogoError {
    fn status_code(&self) -> StatusCode {
        match self {
            CatalogoError::Validacion(_) => StatusCode::BAD_REQUEST,
            CatalogoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
