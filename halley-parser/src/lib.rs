use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;

pub mod lexer;
pub mod parser;

#[derive(Clone)]
pub struct Range {
    pub path: PathBuf,
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {:?} ({:?})", self.start, self.end, self.path)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}