// uast/src/lib.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    VarDecl(VarDecl),
    // Use this for everything else for now so you don't get stuck
    Unknown { source: String, span: Span },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarDecl {
    pub name: String,
    pub value: Option<String>, // Keep expressions as strings for the first pass
    pub span: Span,
}

