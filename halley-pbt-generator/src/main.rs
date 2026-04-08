use std::collections::HashMap;
use std::fs;
use std::path::Path;
use halley_core::name_resolution::{Definition, NameResolver};
use halley_core::name_resolution::resolve::resolve_names;
use halley_core::parse_halley_file;
use halley_lang::ast::{Expression, NamedParameter, Statement, Type, UnaryOperator};
use halley_lang::ast::id::Id;

struct ReferenceCache {
    references: HashMap<Type, Vec<Expression>>
}

impl ReferenceCache {
    pub fn add_reference(&mut self, ty: Type, value: Expression) {
        if !self.references.contains_key(&ty) {
            self.references.insert(ty.clone(), Vec::new());
        }
        self.references.get_mut(&ty).unwrap().push(value);
    }
}

fn count(ty: &Type, size: usize, resolver: &NameResolver) -> usize {
    match ty {
        Type::Bool => if size == 1 { 2 } else { 0 },
        Type::Int => if size == 0 { 0 } else if size <= 2 { 1 } else { 2 },
        Type::Variable { id, .. } => {
            let definition = resolver.get_definition(resolver.resolve(id).unwrap()).unwrap();
            match definition {
                Definition::Data(Statement::Data { constructors, .. }) => {
                    if size == 0 {
                        0
                    } else {
                        constructors.iter().map(|constructor| {
                            let field_types = constructor.fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
                            count_product(&field_types, size - 1, resolver)
                        }).sum()
                    }
                }
                _ => panic!("Invalid variable in type: {} -> {:?}", id.name, definition)
            }
        },
        Type::Pointer { .. } => todo!(),
    }
}

fn count_product(types: &[&Type], size: usize, resolver: &NameResolver) -> usize {
    if types.len() == 0 {
        if size == 0 { 1 } else { 0 }
    } else if types.len() == 1 {
        count(&types[0], size, resolver)
    } else {
        let mut result = 0;
        for i in 0..=size {
            let j = size - i;
            result += count(&types[0], i, resolver) * count_product(&types[1..], j, resolver)
        }
        result
    }
}

fn generate(ty: &Type, index: usize, resolver: &NameResolver) -> Expression {
    let mut c = 0;
    for size in 0usize.. {
        let increment = count(ty, size, resolver);
        if c + increment > index {
            let index = index - c;
            return generate_sized(ty, size, index, resolver).unwrap();
        }
        c += increment;
    }
    unreachable!()
}

fn generate_sized(ty: &Type, size: usize, index: usize, resolver: &NameResolver) -> Option<Expression> {
    match ty {
        Type::Bool => match (index, size) {
            (0, 1) => Some(Expression::Bool { value: false }),
            (1, 1) => Some(Expression::Bool { value: true }),
            _ => None,
        },
        Type::Int => match (index, size) {
            (0, 1) => Some(Expression::Int { value: 0 }),
            (0, 2) => Some(Expression::Int { value: 1 }),
            (0, n) => Some(Expression::Int { value: (n - 1) as u64 }),
            (1, n) => Some(Expression::UnOp { operator: UnaryOperator::Neg, operand: Box::new(Expression::Int { value: (n - 2) as u64 }) }),
            _ => None,
        }
        Type::Variable { id, .. } => {
            let definition = resolver.get_definition(resolver.resolve(id).unwrap()).unwrap();
            match definition {
                Definition::Data(Statement::Data { id: data_id, constructors, .. }) => {
                    if size == 0 {
                        None
                    } else {
                        let mut c = 0;
                        for constructor in constructors {
                            let field_types = constructor.fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
                            let constructor_count = count_product(&field_types, size - 1, resolver);
                            if c + constructor_count > index {
                                let fields = generate_sized_product(&field_types, size - 1, index - c, resolver)?;
                                let parameters = constructor.fields.iter().zip(fields)
                                    .map(|(field, value)| NamedParameter { id: Id::new(field.id.name.clone()), value }).collect::<Vec<_>>();
                                return Some(Expression::Construct { data_id: Id::new(data_id.name.clone()), constructor_id: Id::new(constructor.id.name.clone()), parameters })
                            }
                            c += constructor_count;
                        }
                        None
                    }
                }
                _ => panic!("Invalid variable in type: {} -> {:?}", id.name, definition)
            }
        },
        Type::Pointer { .. } => todo!(),
    }
}

fn generate_sized_product(types: &[&Type], size: usize, index: usize, resolver: &NameResolver) -> Option<Vec<Expression>> {
    if types.len() == 0 {
        if size == 0 { Some(Vec::new()) } else { None }
    } else if types.len() == 1 {
        generate_sized(&types[0], size, index, resolver).map(|e| vec![e])
    } else {
        let mut c = 0;
        for i in 0..=size {
            let j = size - i;
            let count_left = count(&types[0], i, resolver);
            let count_right = count_product(&types[1..], j, resolver);
            if c + count_left * count_right > index {
                let index = index - c;
                let index_left = index / count_right;
                let index_right = index % count_right;
                let left = generate_sized(&types[0], i, index_left, resolver)?;
                let mut right = generate_sized_product(&types[1..], j, index_right, resolver)?;
                right.insert(0, left);
                return Some(right)
            }
            c += count_left * count_right;
        }
        None
    }
}

fn main() {

    let test_file = Path::new("test.halley");
    let program = parse_halley_file(test_file).unwrap();
    let resolver = resolve_names(&program).unwrap();

    let function_definitions = resolver.all_definitions().iter().filter(|def| if let Definition::Function { .. } = def { true } else {false}).map(|d| *d).collect::<Vec<_>>();

    for function in function_definitions {
        if let Definition::Function(Statement::Function { id, arguments, .. }) = function {
            println!("fn {}", id.name);
            for argument in arguments {
                println!(" - {argument}");
                for i in 0..5 {
                    let expr = generate(&argument.ty, i, &resolver);
                    println!("   * {expr}");
                }
            }
        }
    }
}
