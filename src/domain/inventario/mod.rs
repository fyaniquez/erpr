mod inventario;
mod nuevo;
mod nombre;
mod dml;
pub use inventario::Inventario;
pub use inventario::InventarioError;
pub use nuevo::Nuevo;
pub use nombre::Nombre;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
    inserta,
};
