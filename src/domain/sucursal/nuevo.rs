//! src/domain/sucursal/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar sucursal nuevo

use crate::domain::sucursal::nombre::Nombre;

pub struct Nuevo {
    pub nombre: Nombre,
    pub empresa_id: i64,
}
