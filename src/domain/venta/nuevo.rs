//! src/domain/venta/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar venta nuevo

pub struct Nuevo {
    pub subtotal: i32,
    pub descuento: i32,
    pub cliente_id: i64,
    pub puesto_id: i64,
    pub usuario_id: i64,
    pub medio_id: i64,
}
