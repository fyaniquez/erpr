//! src/rutas/vendido/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de vendido

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::vendido::{
    VendidoVe, 
    VendidoError,
    obtiene_ve,
};
use anyhow::Context;

#[tracing::instrument(name="Cambia vendido", skip(pool))]
#[get("/vendido/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, VendidoError> {
    let (id,) = path.into_inner();

    let vendido = obtiene_ve(&pool, id).await
        .context("Error al leer vendido")?;

    let pagina = layout::form::crea(
        "Vendido", 
        format!("/venta/{}/vendidos", vendido.venta_id).as_ref(), 
        "form.css", Some("vendido/cambia.js"), contenido(&vendido));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(vendido: &VendidoVe) -> Markup { html! {
    @let id = vendido.id;
    form method="POST" action={"/vendido/"(id)} {
        input type="hidden" name="id" value={(vendido.venta_id)}

        .form-fila {
            label for="producto_id" {"Producto:" }
            input type="text" name="producto_id" id="producto_id" required
                placeholder="Nombre capítulo" value=(vendido.producto);
        }
        .form-fila {
            label for="cantidad" {"Cantidad:" }
            input type="text" name="cantidad" id="cantidad" required
                placeholder="Nombre capítulo" value=(vendido.cantidad);
        }
        .form-fila {
            label for="precio" {"Precio:" }
            input type="text" name="precio" id="precio" required
                placeholder="Nombre capítulo" value=(vendido.precio);
        }
        .form-fila {
            label for="total" {"Total:" }
            input type="text" name="total" id="total" required
                placeholder="Nombre capítulo" value=(vendido.total);
        }

        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
