pub mod resolve;
mod scope;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use halley_lang::ast::id::Id;

pub struct NameResolver {
    mapping: HashMap<Id, Id>,
}

impl NameResolver {
    
    pub fn new() -> NameResolver {
        NameResolver { mapping: HashMap::new() }
    }
    
    pub fn add_mapping(&mut self, id: Id, definition: Id) {
        self.mapping.insert(id, definition);
    }

    pub fn resolve(&self, id: Id) -> Option<&Id> {
        self.mapping.get(&id)
    }
    
}

#[derive(Debug)]
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