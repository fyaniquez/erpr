//! src/startup.rs
//! author: fyaniquez
//! date: 05/10/2022
//! purpose: encapsulado de la aplicación
//! para su arranque en producción y pruebas

use crate::rutas::capitulo;
use crate::rutas::categoria;
use crate::rutas::pais;
use crate::rutas::login;
use crate::rutas::public::home;
use crate::rutas::usuario;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_static_files::ResourceFiles;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// arranca servidor http, adjunta endpoints del api,
/// directorio de páginas estaticas
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(home::get::home)
            .service(login::email::get::login_email_form)
            .service(login::email::post::login_email)
            .service(login::pass::get::login_pass_form)
            .service(login::pass::post::login_pass)
            .service(pais::lista::get::formulario)
            .service(pais::ve::get::pantalla)
            .service(usuario::email::get::usuario_email_form)
            .service(usuario::email::post::usuario_email)
            .service(capitulo::lista::get::capitulo_lista_form)
            .service(capitulo::ve::get::capitulo_ve)
            .service(capitulo::crea::get::capitulo_crea_form)
            .service(capitulo::crea::post::capitulo_crea)
            .service(capitulo::cambia::get::capitulo_cambia_form)
            .service(capitulo::cambia::post::capitulo_cambia)
            .service(capitulo::borra::delete::capitulo_borra)
            .service(capitulo::json::get::capitulo_json)
            .service(categoria::lista::get::categoria_lista_form)
            .service(categoria::ve::get::categoria_ve)
            .service(categoria::crea::get::categoria_crea_form)
            .service(categoria::crea::post::categoria_crea)
            .service(categoria::cambia::get::categoria_cambia_form)
            .service(categoria::cambia::post::categoria_cambia)
            .service(categoria::borra::delete::categoria_borra)
            .service(pais::crea::get::formulario)
            .service(pais::crea::post::proceso)
            .service(pais::cambia::get::formulario)
            .service(pais::cambia::post::proceso)
            .service(pais::borra::delete::proceso)
           
            //.service(Files::new("/public", "./public")
            //.path_filter(|path, _| {
            //if path.extension() == "js" {
            //self.header("mime-type", http::mime::JAVASCRIPT);
            //}
            //true
            //})
            .service(ResourceFiles::new("/", generated).do_not_resolve_defaults())
            .app_data(db_pool.clone())
            .wrap(TracingLogger::default())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
