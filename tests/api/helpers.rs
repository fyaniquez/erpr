//! tests/helpers.rs
//! author: faniquez
//! date: 12/10/2022
//! purpose: funciones auxiliares/genéricas para tests

use erpr::configuration::{get_configuration, DatabaseSettings};
use erpr::telemetry::{get_subscriber, init_subscriber};
use sqlx::migrate::MigrateDatabase;
use sqlx::PgPool;
use sqlx::Postgres;
use std::net::TcpListener;
use uuid::Uuid;
use once_cell::sync::Lazy;
use std::io::{stdout, sink};
use secrecy::ExposeSecret;

// para que el stack tracing sea inicializado una sola vez
// defina TEST_LOG para mostrar logs o redirigidos a null
// p/ej $ TEST_LOG=true cargo test login_works | bunyan
static TRACING: Lazy<()> = Lazy::new(|| {
    let filter_level = "info".to_string();
    let subs_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subs_name, filter_level, stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subs_name, filter_level, sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub app_address: String,
    pub db_pool: PgPool,
}

/// arranca el servidor web en un puerto aleatorio
/// devuelve la dirección de la aplicacion
/// y la conección a la base de datos
pub async fn spawn_app() -> TestApp {
    // se ejecuta solamente la primera vez que lo llaman 
    Lazy::force(&TRACING);

    // lee la configuracion de las variables de medioambiente o defaults
    let mut conf = get_configuration()
        .expect("Error al leer configuración");
    conf.database.name = Uuid::new_v4().to_string();
    let db_pool = configure_database(&conf.database).await;

    // genera y configura el url para conectar a la aplicación
    let listener = TcpListener::bind(conf.application.get_address())
        .expect("Error al crear listener");
    let port = listener.local_addr().unwrap().port();
    let app_address = format!("http://{}:{}", conf.application.host, port);

    // arranca web server
    let server = erpr::startup::run(listener, db_pool.clone())
        .expect("Error al arrancar app");
    let _ = tokio::spawn(server);
    TestApp {
        app_address,
        db_pool,
    }
}

/// crea una conexión a la base de datos
/// si no existe crea la base de datos
/// config parametros de la base de datos
/// devuelve la conexión a la base de datos
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // crea si no existe

    let db_exists = Postgres::database_exists(
            &config.connection_string().expose_secret()
        )
        .await
        .expect("Error al verificar BD");
    if !db_exists { 
        Postgres::create_database(
            &config.connection_string().expose_secret()
        )
        .await
        .expect("Error al crear BD");
    }

    // migración
    let connection_pool = PgPool::connect(
            &config.connection_string().expose_secret()
        )
        .await
        .expect(&format!(
            "Error al conectar a la BD {}",
            &config.connection_string_without_db().expose_secret()
        ));

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("No se pudo migrar la BD");

    connection_pool
}
