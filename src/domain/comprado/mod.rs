mod comprado;
mod nuevo;
mod dml;
pub use comprado::{
    Comprado,
    Comprados,
    CompradoVe,
    CompradoError,
};
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
    obtiene_ve,
    inserta_mul,
    inserta,
};
