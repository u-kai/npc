use crate::{to_camel, to_chain, to_constant, to_pascal, to_snake, PostConvert, Principal};

#[derive(Debug, Clone)]
struct PascalCaseReservedIdentifiers {
    inner: Vec<String>,
}

impl PascalCaseReservedIdentifiers {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }
    fn wellknown() -> Self {
        let mut this = Self::new();
        vec!["GitHub", "TypeScript", "JavaScript", "DeepL", "ChatGPT"]
            .iter()
            .for_each(|s| this.add(*s));
        this
    }
    fn add(&mut self, target: impl Into<String>) {
        let target = target.into();
        self.inner.push(target);
    }
}

pub struct PascalCaseReservedIdentifiersConverter {
    identifiers: PascalCaseReservedIdentifiers,
    invalids_pascal: Vec<String>,
    invalids_camel: Vec<String>,
}

impl PostConvert for PascalCaseReservedIdentifiersConverter {
    fn convert(&self, source: &str, principal: Principal) -> String {
        self.to(source, principal)
    }
}

impl PascalCaseReservedIdentifiersConverter {
    pub fn to_convertor(self) -> Box<Self> {
        Box::new(self)
    }
    pub fn wellknown() -> Self {
        Self::new(PascalCaseReservedIdentifiers::wellknown())
    }
    fn new(identifiers: PascalCaseReservedIdentifiers) -> Self {
        let invalids_pascal = identifiers
            .inner
            .iter()
            .map(|s| Self::reserved_to_invalid_pascal(s))
            .collect();
        let invalids_camel = identifiers
            .inner
            .iter()
            .map(|s| Self::reserved_to_invalid_camel(s))
            .collect();
        Self {
            identifiers,
            invalids_pascal,
            invalids_camel,
        }
    }
    pub fn add(mut self, target: impl Into<String>) -> Self {
        let target = target.into();
        self.identifiers.add(target.clone());
        self.invalids_pascal
            .push(Self::reserved_to_invalid_pascal(&target));
        self.invalids_camel
            .push(Self::reserved_to_invalid_camel(&target));

        self
    }
    fn reserved_to_invalid_pascal(source: &str) -> String {
        to_pascal(&source.to_lowercase())
    }
    fn reserved_to_invalid_camel(source: &str) -> String {
        to_camel(&source)
    }
    pub fn to(&self, source: &str, principal: Principal) -> String {
        match principal {
            Principal::Snake => self.fix_for_snake_case(source),
            Principal::Chain => self.fix_for_chain_case(source),
            Principal::Constant => self.fix_for_constant_case(source),
            Principal::Pascal => self.fix_for_pascal_case(source),
            Principal::Camel => self.fix_for_camel_case(source),
        }
    }
    fn fix_for_snake_case(&self, sentence: impl Into<String>) -> String {
        let mut result = sentence.into();
        for target in self.identifiers.inner.iter() {
            let chain_case = to_snake(target);
            if result.contains(&chain_case) {
                result = result.replace(&chain_case, target.to_lowercase().as_str());
            }
        }
        result
    }
    fn fix_for_chain_case(&self, sentence: impl Into<String>) -> String {
        let mut result = sentence.into();
        for target in self.identifiers.inner.iter() {
            let chain_case = to_chain(target);
            if result.contains(&chain_case) {
                result = result.replace(&chain_case, target.to_lowercase().as_str());
            }
        }
        result
    }

    fn fix_for_constant_case(&self, sentence: impl Into<String>) -> String {
        let mut result = sentence.into();
        for target in self.identifiers.inner.iter() {
            let constant_case = to_constant(target);
            if result.contains(&constant_case) {
                result = result.replace(&constant_case, target.to_uppercase().as_str());
            }
        }
        result
    }
    fn fix_for_camel_case(&self, sentence: impl Into<String>) -> String {
        let mut result = to_camel(sentence.into().as_str());
        for target in self.invalids_camel.iter() {
            if result.contains(target) {
                result = result.replace(target, target.to_lowercase().as_str());
            }
        }
        result
    }

    fn fix_for_pascal_case(&self, sentence: impl Into<String>) -> String {
        let mut result = to_pascal(sentence.into().as_str());
        for (i, target) in self.invalids_pascal.iter().enumerate() {
            if result.contains(target) {
                result = result.replace(target, self.identifiers.inner[i].as_str());
            }
        }
        result
    }
}

pub struct IgnoreWordsConverter {
    inner: Vec<String>,
}
impl IgnoreWordsConverter {
    pub fn new(inner: &[&str]) -> Self {
        Self {
            inner: inner.iter().map(|s| s.to_string()).collect(),
        }
    }
    pub fn to_convertor(self) -> Box<Self> {
        Box::new(self)
    }
    fn fix_for<F: Fn(&str) -> String>(&self, converted: &str, convert_fn: F) -> String {
        let mut result = converted.to_string();
        for ignore_word in self.inner.iter() {
            let converted_ignore_word = convert_fn(ignore_word);
            if result.contains(&converted_ignore_word) {
                result = result.replace(&converted_ignore_word, ignore_word);
            }
        }
        result
    }
}

impl PostConvert for IgnoreWordsConverter {
    fn convert(&self, source: &str, principal: Principal) -> String {
        match principal {
            Principal::Snake => self.fix_for(source, to_snake),
            Principal::Chain => self.fix_for(source, to_chain),
            Principal::Constant => self.fix_for(source, to_constant),
            Principal::Pascal => self.fix_for(source, to_pascal),
            Principal::Camel => self.fix_for(source, to_camel),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{convert, Parameter, Principal};

    #[test]
    fn consider_reserved_words_in_pascal_case_names() {
        let source = "UKaiUseGitHubEnterpriseGitHub";

        let params = Parameter::new(source, Principal::Snake).add_post_convert(
            PascalCaseReservedIdentifiersConverter::wellknown()
                .add("UKai")
                .to_convertor(),
        );
        assert_eq!(convert(&params), "ukai_use_github_enterprise_github");

        let params = params.change_principal(Principal::Constant);
        assert_eq!(convert(&params), "UKAI_USE_GITHUB_ENTERPRISE_GITHUB");

        let params = params.change_principal(Principal::Chain);
        assert_eq!(convert(&params), "ukai-use-github-enterprise-github");

        let params = params.change_principal(Principal::Camel);
        assert_eq!(convert(&params), "ukaiUseGitHubEnterpriseGitHub");

        let source = "ukai-use-github-enterprise-github";
        let params = Parameter::new(source, Principal::Pascal).add_post_convert(
            PascalCaseReservedIdentifiersConverter::wellknown()
                .add("UKai")
                .to_convertor(),
        );

        assert_eq!(convert(&params), "UKaiUseGitHubEnterpriseGitHub");
    }
    #[test]
    fn consider_ignore_words() {
        let source = "HelloWorld! GoodBye";
        let ignores = IgnoreWordsConverter::new(&["HelloWorld"]);
        let params =
            Parameter::new(source, Principal::Snake).add_post_convert(ignores.to_convertor());
        // TODO:Fix
        assert_eq!(convert(&params), "HelloWorld! _good_bye");

        let params = params.change_principal(Principal::Constant);
        // TODO:Fix
        assert_eq!(convert(&params), "HelloWorld! _GOOD_BYE");

        // TODO:Fix
        let params = params.change_principal(Principal::Chain);
        assert_eq!(convert(&params), "HelloWorld! -good-bye");

        //  TODO:Fix
        let params = params.change_principal(Principal::Camel);
        assert_eq!(convert(&params), "HelloWorld! GoodBye");
    }
    #[test]
    fn consider_pascal_case_names_with_wellknown_words() {
        let source = "UseGitHubEnterpriseGitHub";
        let params = Parameter::new(source, Principal::Snake)
            .add_post_convert(PascalCaseReservedIdentifiersConverter::wellknown().to_convertor());
        assert_eq!(convert(&params), "use_github_enterprise_github");

        let params = params.change_principal(Principal::Constant);
        assert_eq!(convert(&params), "USE_GITHUB_ENTERPRISE_GITHUB");

        let params = params.change_principal(Principal::Chain);
        assert_eq!(convert(&params), "use-github-enterprise-github");
    }
}
