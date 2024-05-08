use crate::lib::Enums::token::Token;

/// Lexer for tokenizing input characters.
pub struct Lexer {
    /// The input characters.
    pub input: Vec<char>,
    /// The current position in the input.
    pub position: usize,
}

impl Lexer {
    /// Creates a new Lexer instance with the given input string.
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Advances the lexer position by one and returns the character at the new position.
    pub fn advance(&mut self) -> Option<char> {
        let current_char = self.input.get(self.position).cloned();
        self.position += 1;
        current_char
    }

    /// Returns the next character in the input without consuming it.
    pub fn peek(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    /// Skips whitespace characters in the input.
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Reads an identifier from the input.
    pub fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        identifier
    }

    /// Reads a number from the input.
    pub fn read_number(&mut self) -> i32 {
        let mut number = String::new();
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }

    /// Reads a string literal from the input.
    pub fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance(); // Consume the opening quote
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            string.push(c);
            self.advance();
        }
        self.advance(); // Consume the closing quote
        string
    }

   /// Returns the next token from the input.
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
    
        if self.position >= self.input.len() {
            return None;
        }
    
        let current_char = self.input[self.position];
        let token = match current_char {
            '=' => {
                self.position += 1;
                if let Some('=') = self.peek() {
                    self.position += 1;
                    Some(Token::Equals)
                } else {
                    Some(Token::Assign)
                }
            }
            '+' => {
                self.position += 1;
                Some(Token::Plus)
            }
            '-' => {
                self.position += 1;
                Some(Token::Minus)
            }
            '*' => {
                self.position += 1;
                Some(Token::Multiply)
            }
            '/' => {
                self.position += 1;
                Some(Token::Divide)
            }
            '%' => {
                self.position += 1;
                Some(Token::Modulo)
            }
            '(' => {
                self.position += 1;
                Some(Token::LParen)
            }
            ')' => {
                self.position += 1;
                Some(Token::RParen)
            }
            '{' => {
                self.position += 1;
                Some(Token::LBrace)
            }
            '}' => {
                self.position += 1;
                Some(Token::RBrace)
            }
            ';' => {
                self.position += 1;
                Some(Token::Semicolon)
            }
            ',' => {
                self.position += 1;
                Some(Token::Comma)
            }
            '<' => {
                self.position += 1;
                Some(Token::LessThan)
            }
            '>' => {
                self.position += 1;
                Some(Token::GreaterThan)
            }
            '!' => {
                self.position += 1;
                Some(Token::Not)
            }
            '&' => {
                self.position += 1;
                if let Some('&') = self.peek() {
                    self.position += 1;
                    Some(Token::And)
                } else {
                    None
                }
            }
            '|' => {
                self.position += 1;
                if let Some('|') = self.peek() {
                    self.position += 1;
                    Some(Token::Or)
                } else {
                    None
                }
            }
            '0'..='9' => Some(Token::IntegerLiteral(self.read_number())),
            'a'..='z' | 'A'..='Z' => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "variable" => Some(Token::Variable),
                    "integer" => Some(Token::Integer),
                    "boolean" => Some(Token::Boolean),
                    "loop" => Some(Token::Loop),
                    "function" => Some(Token::Function),
                    "return" => Some(Token::Return),
                    "true" => Some(Token::BooleanLiteral(true)),
                    "false" => Some(Token::BooleanLiteral(false)),
                    "if" => Some(Token::If),
                    "else" => Some(Token::Else),
                    "print" => Some(Token::Print),
                    "call" => Some(Token::Call),
                    _ => Some(Token::Identifier(identifier)),
                }
            }
            '"' => Some(Token::StringLiteral(self.read_string())),
            _ => None,
        };
    
        token
    }
    

}