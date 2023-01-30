//! src/rutas/cliente/lista/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el muestra() de lista paginada de clientes

use crate::layout;
use crate::layout::lista::Paginado;
use crate::domain::cliente::{Cliente, lista_paginada};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de clientes", skip(pool))]
#[get("/clientes")]
pub async fn muestra(
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClienteError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }
    let (filas, total_filas) = lista_paginada(&pool, &paginado)
        .await
        .context("Error al leer clientes de la BD")?;
    paginado.total_filas = Some(total_filas);

    let pagina = layout::lista::crea(
        "Clientes", "/",
        "lista.css", Some("cliente/lista.js"),
        &paginado, contenido(filas),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// controlador json
#[tracing::instrument(name = "Lista de clientes", skip(pool))]
#[get("/clientes.json")]
pub async fn muestra_json(
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClienteError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    paginado.orden = "nombre".to_string();

    let (filas, total_filas) = lista_paginada(&pool, &paginado)
        .await
        .context("Error al leer clientes de la BD")?;
    paginado.total_filas = Some(total_filas);

 // a json
    let lista_json = serde_json::to_string(&filas)
        .map_err(|err| ClienteError::Validacion(err.to_string()))
        .unwrap();

    // al browser
    Ok(HttpResponse::Ok().body(lista_json))

}

// vista
fn contenido(filas: Vec<Cliente>) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo {"Clientes" }
            .lista {
                .lista-cabecera {
                    span .nombre {"Nombre"}
                    span .documento {"Documento"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id.unwrap())} {
                        span .nombre {(fila.nombre)}
                        span .documento {(fila.documento)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de clientes
#[derive(thiserror::Error)]
pub enum ClienteError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for ClienteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ClienteError {
    fn status_code(&self) -> StatusCode {
        match self {
            ClienteError::Validacion(_) => StatusCode::BAD_REQUEST,
            ClienteError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
