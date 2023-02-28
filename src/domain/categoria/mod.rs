mod categoria;
mod nombre;
mod nuevo;
mod dml;
pub use categoria::Categoria;
pub use categoria::CategoriaError;
pub use nombre::Nombre;
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
    inserta,
};
