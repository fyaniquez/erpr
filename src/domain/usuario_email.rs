use validator::validate_email;

#[derive(Debug)]
pub struct UsuarioEmail(String);

impl UsuarioEmail {
    pub fn parse(s: String) -> Result<UsuarioEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for UsuarioEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UsuarioEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

