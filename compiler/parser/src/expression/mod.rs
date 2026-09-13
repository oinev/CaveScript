pub enum Expression {
    // leaf expr
    Identifier(String),
    Literal(Literal),

    // branch expr
    ArrayLiteral(Vec<Expression>),
    StructLiteral(Vec<FieldInitializer>),


    BinaryExpr {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
    },

    CallExpr {
    callee: Box<Expression>,
    arguments: Vec<Expression>,
    },

    MemberExpr {
    object: Box<Expression>,
    member: String,
    },
    IndexExpr {
    target: Box<Expression>,
    index: Box<Expression>,
    },
}

pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
}

pub enum BinaryOperator {
    Add,
    Substract,
    Multiply,
    Divide,


    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    And,
    Or,
    // to be added
}

pub struct FieldInitializer {
    pub name: String,
    pub value: Expression,
}