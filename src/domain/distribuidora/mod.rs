mod distribuidora;
mod nit;
mod nombre;
mod nuevo;
mod dml;
pub use distribuidora::Distribuidora;
pub use distribuidora::DistribuidoraError;
pub use nit::Nit;
pub use nombre::Nombre;
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
};
