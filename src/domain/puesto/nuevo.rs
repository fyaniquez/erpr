//! src/domain/puesto/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar puesto nuevo

use crate::domain::puesto::Nombre;
use chrono::NaiveDateTime;

pub struct Nuevo {
    pub nombre: Nombre,
    pub sucursal_id: i64,
    pub fecha: NaiveDateTime,
    pub estado: String,
}
