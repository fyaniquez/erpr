mod categoria_marca;
mod dml;
pub use categoria_marca::{
    CategoriaMarca,
    CategoriaMarcaError,
    CategoriaMarcaNombres,
};
pub use dml::{
    lista,
    lista_paginada,
    obtiene,
    inserta,
    borra,
};
