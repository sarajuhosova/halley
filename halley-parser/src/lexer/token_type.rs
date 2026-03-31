use std::mem;
use regex::Regex;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TokenType {
    Identifier,
    Number,

    Let,
    Fn,

    Int,
    Bool,

    Equals,
    Plus,
    Reference,
    Arrow,
    DoubleEquals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    And,
    Or,
    Minus,
    Asterisk,
    Divide,
    Modulo,

    Comma,
    Semicolon,
    Colon,

    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,

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
            TokenType::Fn => r"\bfn\b",

            TokenType::Int => r"\bInt\b",
            TokenType::Bool => r"\bBool\b",

            TokenType::Equals => r"\=",
            TokenType::Plus => r"\+",
            TokenType::Reference => r"&",
            TokenType::Arrow => r"->",
            TokenType::DoubleEquals => r"\=\=",
            TokenType::NotEquals => r"!\=",
            TokenType::LessThan => r"<",
            TokenType::GreaterThan => r">",
            TokenType::LessThanEquals => r"<\=",
            TokenType::GreaterThanEquals => r">\=",
            TokenType::And => r"&&",
            TokenType::Or => r"||",
            TokenType::Minus => r"-",
            TokenType::Asterisk => r"\*",
            TokenType::Divide => r"/",
            TokenType::Modulo => r"%",

            TokenType::Comma => r",",
            TokenType::Semicolon => r";",
            TokenType::Colon => r":",

            TokenType::BraceOpen => r"\{",
            TokenType::BraceClose => r"\}",
            TokenType::ParenOpen => r"\(",
            TokenType::ParenClose => r"\)",

            TokenType::TokenTypesEnd => panic!("Invalid token type"),
        }
    }

}