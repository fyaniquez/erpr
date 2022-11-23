//! src/domain/usuario_nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar un usuario nuevo

use crate::domain::{
    UsuarioApellido, 
    UsuarioEmail, 
    UsuarioNombre,
    UsuarioDocumento,
};

pub struct UsuarioNuevo {
    pub email: UsuarioEmail,
    pub nombre: UsuarioNombre,
    pub apellido: UsuarioApellido,
    pub documento: UsuarioDocumento,
}
impl UsuarioNuevo {
    // retorna el nombre del usuario
    pub fn nombres(&self) -> String {
        if self.apellido.as_ref().is_empty() {
            return self.nombre.as_ref().to_string()
        }
        format!("{}, {}", self.apellido.as_ref(), self.nombre.as_ref())
    }
}
