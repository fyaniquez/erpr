//! src/rutas/producto/crea/get.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: muestra el formulario de alta de producto

use crate::domain::capitulo::{
    Capitulo, 
    lista_alfabetica as capitulo_lista,
};
use crate::domain::categoria::{Categoria, lista as categoria_lista};
use crate::domain::fabrica::{Fabrica, lista as fabrica_lista};
use crate::domain::marca::{Marca, lista as marca_lista};
use crate::domain::unidad::{Unidad, lista as unidad_lista};
use crate::layout;
use actix_web::Result as AwResult;
use actix_web::{get, web, HttpResponse};
use maud::{html, Markup};
use sqlx::PgPool;

#[tracing::instrument(name = "formulario crea producto", skip(pool))]
#[get("/producto")]
pub async fn muestra(pool: web::Data<PgPool>) -> AwResult<Markup> {

    let capitulos = capitulo_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();
    let capitulo_default = capitulos[0].id.unwrap();

    let categorias = categoria_lista(&pool, capitulo_default)
        .await 
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();
    let categoria_default = categorias[0].id.unwrap();

    let marcas = marca_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();
    let marca_default = marcas[0].id.unwrap();

    let unidades = unidad_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();
    let unidad_default = unidades[0].id.unwrap();

    let fabricas = fabrica_lista(&pool)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())
        .unwrap();
    let fabrica_default = fabricas[0].id.unwrap();

    layout::form::crea(
        "Producto",
        "/productos",
        "form.css",
        Some("producto/crea.js"),
        contenido(
            capitulos, categorias, marcas, unidades, fabricas,
            capitulo_default, categoria_default, marca_default,
            unidad_default, fabrica_default,
        ),
    )
}

fn contenido(
    capitulos: Vec<Capitulo>,
    categorias: Vec<Categoria>,
    marcas: Vec<Marca>,
    unidades: Vec<Unidad>,
    fabricas: Vec<Fabrica>,
    capitulo_default: i64,
    categoria_default: i64,
    marca_default: i64,
    unidad_default: i64,
    fabrica_default: i64,
) -> Markup { html! {
    form method="POST" action="/producto" {

        label for="nombre" {"Nombre:" }
        input type="text" id="nombre" 
            placeholder="Nombre categoría";

        label for="caracteristicas" {"Características:" }
        input type="text" name="caracteristicas" id="caracteristicas"
            required placeholder="Características adicionales";

        label for="capitulo" {"Capítulo:" }
        select #capitulo_id name="capitulo_id" {
            @for capitulo in capitulos.into_iter() {
                option value=(capitulo.id.unwrap())
                selected[capitulo.id == Some(capitulo_default)] 
                    {(capitulo.nombre)}
            }
        }

        label for="categoria" {"Categoría:" }
        select #categoria_id name="categoria_id" {
            @for categoria in categorias.into_iter() {
                option value=(categoria.id.unwrap())
                selected[categoria.id == Some(categoria_default)] 
                    {(categoria.nombre)}
            }
        }

        label for="marca" {"Marca:" }
        select #marca_id name="marca_id" {
            @for marca in marcas.into_iter() {
                option value=(marca.id.unwrap())
                selected[marca.id == Some(marca_default)] 
                    {(marca.nombre)}
            }
        }

        label for="unidad" {"Unidad:" }
        select #unidad_id name="unidad_id" {
            @for unidad in unidades.into_iter() {
                option value=(unidad.id.unwrap())
                selected[unidad.id == Some(unidad_default)] 
                    {(unidad.nombre)}
            }
        }

        label for="fabrica_id" {"Fábrica:" }
        select #fabrica_id name="fabrica_id" {
            @for fabrica in fabricas.into_iter() {
                option value=(fabrica.id.unwrap())
                selected[fabrica.id == Some(fabrica_default)] 
                    {(fabrica.nombre)}
            }
        }

        label for="barras" {"Cod.Barras:" }
        input type="text" name="barras" id="barras"
            required placeholder="Código de barras";

        label for="contenido" {"Contenido:" }
        input type="text" name="contenido" id="contenido"
            required placeholder="Contenido sin unidad";

        label for="cantidad" {"Cantidad:" }
        input type="number" name="cantidad" id="cantidad"
            required placeholder="Cantidad p/mayor";

        fieldset {
            legend { "¿Es fraccionable?"}
            input type="radio" name="fraccionable"
                id="fraccionable_si" value="true"
            label for="fraccionable_si" {"Si"}
            input type="radio" name="fraccionable"
                id="fraccionable_no" value="false"
            label for="fraccionable_no" {"No"}
        }

        button #crea .form-submit type="submit" { "Crear" }
        button #cancela .form-submit type="button" { "Cancelar" }
    }
}}
