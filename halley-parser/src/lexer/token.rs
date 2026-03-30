use crate::lexer::token_type::TokenType;
use crate::Range;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub range: Range,
    pub content: String,
}