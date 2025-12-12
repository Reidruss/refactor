use c_sharp::lower_statement;
use parser::GenericParser;
use uast::Statement;

#[test]
fn test_lower_variable_declaration() {
    // 1. Setup the C# Parser
    // We use the raw tree-sitter-c-sharp language definition here
    let language = tree_sitter_c_sharp::language();
    let mut parser = GenericParser::new(language);

    // 2. Parse a simple string
    let code = "int x = 5;";
    let tree = parser.parse(code);

    // 3. Extract the specific node we want to test
    // root_node() is the file; child(0) is the first statement
    let root = tree.root_node();
    let first_node = root.child(0).expect("Code should have one child");

    // 4. Run your lowering logic
    let result = lower_statement(first_node, code.as_bytes());

    // 5. Verify the result using pattern matching
    if let Statement::VarDecl(decl) = result {
        assert_eq!(decl.name, "extracted_name"); // Matches the placeholder logic from before
        assert_eq!(decl.span.start, 0);
        assert_eq!(decl.span.end, 9);
    } else {
        panic!("Expected a VarDecl, but got {:?}", result);
    }
}
