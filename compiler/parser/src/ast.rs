use crate::declaration::Declaration;
use crate::statement::Statement;


pub struct Module {
    pub items: Vec<Object>,
}

pub enum Object {
    Declaration(Declaration),
    Statement(Statement)
}