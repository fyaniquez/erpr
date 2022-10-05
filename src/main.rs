//! src/main.rs

mod rutas;

use crate::rutas::visitante::home::get::home;
use actix_web::{web, App, HttpServer};

/*
 * author: fyaniquez
 * date: 30/09/2022
 * purpose: sistema de ventas minoristas y mayoristas
 */

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(actix_web_static_files::ResourceFiles::new("/", generated))
            .route("/", web::get().to(home))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
