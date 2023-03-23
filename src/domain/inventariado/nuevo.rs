//! src/domain/inventariado/nuevo
//! author: fyaniquez
//! date: 30/10/2022
//! estructura para procesar inventariado nuevo

use chrono::NaiveDate;

pub struct Nuevo {
    pub producto_id: i64,
    pub cantidad: i32,
    pub vencimiento: NaiveDate,
    pub inventario_id: i64,
}
