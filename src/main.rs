use std::fs;
use std::path::Path;
use halley_parser::lexer::tokenise;

fn main() {
    let test_file = Path::new("test.halley");
    let content = fs::read_to_string(test_file).unwrap();
    let tokens = tokenise(&content, test_file).unwrap();
    println!("{:?}", tokens);
}
