//! src/domain/vendido/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar vendido nuevo

pub struct Nuevo {
    pub venta_id: i64,
    pub producto_id: i64,
    pub cantidad: i64,
    pub precio: i64,
    pub descuento: i64,
    pub total: i64,
}
