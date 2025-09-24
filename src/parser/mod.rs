pub mod ast;

use crate::lexer::{token::Token, token::TokenType};
use anyhow::{anyhow, Result};
use ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::new();

        while !self.is_at_end() {
            // Skip newlines and comments at the top level
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            if self.skip_comment() {
                continue;
            }

            match self.parse_statement() {
                Ok(stmt) => program.add_statement(stmt),
                Err(e) => return Err(e),
            }
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        if self.match_keywords(&[TokenType::Let]) {
            self.parse_let_statement()
        } else if self.match_keywords(&[TokenType::Const]) {
            self.parse_const_statement()
        } else if self.match_keywords(&[TokenType::Fn]) {
            self.parse_function_statement()
        } else if self.match_keywords(&[TokenType::If]) {
            self.parse_if_statement()
        } else if self.match_keywords(&[TokenType::For]) {
            self.parse_for_statement()
        } else if self.match_keywords(&[TokenType::While]) {
            self.parse_while_statement()
        } else if self.match_keywords(&[TokenType::Return]) {
            self.parse_return_statement()
        } else if self.match_keywords(&[TokenType::Try]) {
            self.parse_try_statement()
        } else if self.match_keywords(&[TokenType::Match]) {
            self.parse_match_statement()
        } else {
            // Check for 'is' assignment (variable declaration)
            if self.is_is_assignment() {
                self.parse_is_assignment_statement()
            } else if self.is_assignment() {
                self.parse_assignment_statement()
            } else {
                // Expression statement
                let expr = self.parse_expression()?;
                self.consume_newline_or_eof()?;
                Ok(Stmt::Expression(expr))
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Stmt> {
        let name = self.expect_identifier()?;

        let type_annotation = if self.match_token(&TokenType::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect_token(&TokenType::Is)?;
        let value = self.parse_expression()?;
        self.consume_newline_or_eof()?;

        Ok(Stmt::Let {
            name,
            type_annotation,
            value,
        })
    }

    fn parse_const_statement(&mut self) -> Result<Stmt> {
        let name = self.expect_identifier()?;

        let type_annotation = if self.match_token(&TokenType::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect_token(&TokenType::Is)?;
        let value = self.parse_expression()?;
        self.consume_newline_or_eof()?;

        Ok(Stmt::Const {
            name,
            type_annotation,
            value,
        })
    }

    fn parse_function_statement(&mut self) -> Result<Stmt> {
        let name = self.expect_identifier()?;
        self.expect_token(&TokenType::LeftParen)?;

        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let param_name = self.expect_identifier()?;
                let type_annotation = if self.match_token(&TokenType::Colon) {
                    Some(self.parse_type()?)
                } else {
                    None
                };

                parameters.push(Parameter {
                    name: param_name,
                    type_annotation,
                });

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.expect_token(&TokenType::RightParen)?;

        let return_type = if self.match_token(&TokenType::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect_token(&TokenType::LeftBrace)?;
        let body = self.parse_block()?;

        Ok(Stmt::Function {
            name,
            parameters,
            return_type,
            body,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Stmt> {
        let condition = self.parse_expression()?;
        self.expect_token(&TokenType::LeftBrace)?;
        let then_block = self.parse_block()?;

        let else_block = if self.match_keywords(&[TokenType::Else]) {
            self.expect_token(&TokenType::LeftBrace)?;
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_block,
            else_block,
        })
    }

    fn parse_for_statement(&mut self) -> Result<Stmt> {
        let variable = self.expect_identifier()?;
        self.expect_token(&TokenType::In)?;
        let iterable = self.parse_expression()?;
        self.expect_token(&TokenType::LeftBrace)?;
        let body = self.parse_block()?;

        Ok(Stmt::For {
            variable,
            iterable,
            body,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Stmt> {
        let condition = self.parse_expression()?;
        self.expect_token(&TokenType::LeftBrace)?;
        let body = self.parse_block()?;

        Ok(Stmt::While { condition, body })
    }

    fn parse_return_statement(&mut self) -> Result<Stmt> {
        let value = if self.check(&TokenType::Newline) || self.is_at_end() {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume_newline_or_eof()?;
        Ok(Stmt::Return(value))
    }

    fn parse_try_statement(&mut self) -> Result<Stmt> {
        self.expect_token(&TokenType::LeftBrace)?;
        let body = self.parse_block()?;

        let mut catch_clauses = Vec::new();
        while self.match_keywords(&[TokenType::Catch]) {
            let exception_type = if let Ok(name) = self.expect_identifier() {
                Some(name)
            } else {
                None
            };

            let variable = if self.match_keywords(&[TokenType::Identifier("as".to_string())]) {
                Some(self.expect_identifier()?)
            } else {
                None
            };

            self.expect_token(&TokenType::LeftBrace)?;
            let catch_body = self.parse_block()?;

            catch_clauses.push(CatchClause {
                exception_type,
                variable,
                body: catch_body,
            });
        }

        Ok(Stmt::Try {
            body,
            catch_clauses,
        })
    }

    fn parse_match_statement(&mut self) -> Result<Stmt> {
        let expr = self.parse_expression()?;
        self.expect_token(&TokenType::LeftBrace)?;

        let mut arms = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            self.expect_token(&TokenType::Arrow)?;
            self.expect_token(&TokenType::LeftBrace)?;
            let body = self.parse_block()?;

            arms.push(MatchArm { pattern, body });
        }

        self.expect_token(&TokenType::RightBrace)?;
        Ok(Stmt::Match { expr, arms })
    }

    fn parse_assignment_statement(&mut self) -> Result<Stmt> {
        let target = self.parse_assignment_target()?;

        let operator = if self.match_token(&TokenType::Assign) {
            AssignmentOperator::Assign
        } else if self.match_token(&TokenType::PlusAssign) {
            AssignmentOperator::AddAssign
        } else if self.match_token(&TokenType::MinusAssign) {
            AssignmentOperator::SubtractAssign
        } else {
            return Err(anyhow!("Expected assignment operator"));
        };

        let value = self.parse_expression()?;
        self.consume_newline_or_eof()?;

        Ok(Stmt::Assignment {
            target,
            operator,
            value,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            if self.skip_comment() {
                continue;
            }

            statements.push(self.parse_statement()?);
        }

        self.expect_token(&TokenType::RightBrace)?;
        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr> {
        let mut expr = self.parse_and()?;

        while self.match_token(&TokenType::Or) {
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr> {
        let mut expr = self.parse_equality()?;

        while self.match_token(&TokenType::And) {
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = self.match_equality_operator() {
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut expr = self.parse_term()?;

        while let Some(op) = self.match_comparison_operator() {
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = self.match_term_operator() {
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = self.match_factor_operator() {
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        if let Some(op) = self.match_unary_operator() {
            let operand = self.parse_unary()?;
            Ok(Expr::Unary {
                operator: op,
                operand: Box::new(operand),
            })
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&TokenType::LeftParen) {
                // Function call
                let mut args = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.expect_token(&TokenType::RightParen)?;

                if let Expr::Identifier(name) = expr {
                    expr = Expr::Call { name, args };
                } else {
                    return Err(anyhow!("Invalid function call"));
                }
            } else if self.match_token(&TokenType::LeftBracket) {
                // Array/Map indexing
                let index = self.parse_expression()?;
                self.expect_token(&TokenType::RightBracket)?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&TokenType::Dot) {
                // Field access or method call
                let field = self.expect_identifier()?;

                if self.match_token(&TokenType::LeftParen) {
                    // Method call
                    let mut args = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect_token(&TokenType::RightParen)?;

                    expr = Expr::MethodCall {
                        object: Box::new(expr),
                        method: field,
                        args,
                    };
                } else {
                    expr = Expr::FieldAccess {
                        object: Box::new(expr),
                        field,
                    };
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        if let Some(token) = self.advance() {
            match &token.token_type {
                TokenType::Integer(n) => Ok(Expr::Integer(*n)),
                TokenType::Float(f) => Ok(Expr::Float(*f)),
                TokenType::String(s) => Ok(Expr::String(s.clone())),
                TokenType::Boolean(b) => Ok(Expr::Boolean(*b)),
                TokenType::True => Ok(Expr::Boolean(true)),
                TokenType::False => Ok(Expr::Boolean(false)),
                TokenType::Nil => Ok(Expr::Nil),
                TokenType::Identifier(name) => Ok(Expr::Identifier(name.clone())),
                TokenType::Command(cmd) => Ok(Expr::Command(cmd.clone())),
                TokenType::LeftParen => {
                    let expr = self.parse_expression()?;
                    self.expect_token(&TokenType::RightParen)?;
                    Ok(expr)
                }
                TokenType::LeftBracket => {
                    let mut elements = Vec::new();
                    if !self.check(&TokenType::RightBracket) {
                        loop {
                            elements.push(self.parse_expression()?);
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect_token(&TokenType::RightBracket)?;
                    Ok(Expr::Array(elements))
                }
                TokenType::LeftBrace => {
                    let mut pairs = Vec::new();
                    if !self.check(&TokenType::RightBrace) {
                        loop {
                            let key = self.parse_expression()?;
                            self.expect_token(&TokenType::Colon)?;
                            let value = self.parse_expression()?;
                            pairs.push((key, value));
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect_token(&TokenType::RightBrace)?;
                    Ok(Expr::Map(pairs))
                }
                _ => Err(anyhow!("Unexpected token: {:?}", token.token_type)),
            }
        } else {
            Err(anyhow!("Unexpected end of input"))
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern> {
        if self.match_token(&TokenType::Identifier("_".to_string())) {
            Ok(Pattern::Wildcard)
        } else if let Ok(expr) = self.parse_expression() {
            match expr {
                Expr::Identifier(name) => Ok(Pattern::Identifier(name)),
                _ => Ok(Pattern::Literal(expr)),
            }
        } else {
            Err(anyhow!("Expected pattern"))
        }
    }

    fn parse_assignment_target(&mut self) -> Result<AssignmentTarget> {
        let expr = self.parse_postfix()?;
        match expr {
            Expr::Identifier(name) => Ok(AssignmentTarget::Identifier(name)),
            Expr::Index { object, index } => Ok(AssignmentTarget::Index { object, index }),
            Expr::FieldAccess { object, field } => {
                Ok(AssignmentTarget::FieldAccess { object, field })
            }
            _ => Err(anyhow!("Invalid assignment target")),
        }
    }

    fn parse_type(&mut self) -> Result<Type> {
        if let Some(token) = self.advance() {
            match &token.token_type {
                TokenType::Identifier(name) => match name.as_str() {
                    "int" => Ok(Type::Int),
                    "float" => Ok(Type::Float),
                    "string" => Ok(Type::String),
                    "bool" => Ok(Type::Bool),
                    "any" => Ok(Type::Any),
                    _ => Ok(Type::Custom(name.clone())),
                },
                _ => Err(anyhow!("Expected type")),
            }
        } else {
            Err(anyhow!("Expected type"))
        }
    }

    // Helper methods

    fn is_is_assignment(&mut self) -> bool {
        let start = self.current;

        // Check if we have identifier is pattern
        if let Some(token) = self.peek() {
            if matches!(token.token_type, TokenType::Identifier(_)) {
                self.advance();
                let result = matches!(self.peek().map(|t| &t.token_type), Some(TokenType::Is));
                self.current = start; // Reset position
                return result;
            }
        }
        self.current = start; // Reset position
        false
    }

    fn parse_is_assignment_statement(&mut self) -> Result<Stmt> {
        let name = self.expect_identifier()?;
        self.expect_token(&TokenType::Is)?;
        let value = self.parse_expression()?;
        self.consume_newline_or_eof()?;

        Ok(Stmt::Let {
            name,
            type_annotation: None,
            value,
        })
    }

    fn is_assignment(&mut self) -> bool {
        let start = self.current;

        // Try to parse assignment target
        if self.parse_assignment_target().is_ok() {
            let result = matches!(
                self.peek().map(|t| &t.token_type),
                Some(TokenType::Assign)
                    | Some(TokenType::PlusAssign)
                    | Some(TokenType::MinusAssign)
            );
            self.current = start; // Reset position
            result
        } else {
            self.current = start; // Reset position
            false
        }
    }

    fn match_keywords(&mut self, keywords: &[TokenType]) -> bool {
        if let Some(token) = self.peek() {
            for keyword in keywords {
                if std::mem::discriminant(&token.token_type) == std::mem::discriminant(keyword) {
                    self.advance();
                    return true;
                }
            }
        }
        false
    }

    fn match_token(&mut self, expected: &TokenType) -> bool {
        if let Some(token) = self.peek() {
            if std::mem::discriminant(&token.token_type) == std::mem::discriminant(expected) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_equality_operator(&mut self) -> Option<BinaryOperator> {
        if let Some(token) = self.peek() {
            let op = match &token.token_type {
                TokenType::Equal => Some(BinaryOperator::Equal),
                TokenType::NotEqual => Some(BinaryOperator::NotEqual),
                _ => None,
            };
            if op.is_some() {
                self.advance();
            }
            op
        } else {
            None
        }
    }

    fn match_comparison_operator(&mut self) -> Option<BinaryOperator> {
        if let Some(token) = self.peek() {
            let op = match &token.token_type {
                TokenType::Less => Some(BinaryOperator::Less),
                TokenType::LessEqual => Some(BinaryOperator::LessEqual),
                TokenType::Greater => Some(BinaryOperator::Greater),
                TokenType::GreaterEqual => Some(BinaryOperator::GreaterEqual),
                _ => None,
            };
            if op.is_some() {
                self.advance();
            }
            op
        } else {
            None
        }
    }

    fn match_term_operator(&mut self) -> Option<BinaryOperator> {
        if let Some(token) = self.peek() {
            let op = match &token.token_type {
                TokenType::Plus => Some(BinaryOperator::Add),
                TokenType::Minus => Some(BinaryOperator::Subtract),
                _ => None,
            };
            if op.is_some() {
                self.advance();
            }
            op
        } else {
            None
        }
    }

    fn match_factor_operator(&mut self) -> Option<BinaryOperator> {
        if let Some(token) = self.peek() {
            let op = match &token.token_type {
                TokenType::Multiply => Some(BinaryOperator::Multiply),
                TokenType::Divide => Some(BinaryOperator::Divide),
                TokenType::Modulo => Some(BinaryOperator::Modulo),
                TokenType::Power => Some(BinaryOperator::Power),
                _ => None,
            };
            if op.is_some() {
                self.advance();
            }
            op
        } else {
            None
        }
    }

    fn match_unary_operator(&mut self) -> Option<UnaryOperator> {
        if let Some(token) = self.peek() {
            let op = match &token.token_type {
                TokenType::Not => Some(UnaryOperator::Not),
                TokenType::Minus => Some(UnaryOperator::Minus),
                TokenType::BitwiseNot => Some(UnaryOperator::BitwiseNot),
                _ => None,
            };
            if op.is_some() {
                self.advance();
            }
            op
        } else {
            None
        }
    }

    fn expect_token(&mut self, expected: &TokenType) -> Result<()> {
        if let Some(token) = self.advance() {
            if std::mem::discriminant(&token.token_type) == std::mem::discriminant(expected) {
                Ok(())
            } else {
                Err(anyhow!(
                    "Expected {:?}, found {:?}",
                    expected,
                    token.token_type
                ))
            }
        } else {
            Err(anyhow!("Unexpected end of input"))
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        if let Some(token) = self.advance() {
            match &token.token_type {
                TokenType::Identifier(name) => Ok(name.clone()),
                _ => Err(anyhow!("Expected identifier, found {:?}", token.token_type)),
            }
        } else {
            Err(anyhow!("Expected identifier, found end of input"))
        }
    }

    fn consume_newline_or_eof(&mut self) -> Result<()> {
        // Skip any newlines
        while self.match_token(&TokenType::Newline) || self.skip_comment() {
            // Keep consuming newlines and comments
        }

        // Always succeed - we're more lenient about statement termination
        Ok(())
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if let Some(token) = self.peek() {
            std::mem::discriminant(&token.token_type) == std::mem::discriminant(token_type)
        } else {
            false
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            let token = &self.tokens[self.current];
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
            || self
                .peek()
                .map_or(true, |t| matches!(t.token_type, TokenType::EOF))
    }

    fn skip_comment(&mut self) -> bool {
        if let Some(token) = self.peek() {
            if matches!(token.token_type, TokenType::Comment(_)) {
                self.advance();
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_let() {
        let source = "name is \"World\"";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Let { name, value, .. } => {
                assert_eq!(name, "name");
                assert!(matches!(value, Expr::String(_)));
            }
            _ => panic!("Expected let statement"),
        }
    }

    #[test]
    fn test_parse_function_call() {
        let source = "print(\"Hello\")";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Expression(Expr::Call { name, args }) => {
                assert_eq!(name, "print");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected function call"),
        }
    }
}
