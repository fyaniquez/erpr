//! tests/helpers.rs

use std::net::TcpListener;

/*
 * author: faniquez
 * date: 12/10/2022
 * purpose: funciones auxiliares/genéricas para tests
 */

/// arranca el servidor web en un puerto aleatorio
/// devuelve la dirección de arranque con el puerto
pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Error al crear listener");
    let port = listener.local_addr().unwrap().port();
    let server = erpr::startup::run(listener).expect("Error al arrancar app");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
