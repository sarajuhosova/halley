use std::fmt::{Debug, Display, Formatter};
use crate::lexer::token_type::TokenType;
use crate::Range;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub range: Range,
    pub content: String,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} ({:?})", self.token_type, self.content, self.range)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.token_type, self.content)
    }
}