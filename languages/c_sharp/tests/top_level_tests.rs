use c_sharp::lower_top_level;
use tree_sitter::Parser;
use uast::{Expression, FunctionBody, Literal, Statement, TopLevel};

#[test]
fn test_lower_class_with_method() {
    let code = r#"
        public class MyClass {
            public int MyMethod(int a) {
                return 5;
            }
        }
    "#;

    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_c_sharp::language())
        .expect("Error loading C# grammar");
    let tree = parser.parse(code, None).unwrap();
    let root = tree.root_node();

    // Find class declaration
    let mut cursor = root.walk();
    let class_node = root
        .children(&mut cursor)
        .find(|n| n.kind() == "class_declaration")
        .expect("No class declaration found");

    let result = lower_top_level(class_node, code.as_bytes());

    if let TopLevel::Class(class_def) = result {
        assert_eq!(class_def.name, "MyClass");
        assert_eq!(class_def.modifiers, Some(vec!["public".to_string()]));

        let body = class_def.body.expect("Class body should be present");
        assert_eq!(body.len(), 1);

        if let TopLevel::Function(func_def) = &body[0] {
            assert_eq!(func_def.name, "MyMethod");
            assert_eq!(func_def.return_type, Some("int".to_string()));
            assert_eq!(func_def.modifiers, Some(vec!["public".to_string()]));

            // Check parameters
            let params = func_def
                .parameters
                .as_ref()
                .expect("Parameters should be present");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].name, "a");
            assert_eq!(params[0].var_type, Some("int".to_string()));

            // Check body
            let func_body = func_def
                .body
                .as_ref()
                .expect("Function body should be present");
            assert_eq!(func_body.len(), 1);

            if let FunctionBody::Block(block) = &func_body[0] {
                assert_eq!(block.statements.len(), 1);
                // Verify return statement
                match &block.statements[0] {
                    Statement::ReturnStatement(ret) => {
                        if let Some(val) = &ret.value {
                            if let Expression::Literal(Literal::Integer(i)) = **val {
                                assert_eq!(i, 5);
                            } else {
                                panic!("Expected integer literal 5");
                            }
                        } else {
                            panic!("Expected return value");
                        }
                    }
                    _ => panic!("Expected ReturnStatement"),
                }
            } else {
                panic!("Expected FunctionBody::Block");
            }
        } else {
            panic!("Expected FunctionDef inside ClassDef");
        }
    } else {
        panic!("Expected TopLevel::Class");
    }
}
