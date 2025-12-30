use crate::{Refactoring, TextEdit};
use uast::*;

pub struct RenameVariable {
    pub old_name: String,
    pub new_name: String,
}

impl RenameVariable {
    pub fn new(old_name: &str, new_name: &str) -> Self {
        RenameVariable {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
        }
    }
}

impl Refactoring for RenameVariable {
    fn apply(&self, uast: &TopLevel) -> Vec<TextEdit> {
        let mut edits = Vec::new();
        visit_top_level(uast, &self.old_name, &self.new_name, &mut edits);
        edits
    }
}

/// Visit a top level node and parse the approprate variables.
///
/// # Arguments
/// * `node`  - The entry point to the top level node.
/// * `old`   - The old variable name.
/// * `new`   - The new variable name.
/// * `edits` - Contains the edits made to the original source code.
fn visit_top_level(node: &TopLevel, old: &str, new: &str, edits: &mut Vec<TextEdit>) {
    match node {
        TopLevel::Class(class) => {
            if let Some(body) = &class.body {
                for item in body {
                    visit_top_level(item, old, new, edits);
                }
            }
        }
        TopLevel::Function(func) => {
            if let Some(params) = &func.parameters {
                for param in params {
                    if param.name == old {
                        edits.push(TextEdit {
                            start: param.name_span.start,
                            end: param.name_span.end,
                            replacement: new.to_string(),
                        });
                    }
                }
            }
            if let Some(body) = &func.body {
                for item in body {
                    match item {
                        FunctionBodyItems::Block(block) => visit_block(block, old, new, edits),
                        FunctionBodyItems::TopLevel(tl) => visit_top_level(tl, old, new, edits),
                        FunctionBodyItems::Expression(expr) => {
                            visit_expression(expr, old, new, edits)
                        }
                    }
                }
            }
        }
        TopLevel::Statement(stmt) => visit_statement(stmt, old, new, edits),
        TopLevel::Module(mod_def) => {
            for item in &mod_def.body {
                visit_top_level(item, old, new, edits);
            }
        }
        _ => {}
    }
}

fn visit_statement(stmt: &Statement, old: &str, new: &str, edits: &mut Vec<TextEdit>) {
    match stmt {
        Statement::DeclStmt(decl_stmt) => {
            for var in &decl_stmt.var_decls {
                if var.name == old {
                    edits.push(TextEdit {
                        start: var.name_span.start,
                        end: var.name_span.end,
                        replacement: new.to_string(),
                    });
                }
                if let Some(val) = &var.value {
                    visit_expression(val, old, new, edits);
                }
            }
        }
        Statement::IfStatement(if_stmt) => {
            visit_expression(&if_stmt.condition, old, new, edits);
            visit_block(&if_stmt.consequence, old, new, edits);
            if let Some(alt) = &if_stmt.alternative {
                visit_block(alt, old, new, edits);
            }
        }
        Statement::ReturnStatement(ret) => {
            if let Some(val) = &ret.value {
                visit_expression(val, old, new, edits);
            }
        }
        Statement::ExpressionStatement(expr) => {
            visit_expression(&expr.expression, old, new, edits);
        }
        Statement::WhileLoop(w) => {
            visit_expression(&w.condition, old, new, edits);
            visit_block(&w.body, old, new, edits);
        }
        Statement::ForLoop(f) => {
            if let Some(init) = &f.initializer {
                visit_statement(init, old, new, edits);
            }
            if let Some(cond) = &f.condition {
                visit_expression(cond, old, new, edits);
            }
            if let Some(update) = &f.update {
                visit_expression(update, old, new, edits);
            }
            visit_block(&f.body, old, new, edits);
        }
        _ => {}
    }
}

fn visit_block(block: &Block, old: &str, new: &str, edits: &mut Vec<TextEdit>) {
    for stmt in &block.statements {
        visit_statement(stmt, old, new, edits);
    }
}

fn visit_expression(expr: &Expression, old: &str, new: &str, edits: &mut Vec<TextEdit>) {
    match expr {
        Expression::Identifier(id, span) => {
            if id == old {
                edits.push(TextEdit {
                    start: span.start,
                    end: span.end,
                    replacement: new.to_string(),
                });
            }
        }
        Expression::BinaryOp(op) => {
            visit_expression(&op.left, old, new, edits);
            visit_expression(&op.right, old, new, edits);
        }
        Expression::UnaryOp(op) => {
            visit_expression(&op.operand, old, new, edits);
        }
        Expression::Assignment(assign) => {
            visit_expression(&assign.left, old, new, edits);
            visit_expression(&assign.right, old, new, edits);
        }
        Expression::Invocation(inv) => {
            visit_expression(&inv.function, old, new, edits);
            for arg in &inv.arguments {
                visit_expression(arg, old, new, edits);
            }
        }
        Expression::MemberAccess(ma) => {
            visit_expression(&ma.expression, old, new, edits);
        }
        _ => {}
    }
}
