use std::fs;
use std::path::Path;
use anyhow::Result;
use halley_lang::ast::Program;
use halley_parser::lexer::tokenise;
use halley_parser::parser::parse_program;

pub mod name_resolution;

pub fn parse_halley_file<P: AsRef<Path>>(path: P) -> Result<Program> {
    let content = fs::read_to_string(path.as_ref())?;
    let mut tokens = tokenise(&content, path.as_ref())?;
    parse_program(&mut tokens)
}