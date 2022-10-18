//! src/rutas/login/email/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de login con email

use actix_web::post;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
}
/// extrae el email del formulario y verifica su existencia en la BD
#[post("/login_email")]
pub async fn login_email(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let saved = sqlx::query!(
        "SELECT email, nombre FROM usuarios WHERE email=$1", 
        form.email)
    .fetch_one(pool.get_ref())
    .await;
    match saved {
        Ok(fila) => 
            HttpResponse::Found()
                .append_header((
                    "Location", 
                    format!("/login_pass_form?{}", &fila.email.unwrap())))
            .finish(),
        Err(e) => {
            println!("Error al ejecutar query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
