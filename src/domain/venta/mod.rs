mod venta;
mod estado;
mod nuevo;
mod dml;
pub use venta::Venta;
pub use venta::VentaError;
pub use nuevo::Nuevo;
pub use estado::Estado;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
};
