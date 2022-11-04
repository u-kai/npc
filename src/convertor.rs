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
