enum ASTNode {
    Program(Vec<ASTNode>),
    Function(String, Vec<String>, Vec<ASTNode>),
    Call(String, Vec<ASTNode>),
    Number(i32),
    String(String),
    Identifier(String),
    If(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>),
    While(Box<ASTNode>, Box<ASTNode>),
    Assign(String, Box<ASTNode>),
    Return(Box<ASTNode>),
}

