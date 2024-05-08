use crate::lib::Lexer::lexer::Lexer;
use crate::lib::Parser::parser::Parser;
use crate::lib::Asteva::context::Context;
use crate::lib::Asteva::asteva::evaluate_ast;
use crate::lib::Enums::token::Token;
use crate::lib::Enums::astnode::ASTNode;

/// A struct representing an interpreter for a custom language.
pub struct Interpreter<'a> {
    /// The code to be interpreted.
    code: &'a str,
}

impl<'a> Interpreter<'a> {
    /// Creates a new `Interpreter` instance with the given code.
    pub fn new(code: &'a str) -> Self {
        Interpreter { code }
    }

    /// Tokenizes the code and returns a vector of tokens.
    pub fn tokenize(&self) -> Vec<Token> {
        let mut lexer = Lexer::new(self.code);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        tokens
    }

    /// Parses the tokens into an abstract syntax tree (AST) and returns it.
    pub fn parse(&self, tokens: &[Token]) -> Vec<ASTNode> {
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    /// Evaluates the AST and returns a result indicating success or an error message.
    pub fn evaluate(&self, ast: &[ASTNode]) -> Result<(), String> {
        let mut context = Context::new();
        evaluate_ast(ast.to_vec(), &mut context)
    }

    /// Tokenizes the code, parses the tokens into an AST, and evaluates the AST.
    /// Returns a result indicating success or an error message.
    pub fn interpret(&self) -> Result<(), String> {
        let tokens = self.tokenize();
        let ast = self.parse(&tokens);
        self.evaluate(&ast)
    }
}
