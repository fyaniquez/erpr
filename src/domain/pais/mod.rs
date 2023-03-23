mod pais;
mod nombre;
mod sigla;
mod nuevo;
mod dml;
pub use pais::Pais;
pub use pais::PaisError;
pub use nombre::Nombre;
pub use sigla::Sigla;
pub use nuevo::Nuevo;
pub use dml::{
    lista,
    lista_paginada,
    obtiene,
    inserta,
    actualiza,
};
