use tree_sitter::{Language, Parser};

pub struct GenericParser {
    inner: Parser,
}

impl GenericParser {
    pub fn new(language: Language) -> Result<Self, tree_sitter::LanguageError> {
        let mut inner = Parser::new();
        inner.set_language(language)?;
        Ok(Self { inner })
    }
}