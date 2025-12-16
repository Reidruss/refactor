use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Statement {
    DeclStmt(DeclStmt),
    IfStatement(IfStatement),
    Unknown { source: String, span: Span },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReturnType {
    IntegerLiteral,
    RealLiteral,
    Identifier,
    CharacterLiteral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

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
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub consequence: Box<Block>,
    pub alternative: Option<Box<Block>>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div, Equal, NotEqual,
    GreaterThan, LessThan, GreaterThanEq,
    LessThanEQ,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArithmeticOperator {
    Add, Sub, Mul, Div, Mod
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComparisonOperator {
    Equal, NotEqual,
    GreaterThan, LessThan,
    GreaterThanEqual, LessThanEqual
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogicalOperator {
    And, Or, Xor, CondAnd, CondOr
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BitwiseOperator {
    And, Or, Xor, LeftShift, RightShift, UnsignedRightShift
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BinaryOp {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    // TODO: Expand to support Literals, Binary Operations, and Identifiers
    Identifier(String),
    Literal(Literal),
    BinaryOp(BinaryOp),
    Raw {
        source: String,
        span: Span
    },
    Literals {

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeclStmt {
    pub modifiers : Option<Vec<String>>,
    pub var_decl : VarDecl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDecl {
    pub name: String,
}

pub struct ReturnStatement {
    // Need to somehow store the type
    // Temporary but could work
    pub return_type: ReturnType,
    pub value: String,
}

// TODO: Implement ExpressionStatement



// TODO: Implement WhileLoop

// TODO: Implement ForLoop