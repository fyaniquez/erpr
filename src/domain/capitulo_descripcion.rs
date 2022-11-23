//! src/domain/capitulo_descripcion.rs
//! author: fyaniquez
//! date: 29/10/2022
//! validaciones de descripcion de capitulo
//! el largo maximo es de 128 caracteres

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct CapituloDescripcion(String);

impl CapituloDescripcion {
    /// Devuelve una instancia de DescripcionCapitulo validado
    pub fn parse(s: String) -> Result<CapituloDescripcion, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 128;
        let prohibidos = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let tiene_prohibidos = s.chars().any(
            |g| prohibidos.contains(&g));
        if is_empty_or_whitespace || is_too_long || tiene_prohibidos {
            Err(format!("{} Descripcion no valido.", s))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for CapituloDescripcion {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CapituloDescripcion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::CapituloDescripcion;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a̐".repeat(128);
        assert_ok!(CapituloDescripcion::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(129);
        assert_err!(CapituloDescripcion::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(CapituloDescripcion::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(CapituloDescripcion::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(CapituloDescripcion::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Alimentos naturales y procesados".to_string();
        assert_ok!(CapituloDescripcion::parse(name));
    }
}
