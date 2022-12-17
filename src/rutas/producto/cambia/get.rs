//! src/rutas/producto/cambia/get.rs
//! author: fyaniquez
//! date: 7/10/2022
//! purpose: muestra el muestra() de modificacion de producto

use crate::domain::capitulo::{lista as capitulo_lista, Capitulo};
use crate::domain::categoria::{
    lista as categoria_lista, 
    obtiene as categoria_obtiene, 
    Categoria
};
use crate::domain::fabrica::{lista as fabrica_lista, Fabrica};
use crate::domain::marca::{lista as marca_lista, Marca};
use crate::domain::producto::{
    obtiene as producto_obtiene, 
    Producto, 
    ProductoError
};
use crate::domain::unidad::{lista as unidad_lista, Unidad};
use crate::layout;
use actix_web::{get, web, HttpResponse};
use anyhow::Context;
use maud::{html, Markup};
use sqlx::PgPool;

#[tracing::instrument(name = "Formulario cambia producto", skip(pool))]
#[get("/producto/{id}/cambia")]
pub async fn muestra(
    pool: web::Data<PgPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, ProductoError> {
    let (id,) = path.into_inner();
    let producto = producto_obtiene(&pool, id)
        .await
        .context("Error al leer producto")?;

    let categoria = categoria_obtiene(&pool, producto.categoria_id)
        .await
        .context("Error al leer categoria")
        .unwrap();

    let capitulos = capitulo_lista(&pool)
        .await
        .context("Error al leer capitulos")
        .unwrap();

    let categorias = categoria_lista(&pool, categoria.capitulo_id)
        .await
        .context("Error al leer categorias")
        .unwrap();

    let marcas = marca_lista(&pool)
        .await
        .context("Error al leer marcas")
        .unwrap();

    let unidades = unidad_lista(&pool)
        .await
        .context("Error al leer unidades")
        .unwrap();

    let fabricas = fabrica_lista(&pool)
        .await
        .context("Error al leer fabricas")
        .unwrap();

    let pagina = layout::form::crea(
        "Producto",
        "/productos",
        "form.css",
        Some("producto/cambia.js"),
        contenido(
            &producto,
            capitulos,
            categorias,
            marcas,
            unidades,
            fabricas,
            categoria.capitulo_id,
        ),
    );
    Ok(HttpResponse::Ok().body(pagina.unwrap().into_string()))
}

fn contenido(
    producto: &Producto,
    capitulos: Vec<Capitulo>,
    categorias: Vec<Categoria>,
    marcas: Vec<Marca>,
    unidades: Vec<Unidad>,
    fabricas: Vec<Fabrica>,
    capitulo_id: i64,
) -> Markup {
    html! {
        form method="POST" action={"/producto/"(producto.id.unwrap())} {

            label for="nombre" {"Nombre:" }
            input type="text" name="nombre" id="nombre" required
                placeholder="Nombre capítulo" value=(producto.nombre);

            label for="caracteristicas" {"Características:" }
            input type="text" name="caracteristicas" id="caracteristicas"
                required placeholder="Características adicionales"
                value=(producto.caracteristicas);

            label for="capitulo" {"Capítulo:" }
            select #capitulo_id name="capitulo_id" {
                @for capitulo in capitulos.into_iter() {
                    option value=(capitulo.id.unwrap())
                    selected[capitulo.id == Some(capitulo_id)]
                        {(capitulo.nombre)}
                }
            }

            label for="categoria" {"Categoría:" }
            select #categoria_id name="categoria_id" {
                @for categoria in categorias.into_iter() {
                    option value=(categoria.id.unwrap())
                    selected[categoria.id == Some(producto.categoria_id)]
                        {(categoria.nombre)}
                }
            }

            label for="marca" {"Marca:" }
            select #marca_id name="marca_id" {
                @for marca in marcas.into_iter() {
                    option value=(marca.id.unwrap())
                    selected[marca.id == Some(producto.marca_id)]
                        {(marca.nombre)}
                }
            }

            label for="unidad" {"Unidad:" }
            select #unidad_id name="unidad_id" {
                @for unidad in unidades.into_iter() {
                    option value=(unidad.id.unwrap())
                    selected[unidad.id == Some(producto.unidad_id)]
                        {(unidad.nombre)}
                }
            }

            label for="fabrica_id" {"Fábrica:" }
            select #fabrica_id name="fabrica_id" {
                @for fabrica in fabricas.into_iter() {
                    option value=(fabrica.id.unwrap())
                    selected[fabrica.id == Some(producto.fabrica_id)]
                        {(fabrica.nombre)}
                }
            }

            label for="barras" {"Cod.Barras:" }
            input type="text" name="barras" id="barras"
                required placeholder="Código de barras"
                value=(producto.barras.as_ref().unwrap());

            label for="contenido" {"Contenido:" }
            input type="text" name="contenido" id="contenido"
                required placeholder="Contenido sin unidad"
                value=(producto.contenido);

            label for="cantidad" {"Cantidad:" }
            input type="number" name="cantidad" id="cantidad"
                required placeholder="Cantidad p/mayor"
                value=(producto.cantidad);

            fieldset {
                legend { "¿Es fraccionable?"}
                input type="radio" name="fraccionable"
                    id="fraccionable_si" value="true"
                    checked[producto.fraccionable];
                label for="fraccionable_si" {"Si"}
                input type="radio" name="fraccionable"
                    id="fraccionable_no" value="false"
                    checked[!producto.fraccionable];
                label for="fraccionable_no" {"No"}
            }

            fieldset {
                legend { "¿Está activo?"}
                input type="radio" name="activo" id="activo_si" value="true"
                    checked[producto.activo];
                label for="activo_si" {"Si"}
                input type="radio" name="activo" id="activo_no" value="false"
                    checked[!producto.activo];
                label for="activo_no" {"No"}
            }

            button .form-submit #graba type="submit" { "Graba" }
            button .form-submit #cancela type="button" { "Cancela" }
        }
    }
}
// modelo
