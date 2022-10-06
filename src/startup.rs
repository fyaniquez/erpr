//! src/startup.rs

use actix_web::{web, App, HttpServer};
use crate::rutas::visitante::home::get::home;
use actix_web_static_files::ResourceFiles;

/*
 * author: fyaniquez
 * date: 05/10/2022
 * purpose: encapsulado de la aplicación 
 * para su arranque en producción y pruebas
 */
/*
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
*/
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub async fn run() 
-> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .route("/", web::get().to(home))
            .service(ResourceFiles::new("/", generated)
                .do_not_resolve_defaults())
    })
    //.listen(listener)?
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
