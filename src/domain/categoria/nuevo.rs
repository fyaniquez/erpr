//! src/domain/categoria/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar categoria nuevo

use crate::domain::categoria::nombre::Nombre;

pub struct Nuevo {
    pub nombre: Nombre,
    pub capitulo_id: i64,
}
