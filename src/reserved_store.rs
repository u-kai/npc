use std::collections::HashMap;

pub(crate) struct ReservedPascalCaseIdentifies {
    store: Vec<String>,
}

impl ReservedPascalCaseIdentifies {
    pub fn wellknown() -> Self {
        let store = vec!["GitHub", "TypeScript", "JavaScript", "JavaScript", "DeepL"];

        Self {
            store: store.into_iter().map(String::from).collect(),
        }
    }
    pub fn to_other_case(&self, target: &str) -> String {
        if self.store.iter().any(|s| s == target) {
            target.chars().map(|c| c.to_ascii_lowercase()).collect()
        } else {
            target.to_string()
        }
    }
    pub fn add(&mut self, target: impl Into<String>) {
        self.store.push(target.into());
    }
}

#[cfg(test)]
mod tests {
    use crate::reserved_store::ReservedPascalCaseIdentifies;

    #[test]
    #[allow(non_snake_case)]
    fn 登録されたパスカルケースの名称が他のケースに変換される場合はすべて小文字に変換される() {
        let mut sut = ReservedPascalCaseIdentifies::wellknown();
        sut.add("UKai");
        assert_eq!(sut.to_other_case("UKai"), "ukai");
    }
    #[test]
    #[allow(non_snake_case)]
    fn GitHubをパスカルケース以外に変換するとgithub() {
        let sut = ReservedPascalCaseIdentifies::wellknown();
        assert_eq!(sut.to_other_case("GitHub"), "github");
    }
}
