//! src/rutas/login/email/get.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: muestra el formulario de login con email

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/login_email_form")]
pub async fn login_email_form() -> AwResult<Markup> {
    layout::principal::crea("Login", "login.css", contenido())
}

fn contenido() -> Markup { html! {
    .login-principal {
        a href="/" {
            img .login-logo src="/img/logo.png"; }
        h1 .login-titulo {"Iniciar Sesión"}
        form method="POST" action="/login_email" {
            .login-control {
                input .login-input-estilo type="email"
                    name="email" id="email" required
                    placeholder="Correo electrónico";
            }
            .login-info {
                p { "Si no tiene un usuario "
                    a #registrar href="/usuario_registro" {"Regístrese"}
                }
            }
            button .login-boton type="submit" { "Siguiente" }
        }
    }
}}
