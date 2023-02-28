mod capitulo;
mod descripcion;
mod nombre;
mod nuevo;
mod dml;
pub use capitulo::Capitulo;
pub use capitulo::CapituloError;
pub use descripcion::Descripcion;
pub use nombre::Nombre;
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_alfabetica,
    lista_paginada,
    obtiene,
};
