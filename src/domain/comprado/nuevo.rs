//! src/domain/comprado/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar comprado nuevo

pub struct Nuevo {
    pub venta_id: i64,
    pub producto_id: i64,
    pub cantidad: i64,
    pub precio: i64,
    pub vencimiento: chrono::NaiveDate,
    pub descuento: i64,
    pub total: i64,
}
