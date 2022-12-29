//! src/domain/medio/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para validar medio

use crate::domain::medio::{
    nombre::Nombre,
    sigla::Sigla,
};

pub struct Nuevo {
    pub nombre: Nombre,
    pub sigla: Sigla,
}
