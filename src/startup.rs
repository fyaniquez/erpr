//! src/startup.rs
//! author: fyaniquez
//! date: 05/10/2022
//! purpose: encapsulado de la aplicación
//! para su arranque en producción y pruebas

use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use crate::rutas::public::home;
use crate::rutas::login;
use actix_web_static_files::ResourceFiles;
use std::net::TcpListener;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// arranca servidor http, adjunta endpoints del api, 
/// directorio de páginas estaticas
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        let generated = generate();
        App::new()
        .service(home::get::home)
        .service(login::email::get::login_email_form)
        .service(login::email::post::login_email)
        .service(login::pass::get::login_pass_form)
        .service(login::pass::post::login_pass)
        .service(ResourceFiles::new("/", generated)
            .do_not_resolve_defaults())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
