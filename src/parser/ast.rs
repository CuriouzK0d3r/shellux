#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
    
    // Identifiers
    Identifier(String),
    
    // Binary operations
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    
    // Unary operations
    Unary {
        operator: UnaryOperator,
        operand: Box<Expr>,
    },
    
    // Function calls
    Call {
        name: String,
        args: Vec<Expr>,
    },
    
    // Method calls
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    
    // Array literals
    Array(Vec<Expr>),
    
    // Map literals
    Map(Vec<(Expr, Expr)>),
    
    // Array/Map indexing
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    
    // Field access
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
    
    // String interpolation
    Interpolation(Vec<InterpolationPart>),
    
    // Command execution
    Command(String),
    
    // Pipeline
    Pipeline {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    
    // Range
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationPart {
    Text(String),
    Expression(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    
    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Logical
    And,
    Or,
    
    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
    BitwiseNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Expression statement
    Expression(Expr),
    
    // Variable declarations
    Let {
        name: String,
        type_annotation: Option<Type>,
        value: Expr,
    },
    
    Const {
        name: String,
        type_annotation: Option<Type>,
        value: Expr,
    },
    
    // Assignment
    Assignment {
        target: AssignmentTarget,
        operator: AssignmentOperator,
        value: Expr,
    },
    
    // Control flow
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    
    For {
        variable: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    
    // Functions
    Function {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Stmt>,
    },
    
    Return(Option<Expr>),
    
    // Error handling
    Try {
        body: Vec<Stmt>,
        catch_clauses: Vec<CatchClause>,
    },
    
    // Pattern matching
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
    },
    
    // Control flow
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentTarget {
    Identifier(String),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub exception_type: Option<String>,
    pub variable: Option<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(Expr),
    Identifier(String),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Array(Box<Type>),
    Map {
        key_type: Box<Type>,
        value_type: Box<Type>,
    },
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
    Custom(String),
    Any,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
    
    pub fn add_statement(&mut self, stmt: Stmt) {
        self.statements.push(stmt);
    }
}