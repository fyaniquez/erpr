mod venta;
mod estado;
mod nuevo;
mod dml;
pub use venta::{
    Venta,
    VentaError,
    VentaVe,
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
