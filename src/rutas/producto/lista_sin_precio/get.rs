//! src/rutas/producto/lista_sin_precio/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de productos

use crate::layout;
use crate::layout::lista::Paginado;
use crate::domain::catalogo::{
    Catalogo,
    obtiene as catalogo_obtiene,
};
use crate::domain::producto::{
    Producto, 
    lista_paginada_sin_precio,
};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};

#[derive(Debug)]
pub struct QueryData {
    pub catalogo: i64,
}
// obtiene productos sin precio en el catalogo
#[tracing::instrument(name = "Lista de productos sin precio", skip(pool))]
#[get("/catalogo/{id}/productos")]
pub async fn muestra(
    path: web::Path<(i64, )>,
    mut paginado: web::Query<Paginado>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ProductoError> {

    let (catalogo_id,) = path.into_inner();

    let catalogo = catalogo_obtiene(&pool, catalogo_id)
        .await
        .context("Error al leer catálogo")?;

    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }

    let (filas, total_filas) = lista_paginada_sin_precio(
        &pool, 
        &paginado,
        catalogo_id,)
        .await
        .context("Error al leer productos")
        .unwrap();

    paginado.total_filas = Some(total_filas);

    let pagina = layout::lista::crea(
        "Productos no catalogados", 
        format!("/catalogo/{}/precios", catalogo_id).as_ref(), 
        "lista.css", 
        Some("producto/lista_sin_precio.js"), 
        &paginado,
        contenido(filas, &catalogo)
    );

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(filas: Vec<Producto>, catalogo: &Catalogo) -> Option<Markup> { 
    if filas.len() < 1 {
        return None;
    }

    Some(html! {
        .lista-box {
            .lista-titulo {"Productos no catalogados en: "(catalogo.nombre)}
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
