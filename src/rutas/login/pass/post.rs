//! src/rutas/login/pass/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de login con pass

use actix_web::{web, Responder, HttpResponse};
use actix_web::post;

#[derive(serde::Deserialize)]
pub struct FormData {
    _pass: String,
}

/// extrae el email del formulario y verifica su existencia en la BD
#[post("/login_pass")]
pub async fn login_pass(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
