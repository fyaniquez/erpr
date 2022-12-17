//! src/domain/empresa/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar un empresa nuevo

use crate::domain::empresa::{
    nombre::Nombre,
    nit::Nit,
};

pub struct Nuevo {
    pub nombre: Nombre,
    pub nit: Nit,
    pub activa: bool,
}
