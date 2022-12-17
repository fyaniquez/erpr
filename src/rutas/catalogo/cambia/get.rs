//! src/rutas/catalogo/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de catalogo

use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;
use sqlx::PgPool;
use crate::domain::catalogo::{
    Catalogo, CatalogoError, 
    obtiene as catalogo_obtiene,
};
use crate::domain::empresa::{
    Empresa, lista as empresa_lista,
};
use anyhow::Context;

#[tracing::instrument(name="Cambia catalogo", skip(pool))]
#[get("/catalogo/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, CatalogoError> {

    let (id,) = path.into_inner();

    let catalogo = catalogo_obtiene(&pool, id).await
        .context("Error al leer catálogo")?;

    let empresas = empresa_lista(&pool)
        .await
        .context("Error al leer empresas")
        .unwrap();

    let pagina = layout::form::crea(
        "Catálogo", 
        format!("/empresa/{}/catalogos", catalogo.empresa_id).as_ref(), 
        "form.css", Some("catalogo/cambia.js"), 
        contenido(&catalogo, empresas));

    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(catalogo: &Catalogo, propietarios: Vec<Empresa>) 
-> Markup { html! {
    form method="POST" action={"/catalogo/"(catalogo.id.unwrap())} {

        input type="hidden" name="empresa_id" value=(catalogo.empresa_id);

        label for="nombre" {"Nombre:" }
        input type="text" name="nombre" id="nombre" required
            placeholder="Nombre capítulo" value=(catalogo.nombre);

        label for="propietario" {"Propietario:" }
            select #propietario name="propietario" {
                @for propietario in propietarios.into_iter() {
                    option value=(propietario.id.unwrap())
                    selected[propietario.id == Some(catalogo.propietario)]
                        {(propietario.nombre)}
                }
            }

        label for="nombre" {"Fecha:" }
        input type="date" name="fecha" id="fecha" required
            placeholder="Fecha vigencia" 
            value=(catalogo.fecha.format("%d/%m%/%Y").to_string());

        fieldset {
            legend {"¿Está activo?"}
            input type="radio" name="activo" id="activo_si" value="true"
                checked[catalogo.activo];
            label for="activo_si" {"Si"}
            input type="radio" name="activo" id="activo_no" value="false"
                checked[!catalogo.activo];
            label for="activo_no" {"No"}
       }

        button .form-submit #graba type="submit" { "Graba" }
        button .form-submit #cancela type="button" { "Cancela" }
    }
}}
