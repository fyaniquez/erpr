//! src/domain/fabrica/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para validar fabrica

use crate::domain::fabrica::nombre::Nombre;

pub struct Nuevo {
    pub nombre: Nombre,
    pub pais_id: i64,
}
