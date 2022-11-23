//! src/domain/usuario_apellido.rs
//! author: fyaniquez
//! date: 29/10/2022
//! validaciones de un apellido de usuario

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct UsuarioApellido(String);

impl UsuarioApellido {
    /// Devuelve una instancia de ApellidoUsuario validado
    pub fn parse(s: String) -> Result<UsuarioApellido, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let prohibidos = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let tiene_prohibidos = s.chars().any(
            |g| prohibidos.contains(&g));
        if is_empty_or_whitespace || is_too_long || tiene_prohibidos {
            Err(format!("{} Apellido no valido.", s))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for UsuarioApellido {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

