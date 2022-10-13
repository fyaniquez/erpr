//! tests/helpers.rs
//! author: faniquez
//! date: 12/10/2022
//! purpose: funciones auxiliares/genéricas para tests

use std::net::TcpListener;
use erpr::configuration::get_configuration;

/// arranca el servidor web en un puerto aleatorio
/// devuelve la dirección de arranque con el puerto
pub fn spawn_app() -> String {
    let configuration = get_configuration()
        .expect("Error al leer configuración");
    let listener = TcpListener::bind(configuration.application.get_address())
        .expect("Error al crear listener");
    let port = listener.local_addr().unwrap().port();
    let server = erpr::startup::run(listener).expect("Error al arrancar app");
    let _ = tokio::spawn(server);
    format!("http://{}:{}", configuration.application.host, port)
}
