use args::{EntityType, RefactorArgs};
use c_sharp::{codegen::CSharpCodeGenerator, lower_top_level};
use clap::Parser as ClapParser;
use core::{Refactoring, RenameVariable};
use std::fs;
use tree_sitter::Parser;

mod args;

fn main() {
    let args = RefactorArgs::parse();

    match args.entity_type {
        EntityType::RenameVariable(cmd) => {
            let source_code = fs::read_to_string(&cmd.file_path).expect("Unable to read file");

            let mut parser = Parser::new();

            parser
                .set_language(tree_sitter_c_sharp::language())
                .expect("Error loading C# grammar");

            let tree = parser.parse(&source_code, None).expect("Error parsing");
            let root = tree.root_node();

            let mut cursor = root.walk();
            let class_node = root
                .children(&mut cursor)
                .find(|n| n.kind() == "class_declaration");

            if let Some(node) = class_node {
                let mut uast = lower_top_level(node, source_code.as_bytes());
                let refactoring = RenameVariable::new(&cmd.old_name, &cmd.new_name);
                refactoring.apply(&mut uast);

                let mut generator = CSharpCodeGenerator::new("    ");
                let new_code = generator.generate(&uast);
                println!("{}", new_code);
            } else {
                eprintln!("No class declaration found in top-level.");
            }
        }
    }
}
