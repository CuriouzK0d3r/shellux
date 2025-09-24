pub mod token;

pub use token::{keyword_or_identifier, Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
    line: usize,
    column: usize,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();

        Self {
            input,
            chars,
            current_char,
            line: 1,
            column: 1,
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            let is_eof = matches!(token.token_type, TokenType::EOF);
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        tokens
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start_line = self.line;
        let start_column = self.column;

        match self.current_char {
            None => Token::eof(self.line, self.column),

            Some('\n') => {
                self.advance();
                Token::new(TokenType::Newline, start_line, start_column, 1)
            }

            // Single-line comments
            Some('#') => {
                let comment = self.read_line_comment();
                let length = comment.len();
                Token::new(
                    TokenType::Comment(comment),
                    start_line,
                    start_column,
                    length,
                )
            }

            // Multi-line comments
            Some('/') if self.peek() == Some('*') => {
                let comment = self.read_block_comment();
                let length = comment.len();
                Token::new(
                    TokenType::Comment(comment),
                    start_line,
                    start_column,
                    length,
                )
            }

            // Numbers
            Some(c) if c.is_ascii_digit() => self.read_number(),

            // Identifiers and keywords
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.read_identifier(),

            // Strings
            Some('"') => self.read_string(),
            Some('\'') => self.read_single_quote_string(),

            // (Triple-quoted strings handled in read_string method)

            // Command execution $( ... )
            Some('$') if self.peek() == Some('(') => self.read_command(),

            // String interpolation ${ ... }
            Some('$') if self.peek() == Some('{') => {
                self.advance(); // consume '$'
                self.advance(); // consume '{'
                Token::new(TokenType::InterpolationStart, start_line, start_column, 2)
            }

            // Operators and punctuation
            Some('+') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::new(TokenType::PlusAssign, start_line, start_column, 2)
                } else {
                    Token::new(TokenType::Plus, start_line, start_column, 1)
                }
            }

            Some('-') => {
                self.advance();
                match self.current_char {
                    Some('=') => {
                        self.advance();
                        Token::new(TokenType::MinusAssign, start_line, start_column, 2)
                    }
                    Some('>') => {
                        self.advance();
                        Token::new(TokenType::Arrow, start_line, start_column, 2)
                    }
                    _ => Token::new(TokenType::Minus, start_line, start_column, 1),
                }
            }

            Some('*') => {
                self.advance();
                if self.current_char == Some('*') {
                    self.advance();
                    Token::new(TokenType::Power, start_line, start_column, 2)
                } else {
                    Token::new(TokenType::Multiply, start_line, start_column, 1)
                }
            }

            Some('/') => {
                self.advance();
                Token::new(TokenType::Divide, start_line, start_column, 1)
            }

            Some('%') => {
                self.advance();
                Token::new(TokenType::Modulo, start_line, start_column, 1)
            }

            Some('=') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::new(TokenType::Equal, start_line, start_column, 2)
                } else {
                    Token::new(TokenType::Assign, start_line, start_column, 1)
                }
            }

            Some('!') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::new(TokenType::NotEqual, start_line, start_column, 2)
                } else {
                    Token::new(TokenType::Not, start_line, start_column, 1)
                }
            }

            Some('<') => {
                self.advance();
                match self.current_char {
                    Some('=') => {
                        self.advance();
                        Token::new(TokenType::LessEqual, start_line, start_column, 2)
                    }
                    Some('<') => {
                        self.advance();
                        Token::new(TokenType::LeftShift, start_line, start_column, 2)
                    }
                    _ => Token::new(TokenType::Less, start_line, start_column, 1),
                }
            }

            Some('>') => {
                self.advance();
                match self.current_char {
                    Some('=') => {
                        self.advance();
                        Token::new(TokenType::GreaterEqual, start_line, start_column, 2)
                    }
                    Some('>') => {
                        self.advance();
                        Token::new(TokenType::RightShift, start_line, start_column, 2)
                    }
                    _ => Token::new(TokenType::Greater, start_line, start_column, 1),
                }
            }

            Some('&') => {
                self.advance();
                if self.current_char == Some('&') {
                    self.advance();
                    Token::new(TokenType::And, start_line, start_column, 2)
                } else {
                    Token::new(TokenType::BitwiseAnd, start_line, start_column, 1)
                }
            }

            Some('|') => {
                self.advance();
                match self.current_char {
                    Some('|') => {
                        self.advance();
                        Token::new(TokenType::Or, start_line, start_column, 2)
                    }
                    Some('>') => {
                        self.advance();
                        Token::new(TokenType::Pipeline, start_line, start_column, 2)
                    }
                    _ => Token::new(TokenType::BitwiseOr, start_line, start_column, 1),
                }
            }

            Some('^') => {
                self.advance();
                Token::new(TokenType::BitwiseXor, start_line, start_column, 1)
            }

            Some('~') => {
                self.advance();
                Token::new(TokenType::BitwiseNot, start_line, start_column, 1)
            }

            Some(':') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::new(TokenType::ColonAssign, start_line, start_column, 2)
                } else {
                    Token::new(TokenType::Colon, start_line, start_column, 1)
                }
            }

            Some('(') => {
                self.advance();
                Token::new(TokenType::LeftParen, start_line, start_column, 1)
            }

            Some(')') => {
                self.advance();
                Token::new(TokenType::RightParen, start_line, start_column, 1)
            }

            Some('{') => {
                self.advance();
                Token::new(TokenType::LeftBrace, start_line, start_column, 1)
            }

            Some('}') => {
                self.advance();
                Token::new(TokenType::RightBrace, start_line, start_column, 1)
            }

            Some('[') => {
                self.advance();
                Token::new(TokenType::LeftBracket, start_line, start_column, 1)
            }

            Some(']') => {
                self.advance();
                Token::new(TokenType::RightBracket, start_line, start_column, 1)
            }

            Some(',') => {
                self.advance();
                Token::new(TokenType::Comma, start_line, start_column, 1)
            }

            Some(';') => {
                self.advance();
                Token::new(TokenType::Semicolon, start_line, start_column, 1)
            }

            Some('.') => {
                self.advance();
                Token::new(TokenType::Dot, start_line, start_column, 1)
            }

            Some(c) => {
                self.advance();
                Token::error(
                    format!("Unexpected character: '{}'", c),
                    start_line,
                    start_column,
                )
            }
        }
    }

    fn advance(&mut self) {
        if self.current_char == Some('\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.position += 1;
        self.current_char = self.chars.next();
    }

    fn peek(&self) -> Option<char> {
        self.chars.as_str().chars().next()
    }

    fn peek_ahead(&self, n: usize) -> Option<char> {
        self.chars.as_str().chars().nth(n)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_line_comment(&mut self) -> String {
        let mut comment = String::new();

        // Skip the '#'
        self.advance();

        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }

        comment
    }

    fn read_block_comment(&mut self) -> String {
        let mut comment = String::new();

        // Skip the '/*'
        self.advance();
        self.advance();

        while let Some(c) = self.current_char {
            if c == '*' && self.peek() == Some('/') {
                self.advance(); // consume '*'
                self.advance(); // consume '/'
                break;
            }
            comment.push(c);
            self.advance();
        }

        comment
    }

    fn read_number(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut number_str = String::new();
        let mut is_float = false;

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                number_str.push(c);
                self.advance();
            } else if c == '.' && !is_float && self.peek().map_or(false, |ch| ch.is_ascii_digit()) {
                is_float = true;
                number_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let length = number_str.len();

        if is_float {
            match number_str.parse::<f64>() {
                Ok(value) => Token::new(TokenType::Float(value), start_line, start_column, length),
                Err(_) => Token::error(
                    format!("Invalid float: {}", number_str),
                    start_line,
                    start_column,
                ),
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(value) => {
                    Token::new(TokenType::Integer(value), start_line, start_column, length)
                }
                Err(_) => Token::error(
                    format!("Invalid integer: {}", number_str),
                    start_line,
                    start_column,
                ),
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut identifier = String::new();

        while let Some(c) = self.current_char {
            if c.is_ascii_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let length = identifier.len();
        let token_type = keyword_or_identifier(&identifier);

        Token::new(token_type, start_line, start_column, length)
    }

    fn read_string(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut string_value = String::new();

        // Skip opening quote
        self.advance();

        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance(); // consume closing quote
                let length = string_value.len() + 2; // +2 for quotes
                return Token::new(
                    TokenType::String(string_value),
                    start_line,
                    start_column,
                    length,
                );
            } else if c == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    match escaped {
                        'n' => string_value.push('\n'),
                        't' => string_value.push('\t'),
                        'r' => string_value.push('\r'),
                        '\\' => string_value.push('\\'),
                        '"' => string_value.push('"'),
                        '\'' => string_value.push('\''),
                        _ => {
                            string_value.push('\\');
                            string_value.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string_value.push(c);
                self.advance();
            }
        }

        Token::error("Unterminated string".to_string(), start_line, start_column)
    }

    fn read_single_quote_string(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut string_value = String::new();

        // Skip opening quote
        self.advance();

        while let Some(c) = self.current_char {
            if c == '\'' {
                self.advance(); // consume closing quote
                let length = string_value.len() + 2; // +2 for quotes
                return Token::new(
                    TokenType::String(string_value),
                    start_line,
                    start_column,
                    length,
                );
            } else {
                string_value.push(c);
                self.advance();
            }
        }

        Token::error("Unterminated string".to_string(), start_line, start_column)
    }

    fn read_triple_quote_string(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut string_value = String::new();

        // Skip opening triple quotes
        self.advance();
        self.advance();
        self.advance();

        while let Some(c) = self.current_char {
            if c == '"' && self.peek() == Some('"') && self.peek_ahead(2) == Some('"') {
                self.advance(); // consume first "
                self.advance(); // consume second "
                self.advance(); // consume third "
                let length = string_value.len() + 6; // +6 for triple quotes
                return Token::new(
                    TokenType::String(string_value),
                    start_line,
                    start_column,
                    length,
                );
            } else {
                string_value.push(c);
                self.advance();
            }
        }

        Token::error(
            "Unterminated triple-quoted string".to_string(),
            start_line,
            start_column,
        )
    }

    fn read_command(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut command = String::new();
        let mut paren_count = 0;

        // Skip '$('
        self.advance();
        self.advance();
        paren_count = 1;

        while let Some(c) = self.current_char {
            if c == '(' {
                paren_count += 1;
                command.push(c);
                self.advance();
            } else if c == ')' {
                paren_count -= 1;
                if paren_count == 0 {
                    self.advance(); // consume closing paren
                    let length = command.len() + 3; // +3 for $()
                    return Token::new(
                        TokenType::Command(command),
                        start_line,
                        start_column,
                        length,
                    );
                } else {
                    command.push(c);
                    self.advance();
                }
            } else {
                command.push(c);
                self.advance();
            }
        }

        Token::error("Unterminated command".to_string(), start_line, start_column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::TokenType;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("let x is 42");
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 5); // let, x, is, 42, EOF
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[1].token_type, TokenType::Identifier(_)));
        assert!(matches!(tokens[2].token_type, TokenType::Is));
        assert!(matches!(tokens[3].token_type, TokenType::Integer(42)));
        assert!(matches!(tokens[4].token_type, TokenType::EOF));
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new(r#""Hello, world!""#);
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 2); // string, EOF
        if let TokenType::String(s) = &tokens[0].token_type {
            assert_eq!(s, "Hello, world!");
        } else {
            panic!("Expected string token");
        }
    }

    #[test]
    fn test_command_execution() {
        let mut lexer = Lexer::new("$(ls -la)");
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 2); // command, EOF
        if let TokenType::Command(cmd) = &tokens[0].token_type {
            assert_eq!(cmd, "ls -la");
        } else {
            panic!("Expected command token");
        }
    }
}
