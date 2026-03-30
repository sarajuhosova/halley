pub struct Program {
    pub statements: Vec<Statement>,
    pub expression: Expression,
}

pub enum Statement {
    Assign { name: String, value: Expression },
    Let { name: String, ty: Type, value: Expression }
}

pub enum Expression {
    Variable { name: String },
    UnOp { operator: UnaryOperator, operand: Box<Expression> },
    BinOp { operator: BinaryOperator, left: Box<Expression>, right: Box<Expression> },
}

pub enum Type {
    Bool, Int,
}

pub enum UnaryOperator {
    Neg, Not,
}

pub enum BinaryOperator {
    Eq, Neq,
    Lt, Gt, Le, Ge,
    And, Or,
    Add, Sub, Mul, Div, Mod,
}