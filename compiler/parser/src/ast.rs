use crate::statement::Statement;
use crate::declaration::Declaration;


pub struct Module {
    pub items: Vec<Object>,
}

enum Object {
    Declaration(Declaration),
    Statement(Statement)
}