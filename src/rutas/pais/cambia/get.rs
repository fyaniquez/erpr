//! src/rutas/pais/cambia/get.rs
//! author: fyaniquez
//! date: 09/12/2022
//! purpose: muestra el formulario de modificacion de pais

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::pais::{Pais, PaisError};
use anyhow::Context;

//const OBJETO: &str = "pais";
//const OBJETOS: &str = "paises";
//const LOCAL: &str = "país";
    const ERROR_QRY: &str = "Error al leer paises de la BD";
    const LOCAL_MAYUSCULA: &str = "País";
    const OBJETOS_URL: &str = "/paises";
    const OBJETO_JS: &str = "/pais/cambia.js";

#[tracing::instrument(name="Cambia pais", skip(pool))]
#[get("/pais/{id}/cambia")]
pub async fn muestra(


    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, PaisError> {
    let (id,) = path.into_inner();
    let pais = obtiene(&pool, id).await
        .context(ERROR_QRY)?;
    let pagina = layout::form::crea(
        LOCAL_MAYUSCULA, OBJETOS_URL, "form.css", 
        Some(OBJETO_JS), contenido(&pais));
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(pais: &Pais) -> Markup { html! {
    form method="POST" action={"/pais/"(pais.id.unwrap())} {
        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre país" value=(pais.nombre);
        label for="sigla" {"Sigla:" }
        input type="text" name="sigla" id="sigla" required
            placeholder="Sigla país" value=(pais.sigla);
        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
// modelo
// obtiene un pais de la base de datos
#[tracing::instrument(name = "ve pais", skip(pool))]
pub async fn obtiene(
    pool: &PgPool, id: i64
) -> Result<Pais, sqlx::Error> {
    const SELECT: &str 
        = "SELECT id, nombre, sigla FROM paises WHERE id=$1";
    let fila: Pais = sqlx::query_as(SELECT.as_ref())
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(fila)
}
