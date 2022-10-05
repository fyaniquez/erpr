//! src/startup.rs

use crate::configuration::Settings;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use crate::rutas::visitante::home::get::home;

/*
 * author: fyaniquez
 * date: 05/10/2022
 * purpose: encapsulado de la aplicación 
 * para su arranque en producción y pruebas
 */

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, configuration.application.base_url).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener, base_url: String) 
-> Result<Server, anyhow::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(home))
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
