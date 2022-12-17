//! src/domain/usuario/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar un usuario nuevo

use crate::domain::usuario::{
    apellido::Apellido, 
    email::Email, 
    nombre::Nombre,
    documento::Documento,
};

pub struct Nuevo {
    pub email: Email,
    pub nombre: Nombre,
    pub apellido: Apellido,
    pub documento: Documento,
}
impl Nuevo {
    // retorna el nombre del usuario
    pub fn nombres(&self) -> String {
        if self.apellido.as_ref().is_empty() {
            return self.nombre.as_ref().to_string()
        }
        format!("{}, {}", self.apellido.as_ref(), self.nombre.as_ref())
    }
}
