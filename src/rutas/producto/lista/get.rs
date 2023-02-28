//! src/rutas/producto/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de productos

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::domain::producto::{
    Producto, 
    lista_paginada
};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador json
#[tracing::instrument(name = "Lista de productos json", skip(pool))]
#[get("/productos.json")]
pub async fn muestra_json(

    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ProductoError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto

    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }

    let (filas, total_filas) = lista_paginada(&pool, &paginado)
        .await
        .context("Error al leer productos de la BD")?;
    paginado.total_filas = Some(total_filas);

 // a json
    let lista_json = serde_json::to_string(&filas)
        .map_err(|err| ProductoError::Validacion(err.to_string()))
        .unwrap();

    // al browser
    Ok(HttpResponse::Ok().body(lista_json))

}

// controlador
#[tracing::instrument(name = "Lista de productos", skip(pool))]
#[get("/productos")]
pub async fn muestra(
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ProductoError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto

    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }

    let (filas, total_filas) = lista_paginada(&pool, &paginado)
        .await
        .context("Error al leer productos de la BD")?;
    paginado.total_filas = Some(total_filas);

    let pagina = lista::crea(
        "Productos",
        "/",
        "lista.css",
        Some("producto/lista.js"),
        &paginado,
        contenido(filas),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<Producto>) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Productos" }
            .lista {
                .lista-cabecera {
                    span .nombre-largo {"Nombre"}
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

// errores considerados para lista de productos
#[derive(thiserror::Error)]
pub enum ProductoError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for ProductoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ProductoError {
    fn status_code(&self) -> StatusCode {
        match self {
            ProductoError::Validacion(_) => StatusCode::BAD_REQUEST,
            ProductoError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

