//! src/domain/sigla_nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para validar unidad

use crate::domain::{
    UnidadNombre,
    UnidadSigla,
};

pub struct UnidadNuevo {
    pub nombre: UnidadNombre,
    pub sigla: UnidadSigla,
}
