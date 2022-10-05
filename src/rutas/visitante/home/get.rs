//! src/rutas/invitado/home/get.rs
use actix_web::Result as AwResult;
use maud::DOCTYPE;
use maud::{html, Markup};
/*
 * author: fyaniquez
 * date: 30/09/2022
 * purpose: muestra pagina incial a visitante (usuario sin autorización)
 */
pub async fn home() -> AwResult<Markup> {
    Ok(html! {
(DOCTYPE)
html lang="es" {
    head {
        meta charset="utf-8";
        meta name="author" content="Favio Yañiquez";
        meta name="description" content="Ventas al por mayor y menor";
        meta name="keywords" content="almacen, viveres, bazar, tarjetas";
        meta name="viewport" content="with=device-width, initial-scale=1.0";
        title { "Don coco" }
        link rel="stylesheet" href="/css/home.css";
    }
    body {
        .principal {
            header {
                img .header_logo src="/img/logo.png";
                .header_nav { 
                    a href="/" { "Ingresar" } }
                .header_nav { 
                    a href="/" { "Registrarse" } }
            }
            main {
                section {
                }
            }
            footer {
                time datetime="2022-10-01 20:00";
            }
        }
    }
}
    })
}
