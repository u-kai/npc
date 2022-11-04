#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum NamingPrincipal<'a> {
    Snake(&'a str),
    Constant(&'a str),
    Camel(&'a str),
    Pascal(&'a str),
    Chain(&'a str),
    Empty(&'a str),
    Flat(&'a str),
    NonPrincipal(&'a str),
}

impl<'a> NamingPrincipal<'a> {
    pub fn new(source: &'a str) -> Self {
        //flat containe camel and snake and chain that's why is_flat is position top
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
        if Self::is_empty(source) {
            return Self::Empty(source);
        }
        if Self::is_non_principal(source) {
            return Self::NonPrincipal(source);
        }
        panic!("not considering case! have to impl case {}", source)
    }
    pub fn is_non_principal(source: &'a str) -> bool {
        !(Self::is_flat(source)
            || Self::is_pascal(source)
            || Self::is_camel(source)
            || Self::is_chain(source)
            || Self::is_constant(source)
            || Self::is_empty(source)
            || Self::is_snake(source))
    }
    pub fn is_flat(source: &'a str) -> bool {
        !Self::is_empty(source) && source.chars().all(|c| c.is_lowercase())
    }
    pub fn is_chain(source: &'a str) -> bool {
        !Self::is_empty(source)
            && source
                .chars()
                .all(|c| c == '-' || c != '_' && c.is_lowercase())
    }
    pub fn is_constant(source: &'a str) -> bool {
        !Self::is_empty(source)
            && source
                .chars()
                .all(|c| c == '_' || c != '-' && c.is_uppercase())
    }
    pub fn is_snake(source: &'a str) -> bool {
        !Self::is_empty(source)
            && source
                .chars()
                .all(|c| c == '_' || c != '-' && c.is_lowercase())
    }
    pub fn is_pascal(source: &'a str) -> bool {
        if Self::is_empty(source) || source.contains("_") || source.contains("-") {
            return false;
        }
        let first = source.chars().next().unwrap();
        if !first.is_uppercase() {
            return false;
        }
        !source.chars().all(|c| c.is_uppercase())
    }
    pub fn is_camel(source: &'a str) -> bool {
        if Self::is_empty(source) || source.contains("_") || source.contains("-") {
            return false;
        }
        if let Some(first) = source.chars().next() {
            first.is_lowercase()
        } else {
            false
        }
    }
    pub fn is_empty(source: &'a str) -> bool {
        source.len() == 0
    }
}

#[cfg(test)]
pub(super) mod naming_principal_test_data {
    pub const FLATCASE: &'static str = "flatcase";
    pub const EMPTYCASE: &'static str = "";
    pub const SNAKE_CASE1: &'static str = "snake_case";
    pub const SNAKE_CASE2: &'static str = "_snake_case";
    pub const CAMEL_CASE: &'static str = "camelCase";
    pub const PASCAL_CASE1: &'static str = "PascalCase";
    pub const PASCAL_CASE2: &'static str = "ABCData";
    pub const CONSTANT_CASE1: &'static str = "CONSTANT_CASE";
    pub const CONSTANT_CASE2: &'static str = "CONSTANT";
    pub const CONSTANT_CASE3: &'static str = "_CONSTANT_CASE";
    pub const CHAIN_CASE1: &'static str = "chain-case";
    pub const CHAIN_CASE2: &'static str = "-chain-case";
    pub const NONPRINCIPAL_CASE1: &'static str = "A_data";
    pub const NONPRINCIPAL_CASE2: &'static str = "ABC-Data_";
}
#[cfg(test)]
mod test_naming_principal {
    use super::*;
    use naming_principal_test_data::*;
    #[test]
    fn test_is_nonprincipal_and_new_nonprincipal() {
        assert!(NamingPrincipal::is_non_principal(NONPRINCIPAL_CASE1));
        assert!(NamingPrincipal::is_non_principal(NONPRINCIPAL_CASE2));
        assert!(!NamingPrincipal::is_non_principal(FLATCASE));
        assert!(!NamingPrincipal::is_non_principal(EMPTYCASE));
        assert!(!NamingPrincipal::is_non_principal(CHAIN_CASE1));
        assert!(!NamingPrincipal::is_non_principal(CHAIN_CASE2));
        assert!(!NamingPrincipal::is_non_principal(SNAKE_CASE1));
        assert!(!NamingPrincipal::is_non_principal(SNAKE_CASE2));
        assert!(!NamingPrincipal::is_non_principal(PASCAL_CASE1));
        assert!(!NamingPrincipal::is_non_principal(PASCAL_CASE2));
        assert!(!NamingPrincipal::is_non_principal(CAMEL_CASE));
        assert!(!NamingPrincipal::is_non_principal(CONSTANT_CASE1));
        assert!(!NamingPrincipal::is_non_principal(CONSTANT_CASE2));
        let np = NamingPrincipal::new(NONPRINCIPAL_CASE1);
        assert_eq!(np, NamingPrincipal::NonPrincipal(NONPRINCIPAL_CASE1));
        let np = NamingPrincipal::new(NONPRINCIPAL_CASE2);
        assert_eq!(np, NamingPrincipal::NonPrincipal(NONPRINCIPAL_CASE2));
    }
    #[test]
    fn test_is_flat_and_new_flat() {
        assert!(NamingPrincipal::is_flat(FLATCASE));
        assert!(!NamingPrincipal::is_flat(EMPTYCASE));
        assert!(!NamingPrincipal::is_flat(CHAIN_CASE1));
        assert!(!NamingPrincipal::is_flat(CHAIN_CASE2));
        assert!(!NamingPrincipal::is_flat(SNAKE_CASE1));
        assert!(!NamingPrincipal::is_flat(SNAKE_CASE2));
        assert!(!NamingPrincipal::is_flat(PASCAL_CASE1));
        assert!(!NamingPrincipal::is_flat(PASCAL_CASE2));
        assert!(!NamingPrincipal::is_flat(CAMEL_CASE));
        assert!(!NamingPrincipal::is_flat(CONSTANT_CASE1));
        assert!(!NamingPrincipal::is_flat(CONSTANT_CASE2));
        assert!(!NamingPrincipal::is_flat(NONPRINCIPAL_CASE1));
        assert!(!NamingPrincipal::is_flat(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(FLATCASE);
        assert_eq!(np, NamingPrincipal::Flat(FLATCASE));
    }
    #[test]
    fn test_is_chain_and_new_chain() {
        assert!(NamingPrincipal::is_chain(CHAIN_CASE1));
        assert!(NamingPrincipal::is_chain(CHAIN_CASE2));
        assert!(NamingPrincipal::is_chain(FLATCASE));
        assert!(!NamingPrincipal::is_chain(EMPTYCASE));
        assert!(!NamingPrincipal::is_chain(SNAKE_CASE1));
        assert!(!NamingPrincipal::is_chain(SNAKE_CASE2));
        assert!(!NamingPrincipal::is_chain(PASCAL_CASE1));
        assert!(!NamingPrincipal::is_chain(PASCAL_CASE2));
        assert!(!NamingPrincipal::is_chain(CAMEL_CASE));
        assert!(!NamingPrincipal::is_chain(CONSTANT_CASE1));
        assert!(!NamingPrincipal::is_chain(CONSTANT_CASE2));
        assert!(!NamingPrincipal::is_chain(NONPRINCIPAL_CASE1));
        assert!(!NamingPrincipal::is_chain(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(CHAIN_CASE1);
        assert_eq!(np, NamingPrincipal::Chain(CHAIN_CASE1));
        let np = NamingPrincipal::new(CHAIN_CASE2);
        assert_eq!(np, NamingPrincipal::Chain(CHAIN_CASE2));
    }
    #[test]
    fn test_is_constant_and_new_constant() {
        assert!(NamingPrincipal::is_constant(CONSTANT_CASE1));
        assert!(NamingPrincipal::is_constant(CONSTANT_CASE2));
        assert!(NamingPrincipal::is_constant(CONSTANT_CASE3));
        assert!(!NamingPrincipal::is_constant(CHAIN_CASE1));
        assert!(!NamingPrincipal::is_constant(CHAIN_CASE2));
        assert!(!NamingPrincipal::is_constant(FLATCASE));
        assert!(!NamingPrincipal::is_constant(EMPTYCASE));
        assert!(!NamingPrincipal::is_constant(SNAKE_CASE1));
        assert!(!NamingPrincipal::is_constant(SNAKE_CASE2));
        assert!(!NamingPrincipal::is_constant(PASCAL_CASE1));
        assert!(!NamingPrincipal::is_constant(PASCAL_CASE2));
        assert!(!NamingPrincipal::is_constant(CAMEL_CASE));
        assert!(!NamingPrincipal::is_constant(NONPRINCIPAL_CASE1));
        assert!(!NamingPrincipal::is_constant(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(CONSTANT_CASE1);
        assert_eq!(np, NamingPrincipal::Constant(CONSTANT_CASE1));
        let np = NamingPrincipal::new(CONSTANT_CASE2);
        assert_eq!(np, NamingPrincipal::Constant(CONSTANT_CASE2));
        let np = NamingPrincipal::new(CONSTANT_CASE3);
        assert_eq!(np, NamingPrincipal::Constant(CONSTANT_CASE3));
    }
    #[test]
    fn test_is_snake_and_new_snake() {
        assert!(NamingPrincipal::is_snake(SNAKE_CASE1));
        assert!(NamingPrincipal::is_snake(SNAKE_CASE2));
        assert!(NamingPrincipal::is_snake(FLATCASE));
        assert!(!NamingPrincipal::is_snake(CHAIN_CASE1));
        assert!(!NamingPrincipal::is_snake(CHAIN_CASE2));
        assert!(!NamingPrincipal::is_snake(EMPTYCASE));
        assert!(!NamingPrincipal::is_snake(PASCAL_CASE1));
        assert!(!NamingPrincipal::is_snake(PASCAL_CASE2));
        assert!(!NamingPrincipal::is_snake(CAMEL_CASE));
        assert!(!NamingPrincipal::is_snake(CONSTANT_CASE1));
        assert!(!NamingPrincipal::is_snake(CONSTANT_CASE2));
        assert!(!NamingPrincipal::is_snake(NONPRINCIPAL_CASE1));
        assert!(!NamingPrincipal::is_snake(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(SNAKE_CASE1);
        assert_eq!(np, NamingPrincipal::Snake(SNAKE_CASE1));
        let np = NamingPrincipal::new(SNAKE_CASE2);
        assert_eq!(np, NamingPrincipal::Snake(SNAKE_CASE2));
    }
    #[test]
    fn test_is_pascal_and_new_pascal() {
        assert!(NamingPrincipal::is_pascal(PASCAL_CASE1));
        assert!(NamingPrincipal::is_pascal(PASCAL_CASE2));
        assert!(!NamingPrincipal::is_pascal(FLATCASE));
        assert!(!NamingPrincipal::is_pascal(CHAIN_CASE1));
        assert!(!NamingPrincipal::is_pascal(CHAIN_CASE2));
        assert!(!NamingPrincipal::is_pascal(EMPTYCASE));
        assert!(!NamingPrincipal::is_pascal(SNAKE_CASE1));
        assert!(!NamingPrincipal::is_pascal(SNAKE_CASE2));
        assert!(!NamingPrincipal::is_pascal(CAMEL_CASE));
        assert!(!NamingPrincipal::is_pascal(CONSTANT_CASE1));
        assert!(!NamingPrincipal::is_pascal(CONSTANT_CASE2));
        assert!(!NamingPrincipal::is_pascal(NONPRINCIPAL_CASE1));
        assert!(!NamingPrincipal::is_pascal(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(PASCAL_CASE1);
        assert_eq!(np, NamingPrincipal::Pascal(PASCAL_CASE1));
        let np = NamingPrincipal::new(PASCAL_CASE2);
        assert_eq!(np, NamingPrincipal::Pascal(PASCAL_CASE2));
    }
    #[test]
    fn test_is_camel_and_new_camel() {
        assert!(NamingPrincipal::is_camel(CAMEL_CASE));
        assert!(NamingPrincipal::is_camel(FLATCASE));
        assert!(!NamingPrincipal::is_camel(PASCAL_CASE1));
        assert!(!NamingPrincipal::is_camel(PASCAL_CASE2));
        assert!(!NamingPrincipal::is_camel(CHAIN_CASE1));
        assert!(!NamingPrincipal::is_camel(CHAIN_CASE2));
        assert!(!NamingPrincipal::is_camel(EMPTYCASE));
        assert!(!NamingPrincipal::is_camel(SNAKE_CASE1));
        assert!(!NamingPrincipal::is_camel(SNAKE_CASE2));
        assert!(!NamingPrincipal::is_camel(CONSTANT_CASE1));
        assert!(!NamingPrincipal::is_camel(CONSTANT_CASE2));
        assert!(!NamingPrincipal::is_camel(NONPRINCIPAL_CASE1));
        assert!(!NamingPrincipal::is_camel(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(CAMEL_CASE);
        assert_eq!(np, NamingPrincipal::Camel(CAMEL_CASE));
    }
}
