//! src/domain/catalogo/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar catalogo nuevo

use crate::domain::catalogo::nombre::Nombre;

pub struct Nuevo {
    pub nombre: Nombre,
    pub sucursal_id: i64,
}
