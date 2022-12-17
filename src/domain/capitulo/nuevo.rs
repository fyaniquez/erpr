//! src/domain/capitulo/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar un capitulo nuevo

use crate::domain::capitulo::{
    nombre::Nombre,
    descripcion::Descripcion,
};

pub struct Nuevo {
    pub nombre: Nombre,
    pub descripcion: Descripcion,
}
