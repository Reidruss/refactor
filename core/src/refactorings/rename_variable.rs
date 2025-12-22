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
    fn apply(&self, uast: &mut TopLevel) {
        visit_top_level(uast, &self.old_name, &self.new_name);
    }

    fn generate_edits(&self, uast: &TopLevel) -> Vec<TextEdit> {
        let mut edits = Vec::new();
        visit_top_level_edits(uast, &self.old_name, &self.new_name, &mut edits);
        edits
    }
}

fn visit_top_level(node: &mut TopLevel, old: &str, new: &str) {
    match node {
        TopLevel::Class(class) => {
            if let Some(body) = &mut class.body {
                for item in body {
                    visit_top_level(item, old, new);
                }
            }
        }
        TopLevel::Function(func) => {
            if let Some(params) = &mut func.parameters {
                for param in params {
                    if param.name == old {
                        param.name = new.to_string();
                    }
                }
            }
            if let Some(body) = &mut func.body {
                for item in body {
                    match item {
                        FunctionBodyItems::Block(block) => visit_block(block, old, new),
                        FunctionBodyItems::TopLevel(tl) => visit_top_level(tl, old, new),
                        FunctionBodyItems::Expression(expr) => visit_expression(expr, old, new),
                    }
                }
            }
        }
        TopLevel::Statement(stmt) => visit_statement(stmt, old, new),
        TopLevel::Module(mod_def) => {
            for item in &mut mod_def.body {
                visit_top_level(item, old, new);
            }
        }
        _ => {}
    }
}

fn visit_statement(stmt: &mut Statement, old: &str, new: &str) {
    match stmt {
        Statement::DeclStmt(decl_stmt) => {
            for var in decl_stmt.var_decls.iter_mut() {
                if var.name == old {
                    var.name = new.to_string();
                }
                if let Some(val) = &mut var.value {
                    visit_expression(val, old, new);
                }
            }
        }
        Statement::IfStatement(if_stmt) => {
            visit_expression(&mut if_stmt.condition, old, new);
            visit_block(&mut if_stmt.consequence, old, new);
            if let Some(alt) = &mut if_stmt.alternative {
                visit_block(alt, old, new);
            }
        }
        Statement::ReturnStatement(ret) => {
            if let Some(val) = &mut ret.value {
                visit_expression(val, old, new);
            }
        }
        Statement::ExpressionStatement(expr) => {
            visit_expression(&mut expr.expression, old, new);
        }
        Statement::WhileLoop(w) => {
            visit_expression(&mut w.condition, old, new);
            visit_block(&mut w.body, old, new);
        }
        Statement::ForLoop(f) => {
            if let Some(init) = &mut f.initializer {
                visit_statement(init, old, new);
            }
            if let Some(cond) = &mut f.condition {
                visit_expression(cond, old, new);
            }
            if let Some(update) = &mut f.update {
                visit_expression(update, old, new);
            }
            visit_block(&mut f.body, old, new);
        }
        _ => {}
    }
}

fn visit_block(block: &mut Block, old: &str, new: &str) {
    for stmt in &mut block.statements {
        visit_statement(stmt, old, new);
    }
}

fn visit_expression(expr: &mut Expression, old: &str, new: &str) {
    match expr {
        Expression::Identifier(id, _) => {
            if id == old {
                *id = new.to_string();
            }
        }
        Expression::BinaryOp(op) => {
            visit_expression(&mut op.left, old, new);
            visit_expression(&mut op.right, old, new);
        }
        Expression::UnaryOp(op) => {
            visit_expression(&mut op.operand, old, new);
        }
        Expression::Assignment(assign) => {
            visit_expression(&mut assign.left, old, new);
            visit_expression(&mut assign.right, old, new);
        }
        _ => {}
    }
}

fn visit_top_level_edits(node: &TopLevel, old: &str, new: &str, edits: &mut Vec<TextEdit>) {
    match node {
        TopLevel::Class(class) => {
            if let Some(body) = &class.body {
                for item in body {
                    visit_top_level_edits(item, old, new, edits);
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
                        FunctionBodyItems::Block(block) => {
                            visit_block_edits(block, old, new, edits)
                        }
                        FunctionBodyItems::TopLevel(tl) => {
                            visit_top_level_edits(tl, old, new, edits)
                        }
                        FunctionBodyItems::Expression(expr) => {
                            visit_expression_edits(expr, old, new, edits)
                        }
                    }
                }
            }
        }
        TopLevel::Statement(stmt) => visit_statement_edits(stmt, old, new, edits),
        TopLevel::Module(mod_def) => {
            for item in &mod_def.body {
                visit_top_level_edits(item, old, new, edits);
            }
        }
        _ => {}
    }
}

fn visit_statement_edits(stmt: &Statement, old: &str, new: &str, edits: &mut Vec<TextEdit>) {
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
                    visit_expression_edits(val, old, new, edits);
                }
            }
        }
        Statement::IfStatement(if_stmt) => {
            visit_expression_edits(&if_stmt.condition, old, new, edits);
            visit_block_edits(&if_stmt.consequence, old, new, edits);
            if let Some(alt) = &if_stmt.alternative {
                visit_block_edits(alt, old, new, edits);
            }
        }
        Statement::ReturnStatement(ret) => {
            if let Some(val) = &ret.value {
                visit_expression_edits(val, old, new, edits);
            }
        }
        Statement::ExpressionStatement(expr) => {
            visit_expression_edits(&expr.expression, old, new, edits);
        }
        Statement::WhileLoop(w) => {
            visit_expression_edits(&w.condition, old, new, edits);
            visit_block_edits(&w.body, old, new, edits);
        }
        Statement::ForLoop(f) => {
            if let Some(init) = &f.initializer {
                visit_statement_edits(init, old, new, edits);
            }
            if let Some(cond) = &f.condition {
                visit_expression_edits(cond, old, new, edits);
            }
            if let Some(update) = &f.update {
                visit_expression_edits(update, old, new, edits);
            }
            visit_block_edits(&f.body, old, new, edits);
        }
        _ => {}
    }
}

fn visit_block_edits(block: &Block, old: &str, new: &str, edits: &mut Vec<TextEdit>) {
    for stmt in &block.statements {
        visit_statement_edits(stmt, old, new, edits);
    }
}

fn visit_expression_edits(expr: &Expression, old: &str, new: &str, edits: &mut Vec<TextEdit>) {
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
            visit_expression_edits(&op.left, old, new, edits);
            visit_expression_edits(&op.right, old, new, edits);
        }
        Expression::UnaryOp(op) => {
            visit_expression_edits(&op.operand, old, new, edits);
        }
        Expression::Assignment(assign) => {
            visit_expression_edits(&assign.left, old, new, edits);
            visit_expression_edits(&assign.right, old, new, edits);
        }
        _ => {}
    }
}
