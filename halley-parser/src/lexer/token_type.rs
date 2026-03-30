use std::mem;
use regex::Regex;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TokenType {
    Identifier,
    Number,

    Let,

    Equals,
    Semicolon,
    Plus,

    TokenTypesEnd,
}

impl TokenType {

    pub fn iter() -> impl Iterator<Item = TokenType> {
        (0..TokenType::TokenTypesEnd as u8).map(|i| unsafe { mem::transmute(i) })
    }

    pub fn regex(&self) -> Regex {
        Regex::new(self.pattern()).unwrap()
    }

    fn pattern(&self) -> &'static str {
        match self {
            TokenType::Identifier => r"\b[A-Za-z_][A-Za-z0-9_]*\b",
            TokenType::Number => r"\d+",
            TokenType::Let => r"\blet\b",
            TokenType::Equals => r"\=",
            TokenType::Semicolon => r";",
            TokenType::Plus => r"\+",
            _ => unreachable!()
        }
    }

}