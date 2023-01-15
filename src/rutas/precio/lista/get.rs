//! src/rutas/precio/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de precios

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::domain::catalogo::{Catalogo, obtiene};
use crate::domain::precio::{Precio, lista_paginada};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de precios json", skip(pool))]
#[get("/catalogo/{id}/precios.json")]
pub async fn muestra_json(
    path: web::Path<(i64,)>, 
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PrecioError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    let (catalogo_id,) = path.into_inner();
    paginado.orden = "nombre".to_string();

    let (filas, total_filas) = lista_paginada(&pool, &paginado, catalogo_id)
        .await
        .context("Error al leer precios de la BD")?;
    paginado.total_filas = Some(total_filas);

    // a json
    let lista_json = serde_json::to_string(&filas)?;

    // al browser
    Ok(HttpResponse::Ok().body(lista_json))
}

// controlador
#[tracing::instrument(name = "Lista de precios", skip(pool))]
#[get("/catalogo/{id}/precios")]
pub async fn muestra(
    path: web::Path<(i64,)>, 
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PrecioError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    let (catalogo_id,) = path.into_inner();
    paginado.orden = "nombre".to_string();

    let (filas, total_filas) = lista_paginada(&pool, &paginado, catalogo_id)
        .await
        .context("Error al leer precios de la BD")?;
    paginado.total_filas = Some(total_filas);

    let catalogo = obtiene(&pool, catalogo_id).await
        .context("Error al leer catalogo")?;
    
    let pagina = lista::crea(
        "Precios",
        "/catalogos",
        "lista.css",
        Some("precio/lista.js"),
        &paginado,
        contenido(filas, &catalogo),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<Precio>, catalogo: &Catalogo) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Precios en: "(catalogo.nombre) }
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

// errores considerados para lista de precios
#[derive(thiserror::Error)]
pub enum PrecioError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for PrecioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PrecioError {
    fn status_code(&self) -> StatusCode {
        match self {
            PrecioError::Validacion(_) => StatusCode::BAD_REQUEST,
            PrecioError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
