use uast::TopLevel;

pub mod refactorings {
    pub mod rename_variable;
}

pub use refactorings::rename_variable::RenameVariable;

#[derive(Debug, Clone)]
pub struct TextEdit {
    pub start: usize,
    pub end: usize,
    pub replacement: String,
}

pub trait Refactoring {
    fn apply(&self, uast: &mut TopLevel);
    fn generate_edits(&self, uast: &TopLevel) -> Vec<TextEdit>;
}
