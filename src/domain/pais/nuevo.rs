//! src/domain/pais/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para validar pais

use crate::domain::pais::{
    nombre::Nombre,
    sigla::Sigla,
};

pub struct Nuevo {
    pub nombre: Nombre,
    pub sigla: Sigla,
}
