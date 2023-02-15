//! src/domain/distribuidora/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar un distribuidora nuevo

use crate::domain::distribuidora::{
    nombre::Nombre,
    documento::Documento,
};

pub struct Nuevo {
    pub nombre: Nombre,
    pub documento: Documento,
    pub descripcion: String,
    pub preventa: String,
}
