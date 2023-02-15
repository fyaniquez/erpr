//! src/domain/compra/estado.rs
//! author: fyaniquez
//! date: 06/12/2022
//! estados de una compra

#[derive(Debug, PartialEq)]
enum EstadoEnum {
    Anulado,
    Pagado,
    Pendiente,
}

#[derive(Debug)]
pub struct Estado(EstadoEnum);

impl Estado {
    pub fn parse(input: String) -> Result<Estado, String> {
        match input.as_str() {
            "Anulado" => Ok(Self(EstadoEnum::Anulado)),
            "Pagado" => Ok(Self(EstadoEnum::Pagado)),
            "Pendiente" => Ok(Self(EstadoEnum::Pendiente)),
            _ => Err(format!("{} Estado no válido.", input))
        } 
    }
}
//impl AsRef<str> for Estado {
    //fn as_ref(&self) -> &str {
        //&self.0
    //}
//}

impl std::fmt::Display for Estado {
    fn fmt(&self, f: &mut std::fmt::Formatter) 
    -> std::fmt::Result {
        match self.0 {
            EstadoEnum::Anulado => write!(f, "{}", "Anulado"),
            EstadoEnum::Pagado => write!(f, "{}", "Pagado"),
            EstadoEnum::Pendiente => write!(f, "{}", "Pendiente"),
        }
    }
}

