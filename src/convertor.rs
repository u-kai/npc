use crate::naming_principal::NamingPrincipal;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NamingPrincipalConvertor<'a> {
    original: &'a str,
    principal: NamingPrincipal<'a>,
}

impl<'a> NamingPrincipalConvertor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            original: source,
            principal: NamingPrincipal::new(source),
        }
    }
    pub fn to_snake(&self) -> String {
        fn upper_to_snake(acc: &mut String, c: char) {
            if c.is_uppercase() {
                acc.push('_');
                acc.push(c.to_ascii_lowercase());
            } else {
                acc.push(c);
            }
        }
        match self.principal {
            NamingPrincipal::Camel(camel) => camel.chars().fold(String::new(), |mut acc, cur| {
                upper_to_snake(&mut acc, cur);
                acc
            }),
            NamingPrincipal::Pascal(pascal) => {
                pascal
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, cur)| {
                        if i == 0 {
                            acc.push(cur.to_ascii_lowercase());
                            return acc;
                        }
                        upper_to_snake(&mut acc, cur);
                        acc
                    })
            }
            NamingPrincipal::Constant(constant) => constant
                .chars()
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>(),
            NamingPrincipal::Chain(chain) => chain
                .chars()
                .map(|c| {
                    if c == '-' {
                        return '_';
                    }
                    c
                })
                .collect::<String>(),
            _ => self.original.to_string(),
        }
    }
}

#[cfg(test)]
mod test_convertor {
    use super::*;
    use crate::naming_principal::naming_principal_test_data::*;

    #[test]
    fn test_to_snake() {
        let convertor = NamingPrincipalConvertor::new(FLATCASE);
        assert_eq!(convertor.to_snake(), "flatcase".to_string());
        let convertor = NamingPrincipalConvertor::new(EMPTYCASE);
        assert_eq!(convertor.to_snake(), "".to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE1);
        assert_eq!(convertor.to_snake(), SNAKE_CASE1.to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE2);
        assert_eq!(convertor.to_snake(), SNAKE_CASE2.to_string());
        let convertor = NamingPrincipalConvertor::new(CAMEL_CASE);
        assert_eq!(convertor.to_snake(), "camel_case".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE1);
        assert_eq!(convertor.to_snake(), "constant_case".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE2);
        assert_eq!(convertor.to_snake(), "constant".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE1);
        assert_eq!(convertor.to_snake(), "pascal_case".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE2);
        assert_eq!(convertor.to_snake(), "a_b_c_data".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE1);
        assert_eq!(convertor.to_snake(), "chain_case".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE2);
        assert_eq!(convertor.to_snake(), "_chain_case".to_string());
    }
}
