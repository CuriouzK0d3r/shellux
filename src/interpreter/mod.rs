use crate::builtins::{call_builtin, is_builtin, register_builtins};
use crate::parser::ast::*;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

use std::process::Command;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Function {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Stmt>,
        closure: Environment,
    },
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Boolean(_) => "bool",
            Value::Nil => "nil",
            Value::Array(_) => "array",
            Value::Map(_) => "map",
            Value::Function { .. } => "function",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Nil => false,
            Value::Integer(0) => false,
            Value::Float(f) if *f == 0.0 => false,
            Value::String(s) if s.is_empty() => false,
            Value::Array(arr) if arr.is_empty() => false,
            Value::Map(map) if map.is_empty() => false,
            _ => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Nil => "nil".to_string(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Map(map) => {
                let pairs: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
            Value::Function { name, .. } => format!("function {}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    variables: HashMap<String, Value>,
    constants: HashMap<String, bool>,
    parent: Option<Box<Environment>>,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::Function { name: a, .. }, Value::Function { name: b, .. }) => a == b,
            _ => false,
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            constants: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            constants: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.define_variable(name, value, false);
    }

    pub fn define_const(&mut self, name: String, value: Value) {
        self.define_variable(name, value, true);
    }

    fn define_variable(&mut self, name: String, value: Value, is_const: bool) {
        self.variables.insert(name.clone(), value);
        self.constants.insert(name, is_const);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: &str, value: Value) -> Result<()> {
        if self.variables.contains_key(name) {
            // Check if this is a constant
            if let Some(&is_const) = self.constants.get(name) {
                if is_const {
                    return Err(anyhow!("Cannot assign to const variable: {}", name));
                }
            }
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.set(name, value)
        } else {
            Err(anyhow!("Undefined variable: {}", name))
        }
    }
}

pub struct Interpreter {
    environment: Environment,
    globals: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();

        // Add built-in functions
        register_builtins(&mut globals);

        Self {
            environment: globals.clone(),
            globals,
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<Value> {
        let mut result = Value::Nil;

        for stmt in program.statements {
            result = self.execute_statement(stmt)?;
        }

        Ok(result)
    }

    fn execute_statement(&mut self, stmt: Stmt) -> Result<Value> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate_expression(expr),

            Stmt::Let { name, value, .. } => {
                let val = self.evaluate_expression(value)?;
                self.environment.define(name, val.clone());
                Ok(val)
            }

            Stmt::Const { name, value, .. } => {
                let val = self.evaluate_expression(value)?;
                self.environment.define_const(name, val.clone());
                Ok(val)
            }

            Stmt::Assignment { target, value, .. } => {
                let val = self.evaluate_expression(value)?;
                match target {
                    AssignmentTarget::Identifier(name) => {
                        self.environment.set(&name, val.clone())?;
                    }
                    _ => return Err(anyhow!("Complex assignment targets not yet supported")),
                }
                Ok(val)
            }

            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                let condition_value = self.evaluate_expression(condition)?;
                if condition_value.is_truthy() {
                    self.execute_block(then_block)
                } else if let Some(else_stmts) = else_block {
                    self.execute_block(else_stmts)
                } else {
                    Ok(Value::Nil)
                }
            }

            Stmt::Function {
                name,
                parameters,
                body,
                ..
            } => {
                let func = Value::Function {
                    name: name.clone(),
                    parameters,
                    body,
                    closure: self.environment.clone(),
                };
                self.environment.define(name, func.clone());
                Ok(func)
            }

            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    self.evaluate_expression(e)
                } else {
                    Ok(Value::Nil)
                }
            }

            _ => Err(anyhow!("Statement type not yet implemented: {:?}", stmt)),
        }
    }

    fn execute_block(&mut self, statements: Vec<Stmt>) -> Result<Value> {
        let previous = self.environment.clone();
        self.environment = Environment::new_with_parent(previous.clone());

        let mut result = Value::Nil;
        for stmt in statements {
            result = self.execute_statement(stmt)?;
        }

        self.environment = previous;
        Ok(result)
    }

    fn evaluate_expression(&mut self, expr: Expr) -> Result<Value> {
        match expr {
            Expr::Integer(i) => Ok(Value::Integer(i)),
            Expr::Float(f) => Ok(Value::Float(f)),
            Expr::String(s) => Ok(Value::String(s)),
            Expr::Boolean(b) => Ok(Value::Boolean(b)),
            Expr::Nil => Ok(Value::Nil),

            Expr::Identifier(name) => {
                // Check built-ins first, then environment variables, then external commands
                if is_builtin(&name) {
                    call_builtin(&name, &[])
                } else if let Some(value) = self.environment.get(&name) {
                    Ok(value)
                } else {
                    // If it's not a built-in or variable, try to execute as external command
                    self.execute_external_command(&name, &[])
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                self.apply_binary_operator(operator, left_val, right_val)
            }

            Expr::Unary { operator, operand } => {
                let operand_val = self.evaluate_expression(*operand)?;
                self.apply_unary_operator(operator, operand_val)
            }

            Expr::Call { name, args } => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate_expression(arg)?);
                }

                if is_builtin(&name) {
                    // Call built-in function
                    call_builtin(&name, &arg_values)
                } else {
                    // User-defined function
                    if let Some(Value::Function {
                        parameters,
                        body,
                        closure,
                        ..
                    }) = self.environment.get(&name)
                    {
                        if parameters.len() != arg_values.len() {
                            return Err(anyhow!(
                                "Function {} expects {} arguments, got {}",
                                name,
                                parameters.len(),
                                arg_values.len()
                            ));
                        }

                        // Create new environment for function execution
                        let previous = self.environment.clone();
                        self.environment = Environment::new_with_parent(closure);

                        // Bind parameters
                        for (param, arg_val) in parameters.iter().zip(arg_values.iter()) {
                            self.environment.define(param.name.clone(), arg_val.clone());
                        }

                        // Execute function body
                        let result = self.execute_block(body)?;

                        // Restore previous environment
                        self.environment = previous;
                        Ok(result)
                    } else {
                        // Try to execute as external command
                        self.execute_external_command(&name, &arg_values)
                    }
                }
            }

            Expr::Array(elements) => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.evaluate_expression(element)?);
                }
                Ok(Value::Array(values))
            }

            Expr::Map(pairs) => {
                let mut map = HashMap::new();
                for (key_expr, value_expr) in pairs {
                    let key = self.evaluate_expression(key_expr)?;
                    let value = self.evaluate_expression(value_expr)?;

                    match key {
                        Value::String(s) => {
                            map.insert(s, value);
                        }
                        _ => {
                            map.insert(key.to_string(), value);
                        }
                    }
                }
                Ok(Value::Map(map))
            }

            Expr::Command(cmd) => {
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .output()
                    .map_err(|e| anyhow!("Failed to execute command: {}", e))?;

                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(Value::String(stdout))
            }

            _ => Err(anyhow!("Expression type not yet implemented: {:?}", expr)),
        }
    }

    fn apply_binary_operator(
        &self,
        operator: BinaryOperator,
        left: Value,
        right: Value,
    ) -> Result<Value> {
        match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => match operator {
                BinaryOperator::Add => Ok(Value::Integer(l + r)),
                BinaryOperator::Subtract => Ok(Value::Integer(l - r)),
                BinaryOperator::Multiply => Ok(Value::Integer(l * r)),
                BinaryOperator::Divide => {
                    if r == 0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Integer(l / r))
                    }
                }
                BinaryOperator::Modulo => Ok(Value::Integer(l % r)),
                BinaryOperator::Equal => Ok(Value::Boolean(l == r)),
                BinaryOperator::NotEqual => Ok(Value::Boolean(l != r)),
                BinaryOperator::Less => Ok(Value::Boolean(l < r)),
                BinaryOperator::LessEqual => Ok(Value::Boolean(l <= r)),
                BinaryOperator::Greater => Ok(Value::Boolean(l > r)),
                BinaryOperator::GreaterEqual => Ok(Value::Boolean(l >= r)),
                _ => Err(anyhow!("Unsupported operator for integers: {:?}", operator)),
            },

            (Value::Float(l), Value::Float(r)) => match operator {
                BinaryOperator::Add => Ok(Value::Float(l + r)),
                BinaryOperator::Subtract => Ok(Value::Float(l - r)),
                BinaryOperator::Multiply => Ok(Value::Float(l * r)),
                BinaryOperator::Divide => {
                    if r == 0.0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Float(l / r))
                    }
                }
                BinaryOperator::Equal => Ok(Value::Boolean((l - r).abs() < f64::EPSILON)),
                BinaryOperator::NotEqual => Ok(Value::Boolean((l - r).abs() >= f64::EPSILON)),
                BinaryOperator::Less => Ok(Value::Boolean(l < r)),
                BinaryOperator::LessEqual => Ok(Value::Boolean(l <= r)),
                BinaryOperator::Greater => Ok(Value::Boolean(l > r)),
                BinaryOperator::GreaterEqual => Ok(Value::Boolean(l >= r)),
                _ => Err(anyhow!("Unsupported operator for floats: {:?}", operator)),
            },

            (Value::String(l), Value::String(r)) => match operator {
                BinaryOperator::Add => Ok(Value::String(l + &r)),
                BinaryOperator::Equal => Ok(Value::Boolean(l == r)),
                BinaryOperator::NotEqual => Ok(Value::Boolean(l != r)),
                _ => Err(anyhow!("Unsupported operator for strings: {:?}", operator)),
            },

            (Value::Integer(l), Value::Float(r)) => {
                self.apply_binary_operator(operator, Value::Float(l as f64), Value::Float(r))
            }

            (Value::Float(l), Value::Integer(r)) => {
                self.apply_binary_operator(operator, Value::Float(l), Value::Float(r as f64))
            }

            (Value::Boolean(l), Value::Boolean(r)) => match operator {
                BinaryOperator::And => Ok(Value::Boolean(l && r)),
                BinaryOperator::Or => Ok(Value::Boolean(l || r)),
                BinaryOperator::Equal => Ok(Value::Boolean(l == r)),
                BinaryOperator::NotEqual => Ok(Value::Boolean(l != r)),
                _ => Err(anyhow!("Unsupported operator for booleans: {:?}", operator)),
            },

            (l, r) => match operator {
                BinaryOperator::Equal => Ok(Value::Boolean(false)), // Different types are never equal
                BinaryOperator::NotEqual => Ok(Value::Boolean(true)),
                _ => Err(anyhow!(
                    "Unsupported operation: {} {:?} {}",
                    l.type_name(),
                    operator,
                    r.type_name()
                )),
            },
        }
    }

    fn apply_unary_operator(&self, operator: UnaryOperator, operand: Value) -> Result<Value> {
        match (operator, operand) {
            (UnaryOperator::Not, val) => Ok(Value::Boolean(!val.is_truthy())),
            (UnaryOperator::Minus, Value::Integer(i)) => Ok(Value::Integer(-i)),
            (UnaryOperator::Minus, Value::Float(f)) => Ok(Value::Float(-f)),
            (op, val) => Err(anyhow!(
                "Unsupported unary operation: {:?} {}",
                op,
                val.type_name()
            )),
        }
    }

    fn execute_external_command(&mut self, command: &str, args: &[Value]) -> Result<Value> {
        // Convert arguments to strings
        let mut cmd_args = Vec::new();
        for arg in args {
            cmd_args.push(arg.to_string());
        }

        // Execute the command
        let mut cmd = Command::new(command);
        cmd.args(&cmd_args);

        match cmd.output() {
            Ok(output) => {
                // Print stderr if there's any error output
                if !output.stderr.is_empty() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprint!("{}", stderr);
                }

                // Print stdout directly to avoid double output in REPL
                if !output.stdout.is_empty() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    print!("{}", stdout);
                }

                if output.status.success() {
                    Ok(Value::Nil)
                } else {
                    let exit_code = output.status.code().unwrap_or(-1);
                    Err(anyhow!(
                        "Command '{}' failed with exit code {}",
                        command,
                        exit_code
                    ))
                }
            }
            Err(e) => {
                // Check if it's a "command not found" type error
                if e.kind() == std::io::ErrorKind::NotFound {
                    Err(anyhow!("Command not found: {}", command))
                } else {
                    Err(anyhow!("Failed to execute command '{}': {}", command, e))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_simple_arithmetic() {
        let source = "2 + 3 * 4";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();

        let result = interpreter.interpret(program).unwrap();
        assert_eq!(result, Value::Integer(14));
    }

    #[test]
    fn test_variable_assignment() {
        let source = "x is 42\nx";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();

        let result = interpreter.interpret(program).unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_function_definition_and_call() {
        let source = r#"
            fn add(a: int, b: int) -> int {
                return a + b
            }
            add(2, 3)
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();

        let result = interpreter.interpret(program).unwrap();
        assert_eq!(result, Value::Integer(5));
    }

    #[test]
    fn test_const_declaration() {
        let source = "const x is 42\nx";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();

        let result = interpreter.interpret(program).unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_const_reassignment_fails() {
        let source = "const x is 42\nx = 10";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();

        let result = interpreter.interpret(program);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot assign to const variable: x"));
    }

    #[test]
    fn test_let_reassignment_succeeds() {
        let source = "let x is 42\nx = 10\nx";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();

        let result = interpreter.interpret(program).unwrap();
        assert_eq!(result, Value::Integer(10));
    }

    #[test]
    fn test_mixed_let_const_declarations() {
        let source = r#"
            let x is 10
            const y is 20
            x = 30
            x + y
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();

        let result = interpreter.interpret(program).unwrap();
        assert_eq!(result, Value::Integer(50));
    }
}
