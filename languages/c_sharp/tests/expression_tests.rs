use c_sharp::lower_expressions;
use parser::GenericParser;
use uast::{BinaryOperator, Expression, Literal};

fn print_tree(node: tree_sitter::Node, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}{}: \"{}\"", indent, node.kind(), node.utf8_text(source.as_bytes()).unwrap_or(""));
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_tree(child, source, depth + 1);
    }
}

#[test]
fn test_lower_integer_literal() {
    let language = tree_sitter_c_sharp::language();
    let mut parser = GenericParser::new(language);
    let code = "123";
    let tree = parser.parse(code);
    let root = tree.root_node();

    println!("Tree structure for '123':");
    print_tree(root, code, 0);

    fn find_node<'a>(node: tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
        if node.kind() == kind {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = find_node(child, kind) {
                return Some(found);
            }
        }
        None
    }

    let literal_node = find_node(root, "integer_literal").expect("Could not find integer_literal in the tree");

    let result = lower_expressions(literal_node, code.as_bytes());

    if let Expression::Literal(Literal::Integer(val)) = result {
        assert_eq!(val, 123);
    } else {
        panic!("Expected Integer Literal, got {:?}", result);
    }
}

#[test]
fn test_lower_binary_expression() {
    let language = tree_sitter_c_sharp::language();
    let mut parser = GenericParser::new(language);
    let code = "1 + 2";
    let tree = parser.parse(code);
    let root = tree.root_node();

    println!("Tree structure for '1 + 2':");
    print_tree(root, code, 0);

    fn find_node<'a>(node: tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
        if node.kind() == kind {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = find_node(child, kind) {
                return Some(found);
            }
        }
        None
    }

    let binary_node = find_node(root, "binary_expression").expect("Could not find binary_expression in the tree");

    let result = lower_expressions(binary_node, code.as_bytes());

    if let Expression::BinaryOp(bin_op) = result {
        assert_eq!(bin_op.operator, BinaryOperator::Add);

        if let Expression::Literal(Literal::Integer(left_val)) = *bin_op.left {
            assert_eq!(left_val, 1);
        } else {
             panic!("Expected left operand to be 1");
        }

        if let Expression::Literal(Literal::Integer(right_val)) = *bin_op.right {
            assert_eq!(right_val, 2);
        } else {
            panic!("Expected right operand to be 2");
        }

    } else {
        panic!("Expected BinaryOp, got {:?}", result);
    }
}


#[test]
fn test_lower_string_literal() {
    let language = tree_sitter_c_sharp::language();
    let mut parser = GenericParser::new(language);
    let code = "\"Hello World\"";
    let tree = parser.parse(code);
    let root = tree.root_node();

    println!("Tree structure for \"Hello World\":");
    print_tree(root, code, 0);

    fn find_node<'a>(node: tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
        if node.kind() == kind {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = find_node(child, kind) {
                return Some(found);
            }
        }
        None
    }

    let literal_node = find_node(root, "string_literal").expect("Could not find string_literal in the tree");

    let result = lower_expressions(literal_node, code.as_bytes());

    if let Expression::Literal(Literal::String(val)) = result {
        assert_eq!(val, "Hello World");
    } else {
        panic!("Expected String Literal, got {:?}", result);
    }
}


#[test]
fn test_lower_real_literal() {
    let language = tree_sitter_c_sharp::language();
    let mut parser = GenericParser::new(language);
    let code = "123.456";
    let tree = parser.parse(code);
    let root = tree.root_node();

    println!("Tree structure for '123.456':");
    print_tree(root, code, 0);

    fn find_node<'a>(node: tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
        if node.kind() == kind {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = find_node(child, kind) {
                return Some(found);
            }
        }
        None
    }

    let literal_node = find_node(root, "real_literal").expect("Could not find integer_literal in the tree");

    let result = lower_expressions(literal_node, code.as_bytes());

    if let Expression::Literal(Literal::Float(val)) = result {
        assert_eq!(val, 123.456);
    } else {
        panic!("Expected Real Literal, got {:?}", result);
    }
}
