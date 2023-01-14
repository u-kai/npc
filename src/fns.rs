use crate::{convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal};

pub fn to_camel(source: &str) -> String {
    NamingPrincipalConvertor::new(source).to_camel()
}
pub fn to_pascal(source: &str) -> String {
    NamingPrincipalConvertor::new(source).to_pascal()
}
pub fn to_snake(source: &str) -> String {
    NamingPrincipalConvertor::new(source).to_snake()
}
pub fn to_constant(source: &str) -> String {
    NamingPrincipalConvertor::new(source).to_constant()
}
pub fn to_chain(source: &str) -> String {
    NamingPrincipalConvertor::new(source).to_chain()
}
pub fn is_flat(source: &str) -> bool {
    NamingPrincipal::is_flat(source)
}
pub fn is_camel(source: &str) -> bool {
    NamingPrincipal::is_camel(source)
}
pub fn is_pascal(source: &str) -> bool {
    NamingPrincipal::is_pascal(source)
}
pub fn is_snake(source: &str) -> bool {
    NamingPrincipal::is_snake(source)
}
pub fn is_constant(source: &str) -> bool {
    NamingPrincipal::is_constant(source)
}
pub fn is_chain(source: &str) -> bool {
    NamingPrincipal::is_chain(source)
}
pub fn is_non_principal(source: &str) -> bool {
    NamingPrincipal::is_non_principal(source)
}
