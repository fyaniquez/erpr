//! src/rutas/login/email/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de login con email

use actix_web::post;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
}
// controla el ingreso de usaurio
// extrae el email del formulario y verifica su existencia en la BD
#[tracing::instrument(
    name = "Ingreso de usuario",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
    )
)]
#[post("/login_email")]
pub async fn login_email(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let qry_span = tracing::info_span!("Verificando existencia de usuario");
    match sqlx::query!(
        "SELECT email, nombre FROM usuarios WHERE email=$1", 
        form.email)
        .fetch_one(pool.get_ref())
        .instrument(qry_span)
        .await
    {
        Ok(fila) => {
            log::info!("Email de usuario aceptado");
            HttpResponse::Found()
                .append_header((
                    "Location", 
                    format!("/login_pass_form?{}", &fila.email)))
            .finish()
        },
        Err(e) => {
            tracing::error!("Error al ejecutar consulta: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
