use crate::fns::to_snake;

pub struct InvalidCharacterCorrector {
    invalid_characters: Vec<char>,
}

impl InvalidCharacterCorrector {
    pub fn new() -> Self {
        Self {
            invalid_characters: vec![':', '-', ' '],
        }
    }
    pub fn to_snake(&self, source: &str) -> String {
        to_snake(&self.replace(source, "_"))
    }
    fn replace(&self, source: &str, target: &str) -> String {
        source.replace(|c| self.invalid_characters.contains(&c), target)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn プログラム言語にとって不正な文字を修正して任意の命名規則にする() {
        let source = "invalid:identifier";
        let sut = InvalidCharacterCorrector::new();
        assert_eq!(sut.to_snake(source), "invalid_identifier");
    }
}
