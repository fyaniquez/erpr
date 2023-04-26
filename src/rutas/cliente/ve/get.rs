//! src/rutas/cliente/ve/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra un cliente

use crate::layout;
use crate::layout::ErrMsg;
use crate::domain::cliente::{
    Cliente, 
    ClienteError,
    obtiene,
    obtiene_documento
};
use actix_web::{get, web, HttpResponse, Responder};
use maud::{html, Markup};
use sqlx::PgPool;
use anyhow::Context;

#[derive(serde::Deserialize, Debug)]
pub struct Documento {
    documento: String,
}

// controlador
#[tracing::instrument(name="Ve cliente", skip(pool))]
#[get("/cliente/{id}")]
pub async fn muestra(
    path: web::Path<(i64,)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ClienteError> {
    let (id,) = path.into_inner();
    let cliente = obtiene(&pool, id).await
        .context("Error al leer cliente")?;

    let pagina = layout::form::crea(
        "Cliente", "/clientes", 
        "ve.css", Some("cliente/ve.js"), contenido(cliente));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

// controlador json
#[tracing::instrument(name="Ve cliente json", skip(pool))]
#[get("/cliente/{id}.json")]
pub async fn muestra_json(
    path: web::Path<i64>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let cliente_id = path.into_inner();
    match obtiene(&pool, cliente_id).await {
        Ok(cliente) => HttpResponse::Ok().json(cliente),
        Err(err) => match err {
            sqlx::Error::Database(db_err) => 
                HttpResponse::InternalServerError().json( ErrMsg {
                    codigo: 500,
                    mensaje: format!("Error en la BD: {}", db_err) ,
                }),
            sqlx::Error::RowNotFound => 
                HttpResponse::NotFound().json( ErrMsg {
                    codigo: 404,
                    mensaje: format!("No existe cliente: {}", cliente_id)
                }),
            _ => HttpResponse::Conflict().json( ErrMsg {
                    codigo: 500,
                    mensaje: format!("Error: {}", err)
            }),
        }
    }
}

// controlador json por documento
#[tracing::instrument(name="Ve cliente por documento json", skip(pool))]
#[get("/cliente.json")]
pub async fn muestra_documento_json(
    path: web::Query<Documento>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match obtiene_documento(&pool, &path.documento).await {
        Ok(cliente) => HttpResponse::Ok().json(cliente),
        Err(err) => match err {
            sqlx::Error::Database(db_err) => 
                HttpResponse::InternalServerError().json( ErrMsg {
                    codigo: 500,
                    mensaje: format!("Error en la BD: {}", db_err) ,
                }),
            sqlx::Error::RowNotFound => 
                HttpResponse::NotFound().json( ErrMsg {
                    codigo: 404,
                    mensaje: format!(
                        "No existe cliente con documento: {}", 
                        path.documento
                    ),
                }),
            _ => HttpResponse::Conflict().json( ErrMsg {
                    codigo: 500,
                    mensaje: format!("Error: {}", err)
            }),
        }
    }
}

// vista
fn contenido(cliente: Cliente) -> Markup { html! {
    .ve-label { strong {"Nombre: " } (cliente.nombre)}
    .ve-label { strong {"Documento: " } (cliente.documento)}
    button .form-submit #sublista type="button" { "Datos" }
    button .form-submit #cambia type="button" { "Cambiar" }
    button .form-submit #borra type="button" { "Borrar" }
}}

