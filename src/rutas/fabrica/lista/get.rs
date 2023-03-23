//! src/rutas/fabrica/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de fabricas

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::domain::pais::{
    Pais,
    obtiene as pais_obtiene,
};
use crate::domain::fabrica::{
    Fabrica,
    lista_paginada,
};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de fabricas", skip(pool))]
#[get("/pais/{id}/fabricas")]
pub async fn muestra(
    path: web::Path<(i64,)>, 
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, FabricaError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    let (pais_id,) = path.into_inner();
    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }

    let (filas, total_filas) = lista_paginada(&pool, &paginado, pais_id)
        .await
        .context("Error al leer fabricas de la BD")?;
    paginado.total_filas = Some(total_filas);

    let pais = pais_obtiene(&pool, pais_id).await
        .context("Error al leer pais")?;
    
    let pagina = lista::crea(
        "Fabricas",
        "/paises",
        "lista.css",
        Some("fabrica/lista.js"),
        &paginado,
        contenido(filas, &pais),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<Fabrica>, pais: &Pais) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Fabricas en: "(pais.nombre) }
            .lista {
                .lista-cabecera {
                    span .nombre {"Nombre"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id.unwrap())} {
                        span .nombre {(fila.nombre)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de fabricas
#[derive(thiserror::Error)]
pub enum FabricaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for FabricaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for FabricaError {
    fn status_code(&self) -> StatusCode {
        match self {
            FabricaError::Validacion(_) => StatusCode::BAD_REQUEST,
            FabricaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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


