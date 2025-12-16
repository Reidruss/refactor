use serde::{Deserialize, Serialize};

// --- Core Primitives ---
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
}

// --- Operators ---
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div,
    Equal, NotEqual,
    GreaterThan, LessThan,
    GreaterThanEqual, LessThanEqual,
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
pub enum UnaryOperator {
    Negate,       // -x
    Not,          // !x
    PreIncrement, // ++x
    PostIncrement,// x++
    PreDecrement, // --x
    PostDecrement,// x--
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssignmentOperator {
    Assign,       // =
    AddAssign,    // +=
    SubAssign,    // -=
    MulAssign,    // *=
    DivAssign,    // /=
}

// --- Expressions ---
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BinaryOp {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>
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
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Assignment(Assignment),
    Raw {
        source: String,
        span: Span
    },
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
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeclStmt {
    pub modifiers : Option<Vec<String>>,
    pub var_decl : VarDecl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReturnStatement {
    // Need to somehow store the type
    // Temporary but could work
    pub return_type: ReturnType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Statement {
    DeclStmt(DeclStmt),
    IfStatement(IfStatement),
    Unknown { source: String, span: Span },
    WhileLoop(WhileLoop), // Added this
    ForLoop(ForLoop),     // Added this
    ReturnStatement(ReturnStatement), // Added this
    // TODO: Implement ExpressionStatement
}

// --- Top-Level Declarations ---
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReturnType {
    IntegerLiteral,
    RealLiteral,
    Identifier,
    CharacterLiteral,
}