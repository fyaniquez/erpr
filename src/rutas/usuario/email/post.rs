//! src/rutas/usuario/email/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de alta de usuario email

use actix_web::post;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
}
/// extrae el email del formulario y verifica su existencia en la BD
#[post("/usuario_email")]
pub async fn usuario_email(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    log::info!("Alta de usuario registro de email");
    let saved = sqlx::query!(
        "SELECT email FROM usuarios WHERE EMAIL=$1",
        form.email)
    .fetch_one(pool.get_ref())
    .await;
    match saved {
        Ok(fila) => {
            log::info!("Email ya existe");
            HttpResponse::Found()
                .append_header((
                    "Location", 
                    format!("/usuario_email_form?{}", &fila.email)))
            .finish()
        },
        Err(_e) => {
            log::info!("Email aceptado, solicitar nombres y documento");
            HttpResponse::Found()
                .append_header((
                    "Location", 
                    format!("/usuario_nombres_form?{}", &form.email)))
            .finish()
        }
    }
}
