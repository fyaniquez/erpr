mod cliente;
mod documento;
mod nombre;
mod nuevo;
mod dml;
pub use cliente::{
    Cliente,
    ClienteError,
};
pub use documento::Documento;
pub use nombre::Nombre;
pub use nuevo::Nuevo;
pub use dml::{
    lista, lista_paginada,
    obtiene,
};
