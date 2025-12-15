use tree_sitter::Node;
use uast::{Block, DeclStmt, Expression, IfStatement, Span, Statement, VarDecl};

fn extract_modifiers(node: Node, source: &[u8]) -> Option<Vec<String>> {
    let mut modifiers = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "modifier" {
            if let Ok(text) = child.utf8_text(source) {
                modifiers.push(text.to_string());
            }
        }
    }

    Some(modifiers)
}

pub fn lower_statement(node: Node, source: &[u8]) -> Statement {
    match node.kind() {
        "global_statement" => {
            if let Some(child) = node.named_child(0) {
                lower_statement(child, source)
            } else {
                Statement::Unknown {
                    source: node.utf8_text(source).unwrap().to_string(),

                    span: Span {
                        start: node.start_byte(),
                        end: node.end_byte(),
                    },
                }
            }
        }
        "local_declaration_statement" => {
            let modifiers = extract_modifiers(node, source);

            let mut cursor = node.walk();

            let mut variable_decl_node = None;
            for child in node.children(&mut cursor) {
                if child.kind() == "variable_declaration" {
                    variable_decl_node = Some(child);
                    break;
                }
            }

            if let Some(node) = variable_decl_node {
                 let type_node = node
                    .named_child(0)
                    .expect("Expected a type child for variable_declaration");
                let var_type = Some(type_node.utf8_text(source).unwrap().to_string());

                let variable_declarator_node = node
                    .named_child(1)
                    .expect("Expected a variable_declarator child for variable_declaration");

                let identifier_node = variable_declarator_node
                    .named_child(0)
                    .expect("Expected an identifier child for variable_declarator");

                let name = identifier_node.utf8_text(source).unwrap().to_string();

                let mut value: Option<Box<Expression>> = None;
                if let Some(equals_value_clause_node) = variable_declarator_node.named_child(1) {
                    if equals_value_clause_node.kind() == "equals_value_clause" {
                        if let Some(literal_node) = equals_value_clause_node.named_child(0) {
                            value = Some(Box::new(Expression::Raw {
                                source: literal_node.utf8_text(source).unwrap().to_string(),
                                span: Span {
                                    start: literal_node.start_byte(),
                                    end: literal_node.end_byte(),
                                },
                            }));
                        }
                    }
                }

                Statement::DeclStmt(DeclStmt {
                    modifiers,
                    var_decl: VarDecl {
                        name,
                        modifiers: None,
                        var_type,
                        value,
                        span: Span {
                            start: node.start_byte(),
                            end: node.end_byte(),
                        },
                    },
                })
            } else {
                Statement::Unknown {
                    source: node.utf8_text(source).unwrap().to_string(),
                    span: Span {
                        start: node.start_byte(),
                        end: node.end_byte(),
                    },
                }
            }
        }
        "if_statement" => {
            let condition_node = node
                .named_child(0) // Condition is the first named child
                .expect("Expected a condition for if_statement");

            let condition = Box::new(Expression::Raw {
                source: condition_node.utf8_text(source).unwrap().to_string(),
                span: Span {
                    start: condition_node.start_byte(),
                    end: condition_node.end_byte(),
                },
            });

            let consequence_node = node
                .named_child(1) // Consequence (then block) is the second named child
                .expect("Expected a consequence block for if_statement");
            let consequence = Box::new(lower_block(consequence_node, source));

            let mut alternative: Option<Box<Block>> = None;
            if let Some(alternative_node) = node.named_child(2) { // Directly get the alternative block
                alternative = Some(Box::new(lower_block(alternative_node, source)));
            }

            Statement::IfStatement(IfStatement {
                condition,
                consequence,
                alternative,
                span: Span {
                    start: node.start_byte(),
                    end: node.end_byte(),
                },
            })
        }
        _ => Statement::Unknown {
            source: node.utf8_text(source).unwrap().to_string(),
            span: Span {
                start: node.start_byte(),
                end: node.end_byte(),
            },
        },
    }
}

pub fn lower_block(node: Node, source: &[u8]) -> Block {
    let mut statements = Vec::new();
    let block_start_byte = node.start_byte();
    let block_end_byte = node.end_byte();

    // Iterate over named children of the block node
    // Assuming a block node contains statements as its named children
    for i in 0..node.named_child_count() {
        if let Some(child_node) = node.named_child(i) {
            statements.push(lower_statement(child_node, source));
        }
    }

    Block {
        statements,
        span: Span {
            start: block_start_byte,
            end: block_end_byte,
        },
    }
}

// TODO: Implemnt fn lower_expression to recursively parse expresions