//! src/public/layouts/principal.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: plantilla principal html

use actix_web::Result as AwResult;
use maud::DOCTYPE;
use maud::{html, Markup};

/// combina el layout con los parámetros y contenido
pub fn crea(
    titulo: &str, 
    estilo: &str,
    script: Option<&str>,
    contenido: Markup
) -> AwResult<Markup> { Ok(html! {
    (DOCTYPE)
    html lang="es" {
        head {
            meta charset="utf-8";
            meta name="author" content="Favio Yañiquez";
            meta name="description" content="Ventas al por mayor y menor";
          meta name="keywords" content="almacen, viveres, bazar, tarjetas";
        meta name="viewport" content="with=device-width, initial-scale=1.0";
            link rel="icon" type="image/x-icon" href="/img/favicon.ico";
            title { (titulo) }
            link rel="stylesheet" href={"/css/"(estilo)};
            @if ! script.is_none() {
            script type="text/javascript" src={"/js/"(script.unwrap())};
            }
        }
        body {
            (contenido)
        }
    }
})}
