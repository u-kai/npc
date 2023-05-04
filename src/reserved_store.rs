use crate::fns::{to_chain, to_constant, to_snake};

#[derive(Debug, Clone)]
pub(crate) struct PascalCaseReservedIdentifiers {
    store: Vec<String>,
}

impl PascalCaseReservedIdentifiers {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }
    pub fn wellknown() -> Self {
        let store = vec!["GitHub", "TypeScript", "JavaScript", "DeepL"];
        Self {
            store: store.into_iter().map(String::from).collect(),
        }
    }
    pub fn replace_for_snake_case(&self, sentence: impl Into<String>) -> String {
        let mut result = sentence.into();
        for target in self.store.iter() {
            let snake_case = to_snake(target);
            if result.contains(&snake_case) {
                result = result.replace(&snake_case, &Self::to_lower_case(target.as_str()));
            }
        }
        result
    }
    pub fn replace_for_constant_case(&self, sentence: impl Into<String>) -> String {
        let mut result = sentence.into();
        for target in self.store.iter() {
            let constant_case = to_constant(target);
            if result.contains(&constant_case) {
                result = result.replace(&constant_case, &Self::to_upper_case(target.as_str()));
            }
        }
        result
    }
    pub fn replace_for_chain_case(&self, sentence: impl Into<String>) -> String {
        let mut result = sentence.into();
        for target in self.store.iter() {
            let chain_case = to_chain(target);
            if result.contains(&chain_case) {
                result = result.replace(&chain_case, &Self::to_lower_case(target.as_str()));
            }
        }
        result
    }

    fn to_lower_case(target: &str) -> String {
        target.chars().map(|c| c.to_ascii_lowercase()).collect()
    }
    fn to_upper_case(target: &str) -> String {
        target.chars().map(|c| c.to_ascii_uppercase()).collect()
    }
    pub fn add(&mut self, target: impl Into<String>) {
        self.store.push(target.into());
    }
}

#[cfg(test)]
mod tests {
    use crate::reserved_store::PascalCaseReservedIdentifiers;

    #[test]
    #[allow(non_snake_case)]
    fn 登録されたパスカルケースの名称に当てはまる箇所ををすべて小文字に変換する() {
        let sut = PascalCaseReservedIdentifiers::wellknown();
        assert_eq!(
            sut.replace_for_snake_case("use_git_hub_enterprise_git_hub"),
            "use_github_enterprise_github"
        );
    }
}
