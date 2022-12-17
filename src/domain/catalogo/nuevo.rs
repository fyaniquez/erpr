//! src/domain/catalogo/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar catalogo nuevo

use crate::domain::catalogo::nombre::Nombre;
use chrono::NaiveDateTime;

pub struct Nuevo {
    pub nombre: Nombre,
    pub propietario: i64,
    pub empresa_id: i64,
    pub fecha: NaiveDateTime,
    pub activo: bool
}
