//! src/rutas/producto/cambia/post.rs
//! author: fyaniquez
//! date: 30/09/2022
//! purpose: procesa el formulario de modificacion de producto

use crate::domain::inventariado::{
    Nuevo as Inventariado_Nuevo,
    inserta as inventariado_inserta,
};
use crate::domain::precio::{
    Nuevo as Precio_Nuevo,
    inserta as precio_inserta,
};

use crate::domain::producto::{
    Producto, 
    ProductoError,
    Contenido, 
    Caracteristicas,
    actualiza as producto_actualiza,
};
use actix_web::{http::header, post, web, HttpResponse};
use anyhow::Context;

// información que recopila el formulario de alta
#[derive(serde::Deserialize)]
pub struct FormData {
    contenido: String,
    caracteristicas: String,
    activo: bool,
    barras: String,
    cantidad: i32,
    categoria_id: i64,
    fraccionable: bool,
    fabrica_id: i64,
    marca_id: i64,
    unidad_id: i64,
}

// valida y contruye el objeto FormData
impl TryFrom<FormData> for Producto {
    type Error = String;
    fn try_from(form_data: FormData) -> Result<Self, Self::Error> {
        let contenido = Contenido::parse(form_data.contenido)?;
        let caracteristicas = Caracteristicas::parse(form_data.caracteristicas)?;
        Ok( Self{ 
            id: None, 
            nombre: "-.-".to_string(),
            contenido: String::from(contenido.as_ref()), 
            caracteristicas: String::from(caracteristicas.as_ref()), 
            activo: form_data.activo,
            barras: Some(form_data.barras),
            cantidad: form_data.cantidad,
            categoria_id: form_data.categoria_id,
            fraccionable: form_data.fraccionable,
            fabrica_id: form_data.fabrica_id,
            marca_id: form_data.marca_id,
            unidad_id: form_data.unidad_id,
        })
    }
}

// extrae datos del producto del formulario, los verifica
// e inserta en la base de datos
#[tracing::instrument(
    name = "Actualización de producto",
    skip(form, pool),
)]
#[post("/producto/{id}")]
pub async fn procesa(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<sqlx::PgPool>
) -> Result<HttpResponse, ProductoError> {
    let (id,) = path.into_inner();
    let producto = form.0.try_into().map_err(ProductoError::Validacion)?;
    producto_actualiza(&pool, &producto, id)
        .await
        .context("Error al actualizar producto en la BD")?;
    let url_ver =  format!("/producto/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}
fn parse_string(input: &str) -> (i32, chrono::NaiveDate) {
    let mut parts = input.split(';');
    let numberf: f32 = parts.next().unwrap().parse().unwrap();
    let number = (numberf * 100.0) as i32;
    //let datef = parts.next().unwrap().to_string();
    let datef = parts.next().unwrap();
    let date = chrono::NaiveDate::parse_from_str(datef, "%d/%m/%Y").unwrap();
    (number, date)
}

// extrae datos del producto del formulario, los verifica
// e inserta en la base de datos junto al inventario y al catalogo
#[tracing::instrument(
    name = "Actualización de producto total",
    skip(form, pool),
)]
#[post("/productotot/{id}")]
pub async fn procesatot(
    path: web::Path<(i64,)>, 
    form: web::Form<FormData>, 
    pool: web::Data<sqlx::PgPool>
) -> Result<HttpResponse, ProductoError> {
    let (id,) = path.into_inner();
    let producto: Producto = form.0.try_into().map_err(ProductoError::Validacion)?;
    
    // lineas añadidas para ayudar la inventariación
    let barras = producto.barras.as_ref().unwrap();
    let (cantidad, vencimiento) = parse_string(&barras);
    //let cantidadf = &producto.barras;
    //let cantidadg: &String = cantidadf.as_ref().unwrap();
    //let cantidadh: f32 = cantidadg.parse().unwrap();
    //let cantidad: i32 = (cantidadh * 100.0) as i32;
//
    let preciof: &String = &producto.contenido;
    let preciog: &String = preciof;
    let precioh: f32 = preciog.parse().unwrap();
    let precio: i32 = (precioh * 100.0) as i32;

    producto_actualiza(&pool, &producto, id)
        .await
        .context("Error al actualizar producto en la BD")?;

    let inventariado = Inventariado_Nuevo {
        producto_id: id,
        cantidad, 
        vencimiento, 
        inventario_id: 5,
    };

    let _iid = inventariado_inserta(&pool, &inventariado)
        .await
        .map_err(|e| ProductoError::Validacion(e.to_string()));

    let preciobj = Precio_Nuevo {
        precio, 
        producto_id: id,
        catalogo_id: 2,
    };

    let _pid = precio_inserta(&pool, &preciobj)
        .await
        .map(|e| ProductoError::Validacion(e.to_string()));

    let url_ver =  format!("/producto/{}", id);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url_ver))
        .finish())
}

