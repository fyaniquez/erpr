//! src/layouts/lista_paginada.rs
//! crea un menu para la aplicación
//! autor: fyaniquez
//! fecha: 17-05-2023

use maud::{html, Markup};

#[derive(Debug)]
struct Opcion <'a>{
    nombre: &'a str,
    url: &'a str,
    etiqueta: &'a str,
}
const OPCIONES: [Opcion<'_>; 14]  = [
    Opcion { 
        nombre: "Productos",
        url: "/productos",
        etiqueta: "Productos disponibles en el sistema",
    },
    Opcion { 
        nombre: "Paises",
        url: "/paises",
        etiqueta: "Paises sede de las fábricas",
    },
    Opcion {
        nombre: "Marcas",
        url: "/marcas",
        etiqueta: "Marcas de los productos",
    },
    Opcion {
        nombre: "Capítulos",
        url: "/capitulos",
        etiqueta: "Primer nivel de organización de productos",
    },
    Opcion {
        nombre: "Inventarios",
        url: "/inventarios",
        etiqueta: "Inventarios realizados y en curso",
    },
    Opcion {
        nombre: "Catálogos",
        url: "/catalogos",
        etiqueta: "Catálogo de productos incluyendo precios",
    },
    Opcion {
        nombre: "Distribuidoras",
        url: "/distribuidoras",
        etiqueta: "Empresas que distribuyen los productos",
    },
    Opcion {
        nombre: "Compras",
        url: "/compras",
        etiqueta: "Compras realizadas",
    },
    Opcion {
        nombre: "Ventas",
        url: "/venta",
        etiqueta: "Ventas realizadas",
    },
    Opcion {
        nombre: "Medios",
        url: "/medios",
        etiqueta: "Envases en los que se expenden los productos",
    },
    Opcion {
        nombre: "Puestos",
        url: "/puestos",
        etiqueta: "Estructura organizacional de la empresa",
    },
    Opcion {
        nombre: "Empresas",
        url: "/empresas",
        etiqueta: "Empresas que utilizan el sistema",
    },
    Opcion {
        nombre: "Usuarios",
        url: "/usuarios",
        etiqueta: "Usuarios autorizados en el sistema",
    },
    Opcion {
        nombre: "Administración",
        url: "/submenu",
        etiqueta: "Sub menu de administración",
    },
];
pub fn muestra() -> Markup { html! {
    .menu {
        ul .menu { 
        @for opcion in OPCIONES {
            li {
                a href=(opcion.url) 
                    class="tooltip" data-tooltip=(opcion.etiqueta) {
                    (opcion.nombre)
                }
            }
        }
        }
    }
}}
