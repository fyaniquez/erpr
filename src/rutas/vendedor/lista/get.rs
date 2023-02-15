//! src/rutas/vendedor/lista/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el muestra() de lista paginada de vendedores

use crate::layout;
use crate::layout::lista::Paginado;
use crate::domain::vendedor::{Vendedor, lista_paginada};
use crate::domain::distribuidora::{Distribuidora, obtiene};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de vendedores", skip(pool))]
#[get("/distribuidora/{id}/vendedores")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, VendedorError> {

    let (distribuidora_id,) = path.into_inner();

    let distribuidora = obtiene(&pool, distribuidora_id)
        .await
        .context("Error al leer distribuidora")?;

    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }

    let (filas, total_filas) = lista_paginada(
        &pool, 
        &paginado, 
        distribuidora_id)
        .await
        .context("Error al leer vendedores de la BD")?;

    paginado.total_filas = Some(total_filas);

    let pagina = layout::lista::crea(
        "Vendedores", 
        "/distribuidoras",
        "lista.css", 
        Some("vendedor/lista.js"),
        &paginado, 
        contenido(filas, &distribuidora),
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(
    filas: Vec<Vendedor>, 
    distribuidora: &Distribuidora
) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo {"Vendedores de:" (distribuidora.nombre)}
            .lista {
                .lista-cabecera {
                    span .nombre-largo {"Nombre"}
                    span .nombre {"Cargo"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id.unwrap())} {
                        span .nombre-largo {(fila.nombre)}
                        span .nombre {(fila.cargo)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de vendedores
#[derive(thiserror::Error)]
pub enum VendedorError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for VendedorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for VendedorError {
    fn status_code(&self) -> StatusCode {
        match self {
            VendedorError::Validacion(_) => StatusCode::BAD_REQUEST,
            VendedorError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
