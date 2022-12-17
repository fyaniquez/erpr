//! src/domain/unidad/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para validar unidad

use crate::domain::unidad::nombre::Nombre;
use crate::domain::unidad::sigla::Sigla;

pub struct Nuevo {
    pub nombre: Nombre,
    pub sigla: Sigla,
}
