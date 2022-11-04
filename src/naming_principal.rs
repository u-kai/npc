#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NamingPrincipalConvertor<'a> {
    original: &'a str,
    value: NamingPrincipal<'a>,
}

impl<'a> NamingPrincipalConvertor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            original: source,
            value: NamingPrincipal::new(source),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum NamingPrincipal<'a> {
    Snake(&'a str),
    Constant(&'a str),
    Camel(&'a str),
    Pascal(&'a str),
    Chain(&'a str),
    Flat(&'a str),
}

impl<'a> NamingPrincipal<'a> {
    pub fn new(source: &'a str) -> Self {
        if Self::is_camel(source) {
            return Self::Camel(source);
        }
        if Self::is_pascal(source) {
            return Self::Pascal(source);
        }
        if Self::is_snake(source) {
            return Self::Snake(source);
        }
        if Self::is_constant(source) {
            return Self::Constant(source);
        }
        if Self::is_chain(source) {
            return Self::Chain(source);
        }
        Self::Flat(source)
    }
    pub fn is_chain(source: &'a str) -> bool {
        source
            .chars()
            .all(|c| c == '-' || c != '_' && c.is_lowercase())
    }
    pub fn is_constant(source: &'a str) -> bool {
        source
            .chars()
            .all(|c| c == '_' || c != '-' && c.is_uppercase())
    }
    pub fn is_snake(source: &'a str) -> bool {
        source
            .chars()
            .all(|c| c == '_' || c != '-' && c.is_lowercase())
    }
    pub fn is_pascal(source: &'a str) -> bool {
        if source.contains("_") {
            return false;
        }
        let first = source.chars().next().unwrap();
        if !first.is_uppercase() {
            return false;
        }
        !source.chars().all(|c| c.is_uppercase())
    }
    pub fn is_camel(source: &'a str) -> bool {
        if source.contains("_") || source.contains("-") {
            return false;
        }
        if let Some(first) = source.chars().next() {
            first.is_lowercase()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test_naming_principal {
    use super::*;
    #[test]
    fn test_is_chain_and_new_chain() {
        let source = "chain-case";
        assert!(NamingPrincipal::is_chain(source));
        let np = NamingPrincipal::new(source);
        assert_eq!(np, NamingPrincipal::Chain(source));
        let source = "snake_case";
        assert!(!NamingPrincipal::is_chain(source));
    }
    #[test]
    fn test_is_constant_and_new_constant() {
        let source = "CONSTANT_CASE";
        assert!(NamingPrincipal::is_constant(source));
        let np = NamingPrincipal::new(source);
        assert_eq!(np, NamingPrincipal::Constant(source));
        let source = "AWS";
        assert!(NamingPrincipal::is_constant(source));
        let np = NamingPrincipal::new(source);
        assert_eq!(np, NamingPrincipal::Constant(source));
        let source = "PascalCase";
        assert!(!NamingPrincipal::is_constant(source));
        let source = "snake_case";
        assert!(!NamingPrincipal::is_constant(source));
        let source = "chain-case";
        assert!(!NamingPrincipal::is_constant(source));
    }
    #[test]
    fn test_is_snake_and_new_snake() {
        let source = "snake_case";
        assert!(NamingPrincipal::is_snake(source));
        let np = NamingPrincipal::new(source);
        assert_eq!(np, NamingPrincipal::Snake(source));
        let source = "PascalCase";
        assert!(!NamingPrincipal::is_snake(source));
        let source = "CONSTANT_CASE";
        assert!(!NamingPrincipal::is_snake(source));
        let source = "chain-case";
        assert!(!NamingPrincipal::is_snake(source));
    }
    #[test]
    fn test_is_pascal_and_new_pascal() {
        let source = "PascalCase";
        assert!(NamingPrincipal::is_pascal(source));
        let np = NamingPrincipal::new(source);
        assert_eq!(np, NamingPrincipal::Pascal(source));
        let source = "snake_case";
        assert!(!NamingPrincipal::is_pascal(source));
        let source = "CONSTANT_CASE";
        assert!(!NamingPrincipal::is_pascal(source));
    }
    #[test]
    fn test_is_camel_and_new_camel() {
        let source = "camelCase";
        assert!(NamingPrincipal::is_camel(source));
        let np = NamingPrincipal::new(source);
        assert_eq!(np, NamingPrincipal::Camel(source));
        let source = "snake_case";
        assert!(!NamingPrincipal::is_camel(source));
    }
}
