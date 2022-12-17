//! src/domain/producto/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar producto nuevo

use crate::domain::producto::{
    nombre::Nombre,
    contenido::Contenido,
    caracteristicas::Caracteristicas,
};

pub struct Nuevo {
    pub nombre: Nombre,
    pub caracteristicas: Caracteristicas,
    pub categoria_id: i64,
    pub marca_id: i64,
    pub unidad_id: i64,
    pub fabrica_id: i64,
    pub contenido: Contenido,
    pub cantidad: i32,
    pub fraccionable: bool,
    pub barras: Option<String>,
    pub activo: bool,
}
