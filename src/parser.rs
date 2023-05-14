use lazy_static::lazy_static;
use regex::Regex;
use std::env;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
    // Function to parse a given string returning a vector of each word
    pub fn parse(&self, input: &mut String) -> Vec<String> {
        let mut content: Vec<String> = input.split_whitespace().map(|x| x.to_owned()).collect();
        for word in &mut content {
            if ENV_VARIABLE.is_match(word) && !BACKSLASH.is_match(word) {
                while let Some(index) = word.find('$') {
                    let env = word.get(index + 1..).unwrap_or_default();
                    word.replace_range(index.., &env::var(env).unwrap_or_default());
                }
            }
        }
        return content;
    }
}

lazy_static! {
    static ref ENV_VARIABLE: Regex = Regex::new(r".*$.*").unwrap();
    static ref BACKSLASH: Regex = Regex::new(r".*\\.*").unwrap();
}
