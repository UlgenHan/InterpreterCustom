/// Represents a token in the lexer.
#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Variable,
    Integer,
    Boolean,
    Loop,
    Function,
    Return,
    Call,

    // Identifiers
    Identifier(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Not,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Literals
    IntegerLiteral(i32),
    BooleanLiteral(bool),
    StringLiteral(String),
    // Function Call
    Print,

    If,
    Else,
    // End of file
    EOF,
    True,
    False,
    Star,
    Slash,
    Percent
}

impl Token {
    /// Converts the token to a string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use enums::Token;
    ///
    /// let token = Token::IntegerLiteral(42);
    /// assert_eq!(token.to_string(), "IntegerLiteral(42)");
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            Token::Variable => "Variable".to_string(),
            Token::Integer => "Integer".to_string(),
            Token::Boolean => "Boolean".to_string(),
            Token::Loop => "Loop".to_string(),
            Token::Function => "Function".to_string(),
            Token::Return => "Return".to_string(),
            Token::Identifier(name) => format!("Identifier({})", name),
            Token::Assign => "Assign".to_string(),
            Token::Plus => "Plus".to_string(),
            Token::Minus => "Minus".to_string(),
            Token::Multiply => "Multiply".to_string(),
            Token::Divide => "Divide".to_string(),
            Token::Modulo => "Modulo".to_string(),
            Token::Equals => "Equals".to_string(),
            Token::NotEquals => "NotEquals".to_string(),
            Token::LessThan => "LessThan".to_string(),
            Token::LessThanOrEqual => "LessThanOrEqual".to_string(),
            Token::GreaterThan => "GreaterThan".to_string(),
            Token::GreaterThanOrEqual => "GreaterThanOrEqual".to_string(),
            Token::And => "And".to_string(),
            Token::Or => "Or".to_string(),
            Token::Not => "Not".to_string(),
            Token::Comma => "Comma".to_string(),
            Token::Semicolon => "Semicolon".to_string(),
            Token::LParen => "LParen".to_string(),
            Token::RParen => "RParen".to_string(),
            Token::LBrace => "LBrace".to_string(),
            Token::RBrace => "RBrace".to_string(),
            Token::IntegerLiteral(value) => format!("IntegerLiteral({})", value),
            Token::BooleanLiteral(value) => format!("BooleanLiteral({})", value),
            Token::StringLiteral(value) => format!("StringLiteral({})", value),
            Token::Print => "Print".to_string(),
            Token::If => "If".to_string(),
            Token::Else => "Else".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::True => "True".to_string(),
            Token::False => "False".to_string(),
            Token::Star => "Star".to_string(),
            Token::Slash => "Slash".to_string(),
            Token::Percent => "Percent".to_string(),
            Token::Call => "Call".to_string(),
        }
    }
}
