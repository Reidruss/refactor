use uast::{Statement, VarDecl, Span};
use tree_sitter::Node;

pub fn lower_statement(node: Node, source: &[u8]) -> Statement {
    match node.kind() {
        "local_declaration" => {
            // Logic to extract variable name and value from children
            // This requires inspecting the tree-sitter-c-sharp grammar
            Statement::VarDecl(VarDecl {
                name: "extracted_name".to_string(), // Placeholder
                value: None,
                span: Span {
                    start: node.start_byte(),
                    end: node.end_byte()
                },
            })
        },
        _ => Statement::Unknown {
            source: node.utf8_text(source).unwrap().to_string(),
            span: Span { start: node.start_byte(), end: node.end_byte() },
        }
    }
}