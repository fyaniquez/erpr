//! src/rutas/usuario/alta/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de alta de usuario

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/usuario_email")]
pub async fn usuario_email_form() -> AwResult<Markup> {
    layout::form::crea(
        "Usuario", "/usuarios", "login.css", None, contenido())
}

fn contenido() -> Markup { html! {
    .form-box {
        h1 .login-titulo {"Crear cuenta"}
        form method="POST" action="/usuario_email" {
            .login-control {
                input .login-input-estilo type="email"
                    name="email" id="email" required
                    placeholder="Correo electrónico";
            }
            button .login-boton type="submit" { "Siguiente" }
        }
    }
}}
