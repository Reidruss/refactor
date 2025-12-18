use crate::Refactoring;
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
                        FunctionBody::Block(block) => visit_block(block, old, new),
                        FunctionBody::TopLevel(tl) => visit_top_level(tl, old, new),
                        FunctionBody::Expression(expr) => visit_expression(expr, old, new),
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
        Statement::DeclStmt(decl) => {
            if decl.var_decl.name == old {
                decl.var_decl.name = new.to_string();
            }
            if let Some(val) = &mut decl.var_decl.value {
                visit_expression(val, old, new);
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
        Expression::Identifier(id) => {
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
