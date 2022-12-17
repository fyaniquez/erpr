//! src/domain/usuario/email.rs
//! author: fyaniquez
//! date: 29/10/2022
//! validaciones de un email de usuario

use validator::validate_email;

#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} no es un email correcto.", s))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

