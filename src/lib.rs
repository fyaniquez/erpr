//! src/lib.rs

pub mod rutas;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use crate::rutas::public::home::get;
use actix_web_static_files::ResourceFiles;
use std::net::TcpListener;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        let generated = generate();
        App::new()
        .service(get::home)
        .service(ResourceFiles::new("/", generated)
            .do_not_resolve_defaults())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
