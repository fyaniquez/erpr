//! src/rutas/vendedor/cambia/get.rs
//! author: fyaniquez
//! date: 21/10/2022
//! purpose: muestra el formulario de modificacion de vendedor

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::vendedor::{Vendedor, VendedorError};
use anyhow::Context;

#[tracing::instrument(name="Cambia vendedor", skip(pool))]
#[get("/vendedor/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, VendedorError> {

    let (id,) = path.into_inner();

    let vendedor = obtiene(&pool, id).await
        .context("Error al leer vendedor")?;

    let pagina = layout::form::crea(
        "Vendedor", 
        &format!("/distribuidora/{}/vendedores", vendedor.distribuidora_id), 
        "form.css", 
        Some("/vendedor/cambia.js"), contenido(&vendedor));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(vendedor: &Vendedor) -> Markup { html! {
    form method="POST" action={"/vendedor/"(vendedor.id.unwrap())} {

        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre vendedor" value=(vendedor.nombre);

        label for="cargo" {"Cargo:" }
        select #cargo name="cargo" {
            option value="Preventista" selected {"Preventista"}
            option value="Camionero" {"Camionero"}
            option value="Supervisor" {"Supervisor"}
        }

        label for="contactos" {"Contactos:" }
        input type="text" name="contactos" id="contactos" required
            placeholder="Contactos" value=(vendedor.contactos);

        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
// modelo
// obtiene un vendedor de la base de datos
#[tracing::instrument(name = "ve vendedor", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Vendedor, sqlx::Error> {
    const SELECT: &str 
        = r#"SELECT id, distribuidora_id, cargo, 
            nombre, contactos FROM vendedores WHERE id=$1"#;
    let fila: Vendedor = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
