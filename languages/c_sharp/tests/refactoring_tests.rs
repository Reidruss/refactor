use c_sharp::lower_top_level;
use core::{apply_refactoring, ExtractVariable, Refactoring, RenameVariable};
use tree_sitter::Parser;

fn run_test(source_code: &str, old_name: &str, new_name: &str, expected_code: &str) {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_c_sharp::language())
        .expect("Error loading C# grammar");
    let tree = parser.parse(source_code, None).unwrap();
    let root = tree.root_node();

    // Find class declaration - our lower_top_level expects a class or similar
    let mut cursor = root.walk();
    let class_node = root
        .children(&mut cursor)
        .find(|n| n.kind() == "class_declaration")
        .expect("No class declaration found in test source");

    let uast = lower_top_level(class_node, source_code.as_bytes());

    let refactoring = RenameVariable::new(old_name, new_name);
    let edits = refactoring.apply(&uast);
    let new_code = apply_refactoring(source_code, edits);

    assert_eq!(new_code, expected_code);
}

fn run_extract_test(source_code: &str, extraction_str: &str, new_name: &str, expected_code: &str) {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_c_sharp::language())
        .expect("Error loading C# grammar");
    let tree = parser.parse(source_code, None).unwrap();
    let root = tree.root_node();

    // Find class declaration
    let mut cursor = root.walk();
    let class_node = root
        .children(&mut cursor)
        .find(|n| n.kind() == "class_declaration")
        .expect("No class declaration found in test source");

    let uast = lower_top_level(class_node, source_code.as_bytes());

    let mut found = false;
    let mut new_code = String::new();

    // Iterate through all matches to find the valid one, emulating the CLI logic
    for (start, _) in source_code.match_indices(extraction_str) {
        let end = start + extraction_str.len();
        let refactoring = ExtractVariable::new(start, end, new_name, source_code);
        let edits = refactoring.apply(&uast);
        if !edits.is_empty() {
            new_code = apply_refactoring(source_code, edits);
            found = true;
            break;
        }
    }

    if !found {
        panic!(
            "Could not find a valid expression match for '{}'",
            extraction_str
        );
    }

    assert_eq!(new_code, expected_code);
}

#[test]
fn test_extract_variable_with_comment_collision() {
    let source = r#"public class OrderCalculator
{
    public double CalculateFinalPrice(double price, int quantity, double discount, double shipping)
    {
        // The Target: "(price * quantity) - discount + shipping"
        // This expression calculates the total cost, but it's unnamed logic.
        if ((price * quantity) - discount + shipping > 1000.0)
        {
            Console.WriteLine("This is a high-value order.");
        }

        return (price * quantity) - discount + shipping;
    }
}"#;

    // Note: The extraction currently only targets the first match that works.
    // In this case, it should find the one inside the IF condition first (if traversal order allows) or the one in return?
    // Wait, the match_indices iterates linearly.
    // 1. Comment -> No edits.
    // 2. IF condition -> Edits!
    // So it should extract the one in the IF condition.

    let expected = r#"public class OrderCalculator
{
    public double CalculateFinalPrice(double price, int quantity, double discount, double shipping)
    {
        // The Target: "(price * quantity) - discount + shipping"
        // This expression calculates the total cost, but it's unnamed logic.
        var value = (price * quantity) - discount + shipping;
        if (value > 1000.0)
        {
            Console.WriteLine("This is a high-value order.");
        }

        return (price * quantity) - discount + shipping;
    }
}"#;
    run_extract_test(
        source,
        "(price * quantity) - discount + shipping",
        "value",
        expected,
    );
}

#[test]
fn test_rename_parameter() {
    let source = r#"public class MyClass {
    public int MyMethod(int a) {
        return a + 5;
    }
}"#;
    let expected = r#"public class MyClass {
    public int MyMethod(int b) {
        return b + 5;
    }
}"#;
    run_test(source, "a", "b", expected);
}

#[test]
fn test_rename_local_variable() {
    let source = r#"public class Test {
    public void Run() {
        int x = 10;
        if (x > 5) {
            x = 0;
        } else {
            x = 1;
        }
    }
}"#;
    let expected = r#"public class Test {
    public void Run() {
        int y = 10;
        if (y > 5) {
            y = 0;
        } else {
            y = 1;
        }
    }
}"#;
    run_test(source, "x", "y", expected);
}

#[test]
fn test_rename_variable_in_invocation() {
    let source = r#"public class Test {
    public void Run(int val) {
        Console.WriteLine(val);
    }
}"#;
    let expected = r#"public class Test {
    public void Run(int newValue) {
        Console.WriteLine(newValue);
    }
}"#;
    run_test(source, "val", "newValue", expected);
}

#[test]
fn test_rename_object_in_member_access() {
    let source = r#"public class Test {
    public void Run() {
        Console.WriteLine("Hello");
    }
}"#;
    let expected = r#"public class Test {
    public void Run() {
        MyConsole.WriteLine("Hello");
    }
}"#;
    run_test(source, "Console", "MyConsole", expected);
}

#[test]
fn test_extract_variable_basic() {
    let source = r#"public class Test {
    public void Run() {
        int x = 5 + 10;
    }
}"#;
    let expected = r#"public class Test {
    public void Run() {
        var sum = 5 + 10;
        int x = sum;
    }
}"#;
    run_extract_test(source, "5 + 10", "sum", expected);
}
