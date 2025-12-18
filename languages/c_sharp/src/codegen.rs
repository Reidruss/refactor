use uast::*;

pub struct CSharpCodeGenerator {
    indent_level: usize,
    indent_string: String,
}

impl CSharpCodeGenerator {
    pub fn new(indent_string: &str) -> Self {
        CSharpCodeGenerator {
            indent_level: 0,
            indent_string: indent_string.to_string(),
        }
    }

    fn indent(&self) -> String {
        self.indent_string.repeat(self.indent_level)
    }

    pub fn generate(&mut self, node: &TopLevel) -> String {
        match node {
            TopLevel::Class(class_def) => self.generate_class(class_def),
            TopLevel::Function(func_def) => self.generate_function(func_def),
            TopLevel::Statement(stmt) => self.generate_statement(stmt),
            TopLevel::Unknown { source, .. } => source.clone(),
            _ => format!("/* Unimplemented TopLevel: {:?} */", node),
        }
    }

    fn generate_class(&mut self, class_def: &ClassDef) -> String {
        let mut output = String::new();
        
        // Modifiers
        if let Some(modifiers) = &class_def.modifiers {
            output.push_str(&modifiers.join(" "));
            output.push(' ');
        }

        output.push_str("class ");
        output.push_str(&class_def.name);
        output.push_str(" {\n");

        self.indent_level += 1;
        if let Some(body) = &class_def.body {
            for item in body {
                output.push_str(&self.indent());
                output.push_str(&self.generate(item));
                output.push('\n');
            }
        }
        self.indent_level -= 1;

        output.push_str(&self.indent());
        output.push('}');
        output
    }

    fn generate_function(&mut self, func_def: &FunctionDef) -> String {
        let mut output = String::new();

        if let Some(modifiers) = &func_def.modifiers {
            output.push_str(&modifiers.join(" "));
            output.push(' ');
        }

        if let Some(return_type) = &func_def.return_type {
            output.push_str(return_type);
            output.push(' ');
        } else {
            output.push_str("void ");
        }

        output.push_str(&func_def.name);
        output.push('(');

        if let Some(params) = &func_def.parameters {
            for (i, param) in params.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                if let Some(t) = &param.var_type {
                    output.push_str(t);
                    output.push(' ');
                }
                output.push_str(&param.name);
            }
        }

        output.push_str(") ");

        if let Some(body) = &func_def.body {
            for item in body {
                 match item {
                     FunctionBody::Block(block) => output.push_str(&self.generate_block(block)),
                     FunctionBody::TopLevel(tl) => output.push_str(&self.generate(tl)), // Should not happen for methods usually
                     FunctionBody::Expression(expr) => {
                         output.push_str(" => ");
                         output.push_str(&self.generate_expression(expr));
                         output.push(';');
                     }
                 }
            }
        } else {
            output.push(';');
        }

        output
    }

    fn generate_block(&mut self, block: &Block) -> String {
        let mut output = String::new();
        output.push_str("{\n");
        self.indent_level += 1;
        for stmt in &block.statements {
            output.push_str(&self.indent());
            output.push_str(&self.generate_statement(stmt));
            output.push('\n');
        }
        self.indent_level -= 1;
        output.push_str(&self.indent());
        output.push('}');
        output
    }

    fn generate_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::DeclStmt(decl) => {
                let mut output = String::new();
                let v = &decl.var_decl;
                
                if let Some(modifiers) = &decl.modifiers {
                     if !modifiers.is_empty() {
                        output.push_str(&modifiers.join(" "));
                        output.push(' ');
                     }
                }

                if let Some(t) = &v.var_type {
                    output.push_str(t);
                } else {
                    output.push_str("var");
                }
                output.push(' ');
                output.push_str(&v.name);

                if let Some(val) = &v.value {
                    output.push_str(" = ");
                    output.push_str(&self.generate_expression(val));
                }
                output.push(';');
                output
            }
            Statement::ReturnStatement(ret) => {
                let mut output = "return".to_string();
                if let Some(val) = &ret.value {
                    output.push(' ');
                    output.push_str(&self.generate_expression(val));
                }
                output.push(';');
                output
            }
            Statement::IfStatement(if_stmt) => {
                let mut output = String::new();
                output.push_str("if (");
                output.push_str(&self.generate_expression(&if_stmt.condition));
                output.push_str(") ");
                output.push_str(&self.generate_block(&if_stmt.consequence));
                
                if let Some(alt) = &if_stmt.alternative {
                    output.push_str(" else ");
                    output.push_str(&self.generate_block(alt));
                }
                output
            }
             Statement::ExpressionStatement(expr_stmt) => {
                 let mut output = self.generate_expression(&expr_stmt.expression);
                 output.push(';');
                 output
             }
            Statement::Unknown { source, .. } => source.clone(),
            _ => format!("/* Unimplemented Statement: {:?} */", stmt),
        }
    }

    fn generate_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::Identifier(id) => id.clone(),
            Expression::Literal(lit) => match lit {
                Literal::Integer(i) => i.to_string(),
                Literal::Float(f) => f.to_string(),
                Literal::String(s) => format!("\"{}\"", s), // Basic escaping
                Literal::Boolean(b) => b.to_string(),
            },
            Expression::BinaryOp(op) => {
                let left = self.generate_expression(&op.left);
                let right = self.generate_expression(&op.right);
                let operator = match op.operator {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Sub => "-",
                    BinaryOperator::Mul => "*",
                    BinaryOperator::Div => "/",
                    BinaryOperator::Equal => "==",
                    BinaryOperator::NotEqual => "!=",
                    BinaryOperator::GreaterThan => ">",
                    BinaryOperator::LessThan => "<",
                    BinaryOperator::GreaterThanEqual => ">=",
                    BinaryOperator::LessThanEqual => "<=",
                };
                format!("{} {} {}", left, operator, right)
            },
             Expression::Assignment(assign) => {
                let left = self.generate_expression(&assign.left);
                let right = self.generate_expression(&assign.right);
                let op = match assign.operator {
                    AssignmentOperator::Assign => "=",
                    AssignmentOperator::AddAssign => "+=",
                    AssignmentOperator::SubAssign => "-=",
                    AssignmentOperator::MulAssign => "*=",
                    AssignmentOperator::DivAssign => "/=",
                };
                 format!("{} {} {}", left, op, right)
             },
            Expression::Raw { source, .. } => source.clone(),
            _ => format!("/* Unimplemented Expr: {:?} */", expr),
        }
    }
}
