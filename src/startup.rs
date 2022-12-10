//! src/startup.rs
//! author: fyaniquez
//! date: 05/10/2022
//! purpose: encapsulado de la aplicación
//! para su arranque en producción y pruebas

// objetos
use crate::rutas::capitulo;
use crate::rutas::categoria;
use crate::rutas::login;
use crate::rutas::marca;
use crate::rutas::pais;
use crate::rutas::unidad;
use crate::rutas::usuario;
// fin-objetos

use crate::rutas::public::home;
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
            .service(usuario::email::get::usuario_email_form)
            .service(usuario::email::post::usuario_email)
            .service(capitulo::lista::get::muestra)
            .service(capitulo::ve::get::muestra)
            .service(capitulo::crea::get::muestra)
            .service(capitulo::crea::post::procesa)
            .service(capitulo::cambia::get::muestra)
            .service(capitulo::cambia::post::procesa)
            .service(capitulo::borra::delete::procesa)
            .service(capitulo::json::get::muestra)
            .service(categoria::lista::get::muestra)
            .service(categoria::ve::get::muestra)
            .service(categoria::crea::get::muestra)
            .service(categoria::crea::post::procesa)
            .service(categoria::cambia::get::muestra)
            .service(categoria::cambia::post::procesa)
            .service(categoria::borra::delete::procesa)
            .service(pais::lista::get::muestra)
            .service(pais::ve::get::muestra)
            .service(pais::crea::get::muestra)
            .service(pais::crea::post::procesa)
            .service(pais::cambia::get::muestra)
            .service(pais::cambia::post::procesa)
            .service(pais::borra::delete::procesa)
            .service(unidad::lista::get::muestra)
            .service(unidad::ve::get::muestra)
            .service(unidad::crea::get::muestra)
            .service(unidad::crea::post::procesa)
            .service(unidad::cambia::get::muestra)
            .service(unidad::cambia::post::procesa)
            .service(unidad::borra::delete::procesa)
            .service(marca::lista::get::muestra)
            .service(marca::ve::get::muestra)
            .service(marca::crea::get::muestra)
            .service(marca::crea::post::procesa)
            .service(marca::cambia::get::muestra)
            .service(marca::cambia::post::procesa)
            .service(marca::borra::delete::procesa)
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
