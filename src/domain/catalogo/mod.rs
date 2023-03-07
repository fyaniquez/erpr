mod catalogo;
mod nombre;
mod nuevo;
mod dml;
pub use catalogo::Catalogo;
pub use catalogo::CatalogoError;
pub use nombre::Nombre;
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
    inserta,
};
