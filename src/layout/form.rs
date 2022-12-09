//! src/public/layouts/form.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: plantilla para formularios

use actix_web::Result as AwResult;
use maud::{html, Markup};
use crate::layout;

/// combina el layout con los parámetros y contenido
pub fn crea(
    titulo: &str,
    atras: &str,
    estilo: &str,
    script: Option<&str>,
    contenido: Markup
) -> AwResult<Markup> { 
    let formulario = combina(titulo, atras, contenido);
    layout::principal::crea(titulo, estilo, script, formulario)
}

// combina el contenido construido por el cliente
// con el layout para formularios
fn combina(titulo: &str, atras: &str, contenido: Markup) -> Markup { html! {
    .cabecera {
        img .cabecera-logo src="/img/logo.png";
        .cabecera-nav {
            a href="/login_email_form" { "Ingresar" } }
        .cabecera-nav {
            a href="/" { "Registrarse" } }
    }
    .principal { 
        .form-box {
            .form-cabecera {
                span .form-titulo {(titulo)}
                a .form-atras #atras href=(atras) { 
                    img src="/img/lista-24.png"; }
            }
            (contenido) 
        }
    }
}}
