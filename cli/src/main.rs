use args::{EntityType, RefactorArgs};
use c_sharp::lower_top_level;
use clap::Parser as ClapParser;
use core::{Refactoring, RenameVariable, TextEdit};
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
                let uast = lower_top_level(node, source_code.as_bytes());
                let refactoring = RenameVariable::new(&cmd.old_name, &cmd.new_name);
                let edits = refactoring.generate_edits(&uast);
                let new_code = apply_edits(&source_code, edits);
                let _ = fs::write(&cmd.file_path, new_code);
            } else {
                eprintln!("No class declaration found in top-level.");
            }
        }
    }
}

fn apply_edits(source: &str, mut edits: Vec<TextEdit>) -> String {
    // Sort edits by start position descending to avoid shifting indices
    edits.sort_by(|a, b| b.start.cmp(&a.start));

    let mut new_source = source.to_string();

    edits.into_iter().for_each(|edit| {
        if edit.end <= new_source.len() && edit.start <= edit.end {
            new_source.replace_range(edit.start..edit.end, &edit.replacement);
        }
    });

    new_source
}
