use std::path::PathBuf;

pub mod lexer;
pub mod parser;

#[derive(Clone, Debug)]
pub struct Range {
    pub path: PathBuf,
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}