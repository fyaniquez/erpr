//! src/rutas/medio/lista/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de lista paginada de medios

use crate::layout;
use crate::layout::lista::Paginado;
use crate::domain::medio::{
    Medio,
    lista_paginada,
};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de medios", skip(pool))]
#[get("/medios")]
pub async fn muestra(
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, MedioError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }
    let (filas, total_filas) = lista_paginada(&pool, &paginado)
        .await.context("Error al leer fabricas de la BD")?;
    paginado.total_filas = Some(total_filas);

    let pagina = layout::lista::crea(
        "Medios", "/",
        "lista.css", Some("medio/lista.js"),
        &paginado, contenido(filas),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<Medio>) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { ("Medios") }
            .lista {
                .lista-cabecera {
                    span .nombre {"Nombre"}
                    span .sigla {"Sigla"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id.unwrap())} {
                        span .nombre {(fila.nombre)}
                        span .sigla {(fila.sigla)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de medios
#[derive(thiserror::Error)]
pub enum MedioError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for MedioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for MedioError {
    fn status_code(&self) -> StatusCode {
        match self {
            MedioError::Validacion(_) => StatusCode::BAD_REQUEST,
            MedioError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
