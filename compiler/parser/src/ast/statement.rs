use super::{declaration::Declaration, expression::Expression};

pub enum Statement {
    ExpressionStmt(Expression),
    DeclarationStmt(Declaration),

    AssignmentStmt {
        target: Expression,
        operator: AssignmentOperator,
        value: Expression,
    },

    ReturnStmt { value: Expression },

    Block { statements: Vec<Statement> },
}


pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubstractAssign,
    MultiplyAssign,
    DivideAssign,
}
