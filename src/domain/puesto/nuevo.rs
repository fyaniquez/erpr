//! src/domain/puesto/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar puesto nuevo

use crate::domain::puesto::nombre::Nombre;
use crate::domain::puesto::sigla::Sigla;
use crate::domain::puesto::descripcion::Descripcion;

pub struct Nuevo {
    pub nombre: Nombre,
    pub sigla: Sigla,
    pub descripcion: Descripcion,
    pub sucursal_id: i64,
    pub activo: bool
}
