mod sucursal;
mod nombre;
mod nuevo;
mod dml;
pub use sucursal::Sucursal;
pub use sucursal::SucursalError;
pub use nombre::Nombre;
pub use nuevo::Nuevo;
pub use dml::{
    lista, lista_paginada,
    obtiene
};
