use c_sharp::{codegen::CSharpCodeGenerator, lower_top_level};
use core::{Refactoring, RenameVariable};
use std::env;
use std::fs;
use tree_sitter::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: cli <file_path> <old_name> <new_name>");
        return;
    }

    let file_path = &args[1];
    let old_name = &args[2];
    let new_name = &args[3];

    let source_code = fs::read_to_string(file_path).expect("Unable to read file");

    // 1. Parse
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_c_sharp::language())
        .expect("Error loading C# grammar");
    let tree = parser.parse(&source_code, None).expect("Error parsing");
    let root = tree.root_node();

    // Assumption: File contains a class declaration as top-level child (for simplicity)
    // In real app, we'd handle compilation units.
    // Finding the first class_declaration
    let mut cursor = root.walk();
    let class_node = root
        .children(&mut cursor)
        .find(|n| n.kind() == "class_declaration");

    if let Some(node) = class_node {
        // 2. Lower
        let mut uast = lower_top_level(node, source_code.as_bytes());

        // 3. Refactor
        let refactoring = RenameVariable::new(old_name, new_name);
        refactoring.apply(&mut uast);

        // 4. Codegen
        let mut generator = CSharpCodeGenerator::new("    ");
        let new_code = generator.generate(&uast);

        println!("{}", new_code);
    } else {
        eprintln!("No class declaration found in top-level.");
    }
}
