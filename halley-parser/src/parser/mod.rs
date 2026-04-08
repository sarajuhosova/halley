mod operators;

use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use halley_lang::ast::{Argument, BinaryOperator, Block, Constructor, Expression, NamedParameter, Program, Statement, Type, UnaryOperator};
use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use halley_lang::ast::id::Id;
use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use crate::parser::operators::{binary_operator_type, precedence, unary_operator_type};

pub fn parse_program(tokens: &mut VecDeque<Token>) -> Result<Program> {
    let mut statements: Vec<Statement> = Vec::new();
    while !tokens.is_empty() {
        statements.push(parse_statement(tokens)?);
    }
    Ok(Program { statements })
}

pub fn parse_statement(tokens: &mut VecDeque<Token>) -> Result<Statement> {
    match peek_single_token(tokens) {
        Some(TokenType::Fn) => parse_function(tokens),
        Some(TokenType::Data) => parse_data(tokens),
        Some(TokenType::Let) => parse_let(tokens),
        t => bail!("Unknown start of statement: {:?}", t),
    }
}

pub fn parse_function(tokens: &mut VecDeque<Token>) -> Result<Statement> {
    parse_single_token(tokens, TokenType::Fn)?;
    let id = parse_id(tokens)?;
    let arguments = parse_argument_list(tokens)?;
    parse_single_token(tokens, TokenType::Arrow)?;
    let return_type = parse_type(tokens)?;
    let body = parse_block(tokens)?;
    Ok(Statement::Function { id, arguments, return_type, body })
}

pub fn parse_data(tokens: &mut VecDeque<Token>) -> Result<Statement> {
    parse_single_token(tokens, TokenType::Data)?;
    let id = parse_id(tokens)?;
    let constructors = parse_list(tokens, TokenType::BraceOpen, TokenType::BraceClose, TokenType::Comma, parse_constructor)?;
    Ok(Statement::Data { id, constructors })
}

pub fn parse_let(tokens: &mut VecDeque<Token>) -> Result<Statement> {
    parse_single_token(tokens, TokenType::Let)?;
    let id = parse_id(tokens)?;
    parse_single_token(tokens, TokenType::Colon)?;
    let ty = parse_type(tokens)?;
    parse_single_token(tokens, TokenType::Equals)?;
    let value = parse_expression(tokens)?;
    Ok(Statement::Let { id, ty, value })
}

pub fn parse_constructor(tokens: &mut VecDeque<Token>) -> Result<Constructor> {
    let id = parse_id(tokens)?;
    let fields = parse_list(tokens, TokenType::BraceOpen, TokenType::BraceClose, TokenType::Comma, parse_argument)?;
    Ok(Constructor { id, fields })
}

pub fn parse_argument_list(tokens: &mut VecDeque<Token>) -> Result<Vec<Argument>> {
    parse_list(tokens, TokenType::ParenOpen, TokenType::ParenClose, TokenType::Comma, parse_argument)
}

pub fn parse_named_parameter_list(tokens: &mut VecDeque<Token>) -> Result<Vec<NamedParameter>> {
    parse_list(tokens, TokenType::BraceOpen, TokenType::BraceClose, TokenType::Comma, parse_named_parameter)
}

pub fn parse_block(tokens: &mut VecDeque<Token>) -> Result<Block> {
    let mut statements: Vec<Statement> = Vec::new();
    parse_single_token(tokens, TokenType::BraceOpen)?;
    while let Ok(statement) = parse_statement(tokens) {
        statements.push(statement);
        parse_single_token(tokens, TokenType::Semicolon)?;
    }
    let expression = parse_expression(tokens)?;
    parse_single_token(tokens, TokenType::BraceClose)?;
    Ok(Block { statements, expression })
}

pub fn parse_expression(tokens: &mut VecDeque<Token>) -> Result<Expression> {
    parse_speculative(tokens, parse_binop_expression)
        .or_else(|| parse_speculative(tokens, parse_unop_expression))
        .or_else(|| parse_speculative(tokens, parse_postfix_expression))
        .ok_or_else(|| anyhow!("Cannot parse expression"))
}

pub fn parse_binop_expression(tokens: &mut VecDeque<Token>) -> Result<Expression> {
    for level in BinaryOperator::iter().map(|op| precedence(op)).unique().sorted() {
        let operator_set = BinaryOperator::iter().filter(|op| precedence(*op) == level).map(|op| binary_operator_type(op)).collect::<HashSet<_>>();
        let nearest_operators = tokens.iter().enumerate().filter(|(_, t)| operator_set.contains(&t.token_type)).map(|(i, t)| (i, t.token_type)).collect::<Vec<_>>();
        for (pos_of_nearest_operator, _) in nearest_operators {
            if let Some(result) = parse_speculative(tokens, |tokens| {
                let left = Box::new(parse_exhaustive(&mut tokens.drain(0..pos_of_nearest_operator).collect::<VecDeque<_>>(), parse_expression)?);
                let operator = parse_binary_operator(tokens)?;
                let right = Box::new(parse_expression(tokens)?);
                Ok(Expression::BinOp { operator, left, right })
            }) {
                return Ok(result)
            }
        }
    }
    bail!("No binary operator found");
}

pub fn parse_unop_expression(tokens: &mut VecDeque<Token>) -> Result<Expression> {
    let operator = parse_unary_operator(tokens)?;
    let operand = Box::new(parse_postfix_expression(tokens)?);
    Ok(Expression::UnOp { operator, operand })
}

pub fn parse_postfix_expression(tokens: &mut VecDeque<Token>) -> Result<Expression> {
    parse_speculative(tokens, |tokens| {
        let data_id = parse_id(tokens)?;
        parse_single_token(tokens, TokenType::Scope)?;
        let constructor_id = parse_id(tokens)?;
        let parameters = parse_named_parameter_list(tokens)?;
        Ok(Expression::Construct { data_id, constructor_id, parameters })
    })
        .ok_or(|| anyhow!("Cannot parse postfix expression"))
        .or_else(|_| parse_simple_expression(tokens))
}

pub fn parse_simple_expression(tokens: &mut VecDeque<Token>) -> Result<Expression> {
    match peek_single_token(tokens) {
        Some(TokenType::ParenOpen) => {
            parse_single_token(tokens, TokenType::ParenOpen)?;
            let expr = parse_expression(tokens)?;
            parse_single_token(tokens, TokenType::ParenClose)?;
            Ok(expr)
        },
        Some(TokenType::Identifier) => {
            let id = parse_id(tokens)?;
            Ok(Expression::Variable { id })
        }
        Some(TokenType::Number) => {
            let value = parse_integer(tokens)?;
            Ok(Expression::Int { value })
        }
        Some(TokenType::True) | Some(TokenType::False) => {
            let value = parse_boolean(tokens)?;
            Ok(Expression::Bool { value })
        }
        t => bail!("Unknown start of expression: {:?}", t),
    }
}

pub fn parse_argument(tokens: &mut VecDeque<Token>) -> Result<Argument> {
    let id = parse_id(tokens)?;
    parse_single_token(tokens, TokenType::Colon)?;
    let ty = parse_type(tokens)?;
    Ok(Argument { id, ty })
}

pub fn parse_named_parameter(tokens: &mut VecDeque<Token>) -> Result<NamedParameter> {
    let id = parse_id(tokens)?;
    parse_single_token(tokens, TokenType::Colon)?;
    let value = parse_expression(tokens)?;
    Ok(NamedParameter { id, value })
}

pub fn parse_type(tokens: &mut VecDeque<Token>) -> Result<Type> {
    match peek_single_token(tokens) {
        Some(TokenType::Int) => {
            parse_single_token(tokens, TokenType::Int)?;
            Ok(Type::Int)
        },
        Some(TokenType::Bool) => {
            parse_single_token(tokens, TokenType::Bool)?;
            Ok(Type::Bool)
        },
        Some(TokenType::Reference) => {
            parse_single_token(tokens, TokenType::Reference)?;
            let ty = Box::new(parse_type(tokens)?);
            Ok(Type::Pointer { ty })
        },
        Some(TokenType::Identifier) => {
            let id = parse_id(tokens)?;
            Ok(Type::Variable { id })
        }
        t => bail!("Unknown start of type: {:?}", t),
    }
}

pub fn parse_id(tokens: &mut VecDeque<Token>) -> Result<Id> {
    let name = parse_single_token(tokens, TokenType::Identifier)?;
    Ok(Id::new(name.content))
}

pub fn parse_integer(tokens: &mut VecDeque<Token>) -> Result<u64> {
    let value = parse_single_token(tokens, TokenType::Number)?;
    Ok(u64::from_str(&value.content)?)
}

pub fn parse_boolean(tokens: &mut VecDeque<Token>) -> Result<bool> {
    match peek_single_token(tokens) {
        Some(TokenType::True) => {
            parse_single_token(tokens, TokenType::True)?;
            Ok(true)
        },
        Some(TokenType::False) => {
            parse_single_token(tokens, TokenType::False)?;
            Ok(false)
        },
        t => bail!("Invalid boolean: {:?}", t),
    }
}

pub fn parse_binary_operator(tokens: &mut VecDeque<Token>) -> Result<BinaryOperator> {
    if let Some(token_type) = peek_single_token(tokens) {
        if let Some(operator) = BinaryOperator::iter().find(|op| binary_operator_type(*op) == token_type) {
            parse_single_token(tokens, token_type)?;
            return Ok(operator);
        }
    }
    bail!("Invalid binary operator: {:?}", peek_single_token(tokens));
}

pub fn parse_unary_operator(tokens: &mut VecDeque<Token>) -> Result<UnaryOperator> {
    if let Some(token_type) = peek_single_token(tokens) {
        if let Some(operator) = UnaryOperator::iter().find(|op| unary_operator_type(*op) == token_type) {
            parse_single_token(tokens, token_type)?;
            return Ok(operator);
        }
    }
    bail!("Invalid binary operator: {:?}", peek_single_token(tokens));
}

fn parse_exhaustive<T, F>(tokens: &mut VecDeque<Token>, parse: F) -> Result<T> where F: FnOnce(&mut VecDeque<Token>) -> Result<T> {
    let result = parse(tokens)?;
    if !tokens.is_empty() {
        bail!("Leftover tokens: {:?}", tokens);
    }
    Ok(result)
}

fn parse_speculative<T, F>(tokens: &mut VecDeque<Token>, parse: F) -> Option<T> where F: FnOnce(&mut VecDeque<Token>) -> Result<T> {
    let mut copy = tokens.clone();
    match parse(tokens) {
        Ok(res) => Some(res),
        Err(_) => {
            tokens.clear();
            tokens.append(&mut copy);
            None
        }
    }
}

pub fn parse_list<F, E>(tokens: &mut VecDeque<Token>, open: TokenType, close: TokenType, separator: TokenType, parse_element: F) -> Result<Vec<E>>
    where F: Fn(&mut VecDeque<Token>) -> Result<E>
{
    let mut elements: Vec<E> = Vec::new();
    parse_single_token(tokens, open)?;
    if peek_single_token(tokens) == Some(close) {
        parse_single_token(tokens, close)?;
        return Ok(elements);
    }
    loop {
        elements.push(parse_element(tokens)?);
        if peek_single_token(tokens) == Some(separator) {
            parse_single_token(tokens, separator)?;
        } else if peek_single_token(tokens) != Some(close) {
            bail!("Unexpected end of list: expected {:?} or {:?}, got {:?}", separator, close, peek_single_token(tokens));
        }
        if peek_single_token(tokens) == Some(close) {
            parse_single_token(tokens, close)?;
            return Ok(elements)
        }
    }
}

fn parse_single_token(tokens: &mut VecDeque<Token>, token_type: TokenType) -> Result<Token> {
    match tokens.pop_front() {
        None => bail!("Unexpected end of input: expected {:?}", token_type),
        Some(token) if token.token_type != token_type => bail!("Unexpected token: expected {:?}, got {:?}", token_type, token.token_type),
        Some(token) => Ok(token),
    }
}

fn peek_single_token(tokens: &mut VecDeque<Token>) -> Option<TokenType> {
    tokens.front().map(|t| t.token_type)
}
