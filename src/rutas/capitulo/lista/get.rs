//! src/rutas/capitulo/lista/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el muestra() de lista paginada de capitulos

use crate::layout;
use crate::layout::lista::Paginado;
use crate::modelo::capitulo::Capitulo;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de capitulos", skip(pool))]
#[get("/capitulos")]
pub async fn muestra(
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CapituloError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }
    let (filas, total_filas) = lista(&pool, &paginado)
        .await
        .context("Error al leer capitulos de la BD")?;
    paginado.total_filas = Some(total_filas);

    let pagina = layout::lista::crea(
        "Capitulos", "/",
        "lista.css", Some("capitulo/lista.js"),
        &paginado, contenido(filas),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// modelo
// obtiene un fragmento de la tabla de capitulos en la base de datos
#[tracing::instrument(name = "Lista capitulos", skip(pool))]
pub async fn lista(pool: &PgPool, paginado: &Paginado) -> Result<(Vec<Capitulo>, i32), sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre, descripcion FROM capitulos";
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Capitulo> = sqlx::query_as(qry.as_ref())
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// vista
fn contenido(filas: Vec<Capitulo>) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { "Capítulos" }
            .lista {
                .lista-cabecera {
                    span .nombre {"Nombre"}
                    span .descripcion {"Descripción"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id.unwrap())} {
                        span .nombre {(fila.nombre)}
                        span .descripcion {(fila.descripcion)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de capitulos
#[derive(thiserror::Error)]
pub enum CapituloError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for CapituloError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CapituloError {
    fn status_code(&self) -> StatusCode {
        match self {
            CapituloError::Validacion(_) => StatusCode::BAD_REQUEST,
            CapituloError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
