use std::collections::HashMap;
use std::fs;
use std::path::Path;
use halley_core::name_resolution::{Definition, NameResolver};
use halley_core::name_resolution::resolve::resolve_names;
use halley_core::parse_halley_file;
use halley_lang::ast::{Expression, Type};

struct ReferenceCache {
    references: HashMap<Type, Vec<Expression>>
}

impl ReferenceCache {
    pub fn add_reference(&mut self, ty: Type, value: Expression) {
        if !self.references.contains_key(&ty) {
            self.references.insert(ty, Vec::new())
        }
        self.references.get(&ty).insert(value);
    }
}

fn sized(ty: &Type, name_resolver: &NameResolver, reference_cache: &mut ReferenceCache, result: &mut Vec<Expression>) {

}

fn generate(ty: &Type, resolver: &NameResolver) -> Expression {
    todo!()
}

fn main() {

    let test_file = Path::new("test.halley");
    let program = parse_halley_file(test_file).unwrap();
    let resolver = resolve_names(&program).unwrap();

    let data_definitions = resolver.all_definitions().iter().filter(|def| if let Definition::Data { .. } = def { true } else { false }).map(|d| *d).collect::<Vec<_>>();
    println!("{:?}", data_definitions);
    
    // let test_value = generate();

    println!("Hello, world!");
}
