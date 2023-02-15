mod vendedor;
mod cargo;
mod nombre;
mod nuevo;
mod dml;
pub use vendedor::{
    Vendedor,
    VendedorError,
};
pub use cargo::Cargo;
pub use nombre::Nombre;
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
};
