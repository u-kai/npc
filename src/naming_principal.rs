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
    NonePrincipal(&'a str),
}

impl<'a> NamingPrincipal<'a> {
    pub fn new(source: &'a str) -> Self {
        //flat containe camel and snake that's why is_flat is position top
        if Self::is_flat(source) {
            return Self::Flat(source);
        }
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
        Self::NonePrincipal(source)
    }
    pub fn is_flat(source: &'a str) -> bool {
        source.chars().all(|c| c.is_lowercase())
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
    const FLATCASE1: &'static str = "flatcase";
    const FLATCASE2: &'static str = "";
    const SNAKE_CASE1: &'static str = "snake_case";
    const SNAKE_CASE2: &'static str = "_snake_case";
    const CAMEL_CASE: &'static str = "camelCase";
    const PASCAL_CASE: &'static str = "PascalCase";
    const CONSTANT_CASE1: &'static str = "CONSTANT_CASE";
    const CONSTANT_CASE2: &'static str = "CONSTANT";
    const CHAIN_CASE1: &'static str = "chain-case";
    const CHAIN_CASE2: &'static str = "-chain-case";

    #[test]
    fn test_is_flat_and_new_flat() {
        assert!(NamingPrincipal::is_flat(FLATCASE1));
        assert!(NamingPrincipal::is_flat(FLATCASE2));
        assert!(!NamingPrincipal::is_flat(SNAKE_CASE1));
        assert!(!NamingPrincipal::is_flat(SNAKE_CASE2));
        assert!(!NamingPrincipal::is_flat(PASCAL_CASE));
        assert!(!NamingPrincipal::is_flat(CAMEL_CASE));
        assert!(!NamingPrincipal::is_flat(CONSTANT_CASE1));
        assert!(!NamingPrincipal::is_flat(CONSTANT_CASE2));
        let np = NamingPrincipal::new(FLATCASE1);
        assert_eq!(np, NamingPrincipal::Flat(FLATCASE1));
        let np = NamingPrincipal::new(FLATCASE2);
        assert_eq!(np, NamingPrincipal::Flat(FLATCASE2));
    }
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
