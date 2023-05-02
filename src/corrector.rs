use crate::fns::{to_camel, to_constant, to_pascal, to_snake};

/// InvalidCharacterCorrector is a struct to correct invalid characters for programing language.
///
/// For example, if you want to convert "invalid:identifier" to snake_case, you can use this struct.
/// ## Example
/// ```rust
/// #[test]
/// fn test_invalid_character_corrector() {
///     let source = "invalid:identifier";
///     let sut = InvalidCharacterCorrector::new();
///     assert_eq!(sut.to_snake(source), "invalid_identifier");
/// }
/// ```
pub struct InvalidCharacterCorrector {
    invalid_characters: Vec<char>,
}

impl InvalidCharacterCorrector {
    pub fn new() -> Self {
        Self {
            invalid_characters: Vec::new(),
        }
    }
    pub fn to_snake(&self, source: &str) -> String {
        to_snake(&self.replace(source, "_"))
    }
    pub fn to_camel(&self, source: &str) -> String {
        to_camel(&self.to_snake(source))
    }
    pub fn to_pascal(&self, source: &str) -> String {
        to_pascal(&self.to_snake(source))
    }
    pub fn to_constant(&self, source: &str) -> String {
        to_constant(&self.to_snake(source))
    }
    fn replace(&self, source: &str, target: &str) -> String {
        source.replace(|c| self.invalid_characters.contains(&c), target)
    }
}
impl Default for InvalidCharacterCorrector {
    fn default() -> Self {
        Self {
            invalid_characters: vec![
                ':', ' ', '-', '/', '\\', '.', ',', ';', '\'', '"', '[', ']', '{', '}', '(', ')',
                '<', '>', '?', '!', '@', '#', '$', '%', '^', '&', '*', '+', '=', '|', '~', '`',
            ],
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn プログラム言語にとって不正な文字を修正して任意の命名規則にする() {
        let source = "invalid:identifier";
        let sut = InvalidCharacterCorrector::default();
        assert_eq!(sut.to_snake(source), "invalid_identifier");
        assert_eq!(sut.to_camel(source), "invalidIdentifier");
        assert_eq!(sut.to_pascal(source), "InvalidIdentifier");
        assert_eq!(sut.to_constant(source), "INVALID_IDENTIFIER");
    }
}
