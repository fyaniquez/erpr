//! src/domain/puesto/descripcion.rs
//! author: fyaniquez
//! date: 06/12/2022
//! validaciones de un descripcion de puesto
//! la longitud máxima es de 32 caracteres

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Descripcion(String);

impl Descripcion {
    /// Devuelve una instancia de DescripcionPuesto validado
    pub fn parse(s: String) -> Result<Descripcion, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 32;
        let prohibidos = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let tiene_prohibidos = s.chars().any(|g| prohibidos.contains(&g));
        if is_empty_or_whitespace || is_too_long || tiene_prohibidos {
            Err(format!("{} Descripcion no valido.", s))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for Descripcion {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Descripcion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::puesto::Descripcion;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a̐".repeat(32);
        assert_ok!(Descripcion::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(33);
        assert_err!(Descripcion::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(Descripcion::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(Descripcion::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(Descripcion::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Abarrotes".to_string();
        assert_ok!(Descripcion::parse(name));
    }
}
