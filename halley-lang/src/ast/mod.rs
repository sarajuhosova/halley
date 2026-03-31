pub mod id;

use std::fmt::{Display, Formatter};
use std::mem;
use std::os::linux::raw::stat;
use std::os::unix::fs::lchown;
use uuid::Uuid;
use crate::ast::id::Id;

#[derive(Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Function { id: Id, arguments: Vec<Argument>, return_type: Type, body: Block },
    Data { id: Id, constructors: Vec<Constructor> },
    Assign { id: Id, value: Expression },
    Let { id: Id, ty: Type, value: Expression }
}

#[derive(Clone, Debug)]
pub struct Constructor {
    pub id: Id,
    pub fields: Vec<Argument>,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub expression: Expression,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Variable { id: Id },
    Int { value: u64 },
    Bool { value: bool },
    Construct { data_id: Id, constructor_id: Id, parameters: Vec<NamedParameter> },
    UnOp { operator: UnaryOperator, operand: Box<Expression> },
    BinOp { operator: BinaryOperator, left: Box<Expression>, right: Box<Expression> },
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub id: Id,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub struct NamedParameter {
    pub id: Id,
    pub value: Expression,
}

#[derive(Clone, Debug)]
pub enum Type {
    Bool, Int,
    Pointer { ty: Box<Type> },
    Variable { id: Id },
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Neg, Not,
    Reference, Dereference,
    UnaryOperatorsEnd,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Eq, Neq,
    Lt, Gt, Le, Ge,
    And, Or,
    Add, Sub, Mul, Div, Mod,
    BinaryOperatorsEnd
}

impl Expression {
    pub fn parenthesised(&self) -> String {
        match self {
            Expression::Variable { .. } | Expression::Int { .. } | Expression::Bool { .. } | Expression::UnOp { .. } | Expression::Construct { .. } =>
                format!("{}", self),
            _ => format!("({})", self),
        }
    }
}

impl UnaryOperator {
    pub fn iter() -> impl Iterator<Item = UnaryOperator> {
        (0..UnaryOperator::UnaryOperatorsEnd as u8).map(|i| unsafe { mem::transmute(i) })
    }
}

impl BinaryOperator {
    pub fn iter() -> impl Iterator<Item = BinaryOperator> {
        (0..BinaryOperator::BinaryOperatorsEnd as u8).map(|i| unsafe { mem::transmute(i) })
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.statements.iter().map(|statement| statement.to_string()).collect::<Vec<_>>().join("\n\n"))
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Function { id, arguments, return_type, body } => {
                let arguments = arguments.iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "fn {}({}) -> {} {}", id, arguments, return_type, body)
            },
            Statement::Data { id, constructors } => {
                let constructors = constructors.iter().map(|con| con.to_string().lines().map(|line| String::from("     ") + line).collect::<Vec<_>>().join("\n")).collect::<Vec<_>>().join(",\n");
                write!(f, "data {} {{\n{}\n}}", id, constructors)
            }
            Statement::Assign { id, value } => write!(f, "{} = {}", id, value),
            Statement::Let { id, ty, value } => write!(f, "let {}: {} = {}", id, ty, value),
        }
    }
}

impl Display for Constructor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fields = self.fields.iter().map(|field| field.to_string()).collect::<Vec<_>>().join(", ");
        write!(f, "{} {{ {} }}", self.id, fields)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        for statement in &self.statements {
            write!(f, "{};\n", statement.to_string().lines().map(|line| String::from("    ") + line).collect::<Vec<_>>().join("\n"))?;
        }
        write!(f, "{}\n", self.expression.to_string().lines().map(|line| String::from("    ") + line).collect::<Vec<_>>().join("\n"))?;
        write!(f, "}}")
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Variable { id } => write!(f, "{}", id),
            Expression::Int { value } => write!(f, "{}", value),
            Expression::Bool { value } => write!(f, "{}", value),
            Expression::Construct { data_id, constructor_id, parameters } => {
                let parameters = parameters.iter().map(|param| param.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "{}::{} {{ {} }}", data_id, constructor_id, parameters)
            },
            Expression::UnOp { operator, operand } => write!(f, "{}{}", operator, operand.parenthesised()),
            Expression::BinOp { operator, left, right } => write!(f, "{} {} {}", left.parenthesised(), operator, right.parenthesised()),
        }
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.ty)
    }
}

impl Display for NamedParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.value)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Bool => write!(f, "Bool"),
            Type::Int => write!(f, "Int"),
            Type::Pointer { ty } => write!(f, "&{}", *ty),
            Type::Variable { id } => write!(f, "{}", id),
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Neg => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::Reference => write!(f, "&"),
            UnaryOperator::Dereference => write!(f, "*"),
            UnaryOperator::UnaryOperatorsEnd => panic!("Invalid operator"),
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Eq => write!(f, "=="),
            BinaryOperator::Neq => write!(f, "!="),
            BinaryOperator::Lt => write!(f, "<"),
            BinaryOperator::Gt => write!(f, ">"),
            BinaryOperator::Le => write!(f, "<="),
            BinaryOperator::Ge => write!(f, ">="),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Mod => write!(f, "%"),
            BinaryOperator::BinaryOperatorsEnd => panic!("Invalid operator"),
        }
    }
}