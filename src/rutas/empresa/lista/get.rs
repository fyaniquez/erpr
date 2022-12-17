//! src/rutas/empresa/lista/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el muestra() de lista paginada de empresas

use crate::layout;
use crate::layout::lista::Paginado;
use crate::domain::empresa::{Empresa, lista_paginada};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de empresas", skip(pool))]
#[get("/empresas")]
pub async fn muestra(
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, EmpresaError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }
    let (filas, total_filas) = lista_paginada(&pool, &paginado)
        .await
        .context("Error al leer empresas de la BD")?;
    paginado.total_filas = Some(total_filas);

    let pagina = layout::lista::crea(
        "Empresas", "/",
        "lista.css", Some("empresa/lista.js"),
        &paginado, contenido(filas),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<Empresa>) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Empresas" }
            .lista {
                .lista-cabecera {
                    span .nombre {"Nombre"}
                }
                .lista-items {
                    @for fila in filas.into_iter() {
                        div class={"lista-item"
                            @if !fila.activa {" lista-item-inactivo"} }
                        #{(fila.id.unwrap())} {
                            { span .nombre-largo {(fila.nombre)} }
                        }
                    }
                }
            }
        }
    })
}

// errores considerados para lista de empresas
#[derive(thiserror::Error)]
pub enum EmpresaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for EmpresaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for EmpresaError {
    fn status_code(&self) -> StatusCode {
        match self {
            EmpresaError::Validacion(_) => StatusCode::BAD_REQUEST,
            EmpresaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
