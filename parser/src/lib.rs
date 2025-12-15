use tree_sitter::{Language, Parser, Tree};

pub struct GenericParser {
    inner: Parser,
}

impl GenericParser {
    pub fn new(language: Language) -> Self {
        let mut inner: Parser = Parser::new();
        inner.set_language(language).expect("Error Loading Gramar");
        Self { inner }
    }

    pub fn parse(&mut self, text: &str) -> Tree {
        self.inner.parse(text, None).expect("Parse failed")
    }
}
