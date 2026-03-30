pub mod id;

use uuid::Uuid;
use crate::ast::id::Id;

#[derive(Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Function { id: Id, arguments: Vec<Argument>, return_type: Type, body: Block },
    Assign { id: Id, value: Expression },
    Let { id: Id, ty: Type, value: Expression }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub expression: Expression,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Variable { id: Id },
    UnOp { operator: UnaryOperator, operand: Box<Expression> },
    BinOp { operator: BinaryOperator, left: Box<Expression>, right: Box<Expression> },
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub id: Id,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub enum Type {
    Bool, Int,
    Pointer { ty: Box<Type> },
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Neg, Not,
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Eq, Neq,
    Lt, Gt, Le, Ge,
    And, Or,
    Add, Sub, Mul, Div, Mod,
}