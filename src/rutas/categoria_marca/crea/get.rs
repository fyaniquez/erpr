//! src/rutas/categoria_marca/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de categoria

use crate::domain::marca::{
    Marca,
    lista as marca_lista,
};
use crate::domain::categoria::obtiene;
use crate::domain::categoria_marca::CategoriaMarcaError;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub categoria_marca: i64
}

#[get("/categoria/{categoria_id}/categoria_marca")]
pub async fn muestra(
    path: web::Path<(i64,)>, 
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, CategoriaMarcaError> {
    let (categoria_id,) = path.into_inner();
    let categoria =  obtiene(&pool, categoria_id)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();
    let titulo = format!("Categoría: {} - Marca", categoria.nombre);
    let url = format!("/categoria/{}/categoria_marca", categoria_id);
    let marcas = marca_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();

    let pagina = layout::form::crea(
        &titulo,
        &url,
        "form.css", 
        Some("categoria_marca/crea.js"), 
        contenido(&url, categoria_id, marcas),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(url: &str, categoria_id: i64, marcas: Vec<Marca>) -> Markup { html! {
    form method="POST" action=(url) {
        input type="hidden" name="categoria_id" value=(categoria_id);
        .form-fila {
            label for="marca" {"Marca: " }
            select #marca_id name="marca_id" {
                @for marca in marcas.into_iter() {
                    option value=(marca.id.unwrap())
                        {(marca.nombre)}
                }
            }
        }
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
