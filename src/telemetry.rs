//! src/telemetry.rs
//! prepara y arranca los paquetes relacionados a logs
//! author: fyaniquez
//! date: mar 25 oct 2022

use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;

// Obtiene un objeto tracing::subscriber para captura de logs
// para ello compone multipes capas de captura y presentación de logs
// los traits Send y Sync son importantes para devolver el objeto
// name es el nombre del binario por ejemplo erpr
// env_filter es el nivel del tracing "info", "trace", etc
// env_filter es el nivel del tracing "info", "trace", etc
// sink indica donde se redirigiran los logs p/e std output

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync 
    where
        // HRTB sink implementa Makewriter (ver rust nomicon)
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
  // revisa la variable RUST_LOG si no existe usa "info"
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    
    // formatea el log en json y hace que se redirija por/ejem al stdout
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // combina los elementos para crear el tracing
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Inicia el sistema de log
/// Registra un subscriber a nivel global para seguimiento integral
/// solo debe llamarse una vez
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // captura logs de todos los crates
    LogTracer::init().expect("Error al arrancar log tracer");
  
    // publica el tracing para toda la aplicación
    set_global_default(subscriber).expect("Error al globalizar tracing");
}
