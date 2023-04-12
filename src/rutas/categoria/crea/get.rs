//! src/rutas/categoria/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de categoria

use crate::domain::capitulo::obtiene as capitulo_obtiene;
use crate::domain::categoria::CategoriaError;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use crate::layout;

#[derive(serde::Deserialize)]
pub struct QueryData {
    pub capitulo: i64
}

#[get("/capitulo/{id}/categoria")]
pub async fn muestra(
    path: web::Path<(i64,)>, 
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, CategoriaError> {
    let (capitulo_id,) = path.into_inner();
    let capitulo = capitulo_obtiene(&pool, capitulo_id).await
        .map_err(|_e| CategoriaError::Validacion(
            "Error al leer capitulo".to_string()))?;
    let pagina = layout::form::crea(
        &format!("Capítulo: {} - Categoría", capitulo.nombre), 
        &format!("/capitulo/{}/categorias", capitulo_id), 
        "form.css", Some("categoria/crea.js"), contenido(capitulo_id)
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(capitulo_id: i64) -> Markup { html! {
    form method="POST" 
    action=(format!("/capitulo/{}/categoria", capitulo_id)) {
        input type="hidden" name="capitulo_id" value=(capitulo_id);
        .form-fila {
            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre categoría";
        }
        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
