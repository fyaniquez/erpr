//! src/rutas/producto/lista_sin_inventariado/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de productos
//! son inventariar

use crate::layout;
use crate::layout::lista::Paginado;
use crate::domain::inventario::{
    Inventario,
    obtiene as inventario_obtiene,
};
use crate::domain::producto::{
    Producto, 
    lista_paginada_sin_inventariado,
};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};

#[derive(Debug)]
pub struct QueryData {
    pub inventario: i64,
}
// obtiene productos sin inventariado en el inventario
#[tracing::instrument(name = "Lista de productos sin inventariado", skip(pool))]
#[get("/inventario/{id}/productos")]
pub async fn muestra(
    path: web::Path<(i64, )>,
    mut paginado: web::Query<Paginado>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ProductoError> {

    let (inventario_id,) = path.into_inner();

    let inventario = inventario_obtiene(&pool, inventario_id)
        .await
        .context("Error al leer inventario")?;

    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }

    let (filas, total_filas) = lista_paginada_sin_inventariado(
        &pool, 
        &paginado,
        inventario_id,)
        .await
        .context("Error al leer productos")
        .unwrap();

    paginado.total_filas = Some(total_filas);

    let pagina = layout::lista::crea(
        "Productos no inventariados", 
        format!("/inventario/{}/productos", inventario_id).as_ref(), 
        "lista.css", 
        Some("producto/lista_sin_inventariado.js"), 
        &paginado,
        contenido(filas, &inventario)
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(filas: Vec<Producto>, inventario: &Inventario) -> Option<Markup> { 
    if filas.len() < 1 {
        return None;
    }

    Some(html! {
        .lista-box {
            .lista-titulo {"Productos no inventariados en: "(inventario.nombre)}
            .lista {
                .lista-cabecera {
                    span .nombre-largo {"Nombre"}
                }
                .lista-items {
                    @for fila in filas.into_iter() {
                        .lista-item #{(fila.id.unwrap())} {
                            span .nombre-largo {(fila.nombre)}
                        }
                    }
                }

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
