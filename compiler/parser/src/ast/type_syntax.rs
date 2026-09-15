pub enum TypeSyntax {
    PrimitiveType(PrimitiveType),
    
    ArrayType {
        element_type: Box<TypeSyntax>
    },

    NamedType {   // <- user defined types
        name: String
    },
}

pub enum PrimitiveType {
    IntTy,
    FloatTy,
    BoolTy,
    CharTy,
    StringTy,
}
