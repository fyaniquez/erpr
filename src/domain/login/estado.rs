//! src/rutas/login/estado.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: control del estado de la aplicación

use std::fmt::Debug;

#[derive(Debug)]
pub struct Estado {
    pub usuario_id: i64,
    pub sucursal_id: i64,
    pub puesto_id: i64,
    pub catalogo_id: i64,
}

pub fn get_estado() -> Estado {
    return Estado {
        usuario_id: 1, 
        sucursal_id: 1, 
        puesto_id: 1, 
        catalogo_id: 2
    }
}
