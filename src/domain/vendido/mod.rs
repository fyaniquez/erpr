mod vendido;
mod nuevo;
mod dml;
pub use vendido::{
    Vendido,
    VendidoVe,
    VendidoError,
};
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
    obtiene_ve,
};
