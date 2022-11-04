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
    pub fn to_camel(&self) -> String {
        match self.principal {
            NamingPrincipal::Chain(chain) => Self::split_case_to_camel(chain, '-'),
            NamingPrincipal::Snake(snake) => Self::split_case_to_camel(snake, '_'),
            NamingPrincipal::Constant(chain) => {
                let mut result = String::new();
                let mut next_is_upper_flag = false;
                for (i, c) in chain.chars().enumerate() {
                    if next_is_upper_flag {
                        result.push(c);
                        next_is_upper_flag = false;
                        continue;
                    }
                    if c == '_' && i != 0 {
                        next_is_upper_flag = true;
                        continue;
                    }
                    result.push_str(&c.to_lowercase().to_string())
                }
                result
            }
            NamingPrincipal::Pascal(pascal) => pascal
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_lowercase() } else { c })
                .collect::<String>(),
            NamingPrincipal::NonPrincipal(_) => {
                let snake = self.to_snake();
                NamingPrincipalConvertor::new(&snake).to_camel()
            }
            _ => self.original.to_string(),
        }
    }
    pub fn to_snake(&self) -> String {
        match self.principal {
            NamingPrincipal::Camel(camel) => camel.chars().fold(String::new(), |mut acc, cur| {
                Self::upper_to_snake(&mut acc, cur);
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
                        Self::upper_to_snake(&mut acc, cur);
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
            NamingPrincipal::NonPrincipal(non_principal) => {
                non_principal
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, cur)| {
                        if i == 0 && cur.is_uppercase() {
                            acc.push(cur.to_ascii_lowercase());
                            return acc;
                        }
                        if cur == '-' {
                            // case - before upper
                            // ex: A-D
                            // if not impl below process
                            // tobe a__d
                            if let Some(Some(next_char)) = non_principal
                                .get(i + 1..i + 2)
                                .map(|str| str.chars().next())
                            {
                                if next_char.is_uppercase() {
                                    return acc;
                                }
                            }
                            acc.push('_');
                            return acc;
                        }
                        Self::upper_to_snake(&mut acc, cur);
                        acc
                    })
            }
            _ => self.original.to_string(),
        }
    }
    fn upper_to_snake(acc: &mut String, c: char) {
        if c.is_uppercase() {
            acc.push('_');
            acc.push(c.to_ascii_lowercase());
        } else {
            acc.push(c);
        }
    }
    fn split_case_to_camel(source: &str, split: char) -> String {
        let mut result = String::new();
        let mut next_is_upper_flag = false;
        for (i, c) in source.chars().enumerate() {
            if next_is_upper_flag {
                result.push_str(&c.to_uppercase().to_string());
                next_is_upper_flag = false;
                continue;
            }
            if c == split && i != 0 {
                next_is_upper_flag = true;
                continue;
            }
            if c == split && i == 0 {
                continue;
            }
            result.push(c);
        }
        result
    }
}

#[cfg(test)]
mod test_convertor {
    use super::*;
    use crate::naming_principal::naming_principal_test_data::*;

    #[test]
    fn test_to_camel() {
        let convertor = NamingPrincipalConvertor::new(FLATCASE);
        assert_eq!(convertor.to_camel(), "flatcase".to_string());
        let convertor = NamingPrincipalConvertor::new(EMPTYCASE);
        assert_eq!(convertor.to_camel(), "".to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE1);
        assert_eq!(convertor.to_camel(), "snakeCase".to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE2);
        assert_eq!(convertor.to_camel(), "snakeCase".to_string());
        let convertor = NamingPrincipalConvertor::new(CAMEL_CASE);
        assert_eq!(convertor.to_camel(), CAMEL_CASE.to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE1);
        assert_eq!(convertor.to_camel(), "constantCase".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE2);
        assert_eq!(convertor.to_camel(), "constant".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE1);
        assert_eq!(convertor.to_camel(), "pascalCase".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE2);
        assert_eq!(convertor.to_camel(), "aBCData".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE1);
        assert_eq!(convertor.to_camel(), "chainCase".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE2);
        assert_eq!(convertor.to_camel(), "chainCase".to_string());
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE1);
        assert_eq!(convertor.to_camel(), "aData".to_string());
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE2);
        assert_eq!(convertor.to_camel(), "aBCData".to_string());
    }
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
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE1);
        assert_eq!(convertor.to_snake(), "a_data".to_string());
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE2);
        assert_eq!(convertor.to_snake(), "a_b_c_data_".to_string());
    }
}
