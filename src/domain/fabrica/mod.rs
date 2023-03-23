mod fabrica;
mod nombre;
mod nuevo;
mod dml;
pub use fabrica::Fabrica;
pub use fabrica::FabricaError;
pub use nombre::Nombre;
pub use nuevo::Nuevo;
pub use dml::{
    obtiene, 
    lista, 
    lista_paginada,
    inserta,
};
