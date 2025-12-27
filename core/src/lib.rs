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
    fn apply(&self, uast: &TopLevel) -> Vec<TextEdit>;
}

pub fn apply_refactoring(source: &str, edits: Vec<TextEdit>) -> String {
    let mut new_source = source.to_string();

    edits.into_iter().rev().for_each(|edit| {
        if edit.end <= new_source.len() && edit.start <= edit.end {
            new_source.replace_range(edit.start..edit.end, &edit.replacement);
        }
    });

    new_source
}
