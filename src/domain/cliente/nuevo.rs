//! src/domain/cliente/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar un cliente nuevo

use crate::domain::cliente::{
    nombre::Nombre,
    documento::Documento,
};

pub struct Nuevo {
    pub nombre: Nombre,
    pub documento: Documento,
}
