//! src/rutas/login/email/get.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: muestra el formulario de login con email

use actix_web::Result as AwResult;
use actix_web::get;
use maud::DOCTYPE;
use maud::{html, Markup};

#[get("/login_email_form")]
pub async fn login_email_form() -> AwResult<Markup> {
    Ok(html! {
    (DOCTYPE)
    html lang="es" {
        head {
            meta charset="utf-8";
            meta name="author" content="Favio Yañiquez";
            meta name="description" content="Ventas al por mayor y menor";
            meta name="keywords" content="almacen, viveres, bazar, tarjetas";
            meta name="viewport" content="with=device-width, initial-scale=1.0";
            link rel="icon" type="image/x-icon" href="/img/favicon.ico";
            title { "Don coco" }
            link rel="stylesheet" href="/css/login.css";
        }
        body {
            .main {
                .login-principal {
                    img .login-logo src="/img/logo.png";
                    h1 .login-titulo {"Iniciar Sesión"}
                    form method="POST" action="/login_email" {
                        .login-control {
                            input .login-input-estilo type="email"
                                name="email" id="email" required
                                placeholder="Correo electrónico";
                        }
                        .login-info {
                            p { "Si no tiene un usuario "
                                a #registrar href="/usuario_registro"
                                    {"Regístrese"}
                            }
                        }
                        button .login-boton type="submit" { "Siguiente" }
                    }
                }
            }
            footer {
                time datetime="2022-10-01 20:00";
            }
        }
    }
        })
}
