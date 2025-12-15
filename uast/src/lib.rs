use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Statement {
    VarDecl(VarDecl),
    IfStatement(IfStatement),
    Unknown { source: String, span: Span },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub consequence: Box<Block>,
    pub alternative: Option<Box<Block>>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    // TODO: Expand to support Literals, Binary Operations, and Identifiers
    Raw {
        source: String,
        span: Span
    },

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VarDecl {
    pub span: Span,
    pub modifiers: Option<Vec<String>>,
    pub var_type: Option<String>,
    pub name: String,
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDecl {
    pub name: String,
}


// TODO: Implement ReturnStatement

// TODO: Implement ExpressionStatement

// TODO: Implement WhileLoop

// TODO: Implement ForLoop