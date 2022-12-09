//! src/domain/sigla_nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para validar sigla de pais

use crate::domain::{
    PaisNombre,
    PaisSigla,
};

pub struct PaisNuevo {
    pub nombre: PaisNombre,
    pub sigla: PaisSigla,
}
