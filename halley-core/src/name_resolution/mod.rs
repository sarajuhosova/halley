pub mod resolve;
mod scope;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use halley_lang::ast::id::Id;
use halley_lang::ast::{Argument, Constructor, Statement};

pub struct NameResolver<'a> {
    mapping: HashMap<Id, Id>,
    definitions: HashMap<Id, Definition<'a>>,
}

impl<'a> NameResolver<'a> {
    
    pub fn new() -> NameResolver<'a> {
        NameResolver { mapping: HashMap::new(), definitions: HashMap::new() }
    }

    pub fn add_definition(&mut self, id: Id, definition: Definition<'a>) {
        self.definitions.insert(id, definition);
    }
    
    pub fn add_mapping(&mut self, id: Id, definition: Id) {
        self.mapping.insert(id, definition);
    }
    
    pub fn all_definitions(&self) -> Vec<&Definition<'a>> {
        self.definitions.iter().map(|(_, d)| d).collect()
    }

    pub fn get_definition(&self, id: &Id) -> Option<&Definition<'a>> {
        self.definitions.get(id)
    }

    pub fn resolve(&self, id: &Id) -> Option<&Id> {
        self.mapping.get(id)
    }
    
}

#[derive(Debug, Clone)]
pub enum Definition<'a> {
    Function(&'a Statement),
    Data(&'a Statement),
    Constructor(&'a Constructor),
    Argument(&'a Argument),
    Let(&'a Statement),
}

#[derive(Debug, Clone)]
pub enum NameResolutionError {
    DuplicateDefinition { name: String },
    UnboundIdentifier { id: Id },
}

impl Display for NameResolutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NameResolutionError::DuplicateDefinition { name } => write!(f, "'{}' is already defined", name),
            NameResolutionError::UnboundIdentifier { id } => write!(f, "'{}' is not defined", id),
        }
    }
}

impl std::error::Error for NameResolutionError {}