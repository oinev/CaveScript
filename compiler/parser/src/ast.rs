use crate::declaration::Declaration;
use crate::statement::Statement;
use crate::expression::Expression;

pub struct Module {
    pub items: Vec<Object>,
}

pub enum Object {
    Declaration(Declaration),
    Statement(Statement),
    Expression(Expression),
}