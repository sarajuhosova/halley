use std::fs;
use std::path::Path;
use halley_core::name_resolution::resolve::resolve_names;
use halley_parser::lexer::tokenise;
use halley_parser::parser::parse_program;

fn main() {
    let test_file = Path::new("test.halley");
    let content = fs::read_to_string(test_file).unwrap();
    let mut tokens = tokenise(&content, test_file).unwrap();
    let program = parse_program(&mut tokens).unwrap();
    let resolver = resolve_names(&program).unwrap();
    println!("{}", program);
}
