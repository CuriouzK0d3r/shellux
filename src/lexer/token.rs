#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),

    // Identifiers and Keywords
    Identifier(String),

    // Keywords
    Let,
    Const,
    Fn,
    Return,
    If,
    Else,
    For,
    While,
    In,
    Try,
    Catch,
    Match,
    True,
    False,
    Nil,
    Is,

    // Operators
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %
    Power,    // **

    // Assignment
    Assign,      // =
    ColonAssign, // :=
    PlusAssign,  // +=
    MinusAssign, // -=

    // Comparison
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // Logical
    And, // &&
    Or,  // ||
    Not, // !

    // Bitwise
    BitwiseAnd, // &
    BitwiseOr,  // |
    BitwiseXor, // ^
    BitwiseNot, // ~
    LeftShift,  // <<
    RightShift, // >>

    // Punctuation
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;
    Colon,        // :
    Dot,          // .
    Arrow,        // ->
    Pipeline,     // |>

    // String interpolation
    InterpolationStart, // ${
    InterpolationEnd,   // }

    // Command execution
    Command(String), // $(command)

    // Comments
    Comment(String),

    // Special
    Newline,
    EOF,

    // Error
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize, length: usize) -> Self {
        Self {
            token_type,
            line,
            column,
            length,
        }
    }

    pub fn error(message: String, line: usize, column: usize) -> Self {
        Self {
            token_type: TokenType::Error(message),
            line,
            column,
            length: 1,
        }
    }

    pub fn eof(line: usize, column: usize) -> Self {
        Self {
            token_type: TokenType::EOF,
            line,
            column,
            length: 0,
        }
    }
}

pub fn keyword_or_identifier(word: &str) -> TokenType {
    match word {
        "let" => TokenType::Let,
        "const" => TokenType::Const,
        "fn" => TokenType::Fn,
        "return" => TokenType::Return,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "for" => TokenType::For,
        "while" => TokenType::While,
        "in" => TokenType::In,
        "try" => TokenType::Try,
        "catch" => TokenType::Catch,
        "match" => TokenType::Match,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "nil" => TokenType::Nil,
        "is" => TokenType::Is,
        _ => TokenType::Identifier(word.to_string()),
    }
}
