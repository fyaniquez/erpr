//! src/main.rs

use erpr::startup::run;
use erpr::configuration::get_configuration;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration()
        .expect("Error al leer configuración");
    let listener = TcpListener::bind(conf.application.get_address())
        .expect("Error al crear listener");
    run(listener)?.await
}
