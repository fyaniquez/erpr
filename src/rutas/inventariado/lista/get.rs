//! src/rutas/inventariado/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de inventariados

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::domain::inventario::{Inventario, obtiene};
use crate::domain::inventariado::{Inventariado, lista_paginada};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de inventariados", skip(pool))]
#[get("/inventario/{id}/inventariados")]
pub async fn muestra(
    path: web::Path<(i64,)>, 
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, InventariadoError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    let (inventario_id,) = path.into_inner();
    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }

    let (filas, total_filas) = lista_paginada(
        &pool, &paginado, inventario_id)
        .await
        .context("Error al leer inventariados de la BD")?;
    paginado.total_filas = Some(total_filas);

    let inventario = obtiene(&pool, inventario_id).await
        .context("Error al leer inventario")?;
    
    let pagina = lista::crea(
        "Inventariados",
        "/inventarios",
        "lista.css",
        Some("inventariado/lista.js"),
        &paginado,
        contenido(filas, &inventario),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<Inventariado>, inventario: &Inventario) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Inventariados en: "(inventario.nombre) }
            .lista {
                .lista-cabecera {
                    span .nombre {"Nombre"}
                    span .nombre {"Ctd."}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id.unwrap())} {
                        span .nombre-largo {(fila.nombre)}
                        span .numero {(fila.cantidad)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de inventariados
#[derive(thiserror::Error)]
pub enum InventariadoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for InventariadoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for InventariadoError {
    fn status_code(&self) -> StatusCode {
        match self {
            InventariadoError::Validacion(_) => StatusCode::BAD_REQUEST,
            InventariadoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
