mod compra;
mod estado;
mod nuevo;
mod dml;
pub use compra::{
    Compra,
    CompraError,
    CompraVe,
};
pub use nuevo::Nuevo;
pub use estado::Estado;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
    obtiene_ve,
    inserta,
};
