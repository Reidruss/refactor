use uast::{Statement, VarDecl, Span};
use tree_sitter::Node;

pub fn lower_statement(node: Node, source: &[u8]) -> Statement {
    match node.kind() {
        "global_statement" | "local_declaration_statement" => {
             // Unwrap wrappers
             if let Some(child) = node.named_child(0) {
                 lower_statement(child, source)
             } else {
                 Statement::Unknown {
                    source: node.utf8_text(source).unwrap().to_string(),
                    span: Span { start: node.start_byte(), end: node.end_byte() },
                }
             }
        },
        "local_declaration" | "variable_declaration" => {
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