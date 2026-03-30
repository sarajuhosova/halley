use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};
use crate::{Position, Range};
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;

pub mod token;
pub mod token_type;

pub fn tokenise<P: AsRef<Path>>(input: &str, path: P) -> Result<Vec<Token>> {
    let mut result: Vec<Token> = Vec::new();

    let mut position = 0;
    skip_whitespace(input, &mut position);
    while let Some(token) = find_next_token(input, &mut position, path.as_ref())? {
        result.push(token);
        skip_whitespace(input, &mut position);
    }

    Ok(result)
}

fn find_next_token(input: &str, index: &mut usize, path: &Path) -> Result<Option<Token>> {
    if *index >= input.len() {
        return Ok(None);
    }
    let (token_type, best_match) = TokenType::iter()
        .flat_map(|token_type| token_type.regex().find_at(input, *index).map(|m| (token_type, m)))
        .filter(|(_, m)| m.start() == *index)
        .max_by_key(|(t, m)| (m.len(), *t))
        .ok_or_else(|| anyhow!("No token matches at the start of \"{}\"", &input[*index..]))?;
    *index += best_match.len();
    let result = Ok(Some(Token {
        token_type,
        range: Range {
            path: PathBuf::from(path),
            start: index_to_position(input, best_match.start()),
            end: index_to_position(input, best_match.end()),
        },
        content: String::from(best_match.as_str()),
    }));
    result
}

fn index_to_position(input: &str, index: usize) -> Position {
    let line = input[0..index].chars().filter(|c| *c == '\n').count();
    let line_start_at = input[0..index].chars().enumerate().filter(|(_, c)| *c == '\n').last().map(|(i, _)| i).unwrap_or(0);
    Position { line, column: index - line_start_at }
}

fn skip_whitespace(input: &str, position: &mut usize) {
    while input.chars().nth(*position).is_some_and(|c| c.is_whitespace()) {
        *position += 1;
    }
}