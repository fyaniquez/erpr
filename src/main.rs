//! src/main.rs

use erpr::startup::run;
use erpr::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use erpr::telemetry::{get_subscriber, init_subscriber};
use std::io::stdout;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // ajusta el procesamiento de logs
    let subscriber = get_subscriber("erpr".into(), "info".into(), stdout);
    init_subscriber(subscriber);

    // obtiene configuracion de los *.yaml
    let conf = get_configuration()
        .expect("Error al leer configuración");

    // obtiene un pool de conexiones a la BD
    /*
    let connection = PgPool::connect(
            &conf.database.connection_string().expose_secret()
        )
        .await
        .expect("Error al conectarse a la BD");
    connection.connection_timeout(std::time::Duration::from_secs(2));
    */
    let connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        //.connection_timeout(std::time::Duration::from_secs(2)
        //.connect_lazy(conf.database.connection_string().expose_secret())
        //.connect(&conf.database.connection_string().expose_secret())
        .connect_lazy(conf.database.connection_string().expose_secret())
        .unwrap();
        /*
    let connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2)
        .connect_lazy(conf.database.connection_string().expose_secret());
        */

    // establece la dirección del servidor de aplicaciones
    let listener = TcpListener::bind(conf.application.get_address())
        .expect("Error al crear listener");
    let port = listener.local_addr().unwrap().port();
    println!(
        "Servicio disponible en: http://{}:{}", 
        conf.application.host, port);

    // arranca la aplicación
    run(listener, connection)?.await
}


