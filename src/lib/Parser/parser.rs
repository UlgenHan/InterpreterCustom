use crate::lib::Enums::token::Token;
use crate::lib::Enums::astnode::ASTNode;

/// Parser for parsing tokens into an Abstract Syntax Tree (AST).
pub struct Parser<'a> {
    /// The slice of tokens to be parsed.
    pub tokens: &'a [Token],
    /// The current token being parsed.
    pub current_token: Option<&'a Token>,
    /// The current position in the token slice.
    pub position: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` instance with the given tokens.
    pub fn new(tokens: &'a [Token]) -> Self {
        let current_token = tokens.get(0);
        Parser {
            tokens,
            current_token,
            position: 0,
        }
    }

    /// Advances the parser to the next token.
    pub fn advance(&mut self) {
        self.position += 1;
        self.current_token = self.tokens.get(self.position);
    }

    /// Parses the tokens into an AST.
    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut statements = Vec::new();
        while let Some(token) = self.current_token {
            println!("Current token: {:?}", token); // Print current token
            match token {
                Token::Variable => {
                    self.advance(); // Consume "variable"
                    println!("I am inside the token::variable arm");
                    let var_type = match self.current_token {
                        Some(Token::Integer) => {
                            println!("I am a integer");
                            self.advance(); // Consume "integer"
                            "integer".to_string()
                        },
                        Some(Token::Boolean) => {
                            self.advance(); // Consume "boolean"
                            "boolean".to_string()
                        },
                        _ => panic!("Expected variable type after 'variable' keyword"),
                    };
    
                    let name = match self.current_token {
                        Some(Token::Identifier(name)) => {
                            println!("I am inside the identifier");
                            println!("{:?}", self.current_token);
                            name.clone()
                        },
                        _ => panic!("Expected identifier after variable type"),
                    };
                    self.advance(); // Consume identifier
    
                    if let Some(Token::Assign) = self.current_token {
                        self.advance(); // Consume "="
                        let value = self.parse_expression();
                        if let Some(Token::Semicolon) = self.current_token {
                            self.advance(); // Consume ";"
                            statements.push(ASTNode::VariableDeclaration {
                                var_type,
                                name,
                                value: Box::new(value),
                            });
                            println!("{:?}", statements);
                        } else {
                            panic!("Expected semicolon after variable declaration");
                        }
                    } else {
                        panic!("Expected assignment operator after variable declaration");
                    }
                }
                Token::Function => {
                    self.advance(); // Consume "function"
                    
                    // Parse return type
                    let return_type = match self.current_token {
                        Some(Token::Integer) => {
                            self.advance(); // Consume "integer"
                            "integer".to_string()
                        },
                        Some(Token::Boolean) => {
                            self.advance(); // Consume "boolean"
                            "boolean".to_string()
                        },
                        _ => panic!("Expected return type after 'function' keyword"),
                    };
                    
                    // Parse function name
                    let name = match self.current_token {
                        Some(Token::Identifier(name)) => {
                            self.advance(); // Consume function name
                            name.clone()
                        },
                        _ => panic!("Expected function name after return type"),
                    };
                    
                    // Parse parameter list
                    if let Some(Token::LParen) = self.current_token {
                        self.advance(); // Consume "("
                        let parameters = self.parse_parameters();
                        if let Some(Token::RParen) = self.current_token {
                            self.advance(); // Consume ")"
                            
                            // Parse function body
                            if let Some(Token::LBrace) = self.current_token {
                                self.advance(); // Consume "{"
                                let body = self.parse_block();
                                
                                // Create FunctionDeclaration node
                                statements.push(ASTNode::FunctionDeclaration {
                                    name,
                                    return_type,
                                    parameters,
                                    body,
                                });
                            } else {
                                panic!("Expected function body after parameters");
                            }
                        } else {
                            panic!("Expected closing parenthesis after parameters");
                        }
                    } else {
                        panic!("Expected opening parenthesis after function name");
                    }
                },
                Token::Return => {
                    self.advance(); // Consume "return"
                    let value = self.parse_expression();
                    if let Some(Token::Semicolon) = self.current_token {
                        self.advance(); // Consume ";"
                        statements.push(ASTNode::ReturnStatement(Box::new(value)));
                    } else {
                        panic!("Expected semicolon after return statement");
                    }
                },
                Token::RBrace => {
                    // Consume "}"
                    
                    break; // Exit the loop when encountering a closing brace
                },
                Token::Loop => {
                    self.advance(); // Consume "loop"
                    self.advance(); // Consume "("
                    let condition = self.parse_expression(); // Parse the loop condition
                    self.advance(); // Consume ")"
                    if let Some(Token::LBrace) = self.current_token {
                        self.advance(); // Consume "{"
                        let body = self.parse_block(); // Parse the block of statements inside the loop
                        statements.push(ASTNode::Loop { condition:Box::new(condition), body:body });
                    } else {
                        panic!("Expected opening brace after loop condition");
                    }
                },
                Token::Identifier(_) => {
                    let identifier_name = match self.current_token {
                        Some(Token::Identifier(name)) => name.clone(),
                        _ => panic!("Expected identifier"),
                    };
                    
                    self.advance(); // Consume identifier
                
                    if let Some(Token::Assign) = self.current_token {
                        self.advance(); // Consume "="
                        let value = self.parse_expression(); // Parse the value to be assigned
                
                        if let Some(Token::Semicolon) = self.current_token {
                            self.advance(); // Consume ";"
                            let assignment_node = ASTNode::Assignment {
                                identifier: identifier_name,
                                value: Box::new(value),
                            };
                            statements.push(assignment_node);
                        } else {
                            panic!("Expected semicolon after assignment");
                        }
                    } else {
                        panic!("Expected assignment operator after identifier");
                    }
                },
                Token::If => {
                    self.advance(); // Consume "if"
                
                    // Parse condition
                    if let Some(Token::LParen) = self.current_token {
                        self.advance(); // Consume "("
                        let condition = self.parse_expression();
                        if let Some(Token::RParen) = self.current_token {
                            self.advance(); // Consume ")"
                
                            // Parse if block
                            if let Some(Token::LBrace) = self.current_token {
                                self.advance(); // Consume "{"
                                let if_block = self.parse_block();
                
                                // Parse else block
                                let else_block = if let Some(Token::Else) = self.current_token {
                                    self.advance(); // Consume "else"
                                    if let Some(Token::LBrace) = self.current_token {
                                        self.advance(); // Consume "{"
                                        let else_block = self.parse_block();
                                        Some(else_block)
                                    } else {
                                        panic!("Expected block after 'else'");
                                    }
                                } else {
                                    None
                                };
                
                                // Create IfElse node
                                statements.push(ASTNode::IfElse {
                                    condition: Box::new(condition),
                                    if_block,
                                    else_block,
                                });
                            } else {
                                panic!("Expected block after if condition");
                            }
                        } else {
                            panic!("Expected closing parenthesis after if condition");
                        }
                    } else {
                        panic!("Expected opening parenthesis after 'if'");
                    }
                },
                Token::Else => {
                    self.advance(); // Consume "else"
                    
                    // Parse else block
                    if let Some(Token::LBrace) = self.current_token {
                        self.advance(); // Consume "{"
                        let else_block = self.parse_block();
                        
                        statements.push(ASTNode::ElseStatement{
                            body: else_block,
                        });
                    } else {
                        panic!("Expected block after 'else'");
                    }
                },
                Token::Print => {
                    self.advance(); // Consume "print"
                    let expr = self.parse_expression();
                    if let Some(Token::Semicolon) = self.current_token {
                        self.advance(); // Consume ";"
                        
                        statements.push(ASTNode::PrintStatement { context: Box::new(expr) });
                    } else {
                        panic!("Expected semicolon after print statement");
                    }
                
                },
                Token::Call => {
                    println!("I am hereee which is the call");
                    self.advance(); // Consume "call"
                    let function_name = match self.current_token {
                        Some(Token::Identifier(name)) => name.clone(),
                        _ => panic!("Expected function identifier after 'call'"),
                    };
                    self.advance(); // Consume function identifier
                    self.advance(); // Consume "("
                    let arguments = self.parse_argument_list(); // Parse function arguments
                    self.advance(); // Consume ")"
                    self.advance(); // Consume ";"
                
                    let boxed_arguments: Vec<Box<ASTNode>> = arguments.into_iter().map(Box::new).collect();
                    let function_call_node = ASTNode::FunctionCall {
                        name: function_name.clone(),
                        arguments: boxed_arguments,
                    };

                    statements.push(function_call_node);
                }
                _ => panic!("Unexpected this is comes from  token {:?}", token),
            }
        }
        statements
    }
    
    
    /// Parse expression according to defined rules
    pub fn parse_expression(&mut self) -> ASTNode {
        let mut left = self.parse_term();

        while let Some(token) = self.current_token {
            match token {
                Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Percent
                | Token::Equals | Token::NotEquals | Token::LessThan | Token::LessThanOrEqual
                | Token::GreaterThan | Token::GreaterThanOrEqual | Token::And | Token::Or => {
                    self.advance(); // Consume operator
                    let right = self.parse_term();
                    left = ASTNode::BinaryOp {
                        op: token.to_string(),
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        left
    }

    /// Parse arguments of function definition according to rules
    pub fn parse_argument_list(&mut self) -> Vec<ASTNode> {
        let mut arguments = Vec::new();
    
        // Parse the first argument if it exists
        if let Some(Token::RParen) = self.current_token {
            return arguments;
        }
    
        loop {
            let argument = self.parse_expression(); // Parse the argument expression
            arguments.push(argument);
    
            if let Some(Token::Comma) = self.current_token {
                self.advance(); // Consume ","
            } else {
                break;
            }
        }
    
        arguments
    }
    
    /// Parse term of function definition according to rules
    pub fn parse_term(&mut self) -> ASTNode {
        match self.current_token {
            Some(Token::RBrace) => {
                // Exit the loop when encountering a closing brace
                self.advance(); // Consume "}"
                return ASTNode::NoOp; // NoOp or some other way to represent an empty statement
            },
            Some(Token::IntegerLiteral(value)) => {
                self.advance(); // Consume integer literal
                ASTNode::IntegerLiteral(*value)
            }
            Some(Token::BooleanLiteral(value)) => {
                self.advance(); // Consume boolean literal
                ASTNode::BooleanLiteral(*value)
            }
            Some(Token::Identifier(name)) => {
                self.advance(); // Consume identifier
                println!("no problem we are here");
                ASTNode::Identifier(name.clone())
            },
            Some(Token::Call) => {
                self.advance(); // Consume "call"
                let function_name = match self.current_token {
                    Some(Token::Identifier(name)) => name.clone(),
                    _ => panic!("Expected function name after 'call'"),
                };
                self.advance(); // Consume function name
                self.advance(); // Consume "("
                let arguments = self.parse_argument_list(); // Parse the function arguments
                self.advance(); // Consume ")"
                let boxed_arguments: Vec<Box<ASTNode>> = arguments.into_iter().map(Box::new).collect();
                let function_call_node = ASTNode::FunctionCall {
                    name: function_name.clone(),
                    arguments: boxed_arguments,
                };
                function_call_node
            }
            _ => panic!("Expected integer literal, boolean literal, or identifier"),
        }
    }
    
    
    /// Parse parameters of function call according to rules
    pub fn parse_parameters(&mut self) -> Vec<(String, String)> {
        let mut parameters = Vec::new();
        while let Some(Token::Variable) = self.current_token {
            self.advance(); // Consume "variable"
    
            let parameter_type = match self.current_token {
                Some(Token::Integer) => {
                    self.advance(); // Consume "integer"
                    "integer".to_string()
                }
                Some(Token::Boolean) => {
                    self.advance(); // Consume "boolean"
                    "boolean".to_string()
                }
                _ => unreachable!(), // This should not happen if the input is correct
            };
    
            let name = match self.current_token {
                Some(Token::Identifier(name)) => {
                    self.advance(); // Consume parameter name
                    name.clone()
                }
                _ => panic!("Expected identifier for parameter"),
            };
    
            parameters.push((parameter_type, name));
    
            if let Some(Token::Comma) = self.current_token {
                self.advance(); // Consume ","
            } else {
                break;
            }
        }
        parameters
    }
    
    /// Parse block scope of function according to rules
    pub fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut statements = Vec::new();
        while let Some(token) = self.current_token {
            match token {
                Token::RBrace => {
                    self.advance(); // Consume "}"
                    break;
                }
                _ => {
                    println!("Comes from different section");
                    println!("{:?}",self.current_token);
                    let statement = self.parse();
                    statements.extend(statement);
                }
            }
        }
        statements
    }
    
}