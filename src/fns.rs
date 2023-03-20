use crate::{
    convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal,
    reserved_store::ReservedPascalCaseIdentifies,
};

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

pub fn to_snake_consider_with_wellknown_word(source: &str) -> String {
    let snake = to_snake(source);
    ReservedPascalCaseIdentifies::wellknown().replace_for_snake_case(&snake)
}

#[cfg(test)]
mod tests {
    use crate::fns::to_snake_consider_with_wellknown_word;

    #[test]
    fn 登録されたpascal_caseの名称を考慮できる() {
        let source = "UseGitHubEnterpriseGitHub";

        let result = to_snake_consider_with_wellknown_word(source);

        assert_eq!(result, "use_github_enterprise_github");
    }
}
