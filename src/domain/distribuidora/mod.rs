mod distribuidora;
mod documento;
mod nombre;
mod nuevo;
mod dml;
pub use distribuidora::Distribuidora;
pub use distribuidora::DistribuidoraError;
pub use documento::Documento;
pub use nombre::Nombre;
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    obtiene,
};
