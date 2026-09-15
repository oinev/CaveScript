mod declaration;
mod statement;
mod expression;
mod type_syntax;

use declaration::Declaration;
use statement::Statement;
use expression::Expression;
pub struct Module {
    pub items: Vec<Object>,
}

pub enum Object {
    Declaration(Declaration),
    Statement(Statement),
    Expression(Expression),
}