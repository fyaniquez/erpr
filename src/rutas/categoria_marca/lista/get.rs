//! src/rutas/categoria-marca/lista/get.rs
//! author: fyaniquez
//! date: 5/12/2022
//! purpose: muestra el formulario de lista paginada de categorias

use crate::layout::lista;
use crate::layout::lista::Paginado;
use crate::domain::categoria_marca::{
    CategoriaMarcaError,
    CategoriaMarcaNombres,
    lista,
    lista_paginada,
};
use crate::domain::categoria::{
    obtiene,
};
use actix_web::get;
use actix_web::{web, HttpResponse,};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

// controlador
#[tracing::instrument(name = "Lista de categorias marcas en json", skip(pool))]
#[get("/categoria/{id}/categorias_marcas.json")]
pub async fn muestra_json(
    path: web::Path<(i64,)>, 
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaMarcaError> {
    let (categoria_id,) = path.into_inner();

    let filas = lista(&pool, categoria_id)
        .await .context("Error al leer categorias de la BD")?;

    // a json
    let lista_json = serde_json::to_string(&filas)
        .map_err(|err| CategoriaMarcaError::Validacion(err.to_string()))
        .unwrap();

    // al browser
    Ok(HttpResponse::Ok().body(lista_json))
}

// controlador
#[tracing::instrument(name = "Lista de categorias - marcas", skip(pool))]
#[get("/categoria/{id}/categorias_marcas")]
pub async fn muestra(
    path: web::Path<(i64,)>, 
    mut paginado: web::Query<Paginado>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CategoriaMarcaError> {
    // TODO: ver como implementar  un trait si no esta en el mismo archivet:
    // en la implentacion de default puede colocarse los valores p/defecto

    let (categoria_id,) = path.into_inner();
    let categoria = obtiene(&pool, categoria_id)
        .await
        .context("Error al leer categoria");

    if paginado.orden.is_empty() {
        paginado.orden = "c.nombre, m.nombre".to_string();
    }

    let (filas, total_filas) = lista_paginada( &pool, &paginado, categoria_id)
        .await
        .context("Error al leer categorias marcas de la BD")?;
    paginado.total_filas = Some(total_filas);

    let titulo = format!("Categoria: {} - Marcas", categoria.unwrap().nombre);
    let pagina = lista::crea(
        &titulo,
        "/categorias",
        "lista.css",
        Some("categoria_marca/lista.js"),
        &paginado,
        contenido(filas, &titulo),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}
// vista
fn contenido(filas: Vec<CategoriaMarcaNombres>, titulo: &str) -> Option<Markup> {
    if filas.len() < 1 {
        return None;
    }
    Some(html! {
        .lista-box {
            .lista-titulo { (titulo) }
            .lista {
                .lista-cabecera {
                    span .nombre {"Marca"}
                }
                .lista-items {
                @for fila in filas.into_iter() {
                    .lista-item #{(fila.id)} {
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
    writeln!(f, "{}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Causa:{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
