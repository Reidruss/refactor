Architecting a Polyglot Refactoring Engine in Rust: A Comprehensive Systems Analysis and Implementation Roadmap1. Executive Summary and Strategic Architectural VisionThe initiative to construct a polyglot refactoring tool utilizing Rust represents a sophisticated challenge at the intersection of compiler theory, systems programming, and software ergonomics. The user's proposed project structure lays a foundational baseline, yet the transition from a directory tree to a functioning semantic analysis engine requires a rigorous implementation strategy. This report provides an exhaustive technical analysis and roadmap for building this system, with a specific focus on the initial integration of C# followed by an expansion into Python, JavaScript, and other languages.Refactoring is distinct from compilation. While a compiler lowers high-level code into machine instructions, discarding formatting and comments along the way, a refactoring engine must understand the code semantically while preserving its textual integrity. This duality—semantic mutability and syntactic preservation—is the central tension of the project. The choice of Rust is architecturally sound; its ownership model ensures memory safety without garbage collection pauses, which is critical when constructing and traversing massive Abstract Syntax Trees (ASTs) for large enterprise codebases. Furthermore, Rust’s type system, particularly its algebraic data types (enums), provides the ideal mechanism for modeling the heterogeneous structure of a Universal AST (UAST).The analysis herein validates the user’s proposed workspace structure but recommends specific refinements to manage dependency graphs and compilation times. It details the necessity of a "Hybrid UAST" approach, where language-agnostic refactorings (like Rename) operate on a shared core, while language-specific idiosyncrasies are handled via an "Escape Hatch" mechanism. The report delineates the path from raw text to Tree-sitter Concrete Syntax Trees (CST), through the "Lowering" process to the UAST, into the semantic analysis phase, and finally to the generation of atomic TextEdit operations.2. Workspace Architecture and Dependency ManagementThe provided directory structure (core, uast, languages, parser, cli, tests) is a canonical Rust workspace layout. However, in a multi-language tool, the relationship between these crates dictates the system's extensibility.2.1 Component Role AnalysisThe following table details the architectural responsibilities and necessary dependency constraints for each component in the workspace.ComponentRoleArchitectural ResponsibilityAllowed DependenciesuastData DefinitionDefines the universal schema (enums/structs) for code representation. Must be strictly data-only to ensure it remains lightweight.serde, thiserror. No internal crate dependencies.parserCST AbstractionWraps tree-sitter libraries. Provides a generic API for cursor movement and text slicing, shielding the rest of the system from raw FFI details.tree-sitter, uast.coreLogic KernelContains the Refactoring Engine, Semantic Analyzer, and Trait Definitions (RefactorRule, LanguageAdapter). Orchestrates the transformation pipeline.uast, parser, tracing.languages/*AdaptersEach crate (e.g., c_sharp) implements core::LanguageAdapter. Responsible for "Lowering" Tree-sitter nodes into UAST nodes.core, uast, parser, tree-sitter-c-sharp (etc).cliEntry PointHandles CLI argument parsing, file I/O, parallelism (rayon), and error reporting (miette). Wires up the registry of languages.core, languages/*, clap.2.2 The Dependency Inversion PrincipleA critical insight for this architecture is the prevention of circular dependencies. The core crate cannot depend on languages/c_sharp because languages/c_sharp must depend on core to implement its traits.Mechanism: The cli crate acts as the "Dependency Injection Root." It imports both core and languages/c_sharp. It instantiates the C# adapter and passes it to the core engine as a trait object (Box<dyn LanguageAdapter>).Implication: The core engine remains truly agnostic. It never knows "C#" exists; it only knows that a LanguageAdapter exists that can parse text into a UAST.2.3 Rust Workspace ConfigurationThe root Cargo.toml should utilize workspace inheritance to keep dependencies synchronized across the 10+ crates this project will eventually contain.Ini, TOML# Root Cargo.toml
[workspace]
members = [
    "core",
    "uast",
    "parser",
    "languages/c_sharp",
    "languages/python",
    "cli",
    #...
]

[workspace.dependencies]
tree-sitter = "0.20"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rayon = "1.7"
This setup ensures that tree-sitter versions do not drift between the generic parser wrapper and the specific languages/python implementation, which is a common source of build failures in Rust tooling.3. The Parsing Layer: Tree-Sitter IntegrationThe user correctly identified the need to "convert each program into AST." For this, Tree-sitter is the industry standard choice for modern tooling (used by Neovim, GitHub Semantic, Semgrep). Unlike traditional compiler parsers (like Roslyn for C# or javac for Java), Tree-sitter is designed for observability and error tolerance.3.1 Concrete Syntax Trees (CST) vs. Abstract Syntax Trees (AST)It is vital to distinguish between what Tree-sitter produces and what the Refactoring Engine needs.Tree-sitter produces a CST: It represents the exact syntax. If the user writes if ( x ), the CST includes nodes for if, (, x, ), and the whitespace.Refactoring needs a UAST: The engine needs to know that a "Conditional Expression" exists with a condition x. It does not care about the parentheses unless it is regenerating text.3.2 The Generic Parser Wrapper (parser/src/lib.rs)The parser crate must abstract the raw Tree-sitter API. This is crucial because Tree-sitter is a C library interop (FFI). Isolating unsafe FFI calls in one crate protects the safety of the wider Rust codebase.Key Implementation Detail: The Cursor WrapperTree-sitter uses a stateful "Cursor" to traverse the tree. The parser crate should provide a safe, high-level iterator over these nodes.Rust// parser/src/lib.rs
use tree_sitter::{Node, Parser, Tree};

pub struct GenericParser {
    inner: Parser,
}

impl GenericParser {
    pub fn new(language: tree_sitter::Language) -> Self {
        let mut inner = Parser::new();
        inner.set_language(language).expect("Error loading grammar");
        Self { inner }
    }

    pub fn parse(&mut self, text: &str) -> Tree {
        self.inner.parse(text, None).expect("Parse failed")
    }
}
This abstraction allows the user to later swap Tree-sitter for a different parsing backend (like a native Rust parser) without rewriting the entire refactoring engine.4. Designing the Universal AST (UAST)The uast crate is the linchpin of the multi-language capability. The design goal is to create a data structure that can represent C#, Python, and JavaScript with high fidelity.4.1 The Intersection vs. Union ProblemThere are two philosophies for UAST design:The Union Approach: The UAST contains every possible feature of every language. (e.g., specific nodes for Python decorators, C# attributes, Java annotations, Rust macros).Risk: The generic Function struct becomes bloated with 50 optional fields, most of which are None for any given language.The Intersection Approach: The UAST represents only the shared core (Functions, Loops, Variables). Anything unique is stored in a generic "Bag of Properties."Risk: Semantic analysis becomes weak for language-specific features.Recommended Strategy: The Hybrid Typed EnumThe uast should define high-level semantic categories using Rust Enums.Rust// uast/src/lib.rs

// The Span is critical for refactoring - it points back to the original source text.
#
pub struct Span {
    pub start_byte: usize,
    pub end_byte: usize,
}

#
pub enum TopLevel {
    Class(ClassDef),
    Function(FunctionDef),
    Module(ModuleDef),
    Statement(Statement),
}

#
pub struct ClassDef {
    pub name: String,
    pub span: Span,
    pub body: Vec<TopLevel>,
    pub modifiers: Vec<String>, // "public", "static", "abstract"
    pub annotations: Vec<Annotation>, // "", "@decorator"
}

#
pub enum Statement {
    VarDecl(VarDecl),
    Return(ReturnStmt),
    Expression(Expr),
    If(IfStmt),
    Loop(LoopStmt),
    // The Escape Hatch: captures syntax not representable in UAST
    Unknown { source: String, span: Span },
}
4.2 Handling Language Divergence via "Metadata"To support C# specific features like "Regions" or "Preprocessor Directives" without polluting the core UAST, use a metadata: HashMap<String, String> field on base nodes.Example: A C# unsafe block.UAST: BlockMetadata: {"safety": "unsafe"}Refactoring Rule: Can check metadata if it cares, or ignore it if it only cares about variable names.5. Language Integration: The C# FocusThe user's roadmap begins with C#. This is a strategic choice: C# is a statically typed, block-structured language with a complex grammar (Generics, LINQ, Async/Await). Solving C# proves the architecture can handle complexity.5.1 The languages/c_sharp CrateThis crate has one primary job: Lowering.Lowering is the translation of the Tree-sitter CST (specific to C#) into the UAST (generic).Dependency: tree-sitter-c-sharp5.2 Mapping C# Concepts to UASTThe C# adapter must walk the Tree-sitter tree and instantiate UAST nodes.C# SourceTree-Sitter NodeUAST NodeMapping Logicclass Userclass_declarationuast::ClassDefExtract identifier child for name.public static void Main()method_declarationuast::FunctionDefpublic & static -> modifiers. void -> return_type.var x = 10;variable_declarationuast::VarDeclvar implies inference. Map to Type::Inferred.List<string>generic_nameuast::Type::GenericRecursively parse type arguments.from x in y select xquery_expressionuast::Expr::QueryComplex. Requires specific UAST support or fallback to Unknown.5.3 The Lowering Algorithm (Recursive Descent)The implementation in languages/c_sharp/src/lib.rs should look like a recursive visitor:Ruststruct CSharpConverter<'a> {
    source: &'a str,
}

impl<'a> CSharpConverter<'a> {
    fn convert_node(&self, node: tree_sitter::Node) -> uast::TopLevel {
        match node.kind() {
            "class_declaration" => self.convert_class(node),
            "method_declaration" => self.convert_method(node),
            //...
            _ => {
                // Fallback for unhandled syntax
                uast::TopLevel::Statement(uast::Statement::Unknown {
                    source: node.utf8_text(self.source.as_bytes()).unwrap().to_string(),
                    span: self.node_span(node),
                })
            }
        }
    }
}
Insight: By defaulting to Unknown for unhandled nodes, the tool remains robust. The user can refactor a C# file containing advanced syntax features (like C# 11 raw string literals) even if the converter doesn't fully understand them, provided the refactoring target (e.g., a variable name) is not inside the unknown block.6. Semantic Analysis: Beyond the ASTA raw AST is insufficient for safe refactoring. To rename a variable, the system must understand Scope.Shadowing: If a local variable x is defined inside a function, it shadows the class member x. Renaming the class member should not rename the local variable.6.1 The Symbol TableThe core crate must implement a SemanticModel. This is built after the UAST is generated.Scope Graph: A tree mirroring the code's block structure (Global -> Class -> Method -> If-Block).Symbol Table: A map within each scope: Name -> DefinitionID.Reference Listing: A list of UsageID -> DefinitionID.Algorithm for Scope Resolution:Traverse the UAST.On entering a Block (Function, Loop), push a new Scope.On VarDecl, add the name to the current Scope.On Identifier usage, search the Scope stack from top to bottom. The first match is the binding definition.6.2 Data Flow Analysis (Future)For advanced refactorings (like "Extract Method"), the system needs Data Flow Analysis (DFA) to determine which variables need to be passed as arguments to the new method. This requires building a Control Flow Graph (CFG).Recommendation: Start with simple scope analysis. CFG analysis is an order of magnitude more complex.7. The Refactoring Engine and "Text Edits"The most critical architectural decision is Immutability.Do not mutate the UAST.If you modify the UAST and try to "print" it back to code, you will lose formatting, comments, and whitespace.7.1 The TextEdit ProtocolInstead, the Refactoring Engine should function like the Language Server Protocol (LSP). It takes the Source Code and UAST, calculates changes, and produces a list of Edits.Rust// core/src/edit.rs
pub struct TextEdit {
    pub start_byte: usize,
    pub end_byte: usize,
    pub new_text: String,
}
7.2 The Refactoring LoopInput: User selects "Rename calculate to compute".Search: The engine queries the Semantic Model: "Find all usages of the symbol defined at FunctionDef(name='calculate')".Plan: The engine generates TextEdit objects for the definition and all references.Edit { start: 100, end: 109, text: "compute" } (The definition)Edit { start: 450, end: 459, text: "compute" } (A call site)Verify: Check for conflicts. Do any edits overlap? (They shouldn't).Apply: Apply the edits to the string buffer in reverse order (highest byte offset first) to avoid invalidating the offsets of earlier edits.8. Expansion Strategy: Python and Other LanguagesOnce the C# pipeline (Parser -> Lowering -> UAST -> Scope -> Edit) is established, adding Python becomes a parallel task.8.1 Python IntegrationAdd tree-sitter-python to languages/python.Implement PythonConverter.Challenge: Python is indentation-based. The UAST Block structure must be inferred from the indent / dedent tokens in the Tree-sitter CST.Challenge: Python is dynamic. The VarDecl node in UAST might need a type_hint field that is None for Python but populated for C#.8.2 The "Core" Abstraction TestTo verify the architecture, write a test in core that performs a "Rename" on a generic UAST without knowing the source language.Construct a UAST manually in the test.Run the "Rename" logic.Assert the resulting TextEdit list is correct.This proves the core logic is truly reusable across C#, Python, and JS.9. Operational Excellence: Testing and CLI9.1 Testing StrategyA refactoring tool must be trustworthy. If it breaks code, users will abandon it.Snapshot Testing (insta crate):Store a .cs file and its expected .uast output (serialized to YAML).Store the expected .cs output after a specific refactoring.This ensures regressions are caught immediately.Round-Trip Property Testing:Parse -> Lower -> UAST.Ensure that every UAST node's span corresponds to the correct text in the source file.9.2 The CLI (cli crate)Use clap for argument parsing.Bashmy-refactor rename --lang csharp --file src/Program.cs --symbol MyClass --new-name NewClass
Use rayon to parallelize processing.Refactoring is often embarrassingly parallel (parsing files).However, Semantic Analysis (Symbol Tables) might need a "Global Phase" where all files are indexed before any refactoring plans are generated.10. ConclusionThe transition from a project skeleton to a working refactoring engine requires a disciplined adherence to the "Separation of Concerns" principle. The parser must isolate FFI complexity; the uast must balance intersection and union; the languages crates must handle the dirty work of lowering; and the core must remain a pure logic engine.By starting with C#, the user effectively stress-tests the system against a complex, static type system. The proposed architecture—leveraging Rust's safety guarantees and Tree-sitter's robustness—is capable not just of renaming variables, but of evolving into a platform for sophisticated code transformations like architectural restructuring and automated migration. The immediate next step is to populate the uast crate with the Enum definitions provided in Section 4 and implement the recursive descent converter for C# in Section 5. This will bridge the gap between the empty directory structure and the first successful parse.