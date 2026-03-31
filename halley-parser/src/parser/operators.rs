use halley_lang::ast::{BinaryOperator, UnaryOperator};
use crate::lexer::token_type::TokenType;
use crate::lexer::token_type::TokenType::TokenTypesEnd;

pub fn precedence(operator: BinaryOperator) -> u32 {
    match operator {
        BinaryOperator::Eq | BinaryOperator::Neq => 50,
        BinaryOperator::Lt | BinaryOperator::Gt | BinaryOperator::Le | BinaryOperator::Ge => 60,
        BinaryOperator::Or => 70,
        BinaryOperator::And => 80,
        BinaryOperator::Add | BinaryOperator::Sub => 90,
        BinaryOperator::Mul | BinaryOperator::Div | BinaryOperator::Mod => 100,
        BinaryOperator::BinaryOperatorsEnd => panic!("Invalid operator"),
    }
}

pub fn unary_operator_type(operator: UnaryOperator) -> TokenType {
    match operator {
        UnaryOperator::Neg => TokenType::Minus,
        UnaryOperator::Not => TokenType::Not,
        UnaryOperator::Reference => TokenType::Reference,
        UnaryOperator::Dereference => TokenType::Asterisk,
        UnaryOperator::UnaryOperatorsEnd => panic!("Invalid operator"),
    }
}

pub fn binary_operator_type(operator: BinaryOperator) -> TokenType {
    match operator {
        BinaryOperator::Eq => TokenType::DoubleEquals,
        BinaryOperator::Neq => TokenType::NotEquals,
        BinaryOperator::Lt => TokenType::LessThan,
        BinaryOperator::Gt => TokenType::GreaterThan,
        BinaryOperator::Le => TokenType::LessThanEquals,
        BinaryOperator::Ge => TokenType::GreaterThanEquals,
        BinaryOperator::And => TokenType::And,
        BinaryOperator::Or => TokenType::Or,
        BinaryOperator::Add => TokenType::Plus,
        BinaryOperator::Sub => TokenType::Minus,
        BinaryOperator::Mul => TokenType::Asterisk,
        BinaryOperator::Div => TokenType::Divide,
        BinaryOperator::Mod => TokenType::Modulo,
        BinaryOperator::BinaryOperatorsEnd => panic!("Invalid operator"),
    }
}