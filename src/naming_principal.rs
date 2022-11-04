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
    ConstantCase(&'a str),
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
        Self::Flat(source)
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
        if source.contains("_") {
            return false;
        }
        let first = source.chars().next().unwrap();
        if first.is_uppercase() {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod test_naming_principal {
    use super::*;
    #[test]
    fn test_is_pascal_and_new_pascal() {
        let source = "PascalCase";
        assert!(NamingPrincipal::is_pascal(source));
        let np = NamingPrincipal::new(source);
        assert_eq!(np, NamingPrincipal::Pascal(source));
        let source = "snake_case";
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
