use std::collections::HashMap;
use std::rc::Weak;
use halley_lang::ast::id::Id;
use crate::name_resolution::NameResolutionError;

pub struct Scope {
    parent: Option<* const Scope>,
    definitions: HashMap<String, Id>,
    child_scopes: HashMap<String, Scope>,
}

impl Scope {

    pub fn new() -> Scope {
        Scope { parent: None, definitions: HashMap::new(), child_scopes: HashMap::new() }
    }

    fn new_child(parent: &Scope) -> Scope {
        Scope { parent: Some(parent as *const Scope), definitions: HashMap::new(), child_scopes: HashMap::new() }
    }

    pub fn add_definition(&mut self, name: String, id: Id) -> Result<(), NameResolutionError> {
        if self.definitions.contains_key(&name) {
            return Err(NameResolutionError::DuplicateDefinition { name })
        }
        self.definitions.insert(name, id);
        Ok(())
    }

    pub fn add_child(&mut self, name: String) -> Result<&mut Scope, NameResolutionError> {
        if self.child_scopes.contains_key(&name) {
            return Err(NameResolutionError::DuplicateDefinition { name })
        }
        self.child_scopes.insert(name.clone(), Scope::new_child(self));
        Ok(self.child_scopes.get_mut(&name).unwrap())
    }

    pub fn resolve(&self, path: &[&str]) -> Option<&Id> {
        if path.is_empty() {
            return None;
        }
        if path.len() == 1 {
            return self.resolve_definition(path[0]);
        }
        let scope = self.resolve_scope(path[0])?;
        scope.resolve_internal(&path[1..])
    }

    fn resolve_internal(&self, path: &[&str]) -> Option<&Id> {
        if path.is_empty() {
            return None;
        }
        if path.len() == 1 {
            return self.definitions.get(path[0]);
        }
        self.child_scopes.get(path[0]).and_then(|child| child.resolve_internal(&path[1..]))
    }

    fn resolve_scope(&self, name: &str) -> Option<&Scope> {
        if self.child_scopes.contains_key(name) {
            return self.child_scopes.get(name);
        }
        self.parent.and_then(|parent| unsafe { &*parent }.resolve_scope(name))
    }

    fn resolve_definition(&self, name: &str) -> Option<&Id> {
        if self.definitions.contains_key(name) {
            return self.definitions.get(name);
        }
        self.parent.and_then(|parent| unsafe { &*parent }.resolve_definition(name))
    }

}