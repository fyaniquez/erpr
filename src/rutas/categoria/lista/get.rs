//! src/rutas/categoria/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de categorias

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::domain::capitulo::{Capitulo, obtiene};
use crate::domain::categoria::{
    Categoria, 
    CategoriaError,
    lista_paginada,
    lista,
};
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de categorias en json", skip(pool))]
#[get("/capitulo/{id}/categorias.json")]
pub async fn muestra_json(
    path: web::Path<(i64,)>, 
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaError> {
    let (capitulo_id,) = path.into_inner();

    let filas = lista(&pool, capitulo_id)
        .await .context("Error al leer categorias de la BD")?;

    // a json
    let lista_json = serde_json::to_string(&filas)
        .map_err(|err| CategoriaError::Validacion(err.to_string()))
        .unwrap();

    // al browser
    Ok(HttpResponse::Ok().body(lista_json))
}

// controlador
#[tracing::instrument(name = "Lista de categorias", skip(pool))]
#[get("/capitulo/{id}/categorias")]
pub async fn muestra(
    path: web::Path<(i64,)>, 
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto

    let (capitulo_id,) = path.into_inner();

    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }

    let capitulo = obtiene(&pool, capitulo_id).await
        .context("Error al leer capitulo")?;

    let (filas, total_filas) = lista_paginada(&pool, &paginado, capitulo_id)
        .await
        .context("Error al leer categorias de la BD")?;
    paginado.total_filas = Some(total_filas);

    let pagina = lista::crea(
        "Categorias",
        "/capitulos",
        "lista.css",
        Some("categoria/lista.js"),
        &paginado,
        contenido(filas, &capitulo),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}
// vista
fn contenido(filas: Vec<Categoria>, capitulo: &Capitulo) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Categorias en: "(capitulo.nombre) }
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

// errores considerados para lista de categorias

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
