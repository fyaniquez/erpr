//! src/domain/vendedor/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar un vendedor nuevo

use crate::domain::vendedor::{
    nombre::Nombre,
    cargo::Cargo,
};

pub struct Nuevo {
    pub nombre: Nombre,
    pub cargo: Cargo,
    pub contactos: String,
}
