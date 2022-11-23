//! src/domain/capitulo_nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar un capitulo nuevo

use crate::domain::{
    CapituloNombre,
    CapituloDescripcion,
};

pub struct CapituloNuevo {
    pub nombre: CapituloNombre,
    pub descripcion: CapituloDescripcion,
}
