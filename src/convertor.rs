use crate::naming_principal::NamingPrincipal;

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

#[cfg(test)]
mod test_convertor {
    use super::*;
    use crate::naming_principal::naming_principal_test_data;

    #[test]
    fn test_to_snake() {}
}
