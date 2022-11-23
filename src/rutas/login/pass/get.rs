//! src/rutas/login/pass/get.rs
//! author: fyaniquez
//! date: 12/10/2022
//! purpose: muestra el formulario de login con password

use actix_web::Result as AwResult;
use actix_web::get;
use maud::{html, Markup};
use crate::layout;

#[get("/login_pass_form")]
pub async fn login_pass_form() -> AwResult<Markup> {
    let email = "fyaniquez@gmail.com";
    layout::principal::crea("Password", "login.css", None, contenido(email))
}

fn contenido(email: &str) -> Markup { html! {
    .principal {
        .login-box {
            a href="/" {
                img .login-logo src="/img/logo.png"; }
            p { (email) }
            h1 .login-titulo {"Escribe tu contraseña"}
            form method="POST" action="/login_pass" {
                .login-control {
                    input .login-input-estilo type="password"
                        name="pass" id="pass" required
                        placeholder="Introduzca contraseña";
                }
                .login-info {
                    p { "¿Olvidaste la contraseña?"
                        a #reset href="/login_pass_reset_form" 
                            {"Cambio de contraseña"}
                    }
                    p { "¿No es el email correcto? "
                        a #login href="/login_email_form" 
                            {"Cambio de email"}
                    }
                }
                button .login-boton type="submit" { "Siguiente" }
            }
        }
    }
}}
