//! src/rutas/login/email/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de login con email

use actix_web::{web, Responder, HttpResponse};
use actix_web::post;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
}
/// extrae el email del formulario y verifica su existencia en la BD
#[post("/login_email")]
pub async fn login_email(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/login_pass_form")).finish() 
}
