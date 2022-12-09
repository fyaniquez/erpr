//! src/domain/categoria_nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar categoria nuevo

use crate::domain::CategoriaNombre;

pub struct CategoriaNuevo {
    pub nombre: CategoriaNombre,
    pub capitulo_id: i64,
}
