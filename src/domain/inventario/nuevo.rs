//! src/domain/inventario/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar inventario nuevo

use crate::domain::inventario::Nombre;
use chrono::NaiveDateTime;

pub struct Nuevo {
    pub nombre: Nombre,
    pub sucursal_id: i64,
    pub fecha: NaiveDateTime,
    pub estado: String,
}
