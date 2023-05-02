/// This module provides a struct `NamingPrincipalConvertor` that converts
/// a string to various naming conventions such as camel case, snake case,
/// pascal case, and chain case.
use crate::naming_principal::NamingPrincipal;

/// A struct that converts a string to various naming conventions.
///
/// # Examples
///
/// ```
/// use npc::convertor::NamingPrincipalConvertor;
///
/// let convertor = NamingPrincipalConvertor::new("some_snake_case_name");
/// assert_eq!(convertor.to_camel(), "someSnakeCaseName");
/// assert_eq!(convertor.to_pascal(), "SomeSnakeCaseName");
/// assert_eq!(convertor.to_snake(), "some_snake_case_name");
/// assert_eq!(convertor.to_chain(), "some-snake-case-name");
/// assert_eq!(convertor.to_constant(), "SOME_SNAKE_CASE_NAME");
/// ```
#[derive(Debug, Clone)]
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
    pub fn is_non_principal(&self) -> bool {
        NamingPrincipal::is_non_principal(self.original)
    }
    pub fn is_constant(&self) -> bool {
        NamingPrincipal::is_constant(self.original)
    }
    pub fn is_chain(&self) -> bool {
        NamingPrincipal::is_chain(self.original)
    }
    pub fn is_pascal(&self) -> bool {
        NamingPrincipal::is_pascal(self.original)
    }
    pub fn is_snake(&self) -> bool {
        NamingPrincipal::is_snake(self.original)
    }
    pub fn is_camel(&self) -> bool {
        NamingPrincipal::is_camel(self.original)
    }
    pub fn original(&self) -> &str {
        self.original
    }
    pub fn to_chain(&self) -> String {
        match self.principal {
            NamingPrincipal::Snake(snake) => snake.replace("_", "-"),
            NamingPrincipal::Camel(camel) => camel.chars().fold(String::new(), |mut acc, c| {
                Self::upper_to_partition(&mut acc, c, '-');
                acc
            }),
            NamingPrincipal::Constant(constant) => constant
                .chars()
                .map(|c| {
                    if c == '_' {
                        '-'
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect(),
            NamingPrincipal::Pascal(pascal) => {
                pascal
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, cur)| {
                        if i == 0 {
                            acc.push(cur.to_ascii_lowercase());
                            return acc;
                        }
                        Self::upper_to_partition(&mut acc, cur, '-');
                        acc
                    })
            }
            NamingPrincipal::NonPrincipal(_) => {
                let snake = self.to_snake();
                let np = NamingPrincipalConvertor::new(&snake);
                np.to_chain()
            }
            _ => self.original.to_string(),
        }
    }
    pub fn to_constant(&self) -> String {
        match self.principal {
            NamingPrincipal::Snake(snake) => {
                snake.chars().map(|c| c.to_ascii_uppercase()).collect()
            }
            NamingPrincipal::Chain(chain) => chain
                .chars()
                .map(|c| {
                    if c == '-' {
                        '_'
                    } else {
                        c.to_ascii_uppercase()
                    }
                })
                .collect(),
            NamingPrincipal::Flat(flat) => flat.chars().map(|c| c.to_ascii_uppercase()).collect(),
            NamingPrincipal::Camel(_) => {
                let snake = self.to_snake();
                let np = NamingPrincipalConvertor::new(&snake);
                np.to_constant()
            }
            NamingPrincipal::Pascal(_) => {
                let snake = self.to_snake();
                let np = NamingPrincipalConvertor::new(&snake);
                np.to_constant()
            }
            NamingPrincipal::NonPrincipal(_) => {
                let snake = self.to_snake();
                let np = NamingPrincipalConvertor::new(&snake);
                np.to_constant()
            }
            _ => self.original.to_string(),
        }
    }
    pub fn to_pascal(&self) -> String {
        match self.principal {
            NamingPrincipal::Snake(snake) => Self::split_case_to_pascal(snake, '_'),
            NamingPrincipal::Chain(chain) => Self::split_case_to_pascal(chain, '-'),
            NamingPrincipal::Flat(flat) => Self::first_char_to_upper(flat),
            NamingPrincipal::Camel(camel) => Self::first_char_to_upper(camel),
            NamingPrincipal::NonPrincipal(_) => {
                let snake = self.to_snake();
                let np = NamingPrincipalConvertor::new(&snake);
                np.to_pascal()
            }
            NamingPrincipal::Constant(constant) => {
                let mut result = String::new();
                let mut next_is_upper_flag = true;
                for (i, c) in constant.chars().enumerate() {
                    if c == '_' {
                        if i != 0 {
                            next_is_upper_flag = true;
                        }
                        continue;
                    }
                    if next_is_upper_flag {
                        result.push(c);
                        next_is_upper_flag = false;
                        continue;
                    }
                    result.push(c.to_ascii_lowercase())
                }
                result
            }
            _ => self.original.to_string(),
        }
    }
    pub fn to_camel(&self) -> String {
        match self.principal {
            NamingPrincipal::Chain(chain) => Self::split_case_to_camel(chain, '-'),
            NamingPrincipal::Snake(snake) => Self::split_case_to_camel(snake, '_'),
            NamingPrincipal::Constant(constant) => {
                let mut result = String::new();
                let mut next_is_upper_flag = false;
                for (i, c) in constant.chars().enumerate() {
                    if next_is_upper_flag {
                        result.push(c);
                        next_is_upper_flag = false;
                        continue;
                    }
                    if c == '_' {
                        if i != 0 {
                            next_is_upper_flag = true;
                        }
                        continue;
                    }
                    result.push(c.to_ascii_lowercase())
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
            NamingPrincipal::Camel(camel) => {
                let mut is_upper_sequence = false;
                let mut result = String::new();
                for c in camel.chars() {
                    if is_upper_sequence && c.is_uppercase() {
                        result.push(c.to_ascii_lowercase());
                        continue;
                    }
                    if !is_upper_sequence && c.is_uppercase() {
                        is_upper_sequence = true;
                        Self::upper_to_partition(&mut result, c, '_');
                        continue;
                    }
                    result.push(c);
                    is_upper_sequence = false;
                }
                result
            }
            NamingPrincipal::Pascal(pascal) => {
                pascal
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, cur)| {
                        if i == 0 {
                            acc.push(cur.to_ascii_lowercase());
                            return acc;
                        }
                        Self::upper_to_partition(&mut acc, cur, '_');
                        acc
                    })
            }
            NamingPrincipal::Constant(constant) => constant
                .chars()
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>(),
            NamingPrincipal::Chain(chain) => chain.replace("-", "_"),
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
                        Self::upper_to_partition(&mut acc, cur, '_');
                        acc
                    })
            }
            _ => self.original.to_string(),
        }
    }
    fn upper_to_partition(acc: &mut String, c: char, partition: char) {
        if c.is_uppercase() {
            acc.push(partition);
            acc.push(c.to_ascii_lowercase());
        } else {
            acc.push(c);
        }
    }
    fn first_char_to_upper(source: &str) -> String {
        source
            .chars()
            .enumerate()
            .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
            .collect()
    }
    fn split_case_to_camel(source: &str, split: char) -> String {
        let mut result = String::new();
        let mut next_is_upper_flag = false;
        for (i, c) in source.chars().enumerate() {
            if next_is_upper_flag {
                result.push(c.to_ascii_uppercase());
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
    fn split_case_to_pascal(source: &str, split: char) -> String {
        let mut result = String::new();
        let mut next_is_upper_flag = false;
        for (i, c) in source.chars().enumerate() {
            if next_is_upper_flag {
                result.push(c.to_ascii_uppercase());
                next_is_upper_flag = false;
                continue;
            }
            if c == split {
                next_is_upper_flag = true;
                continue;
            }
            if c != split && i == 0 {
                result.push(c.to_ascii_uppercase());
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
    fn test_to_chain() {
        let convertor = NamingPrincipalConvertor::new(FLATCASE);
        assert_eq!(convertor.to_chain(), "flatcase".to_string());
        let convertor = NamingPrincipalConvertor::new(EMPTYCASE);
        assert_eq!(convertor.to_chain(), "".to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE1);
        assert_eq!(convertor.to_chain(), "snake-case".to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE2);
        assert_eq!(convertor.to_chain(), "-snake-case".to_string());
        let convertor = NamingPrincipalConvertor::new(CAMEL_CASE);
        assert_eq!(convertor.to_chain(), "camel-case".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE1);
        assert_eq!(convertor.to_chain(), "constant-case".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE2);
        assert_eq!(convertor.to_chain(), "constant".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE3);
        assert_eq!(convertor.to_chain(), "-constant-case".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE1);
        assert_eq!(convertor.to_chain(), "pascal-case".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE2);
        assert_eq!(convertor.to_chain(), "a-b-c-data".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE1);
        assert_eq!(convertor.to_chain(), "chain-case".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE2);
        assert_eq!(convertor.to_chain(), "-chain-case".to_string());
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE1);
        assert_eq!(convertor.to_chain(), "a-data".to_string());
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE2);
        assert_eq!(convertor.to_chain(), "a-b-c-data-".to_string());
    }
    #[test]
    fn test_to_constant() {
        let convertor = NamingPrincipalConvertor::new(FLATCASE);
        assert_eq!(convertor.to_constant(), "FLATCASE".to_string());
        let convertor = NamingPrincipalConvertor::new(EMPTYCASE);
        assert_eq!(convertor.to_constant(), "".to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE1);
        assert_eq!(convertor.to_constant(), "SNAKE_CASE".to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE2);
        assert_eq!(convertor.to_constant(), "_SNAKE_CASE".to_string());
        let convertor = NamingPrincipalConvertor::new(CAMEL_CASE);
        assert_eq!(convertor.to_constant(), "CAMEL_CASE".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE1);
        assert_eq!(convertor.to_constant(), "CONSTANT_CASE".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE2);
        assert_eq!(convertor.to_constant(), "CONSTANT".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE3);
        assert_eq!(convertor.to_constant(), "_CONSTANT_CASE".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE1);
        assert_eq!(convertor.to_constant(), "PASCAL_CASE".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE2);
        assert_eq!(convertor.to_constant(), "A_B_C_DATA".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE1);
        assert_eq!(convertor.to_constant(), "CHAIN_CASE".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE2);
        assert_eq!(convertor.to_constant(), "_CHAIN_CASE".to_string());
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE1);
        assert_eq!(convertor.to_constant(), "A_DATA".to_string());
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE2);
        assert_eq!(convertor.to_constant(), "A_B_C_DATA_".to_string());
    }

    #[test]
    fn test_to_pascal() {
        let convertor = NamingPrincipalConvertor::new(FLATCASE);
        assert_eq!(convertor.to_pascal(), "Flatcase".to_string());
        let convertor = NamingPrincipalConvertor::new(EMPTYCASE);
        assert_eq!(convertor.to_pascal(), "".to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE1);
        assert_eq!(convertor.to_pascal(), "SnakeCase".to_string());
        let convertor = NamingPrincipalConvertor::new(SNAKE_CASE2);
        assert_eq!(convertor.to_pascal(), "SnakeCase".to_string());
        let convertor = NamingPrincipalConvertor::new(CAMEL_CASE);
        assert_eq!(convertor.to_pascal(), "CamelCase".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE1);
        assert_eq!(convertor.to_pascal(), "ConstantCase".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE2);
        assert_eq!(convertor.to_pascal(), "Constant".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE3);
        assert_eq!(convertor.to_pascal(), "ConstantCase".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE1);
        assert_eq!(convertor.to_pascal(), "PascalCase".to_string());
        let convertor = NamingPrincipalConvertor::new(PASCAL_CASE2);
        assert_eq!(convertor.to_pascal(), "ABCData".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE1);
        assert_eq!(convertor.to_pascal(), "ChainCase".to_string());
        let convertor = NamingPrincipalConvertor::new(CHAIN_CASE2);
        assert_eq!(convertor.to_pascal(), "ChainCase".to_string());
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE1);
        assert_eq!(convertor.to_pascal(), "AData".to_string());
        let convertor = NamingPrincipalConvertor::new(NONPRINCIPAL_CASE2);
        assert_eq!(convertor.to_pascal(), "ABCData".to_string());
    }
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
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE3);
        assert_eq!(convertor.to_camel(), "constantCase".to_string());
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
        let convertor = NamingPrincipalConvertor::new(CAMEL_CASE2);
        assert_eq!(convertor.to_snake(), "internet_ip".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE1);
        assert_eq!(convertor.to_snake(), "constant_case".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE2);
        assert_eq!(convertor.to_snake(), "constant".to_string());
        let convertor = NamingPrincipalConvertor::new(CONSTANT_CASE3);
        assert_eq!(convertor.to_snake(), "_constant_case".to_string());
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
