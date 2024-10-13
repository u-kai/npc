pub mod convertor;
pub mod corrector;

pub trait PreConvert {
    fn convert(&self, source: &str, principal: Principal) -> String;
}
pub trait PostConvert {
    fn convert(&self, source: &str, principal: Principal) -> String;
}

impl<F> PreConvert for F
where
    F: Fn(&str, Principal) -> String,
{
    fn convert(&self, source: &str, principal: Principal) -> String {
        self(source, principal)
    }
}
impl<F> PostConvert for F
where
    F: Fn(&str, Principal) -> String,
{
    fn convert(&self, source: &str, principal: Principal) -> String {
        self(source, principal)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Principal {
    Camel,
    Pascal,
    Snake,
    Constant,
    Chain,
}

pub struct Parameter {
    source: String,
    to: Principal,
    pres: Vec<Box<dyn PreConvert>>,
    posts: Vec<Box<dyn PostConvert>>,
}
impl Parameter {
    pub fn new(source: impl Into<String>, to: Principal) -> Self {
        Self {
            source: source.into(),
            to,
            posts: Vec::new(),
            pres: Vec::new(),
        }
    }
    pub fn add_pre_convert(mut self, convert: Box<dyn PreConvert>) -> Self {
        self.pres.push(convert);
        self
    }
    pub fn add_post_convert(mut self, convert: Box<dyn PostConvert>) -> Self {
        self.posts.push(convert);
        self
    }
    pub fn change_principal(mut self, to: Principal) -> Self {
        self.to = to;
        self
    }
}

pub fn convert(param: &Parameter) -> String {
    let result = param
        .pres
        .iter()
        .fold(param.source.clone(), |acc, c| c.convert(&acc, param.to));

    let result = match param.to {
        Principal::Camel => to_camel(&result),
        Principal::Pascal => to_pascal(&result),
        Principal::Snake => to_snake(&result),
        Principal::Constant => to_constant(&result),
        Principal::Chain => to_chain(&result),
    };

    param
        .posts
        .iter()
        .fold(result, |acc, c| c.convert(&acc, param.to))
}

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

/// This module provides a struct `NamingPrincipalConvertor` that converts
/// a string to various naming conventions such as camel case, snake case,
/// pascal case, and chain case.
/// A struct that converts a string to various naming conventions.
///
/// # Examples
///
/// ```
/// use npc::NamingPrincipalConvertor;
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
        is_non_principal(self.original)
    }
    pub fn is_constant(&self) -> bool {
        is_constant(self.original)
    }
    pub fn is_chain(&self) -> bool {
        is_chain(self.original)
    }
    pub fn is_pascal(&self) -> bool {
        is_pascal(self.original)
    }
    pub fn is_snake(&self) -> bool {
        is_snake(self.original)
    }
    pub fn is_camel(&self) -> bool {
        is_camel(self.original)
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
                snake.chars().map(|c| c.to_ascii_uppercase()).collect()
            }
            NamingPrincipal::Pascal(_) => {
                let snake = self.to_snake();
                snake.chars().map(|c| c.to_ascii_uppercase()).collect()
            }
            NamingPrincipal::NonPrincipal(_) => {
                let snake = self.to_snake();
                snake.chars().map(|c| c.to_ascii_uppercase()).collect()
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
                Self::split_case_to_pascal(&snake, '_')
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
                            // to be a__d
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
pub(self) mod naming_principal_test_data {
    pub(crate) const FLATCASE: &'static str = "flatcase";
    pub(crate) const EMPTYCASE: &'static str = "";
    pub(crate) const SNAKE_CASE1: &'static str = "snake_case";
    pub(crate) const SNAKE_CASE2: &'static str = "_snake_case";
    pub(crate) const CAMEL_CASE: &'static str = "camelCase";
    pub(crate) const CAMEL_CASE2: &'static str = "internetIP";
    pub(crate) const PASCAL_CASE1: &'static str = "PascalCase";
    pub(crate) const PASCAL_CASE2: &'static str = "ABCData";
    pub(crate) const CONSTANT_CASE1: &'static str = "CONSTANT_CASE";
    pub(crate) const CONSTANT_CASE2: &'static str = "CONSTANT";
    pub(crate) const CONSTANT_CASE3: &'static str = "_CONSTANT_CASE";
    pub(crate) const CHAIN_CASE1: &'static str = "chain-case";
    pub(crate) const CHAIN_CASE2: &'static str = "-chain-case";
    pub(crate) const NONPRINCIPAL_CASE1: &'static str = "A_data";
    pub(crate) const NONPRINCIPAL_CASE2: &'static str = "ABC-Data_";
    pub(crate) const NONPRINCIPAL_CASE3: &'static str = "ABC- Data";
}

#[cfg(test)]
mod test_convertor {
    use super::*;
    use naming_principal_test_data::*;
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

#[derive(Debug, PartialEq, Eq, Clone)]
enum NamingPrincipal<'a> {
    Snake(&'a str),
    Constant(&'a str),
    Camel(&'a str),
    Pascal(&'a str),
    Chain(&'a str),
    Empty(&'a str),
    Flat(&'a str),
    NonPrincipal(&'a str),
}

impl<'a> NamingPrincipal<'a> {
    fn new(source: &'a str) -> Self {
        //flat contain camel and snake and chain that's why is_flat is position top
        if is_flat(source) {
            return Self::Flat(source);
        }
        if is_camel(source) {
            return Self::Camel(source);
        }
        if is_pascal(source) {
            return Self::Pascal(source);
        }
        if is_snake(source) {
            return Self::Snake(source);
        }
        if is_constant(source) {
            return Self::Constant(source);
        }
        if is_chain(source) {
            return Self::Chain(source);
        }
        if is_empty(source) {
            return Self::Empty(source);
        }
        if is_non_principal(source) {
            return Self::NonPrincipal(source);
        }
        panic!("not considering case! have to impl case {}", source)
    }
}

fn is_non_principal(source: &str) -> bool {
    !(is_flat(source)
        || is_pascal(source)
        || is_camel(source)
        || is_chain(source)
        || is_constant(source)
        || is_empty(source)
        || is_snake(source))
}
fn is_flat(source: &str) -> bool {
    !is_empty(source) && source.chars().all(|c| c.is_lowercase() || c.is_numeric())
}
fn is_chain(source: &str) -> bool {
    !is_empty(source)
        && source
            .chars()
            .all(|c| c == '-' || c != '_' && c.is_lowercase() || c.is_numeric())
}
fn is_constant(source: &str) -> bool {
    !is_empty(source)
        && source
            .chars()
            .all(|c| c == '_' || c != '-' && c.is_uppercase() || c.is_numeric())
}
fn is_snake(source: &str) -> bool {
    !is_empty(source)
        && source
            .chars()
            .all(|c| c == '_' || c != '-' && c.is_lowercase() || c.is_numeric())
}
fn is_pascal(source: &str) -> bool {
    if is_empty(source) || source.contains("_") || source.contains("-") {
        return false;
    }
    let first = source.chars().next().unwrap();
    if !first.is_uppercase() {
        return false;
    }
    !source.chars().all(|c| c.is_uppercase() || c.is_numeric())
}
fn is_camel(source: &str) -> bool {
    if is_empty(source) || source.contains("_") || source.contains("-") {
        return false;
    }
    if let Some(first) = source.chars().next() {
        first.is_lowercase()
    } else {
        false
    }
}
fn is_empty(source: &str) -> bool {
    source.len() == 0
}

#[cfg(test)]
mod test_naming_principal {

    use super::*;
    use naming_principal_test_data::*;
    #[test]
    fn test_is_nonprincipal_and_new_nonprincipal() {
        assert!(is_non_principal(NONPRINCIPAL_CASE1));
        assert!(is_non_principal(NONPRINCIPAL_CASE2));
        assert!(!is_non_principal(FLATCASE));
        assert!(!is_non_principal(EMPTYCASE));
        assert!(!is_non_principal(CHAIN_CASE1));
        assert!(!is_non_principal(CHAIN_CASE2));
        assert!(!is_non_principal(SNAKE_CASE1));
        assert!(!is_non_principal(SNAKE_CASE2));
        assert!(!is_non_principal(PASCAL_CASE1));
        assert!(!is_non_principal(PASCAL_CASE2));
        assert!(!is_non_principal(CAMEL_CASE));
        assert!(!is_non_principal(CONSTANT_CASE1));
        assert!(!is_non_principal(CONSTANT_CASE2));
        let np = NamingPrincipal::new(NONPRINCIPAL_CASE1);
        assert_eq!(np, NamingPrincipal::NonPrincipal(NONPRINCIPAL_CASE1));
        let np = NamingPrincipal::new(NONPRINCIPAL_CASE2);
        assert_eq!(np, NamingPrincipal::NonPrincipal(NONPRINCIPAL_CASE2));
    }
    #[test]
    fn test_is_flat_and_new_flat() {
        assert!(is_flat(FLATCASE));
        assert!(!is_flat(EMPTYCASE));
        assert!(!is_flat(CHAIN_CASE1));
        assert!(!is_flat(CHAIN_CASE2));
        assert!(!is_flat(SNAKE_CASE1));
        assert!(!is_flat(SNAKE_CASE2));
        assert!(!is_flat(PASCAL_CASE1));
        assert!(!is_flat(PASCAL_CASE2));
        assert!(!is_flat(CAMEL_CASE));
        assert!(!is_flat(CONSTANT_CASE1));
        assert!(!is_flat(CONSTANT_CASE2));
        assert!(!is_flat(NONPRINCIPAL_CASE1));
        assert!(!is_flat(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(FLATCASE);
        assert_eq!(np, NamingPrincipal::Flat(FLATCASE));
    }
    #[test]
    fn test_is_chain_and_new_chain() {
        assert!(is_chain(CHAIN_CASE1));
        assert!(is_chain(CHAIN_CASE2));
        assert!(is_chain(FLATCASE));
        assert!(!is_chain(EMPTYCASE));
        assert!(!is_chain(SNAKE_CASE1));
        assert!(!is_chain(SNAKE_CASE2));
        assert!(!is_chain(PASCAL_CASE1));
        assert!(!is_chain(PASCAL_CASE2));
        assert!(!is_chain(CAMEL_CASE));
        assert!(!is_chain(CONSTANT_CASE1));
        assert!(!is_chain(CONSTANT_CASE2));
        assert!(!is_chain(NONPRINCIPAL_CASE1));
        assert!(!is_chain(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(CHAIN_CASE1);
        assert_eq!(np, NamingPrincipal::Chain(CHAIN_CASE1));
        let np = NamingPrincipal::new(CHAIN_CASE2);
        assert_eq!(np, NamingPrincipal::Chain(CHAIN_CASE2));
    }
    #[test]
    fn test_is_constant_and_new_constant() {
        assert!(is_constant(CONSTANT_CASE1));
        assert!(is_constant(CONSTANT_CASE2));
        assert!(is_constant(CONSTANT_CASE3));
        assert!(!is_constant(CHAIN_CASE1));
        assert!(!is_constant(CHAIN_CASE2));
        assert!(!is_constant(FLATCASE));
        assert!(!is_constant(EMPTYCASE));
        assert!(!is_constant(SNAKE_CASE1));
        assert!(!is_constant(SNAKE_CASE2));
        assert!(!is_constant(PASCAL_CASE1));
        assert!(!is_constant(PASCAL_CASE2));
        assert!(!is_constant(CAMEL_CASE));
        assert!(!is_constant(NONPRINCIPAL_CASE1));
        assert!(!is_constant(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(CONSTANT_CASE1);
        assert_eq!(np, NamingPrincipal::Constant(CONSTANT_CASE1));
        let np = NamingPrincipal::new(CONSTANT_CASE2);
        assert_eq!(np, NamingPrincipal::Constant(CONSTANT_CASE2));
        let np = NamingPrincipal::new(CONSTANT_CASE3);
        assert_eq!(np, NamingPrincipal::Constant(CONSTANT_CASE3));
    }
    #[test]
    fn test_is_snake_and_new_snake() {
        assert!(is_snake(SNAKE_CASE1));
        assert!(is_snake(SNAKE_CASE2));
        assert!(is_snake(FLATCASE));
        assert!(!is_snake(CHAIN_CASE1));
        assert!(!is_snake(CHAIN_CASE2));
        assert!(!is_snake(EMPTYCASE));
        assert!(!is_snake(PASCAL_CASE1));
        assert!(!is_snake(PASCAL_CASE2));
        assert!(!is_snake(CAMEL_CASE));
        assert!(!is_snake(CONSTANT_CASE1));
        assert!(!is_snake(CONSTANT_CASE2));
        assert!(!is_snake(NONPRINCIPAL_CASE1));
        assert!(!is_snake(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(SNAKE_CASE1);
        assert_eq!(np, NamingPrincipal::Snake(SNAKE_CASE1));
        let np = NamingPrincipal::new(SNAKE_CASE2);
        assert_eq!(np, NamingPrincipal::Snake(SNAKE_CASE2));
    }
    #[test]
    fn test_is_pascal_and_new_pascal() {
        assert!(is_pascal(PASCAL_CASE1));
        assert!(is_pascal(PASCAL_CASE2));
        assert!(!is_pascal(FLATCASE));
        assert!(!is_pascal(CHAIN_CASE1));
        assert!(!is_pascal(CHAIN_CASE2));
        assert!(!is_pascal(EMPTYCASE));
        assert!(!is_pascal(SNAKE_CASE1));
        assert!(!is_pascal(SNAKE_CASE2));
        assert!(!is_pascal(CAMEL_CASE));
        assert!(!is_pascal(CONSTANT_CASE1));
        assert!(!is_pascal(CONSTANT_CASE2));
        assert!(!is_pascal(NONPRINCIPAL_CASE1));
        assert!(!is_pascal(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(PASCAL_CASE1);
        assert_eq!(np, NamingPrincipal::Pascal(PASCAL_CASE1));
        let np = NamingPrincipal::new(PASCAL_CASE2);
        assert_eq!(np, NamingPrincipal::Pascal(PASCAL_CASE2));
    }
    #[test]
    fn test_is_camel_and_new_camel() {
        assert!(is_camel(CAMEL_CASE));
        assert!(is_camel(FLATCASE));
        assert!(!is_camel(PASCAL_CASE1));
        assert!(!is_camel(PASCAL_CASE2));
        assert!(!is_camel(CHAIN_CASE1));
        assert!(!is_camel(CHAIN_CASE2));
        assert!(!is_camel(EMPTYCASE));
        assert!(!is_camel(SNAKE_CASE1));
        assert!(!is_camel(SNAKE_CASE2));
        assert!(!is_camel(CONSTANT_CASE1));
        assert!(!is_camel(CONSTANT_CASE2));
        assert!(!is_camel(NONPRINCIPAL_CASE1));
        assert!(!is_camel(NONPRINCIPAL_CASE2));
        let np = NamingPrincipal::new(CAMEL_CASE);
        assert_eq!(np, NamingPrincipal::Camel(CAMEL_CASE));
    }
}
