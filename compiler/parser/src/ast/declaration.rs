use super::{expression::Expression, statement::Statement, type_syntax::TypeSyntax};

pub enum Declaration {
    VariableDecl {
        mutable: bool,
        name: String,
        type_annotation: Option<TypeSyntax>,
        initializer: Expression,
    },

    FunctionDecl(FunctionDeclaration),

    StructDecl {
        name: String,
        fields: Vec<FieldDeclaration>,
    },

    EnumDecl {
        name: String,
        variants: EnumVariants,
    },
    CellDecl {
        name: String,
        underlying_type: TypeSyntax
    },
    ImplDecl {
    target: TypeSyntax, // <- must always be NamedType
    methods: Vec<FunctionDeclaration>,
},
}

pub struct NameAndType {
    pub name: String,
    pub type_annotation: TypeSyntax,
}

pub struct FieldDeclaration {
    pub name: String,
    pub type_annotation: TypeSyntax,
}

pub struct EnumVariants {
    name: String,
}

pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<NameAndType>,
    pub return_type: Option<TypeSyntax>,
    pub body: Box<Statement>,
}

// should you allow declarations inside statement ? if not, that means variableDecl is a statement and not a declaration