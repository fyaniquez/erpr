//! tests/helpers.rs
//! author: faniquez
//! date: 12/10/2022
//! purpose: funciones auxiliares/genéricas para tests

use std::net::TcpListener;
use erpr::configuration::get_configuration;
use sqlx::PgPool;

pub struct TestApp {
    pub app_address: String,
    pub db_pool: PgPool,
}

/// arranca el servidor web en un puerto aleatorio
/// devuelve la dirección de la aplicacion
/// y la conección a la base de datos
pub async fn spawn_app() -> TestApp {
    let configuration = get_configuration()
        .expect("Error al leer configuración");
    let listener = TcpListener::bind(configuration.application.get_address())
        .expect("Error al crear listener");
    let port = listener.local_addr().unwrap().port();
    let app_address = format!(
        "http://{}:{}", 
        configuration.application.host, 
        port);
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Error al conectar a la base de datos");
    let server = erpr::startup::run(listener, db_pool.clone())
        .expect("Error al arrancar app");
    let _ = tokio::spawn(server);
    TestApp {
        app_address,
        db_pool,
    }
}
