use uast::TopLevel;

pub mod refactorings {
    pub mod rename_variable;
}

pub use refactorings::rename_variable::RenameVariable;

pub trait Refactoring {
    fn apply(&self, uast: &mut TopLevel);
}
