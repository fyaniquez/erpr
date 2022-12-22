mod precio;
mod nuevo;
mod dml;
pub use precio::Precio;
pub use precio::PrecioError;
pub use nuevo::Nuevo;
pub use dml::{
    lista, lista_paginada,
    obtiene
};
