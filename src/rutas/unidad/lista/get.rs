//! src/rutas/unidad/lista/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de lista paginada de unidads

use crate::layout;
use crate::layout::lista::Paginado;
use crate::domain::unidad::{
    Unidad,
    UnidadError,
    lista_paginada,
};
use actix_web::get;
use actix_web::{web, HttpResponse, };
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

const OBJETO: &str = "unidad";
const OBJETOS: &str = "unidades";
const LOCAL_MAYUSCULAS: &str = "Unidades";
const ERROR_QRY: &str = "Error al leer unidades de la BD";

// controlador
#[tracing::instrument(name = "Lista de unidades", skip(pool))]
#[get("/unidades")]
pub async fn muestra(
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UnidadError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivo
    // en la implentacion de default puede colocarse los valores p/defecto
    if paginado.orden.is_empty() {
        paginado.orden = "nombre".to_string();
    }
    let (filas, total_filas) = lista_paginada(&pool, &paginado)
        .await.context(ERROR_QRY)?;
    paginado.total_filas = Some(total_filas);

    let pagina = layout::lista::crea(
        OBJETOS, "/",
        "lista.css", Some(&format!("{}/lista.js", OBJETO)),
        &paginado, contenido(filas),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// vista
fn contenido(filas: Vec<Unidad>) -> Option<Markup> {
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
