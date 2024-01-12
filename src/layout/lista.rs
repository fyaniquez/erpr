//! src/layouts/lista_paginada.rs
//! estructuras para paginar una lista
//! layout base para las páginas con listas paginadas del sistema
//! autor: fyaniquez
//! fecha: 17-05-2022

use crate::layout;
use actix_web::Result as AwResult;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};

const PAGS_X_HOJA: i32 = 10;

// parametros de Paginado
#[derive(Deserialize)]
#[serde(default)]
pub struct Paginado {
    pub pagina: String,
    pub longitud: String,
    #[serde(default)]
    pub orden: String,
    #[serde(default)]
    pub filtro: String,
    pub total_filas: Option<i32>,
}

impl Default for Paginado {
    fn default() -> Self {
        Self {
            pagina: String::from("1"),
            longitud: String::from("10"),
            orden: String::new(),
            filtro: String::new(),
            total_filas: None,
        }
    }
}

impl std::fmt::Debug for Paginado {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "orden:{}; filtro:{}; pagina:{}; longitud:{}\n",
            self.orden, self.filtro, self.pagina, self.longitud
        )?;
        Ok(())
    }
}

impl Paginado {
    // pagina en numerico
    pub fn get_pagina(&self) -> i32 {
        self.pagina.parse().unwrap()
    }

    // pagina en numerico
    pub fn get_longitud(&self) -> i32 {
        if self.longitud == "todos" {
            0
        } else {
            self.longitud.parse().unwrap()
        }
    }

    /// calcula el numero de paginas
    pub fn get_nro_paginas(&self, nro_filas: i32) -> i32 {
        let regs_x_pag = &self.get_longitud();
        if nro_filas % regs_x_pag > 0 {
            nro_filas / regs_x_pag + 1
        } else {
            nro_filas / regs_x_pag
        }
    }

    // calcula el numero de registros
    pub fn get_nro_registros(&self) -> i32 {
        (self.get_pagina() - 1) * self.get_longitud()
    }

    // calcula el grupo de paginas
    pub fn get_tomo(&self) -> i32 {
        (&self.get_pagina() - 1) / PAGS_X_HOJA
    }

    // calcula la primera pagina de un tomo
    pub fn get_inferior(&self) -> i32 {
        self.get_tomo() * PAGS_X_HOJA + 1
    }

    // calcula la ultima página de un tomo
    pub fn get_superior(&self, nro_filas: i32) -> i32 {
        let superior = self.get_inferior() + (PAGS_X_HOJA - 1);
        if superior > self.get_nro_paginas(nro_filas) {
            self.get_nro_paginas(nro_filas)
        } else {
            superior
        }
    }

    // complementa el query con los parametros
    pub fn get_qry(&self, qry: &str) -> String {
        let filtro;
        if self.filtro != "" {
            if qry.contains("WHERE") {
                filtro = format!(
                    " AND {} ILIKE '%{}%'", self.orden, self.filtro);
            } else {
                filtro = format!(
                    " WHERE {} ILIKE '%{}%'", self.orden, self.filtro);
            }
        } else {
            filtro = "".to_string();
        }
        format!(
            "{} {} ORDER BY {} LIMIT {} OFFSET {}",
            qry,
            filtro,
            self.orden,
            self.get_longitud(),
            self.get_offset()
        )
    }

    // obtiene el registro a partir del que se obtiene el listado (offset)
    pub fn get_offset(&self) -> i32 {
        (self.get_pagina() - 1) * self.get_longitud()
    }

    // genera un query para count
    pub fn get_qry_count(&self, qry: &str) -> String {
        // reemplaza la lista de campos por count(campo_1)
        let i = qry.find("FROM").unwrap();
        let v: Vec<&str> = qry[7..i].splitn(2, ',').collect();
        let f = qry[7..i].to_string();
        let mut columna = v[0].to_lowercase();
        if let Some(index) = columna.find(" as ") {
            columna.truncate(index);
        }
        let c = format!("COUNT({}) ", columna);
        let query = qry.replace(&f, &c);

        // agrega el filtro
        let filtro;
        if self.filtro != "" {
            if qry.contains("WHERE") {
                filtro = format!(
                    " AND {} ILIKE '%{}%'", self.orden, self.filtro);
            } else {
                filtro = format!(
                    " WHERE {} ILIKE '%{}%'", self.orden, self.filtro);
            }
        } else {
            filtro = "".to_string();
        }
        format!("{} {}", query, filtro)
    }
}

// Paginado para interfaz
#[derive(Serialize)]
pub struct PaginadoInt<T> {
    pub filas: Vec<T>,
    pub nro_filas: i32,
}

// barra con los controles para avanzar por páginas
fn barra_paginado(contenido: bool, paginado: &Paginado) -> Markup {
    html! {
        .paginado-barra #pagebar {
            #paginas .paginas {
            @if contenido {
                span .pagina #primero { "<<" }
                span .pagina #previo { "<" }
                @for p in paginado.get_inferior()..=
                    paginado.get_superior(paginado.total_filas.unwrap()) {
                    @if p == paginado.get_pagina() {
                        span .pagina .active #actual {(p)}
                    } @else {
                        span .pagina #pagina {(p)}
                    }
                }
                span .pagina #siguiente { ">" }
                span .pagina #ultimo
                    data-index=(
                        paginado.get_nro_paginas(
                            paginado.total_filas.unwrap())) { ">> " }
           }}
           button .pagina-boton #crea type="button" value="agregar" { "+" }
        }
    }
}

// crea un marco funcional para el Paginado de listas
pub fn crea(
    titulo: &str,
    atras: &str,
    estilo: &str,
    script: Option<&str>,
    paginado: &Paginado,
    contenido: Option<Markup>,
) -> AwResult<Markup> {
    let formulario = combina(titulo, atras, contenido, paginado);
    layout::principal::crea(titulo, estilo, script, formulario)
}

// combina el contenido construido por el cliente
// con el layout para listas
fn combina(
    titulo: &str,
    atras: &str,
    contenido: Option<Markup>, 
    paginado: &Paginado
) -> Markup { html! {
    .cabecera {
        img .cabecera-logo src="/img/logo.png";
        .cabecera-nav {
            a href="/login_email_form" { "Ingresar" } }
        .cabecera-nav {
            a href="/" { "Registrarse" } }
    }
    (layout::menu::muestra())
    .principal {
        .lista-box {
            .lista-cabecera {
                span {(titulo)}
                input #filtro value=(paginado.filtro);
                button .pagina-buscar #buscar {};
                a #atras href=(atras) { 
                    img src="/img/lista-24.png"; 
                }
            }
            @match contenido {
                Some(contenido) => {
                    (contenido)
                    (barra_paginado(true, paginado))
                },
                None => {
                    (barra_paginado(false, paginado))
                },
            }
        }
    }
}}
