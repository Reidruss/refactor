use crate::{Refactoring, TextEdit};
use uast::*;

pub struct ExtractVariable {
    pub selection_start: usize,
    pub selection_end: usize,
    pub new_variable_name: String,
    pub source: String,
}

impl ExtractVariable {
    pub fn new(
        selection_start: usize,
        selection_end: usize,
        new_variable_name: &str,
        source: &str,
    ) -> Self {
        ExtractVariable {
            selection_start,
            selection_end,
            new_variable_name: new_variable_name.to_string(),
            source: source.to_string(),
        }
    }
}

impl Refactoring for ExtractVariable {
    fn apply(&self, uast: &TopLevel) -> Vec<TextEdit> {
        let mut edits = Vec::new();
        visit_top_level(
            uast,
            self.selection_start,
            self.selection_end,
            &self.new_variable_name,
            &self.source,
            &mut edits,
        );
        edits
    }
}

fn visit_top_level(
    node: &TopLevel,
    start: usize,
    end: usize,
    new_name: &str,
    source: &str,
    edits: &mut Vec<TextEdit>,
) {
    match node {
        TopLevel::Class(class) => {
            if let Some(body) = &class.body {
                for item in body {
                    visit_top_level(item, start, end, new_name, source, edits);
                }
            }
        }
        TopLevel::Function(func) => {
            if let Some(body) = &func.body {
                for item in body {
                    match item {
                        FunctionBodyItems::Block(block) => {
                            visit_block(block, start, end, new_name, source, edits)
                        }
                        FunctionBodyItems::TopLevel(tl) => {
                            visit_top_level(tl, start, end, new_name, source, edits)
                        }
                        FunctionBodyItems::Expression(expr) => {
                            let span = get_expression_span(expr);
                            if let Some(span) = span {
                                if span.start <= start && span.end >= end {
                                    if find_expression_in_expression(expr, start, end).is_some() {
                                        insert_declaration(
                                            span.start, start, end, new_name, source, edits,
                                        );
                                        replace_expression(expr, start, end, new_name, edits);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        TopLevel::Statement(stmt) => {
            let stmt_span = get_statement_span(stmt);
            if let Some(span) = stmt_span {
                if span.start <= start && span.end >= end {
                    if find_expression_in_statement(stmt, start, end) {
                        insert_declaration(span.start, start, end, new_name, source, edits);
                        replace_in_statement(stmt, start, end, new_name, edits);
                    }
                }
            }
        }
        TopLevel::Module(mod_def) => {
            for item in &mod_def.body {
                visit_top_level(item, start, end, new_name, source, edits);
            }
        }
        _ => {}
    }
}

fn visit_block(
    block: &Block,
    start: usize,
    end: usize,
    new_name: &str,
    source: &str,
    edits: &mut Vec<TextEdit>,
) {
    for stmt in &block.statements {
        let stmt_span = get_statement_span(stmt);
        if let Some(span) = stmt_span {
            if span.start <= start && span.end >= end {
                if find_expression_in_statement(stmt, start, end) {
                    insert_declaration(span.start, start, end, new_name, source, edits);
                    replace_in_statement(stmt, start, end, new_name, edits);
                    return;
                }
            }
        }
    }
}

fn get_statement_span(stmt: &Statement) -> Option<Span> {
    match stmt {
        Statement::ExpressionStatement(s) => Some(s.span.clone()),
        Statement::IfStatement(s) => Some(s.span.clone()),
        Statement::WhileLoop(s) => Some(s.span.clone()),
        Statement::ForLoop(s) => Some(s.span.clone()),
        Statement::Unknown { span, .. } => Some(span.clone()),
        Statement::DeclStmt(s) => {
            if let Some(first) = s.var_decls.first() {
                if let Some(last) = s.var_decls.last() {
                    return Some(Span {
                        start: first.span.start,
                        end: last.span.end,
                    });
                }
            }
            None
        }
        Statement::ReturnStatement(s) => {
            s.value.as_ref().and_then(|v| get_expression_span(v))
        }
    }
}

fn get_expression_span(expr: &Expression) -> Option<Span> {
    match expr {
        Expression::Identifier(_, span) => Some(span.clone()),
        Expression::Literal(_, span) => Some(span.clone()),
        Expression::BinaryOp(op) => Some(op.span.clone()),
        Expression::UnaryOp(op) => Some(op.span.clone()),
        Expression::Assignment(op) => Some(op.span.clone()),
        Expression::Invocation(op) => Some(op.span.clone()),
        Expression::MemberAccess(op) => Some(op.span.clone()),
        Expression::Raw { span, .. } => Some(span.clone()),
    }
}

fn find_expression_in_statement(stmt: &Statement, start: usize, end: usize) -> bool {
    match stmt {
        Statement::ExpressionStatement(s) => {
            find_expression_in_expression(&s.expression, start, end).is_some()
        }
        Statement::DeclStmt(s) => {
            for var in &s.var_decls {
                if let Some(val) = &var.value {
                    if find_expression_in_expression(val, start, end).is_some() {
                        return true;
                    }
                }
            }
            false
        }
        Statement::IfStatement(s) => {
            if find_expression_in_expression(&s.condition, start, end).is_some() {
                return true;
            }
            false
        }
        Statement::ReturnStatement(s) => {
            if let Some(val) = &s.value {
                find_expression_in_expression(val, start, end).is_some()
            } else {
                false
            }
        }
        Statement::WhileLoop(s) => {
            if find_expression_in_expression(&s.condition, start, end).is_some() {
                return true;
            }
            false
        }
        Statement::ForLoop(s) => {
            if let Some(init) = &s.initializer {
                if find_expression_in_statement(init, start, end) {
                    return true;
                }
            }
            if let Some(cond) = &s.condition {
                if find_expression_in_expression(cond, start, end).is_some() {
                    return true;
                }
            }
            if let Some(update) = &s.update {
                if find_expression_in_expression(update, start, end).is_some() {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

fn find_expression_in_expression(
    expr: &Expression,
    start: usize,
    end: usize,
) -> Option<&Expression> {
    let span = get_expression_span(expr)?;
    if span.start == start && span.end == end {
        return Some(expr);
    }

    match expr {
        Expression::BinaryOp(op) => find_expression_in_expression(&op.left, start, end)
            .or_else(|| find_expression_in_expression(&op.right, start, end)),
        Expression::UnaryOp(op) => find_expression_in_expression(&op.operand, start, end),
        Expression::Assignment(op) => find_expression_in_expression(&op.left, start, end)
            .or_else(|| find_expression_in_expression(&op.right, start, end)),
        Expression::Invocation(op) => find_expression_in_expression(&op.function, start, end)
            .or_else(|| {
                op.arguments
                    .iter()
                    .find_map(|arg| find_expression_in_expression(arg, start, end))
            }),
        Expression::MemberAccess(op) => find_expression_in_expression(&op.expression, start, end),
        _ => None,
    }
}

fn replace_in_statement(
    stmt: &Statement,
    start: usize,
    end: usize,
    new_name: &str,
    edits: &mut Vec<TextEdit>,
) {
    match stmt {
        Statement::ExpressionStatement(s) => {
            replace_expression(&s.expression, start, end, new_name, edits)
        }
        Statement::DeclStmt(s) => {
            for var in &s.var_decls {
                if let Some(val) = &var.value {
                    replace_expression(val, start, end, new_name, edits);
                }
            }
        }
        Statement::IfStatement(s) => replace_expression(&s.condition, start, end, new_name, edits),
        Statement::ReturnStatement(s) => {
            if let Some(val) = &s.value {
                replace_expression(val, start, end, new_name, edits);
            }
        }
        Statement::WhileLoop(s) => replace_expression(&s.condition, start, end, new_name, edits),
        Statement::ForLoop(s) => {
            if let Some(init) = &s.initializer {
                replace_in_statement(init, start, end, new_name, edits);
            }
            if let Some(cond) = &s.condition {
                replace_expression(cond, start, end, new_name, edits);
            }
            if let Some(update) = &s.update {
                replace_expression(update, start, end, new_name, edits);
            }
        }
        _ => {}
    }
}

fn replace_expression(
    expr: &Expression,
    start: usize,
    end: usize,
    new_name: &str,
    edits: &mut Vec<TextEdit>,
) {
    let span = if let Some(s) = get_expression_span(expr) {
        s
    } else {
        return;
    };

    if span.start == start && span.end == end {
        edits.push(TextEdit {
            start,
            end,
            replacement: new_name.to_string(),
        });
        return;
    }

    match expr {
        Expression::BinaryOp(op) => {
            replace_expression(&op.left, start, end, new_name, edits);
            replace_expression(&op.right, start, end, new_name, edits);
        }
        Expression::UnaryOp(op) => replace_expression(&op.operand, start, end, new_name, edits),
        Expression::Assignment(op) => {
            replace_expression(&op.left, start, end, new_name, edits);
            replace_expression(&op.right, start, end, new_name, edits);
        }
        Expression::Invocation(op) => {
            replace_expression(&op.function, start, end, new_name, edits);
            for arg in &op.arguments {
                replace_expression(arg, start, end, new_name, edits);
            }
        }
        Expression::MemberAccess(op) => {
            replace_expression(&op.expression, start, end, new_name, edits)
        }
        _ => {}
    }
}

fn insert_declaration(
    stmt_start: usize,
    expr_start: usize,
    expr_end: usize,
    new_name: &str,
    source: &str,
    edits: &mut Vec<TextEdit>,
) {
    let mut line_start = stmt_start;
    while line_start > 0 && source.as_bytes()[line_start - 1] != b'\n' {
        line_start -= 1;
    }
    let indentation = &source[line_start..stmt_start];

    if expr_end > source.len() {
        return;
    }
    let expr_text = &source[expr_start..expr_end];

    let decl = format!("var {} = {};\n{}", new_name, expr_text, indentation);

    edits.push(TextEdit {
        start: stmt_start,
        end: stmt_start,
        replacement: decl,
    });
}
