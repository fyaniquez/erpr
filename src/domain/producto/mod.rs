mod producto;
mod nombre;
mod caracteristicas;
mod contenido;
mod nuevo;
mod dml;
pub use producto::Producto;
pub use producto::ProductoError;
pub use nombre::Nombre;
pub use caracteristicas::Caracteristicas;
pub use contenido::Contenido;
pub use nuevo::Nuevo;
pub use dml::{
    lista, 
    lista_paginada,
    lista_sin_precio, 
    lista_paginada_sin_precio,
    obtiene,
};
