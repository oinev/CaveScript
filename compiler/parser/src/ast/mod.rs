pub mod declaration;
pub mod statement;
pub mod expression;
pub mod type_syntax;

use declaration::Declaration;
use statement::Statement;
use expression::Expression;
pub struct Module {
    pub objects: Vec<Statement>,
}

