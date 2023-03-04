mod producto;
mod caracteristicas;
mod contenido;
mod nuevo;
mod dml;
pub use producto::{
    Producto,
    ProductoVe,
    ProductoError,
};
pub use caracteristicas::Caracteristicas;
pub use contenido::Contenido;
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    lista_sin_precio, 
    lista_paginada_sin_precio,
    obtiene_ve,
    obtiene,
    inserta,
};
