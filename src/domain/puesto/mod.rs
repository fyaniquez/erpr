mod puesto;
mod nombre;
mod descripcion;
mod sigla;
mod nuevo;
mod dml;
pub use puesto::Puesto;
pub use puesto::PuestoError;
pub use nombre::Nombre;
pub use descripcion::Descripcion;
pub use sigla::Sigla;
pub use nuevo::Nuevo;
pub use dml::{
    lista, lista_paginada,
    obtiene
};
