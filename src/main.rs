//! src/main.rs

use erpr::startup::run;
use erpr::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration()
        .expect("Error al leer configuración");
    let connection = PgPool::connect(&conf.database.connection_string())
        .await
        .expect("Error al conectarse a la BD");
    let listener = TcpListener::bind(conf.application.get_address())
        .expect("Error al crear listener");
    
    let port = listener.local_addr().unwrap().port();
    println!(
        "Servicio disponible en: http://{}:{}", 
        conf.application.host, port);

    run(listener, connection)?.await
}
