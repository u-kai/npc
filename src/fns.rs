use crate::{
    convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal,
    reserved_store::PascalCaseReservedIdentifiers,
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
    PascalCaseReservedIdentifiers::wellknown().replace_for_snake_case(snake)
}
pub fn to_snake_consider_with_words(source: &str, words: &[&str]) -> String {
    let mut reserved_store = PascalCaseReservedIdentifiers::new();
    words.into_iter().for_each(|s| reserved_store.add(*s));
    let snake = to_snake(source);
    reserved_store.replace_for_snake_case(snake)
}
pub fn to_snake_consider_with_wellknown_and_others(source: &str, words: &[&str]) -> String {
    let mut reserved_store = PascalCaseReservedIdentifiers::wellknown();
    words.into_iter().for_each(|s| reserved_store.add(*s));
    let snake = to_snake(source);
    reserved_store.replace_for_snake_case(snake)
}
pub fn to_constant_consider_with_wellknown_word(source: &str) -> String {
    let constant = to_constant(source);
    PascalCaseReservedIdentifiers::wellknown().replace_for_constant_case(constant)
}
pub fn to_constant_with_words(source: &str, words: &[&str]) -> String {
    let mut reserved_store = PascalCaseReservedIdentifiers::new();
    words.into_iter().for_each(|s| reserved_store.add(*s));
    let constant = to_constant(source);
    reserved_store.replace_for_constant_case(constant)
}
pub fn to_constant_consider_with_wellknown_and_others(source: &str, words: &[&str]) -> String {
    let mut reserved_store = PascalCaseReservedIdentifiers::wellknown();
    words.into_iter().for_each(|s| reserved_store.add(*s));
    let constant = to_constant(source);
    reserved_store.replace_for_constant_case(constant)
}
pub fn to_chain_consider_with_wellknown_word(source: &str) -> String {
    let chain = to_chain(source);
    PascalCaseReservedIdentifiers::wellknown().replace_for_chain_case(chain)
}
pub fn to_chain_consider_with_words(source: &str, words: &[&str]) -> String {
    let mut reserved_store = PascalCaseReservedIdentifiers::new();
    words.into_iter().for_each(|s| reserved_store.add(*s));
    let chain = to_chain(source);
    reserved_store.replace_for_chain_case(chain)
}
pub fn to_chain_consider_with_wellknown_word_and_others(source: &str, words: &[&str]) -> String {
    let mut reserved_store = PascalCaseReservedIdentifiers::wellknown();
    words.into_iter().for_each(|s| reserved_store.add(*s));
    let chain = to_chain(source);
    reserved_store.replace_for_chain_case(chain)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::to_pascal;

    #[test]
    fn pascal_caseの名称を予約語を考慮できる() {
        let source = "UKaiUseGitHubEnterpriseGitHub";
        let words = vec!["UKai"];

        let result = to_snake_consider_with_wellknown_and_others(source, &words);
        assert_eq!(result, "ukai_use_github_enterprise_github");
        let result = to_constant_consider_with_wellknown_and_others(source, &words);
        assert_eq!(result, "UKAI_USE_GITHUB_ENTERPRISE_GITHUB");
        let result = to_chain_consider_with_wellknown_word_and_others(source, &words);
        assert_eq!(result, "ukai-use-github-enterprise-github");
    }
    #[test]
    fn pascal_caseの名称を登録して考慮できる() {
        let source = "UKaiUseGitHubEnterpriseGitHub";
        let words = vec!["UKai"];

        let result = to_snake_consider_with_words(source, &words);
        assert_eq!(result, "ukai_use_git_hub_enterprise_git_hub");
        let result = to_constant_with_words(source, &words);
        assert_eq!(result, "UKAI_USE_GIT_HUB_ENTERPRISE_GIT_HUB");
        let result = to_chain_consider_with_words(source, &words);
        assert_eq!(result, "ukai-use-git-hub-enterprise-git-hub");
    }
    #[test]
    fn 登録されたpascal_caseの名称を考慮できる() {
        let source = "UseGitHubEnterpriseGitHub";

        let result = to_snake_consider_with_wellknown_word(source);
        assert_eq!(result, "use_github_enterprise_github");
        let result = to_constant_consider_with_wellknown_word(source);
        assert_eq!(result, "USE_GITHUB_ENTERPRISE_GITHUB");
        let result = to_chain_consider_with_wellknown_word(source);
        assert_eq!(result, "use-github-enterprise-github");
    }
    #[test]
    fn うまくいかなかったケースを調査() {
        let sut = "get_edge_configuration_stack_with_edge_id9012";
        assert_eq!(to_pascal(sut), "GetEdgeConfigurationStackWithEdgeId9012");
    }
}
