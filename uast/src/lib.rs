use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Core Primitives ---
pub type Metadata = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Char(char),
}

// --- Operators ---
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
    Xor,
    CondAnd,
    CondOr,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Not,
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

// --- Expressions ---
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BinaryOp {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnaryOp {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Assignment {
    pub left: Box<Expression>,
    pub operator: AssignmentOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invocation {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemberAccess {
    pub expression: Box<Expression>,
    pub member: String,
    pub member_span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    Identifier(String, Span),
    Literal(Literal),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Assignment(Assignment),
    Invocation(Invocation),
    MemberAccess(MemberAccess),
    Raw { source: String, span: Span },
}

// --- Statements ---
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
pub struct WhileLoop {
    pub condition: Box<Expression>,
    pub body: Box<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForLoop {
    pub initializer: Option<Box<Statement>>,
    pub condition: Option<Box<Expression>>,
    pub update: Option<Box<Expression>>,
    pub body: Box<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VarDecl {
    pub span: Span,
    pub modifiers: Option<Vec<String>>,
    pub var_type: Option<String>,
    pub name: String,
    pub name_span: Span,
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeclStmt {
    pub modifiers: Option<Vec<String>>,
    pub var_decls: Vec<VarDecl>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReturnStatement {
    // Need to somehow store the type if inferred, or just rely on value expression
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Statement {
    DeclStmt(DeclStmt),
    IfStatement(IfStatement),
    Unknown { source: String, span: Span },
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

// --- Top-Level Declarations ---
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopLevel {
    Class(ClassDef),
    Function(FunctionDef),
    Module(ModuleDef),
    Statement(Statement),
    Unknown { source: String, span: Span },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassDef {
    pub name: String,
    pub span: Span,
    pub body: Option<Vec<TopLevel>>,
    pub modifiers: Option<Vec<String>>,
    pub annotations: Option<Vec<Annotation>>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub span: Span,
    pub body: Option<Vec<FunctionBodyItems>>,
    pub modifiers: Option<Vec<String>>,
    pub parameters: Option<Vec<VarDecl>>,
    pub return_type: Option<String>,
    pub annotations: Option<Vec<Annotation>>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleDef {
    pub name: String,
    pub body: Vec<TopLevel>,
    pub span: Span,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Annotation {
    pub name: String,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FunctionBodyItems {
    Block(Block),
    TopLevel(TopLevel),
    Expression(Expression),
}
