//! src/rutas/layout/errmsg.rs
//! author: fyaniquez
//! date: 6/12/2022
//! purpose: formato de mensajes de error para el frontend

#[derive(serde::Serialize)]
pub struct ErrMsg {
    pub codigo: i32,
    pub mensaje: String,
}
