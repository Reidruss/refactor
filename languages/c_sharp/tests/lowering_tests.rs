use c_sharp::lower_statement;
use parser::GenericParser;
use uast::{Expression, Literal, Statement};

#[test]
fn test_lower_variable_declaration() {
    let language: tree_sitter::Language = tree_sitter_c_sharp::language();
    let mut parser: GenericParser = GenericParser::new(language);

    let code: &str = "const double i = 5.0;";
    let tree: tree_sitter::Tree = parser.parse(code);

    let root: tree_sitter::Node<'_> = tree.root_node();
    let first_node: tree_sitter::Node<'_> = root.child(0).expect("Code should have one child");

    let result: Statement = lower_statement(first_node, code.as_bytes());

    if let Statement::DeclStmt(decl_stmt) = result {
        assert_eq!(
            decl_stmt.modifiers.as_ref().map(|m| &m[0]),
            Some(&"const".to_string())
        );

        let decl = &decl_stmt.var_decl;
        assert_eq!(decl.name, "i");

        assert_eq!(decl.var_type, Some("double".to_string()));

        assert_eq!(
            decl.value,
            Some(Box::new(Expression::Literal(Literal::Float(5.0))))
        );
    } else {
        panic!("Expected a DeclStmt, but got {:?}", result);
    }
}

#[test]
fn test_lower_if_statement() {
    let language = tree_sitter_c_sharp::language();
    let mut parser = GenericParser::new(language);

    let code = "if (true) { int x = 1; }";
    let tree = parser.parse(code);
    let root = tree.root_node();
    let if_node = root.child(0).expect("Code should have an if statement");

    let result = lower_statement(if_node, code.as_bytes());

    if let Statement::IfStatement(if_stmt) = result {
        assert_eq!(
            if_stmt.condition,
            Box::new(Expression::Literal(Literal::Boolean(true)))
        );
        assert_eq!(if_stmt.alternative, None);
        assert_eq!(if_stmt.span.start, 0);
        assert_eq!(if_stmt.span.end, 24); // Span for "if (true) { int x = 1; }"

        // Verify the consequence block
        assert_eq!(if_stmt.consequence.statements.len(), 1);
        if let Statement::DeclStmt(decl_stmt) = &if_stmt.consequence.statements[0] {
            let var_decl = &decl_stmt.var_decl;
            assert_eq!(var_decl.name, "x");
            assert_eq!(var_decl.var_type, Some("int".to_string()));
            assert_eq!(
                var_decl.value,
                Some(Box::new(Expression::Literal(Literal::Integer(1))))
            );
        } else {
            panic!(
                "Expected DeclStmt in if consequence, got {:?}",
                if_stmt.consequence.statements[0]
            );
        }
    } else {
        panic!("Expected IfStatement, got {:?}", result);
    }
}

#[test]
fn test_lower_if_else_statement() {
    let language = tree_sitter_c_sharp::language();
    let mut parser = GenericParser::new(language);
    let code = "if (false) { int y = 2; } else { int z = 3; }";
    let tree = parser.parse(code);

    let root = tree.root_node();
    let if_else_node = root
        .child(0)
        .expect("Code should have an if-else statement");

    let result = lower_statement(if_else_node, code.as_bytes());

    if let Statement::IfStatement(if_stmt) = result {
        assert_eq!(
            if_stmt.condition,
            Box::new(Expression::Literal(Literal::Boolean(false)))
        );
        assert_eq!(if_stmt.span.start, 0);
        assert_eq!(if_stmt.span.end, 45); // Span for "if (false) { int y = 2; } else { int z = 3; }"

        // Verify the consequence block
        assert_eq!(if_stmt.consequence.statements.len(), 1);
        if let Statement::DeclStmt(decl_stmt) = &if_stmt.consequence.statements[0] {
            let var_decl = &decl_stmt.var_decl;
            assert_eq!(var_decl.name, "y");
            assert_eq!(var_decl.var_type, Some("int".to_string()));
            assert_eq!(
                var_decl.value,
                Some(Box::new(Expression::Literal(Literal::Integer(2))))
            );
        } else {
            panic!(
                "Expected DeclStmt in if consequence, got {:?}",
                if_stmt.consequence.statements[0]
            );
        }

        // Verify the alternative block
        assert!(if_stmt.alternative.is_some());
        if let Some(alt_block) = if_stmt.alternative {
            assert_eq!(alt_block.statements.len(), 1);
            if let Statement::DeclStmt(decl_stmt) = &alt_block.statements[0] {
                let var_decl = &decl_stmt.var_decl;
                assert_eq!(var_decl.name, "z");
                assert_eq!(var_decl.var_type, Some("int".to_string()));
                assert_eq!(
                    var_decl.value,
                    Some(Box::new(Expression::Literal(Literal::Integer(3))))
                );
            } else {
                panic!(
                    "Expected DeclStmt in else alternative, got {:?}",
                    alt_block.statements[0]
                );
            }
        }
    } else {
        panic!("Expected IfStatement, got {:?}", result);
    }
}
