//! src/rutas/categoria/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de categorias

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::modelo::capitulo::Capitulo;
use crate::modelo::categoria::Categoria;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de categorias", skip(pool))]
#[get("/capitulo/{id}/categorias")]
pub async fn categoria_lista_form(
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

    let (filas, total_filas) = lista(&pool, &paginado, capitulo_id)
        .await
        .context("Error al leer categorias de la BD")?;
    paginado.total_filas = Some(total_filas);

    let capitulo = obtiene(&pool, capitulo_id).await
        .context("Error al leer capitulo")?;
    
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

// modelo
// obtiene un fragmento de la tabla de categorias en la base de datos
#[tracing::instrument(name = "Lista categorias", skip(pool))]
pub async fn lista(
    pool: &PgPool, 
    paginado: &Paginado, 
    capitulo_id: i64
) -> Result<(Vec<Categoria>, i32), sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre, capitulo_id FROM categorias where capitulo_id = $1";
    
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Categoria> = sqlx::query_as(qry.as_ref())
        .bind(capitulo_id)
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .bind(capitulo_id)
        .fetch_one(pool).await?;

    Ok((filas, nro_filas.0 as i32))
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
                        span .nombre {(fila.nombre)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de categorias
#[derive(thiserror::Error)]
pub enum CategoriaError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for CategoriaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CategoriaError {
    fn status_code(&self) -> StatusCode {
        match self {
            CategoriaError::Validacion(_) => StatusCode::BAD_REQUEST,
            CategoriaError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

// modelo
// obtiene un capitulo de la base de datos
#[tracing::instrument(name = "ve capitulo", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Capitulo, sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre, descripcion FROM capitulos WHERE id=$1";
    let fila: Capitulo = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
