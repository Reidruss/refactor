
use tree_sitter::Parser;
use c_sharp::lower_top_level;
use c_sharp::codegen::CSharpCodeGenerator;

#[test]
fn test_round_trip_class() {
    let source_code = r###"public class MyClass {
    public int MyMethod(int a) {
        return 5;
    }
}"###;

    // 1. Parse & Lower
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c_sharp::language()).expect("Error loading C# grammar");
    let tree = parser.parse(source_code, None).unwrap();
    let root = tree.root_node();
    let class_node = root.child(0).expect("Class node"); 
    let uast = lower_top_level(class_node, source_code.as_bytes());

    // 2. Generate
    let mut generator = CSharpCodeGenerator::new("    ");
    let generated_code = generator.generate(&uast);

    println!("Original:\n{}", source_code);
    println!("Generated:\n{}", generated_code);

    // 3. Verify
    // Note: Exact string match might be brittle due to whitespace, so we trim
    let expected = r###"public class MyClass {
    public int MyMethod(int a) {
        return 5;
    }
}"###;

    assert_eq!(generated_code.trim(), expected.trim());
}

#[test]
fn test_assignment_and_if() {
     let source_code = r###"public class Test {
    public void Run() {
        int x = 10;
        if (x > 5) {
            x = 0;
        } else {
            x = 1;
        }
    }
}"###;
    // 1. Parse & Lower
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c_sharp::language()).expect("Error loading C# grammar");
    let tree = parser.parse(source_code, None).unwrap();
    let root = tree.root_node();
    let class_node = root.child(0).expect("Class node");
    let uast = lower_top_level(class_node, source_code.as_bytes());

    // 2. Generate
    let mut generator = CSharpCodeGenerator::new("    ");
    let generated_code = generator.generate(&uast);
    
    println!("Original:\n{}", source_code);
    println!("Generated:\n{}", generated_code);

    // Naive verification: check if key parts are present
    assert!(generated_code.contains("int x = 10;"));
    assert!(generated_code.contains("if (x > 5)"));
    assert!(generated_code.contains("else"));
}

