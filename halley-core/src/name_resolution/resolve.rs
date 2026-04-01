use std::collections::HashMap;
use halley_lang::ast::id::Id;
use halley_lang::ast::{Argument, Block, Constructor, Expression, Program, Statement, Type};
use crate::name_resolution::{Definition, NameResolutionError, NameResolver};
use crate::name_resolution::scope::Scope;

pub fn resolve_names(program: &'_ Program) -> Result<NameResolver<'_>, NameResolutionError> {
    let mut resolver: NameResolver = NameResolver::new();
    let mut root_scope: Scope = Scope::new();
    
    // Insert top-level definitions
    for statement in &program.statements {
        if let Some((id, definition)) = get_definition(statement) {
            // TODO maybe collect these errors and display all at once
            root_scope.add_definition(id.name.clone(), id.clone())?;
            resolver.add_definition(id.clone(), definition);
        }
    }
    
    // Resolve data types first to add constructors
    for statement in &program.statements {
        if let Statement::Data { .. } = statement {
            resolve_statement_names(statement, &mut resolver, &mut root_scope)?;
        }
    }
    
    // Resolve functions
    for statement in &program.statements {
        if let Statement::Function { .. } = statement {
            resolve_statement_names(statement, &mut resolver, &mut root_scope)?;
        }
    }
    
    Ok(resolver)
}

fn resolve_statement_names<'a>(statement: &'a Statement, resolver: &mut NameResolver<'a>, parent_scope: &mut Scope) -> Result<(), NameResolutionError> {
    match statement {
        Statement::Function { id, arguments, return_type, body } => {
            for argument in arguments {
                resolve_argument_names(argument, resolver, parent_scope)?;
            }
            resolve_type_names(return_type, resolver, parent_scope)?;
            let body_scope = parent_scope.add_child(id.name.clone())?;
            for argument in arguments {
                body_scope.add_definition(argument.id.name.clone(), argument.id.clone())?;
            }
            resolve_block_names(body, resolver, body_scope)?;
        }

        Statement::Data { id, constructors } => {
            let data_scope = parent_scope.add_child(id.name.clone())?;
            for constructor in constructors {
                resolve_constructor_names(constructor, resolver, data_scope)?;
            }
        }

        Statement::Let { id, ty, value } => {
            resolve_type_names(ty, resolver, parent_scope)?;
            let scope = parent_scope.add_child(id.name.clone())?;
            resolve_expression_names(value, resolver, scope)?;
        }

        Statement::Assign { .. } => { todo!() }
    }
    Ok(())
}

fn resolve_constructor_names<'a>(constructor: &'a Constructor, resolver: &mut NameResolver<'a>, parent_scope: &mut Scope) -> Result<(), NameResolutionError> {
    parent_scope.add_definition(constructor.id.name.clone(), constructor.id.clone())?;
    for field in &constructor.fields {
        resolve_argument_names(field, resolver, parent_scope)?;
    }
    let scope = parent_scope.add_child(constructor.id.name.clone())?;
    for field in &constructor.fields {
        scope.add_definition(field.id.name.clone(), field.id.clone())?;
    }
    Ok(())
}

fn resolve_block_names<'a>(block: &'a Block, resolver: &mut NameResolver<'a>, parent_scope: &mut Scope) -> Result<(), NameResolutionError> {
    let mut scope = parent_scope;
    for (i, statement) in block.statements.iter().enumerate() {
        resolve_statement_names(statement, resolver, scope)?;
        scope = scope.add_child(i.to_string())?;
        if let Some((id, definition)) = get_definition(statement) {
            scope.add_definition(id.name.clone(), id.clone())?;
            resolver.add_definition(id.clone(), definition);
        }
    }
    resolve_expression_names(&block.expression, resolver, scope)?;
    Ok(())
}

fn resolve_argument_names<'a>(argument: &'a Argument, resolver: &mut NameResolver<'a>, parent_scope: &mut Scope) -> Result<(), NameResolutionError> {
    resolver.add_definition(argument.id.clone(), Definition::Argument(argument));
    resolve_type_names(&argument.ty, resolver, parent_scope)
}

fn resolve_expression_names(expression: &Expression, resolver: &mut NameResolver, parent_scope: &mut Scope) -> Result<(), NameResolutionError> {
    match expression {
        Expression::Variable { id } => {
            let resolved = parent_scope.resolve(&[&id.name]).ok_or_else(|| NameResolutionError::UnboundIdentifier { id: id.clone() })?;
            resolver.add_mapping(id.clone(), resolved.clone());
        }

        Expression::Construct { data_id, constructor_id, parameters } => {
            let resolved_data = parent_scope.resolve(&[&data_id.name]).ok_or_else(|| NameResolutionError::UnboundIdentifier { id: data_id.clone() })?;
            resolver.add_mapping(data_id.clone(), resolved_data.clone());
            let resolved_constructor = parent_scope.resolve(&[&data_id.name, &constructor_id.name]).ok_or_else(|| NameResolutionError::UnboundIdentifier { id: constructor_id.clone() })?;
            resolver.add_mapping(constructor_id.clone(), resolved_constructor.clone());
            for parameter in parameters {
                // TODO check if parameter name exists
                resolve_expression_names(&parameter.value, resolver, parent_scope)?;
            }
        }

        Expression::UnOp { operand, .. } => {
            resolve_expression_names(&*operand, resolver, parent_scope)?;
        }
        Expression::BinOp { left, right, .. } => {
            resolve_expression_names(&*left, resolver, parent_scope)?;
            resolve_expression_names(&*right, resolver, parent_scope)?;
        }

        Expression::Int { .. } => {}
        Expression::Bool { .. } => {}
    }
    Ok(())
}

fn resolve_type_names(ty: &Type, resolver: &mut NameResolver, parent_scope: &mut Scope) -> Result<(), NameResolutionError> {
    match ty {
        Type::Bool => {}
        Type::Int => {}
        Type::Pointer { ty } => {
            resolve_type_names(ty, resolver, parent_scope)?;
        }
        Type::Variable { id } => {
            let resolved = parent_scope.resolve(&[&id.name]).ok_or_else(|| NameResolutionError::UnboundIdentifier { id: id.clone() })?;
            resolver.add_mapping(id.clone(), resolved.clone());
        }
    }
    Ok(())
}

fn get_definition(statement: &Statement) -> Option<(&Id, Definition)> {
    match statement {
        Statement::Function { id, .. } => Some((id, Definition::Function(&statement))),
        Statement::Data { id, .. } => Some((id, Definition::Data(&statement))),
        Statement::Let { id, .. } => Some((id, Definition::Let(&statement))),
        
        Statement::Assign { .. } => None,
    }
}