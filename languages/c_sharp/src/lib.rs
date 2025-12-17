use tree_sitter::Node;
use uast::{
    Assignment, AssignmentOperator, BinaryOp, BinaryOperator, Block, DeclStmt, Expression,
    IfStatement, Literal, Span, Statement, UnaryOp, UnaryOperator, VarDecl,
};

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
                            value = Some(Box::new(lower_expressions(literal_node, source)));
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

            let condition = Box::new(lower_expressions(condition_node, source));

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
    let mut statements: Vec<Statement> = Vec::new();
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

pub fn lower_expressions(node: Node, source: &[u8]) -> Expression {
    match node.kind() {
        "integer_literal" => {
            let text = node.utf8_text(source).unwrap();
            let value = text.parse::<i64>().unwrap_or(0);
            Expression::Literal(Literal::Integer(value))
        }
        "real_literal" => {
            let text = node.utf8_text(source).unwrap();
            let value = text.parse::<f64>().unwrap_or(0.0);
            Expression::Literal(Literal::Float(value))
        }
        "string_literal" => {
            let text = node.utf8_text(source).unwrap();
            let content = if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
                &text[1..text.len() - 1]
            } else {
                text
            };
            Expression::Literal(Literal::String(content.to_string()))
        }
        "boolean_literal" => {
            let text = node.utf8_text(source).unwrap();
            let val = text == "true";
            Expression::Literal(Literal::Boolean(val))
        }
        "identifier" => {
            let text = node.utf8_text(source).unwrap();
            Expression::Identifier(text.to_string())
        }
        "binary_expression" => {
            let left_node = node.child_by_field_name("left").expect("Binary expr missing left");
            let right_node = node.child_by_field_name("right").expect("Binary expr missing right");
            let operator_node = node.child_by_field_name("operator").expect("Binary expr missing op");

            let op_text = operator_node.utf8_text(source).unwrap();
            let operator = match op_text {
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Sub,
                "*" => BinaryOperator::Mul,
                "/" => BinaryOperator::Div,
                "==" => BinaryOperator::Equal,
                "!=" => BinaryOperator::NotEqual,
                ">" => BinaryOperator::GreaterThan,
                "<" => BinaryOperator::LessThan,
                ">=" => BinaryOperator::GreaterThanEqual,
                "<=" => BinaryOperator::LessThanEqual,
                _ => BinaryOperator::Add, // Default/Fallback
            };

            Expression::BinaryOp(BinaryOp {
                left: Box::new(lower_expressions(left_node, source)),
                operator,
                right: Box::new(lower_expressions(right_node, source)),
            })
        }
        // Fallback for unimplemented types
        _ => Expression::Raw {
            source: node.utf8_text(source).unwrap_or("").to_string(),
            span: Span {
                start: node.start_byte(),
                end: node.end_byte(),
            },
        },
    }
}