mod puesto;
mod nuevo;
mod nombre;
mod dml;
pub use puesto::Puesto;
pub use puesto::PuestoError;
pub use nuevo::Nuevo;
pub use nombre::Nombre;
pub use dml::{
    lista, lista_paginada,
    obtiene
};
