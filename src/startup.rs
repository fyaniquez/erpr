//! src/startup.rs
//! author: fyaniquez
//! date: 05/10/2022
//! purpose: encapsulado de la aplicación
//! para su arranque en producción y pruebas

// objetos
use crate::rutas::capitulo;
use crate::rutas::categoria;
use crate::rutas::fabrica;
use crate::rutas::login;
use crate::rutas::marca;
use crate::rutas::pais;
use crate::rutas::unidad;
use crate::rutas::usuario;
use crate::rutas::producto;
use crate::rutas::empresa;
use crate::rutas::catalogo;
use crate::rutas::precio;
use crate::rutas::sucursal;
use crate::rutas::inventario;
use crate::rutas::inventariado;
use crate::rutas::puesto;
use crate::rutas::cliente;
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
            .service(fabrica::lista::get::muestra)
            .service(fabrica::ve::get::muestra)
            .service(fabrica::crea::get::muestra)
            .service(fabrica::crea::post::procesa)
            .service(fabrica::cambia::get::muestra)
            .service(fabrica::cambia::post::procesa)
            .service(fabrica::borra::delete::procesa)
            .service(producto::lista::get::muestra)
            .service(producto::lista_sin_precio::get::muestra)
            .service(producto::ve::get::muestra)
            .service(producto::crea::get::muestra)
            .service(producto::crea::post::procesa)
            .service(producto::cambia::get::muestra)
            .service(producto::cambia::post::procesa)
            .service(producto::borra::delete::procesa)
            .service(empresa::lista::get::muestra)
            .service(empresa::ve::get::muestra)
            .service(empresa::crea::get::muestra)
            .service(empresa::crea::post::procesa)
            .service(empresa::cambia::get::muestra)
            .service(empresa::cambia::post::procesa)
            .service(empresa::borra::delete::procesa)
            .service(catalogo::lista::get::muestra)
            .service(catalogo::ve::get::muestra)
            .service(catalogo::crea::get::muestra)
            .service(catalogo::crea::post::procesa)
            .service(catalogo::cambia::get::muestra)
            .service(catalogo::cambia::post::procesa)
            .service(catalogo::borra::delete::procesa)
            .service(precio::lista::get::muestra)
            .service(precio::ve::get::muestra)
            .service(precio::crea::get::muestra)
            .service(precio::crea::post::procesa)
            .service(precio::cambia::get::muestra)
            .service(precio::cambia::post::procesa)
            .service(precio::borra::delete::procesa)
            .service(sucursal::lista::get::muestra)
            .service(sucursal::ve::get::muestra)
            .service(sucursal::crea::get::muestra)
            .service(sucursal::crea::post::procesa)
            .service(sucursal::cambia::get::muestra)
            .service(sucursal::cambia::post::procesa)
            .service(sucursal::borra::delete::procesa)
            .service(inventario::lista::get::muestra)
            .service(inventario::ve::get::muestra)
            .service(inventario::crea::get::muestra)
            .service(inventario::crea::post::procesa)
            .service(inventario::cambia::get::muestra)
            .service(inventario::cambia::post::procesa)
            .service(inventario::borra::delete::procesa)
            .service(inventariado::lista::get::muestra)
            .service(inventariado::ve::get::muestra)
            .service(inventariado::crea::get::muestra)
            .service(inventariado::crea::post::procesa)
            .service(inventariado::cambia::get::muestra)
            .service(inventariado::cambia::post::procesa)
            .service(inventariado::borra::delete::procesa)
            .service(puesto::lista::get::muestra)
            .service(puesto::ve::get::muestra)
            .service(puesto::crea::get::muestra)
            .service(puesto::crea::post::procesa)
            .service(puesto::cambia::get::muestra)
            .service(puesto::cambia::post::procesa)
            .service(puesto::borra::delete::procesa)
            .service(cliente::lista::get::muestra)
            .service(cliente::ve::get::muestra)
            .service(cliente::crea::get::muestra)
            .service(cliente::crea::post::procesa)
            .service(cliente::cambia::get::muestra)
            .service(cliente::cambia::post::procesa)
            .service(cliente::borra::delete::procesa)
            // fin-servicios
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
