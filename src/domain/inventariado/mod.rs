mod inventariado;
mod nuevo;
mod dml;
pub use inventariado::Inventariado;
pub use inventariado::InventariadoError;
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
    inserta,
    actualiza,
};
