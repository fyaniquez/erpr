//! src/rutas/pais/lista/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de lista paginada de paiss

use crate::layout;
use crate::layout::lista::Paginado;
use crate::modelo::pais::Pais;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

const OBJETO: &str = "pais";
const OBJETOS: &str = "paises";
const LOCAL_MAYUSCULAS: &str = "Paises";
const ERROR_QRY: &str = "Error al leer paises de la BD";

// controlador
#[tracing::instrument(name = "Lista de paises", skip(pool))]
#[get("/paises")]
pub async fn muestra(
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PaisError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }
    let (filas, total_filas) = lista(&pool, &paginado)
        .await.context(ERROR_QRY)?;
    paginado.total_filas = Some(total_filas);

    let pagina = layout::lista::crea(
        OBJETOS, "/",
        "lista.css", Some(&format!("{}/lista.js", OBJETO)),
        &paginado, contenido(filas),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// modelo
// obtiene un fragmento de la tabla de paiss en la base de datos
#[tracing::instrument(name = "query de paises", skip(pool))]
pub async fn lista(pool: &PgPool, paginado: &Paginado) -> Result<(Vec<Pais>, i32), sqlx::Error> {
    const SELECT: &str = "SELECT id, nombre, sigla FROM paises";
    let qry = paginado.get_qry(SELECT);
    let filas: Vec<Pais> = sqlx::query_as(qry.as_ref())
        .fetch_all(pool).await?;

    let qry_count = paginado.get_qry_count(SELECT);
    let nro_filas: (i64,) = sqlx::query_as(qry_count.as_ref())
        .fetch_one(pool).await?;
    Ok((filas, nro_filas.0 as i32))
}

// vista
fn contenido(filas: Vec<Pais>) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { (LOCAL_MAYUSCULAS) }
            .lista {
                .lista-cabecera {
                    span .nombre {"Nombre"}
                    span .sigla {"Sigla"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id.unwrap())} {
                        span .nombre {(fila.nombre)}
                        span .sigla {(fila.sigla)}
                    }
                }}
            }
        }
    })
}

// errores considerados para lista de paiss
#[derive(thiserror::Error)]
pub enum PaisError {
    #[error("{0}")]
    Validacion(String),
    #[error(transparent)]
    Otro(#[from] anyhow::Error),
}

impl std::fmt::Debug for PaisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PaisError {
    fn status_code(&self) -> StatusCode {
        match self {
            PaisError::Validacion(_) => StatusCode::BAD_REQUEST,
            PaisError::Otro(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
