mod vendido;
mod nuevo;
mod dml;
pub use vendido::{
    Vendido,
    Vendidos,
    VendidoVe,
    VendidoError,
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
