/// Represents a node in the abstract syntax tree (AST).
#[derive(Debug)]
pub enum ASTNode {
    /// An integer literal node.
    IntegerLiteral(i32),
    /// A boolean literal node.
    BooleanLiteral(bool),
    /// A string literal node.
    StringLiteral(String),
    /// An identifier node.
    Identifier(String),
    /// A binary operation node.
    BinaryOp {
        /// The operator.
        op: String,
        /// The left operand.
        left: Box<ASTNode>,
        /// The right operand.
        right: Box<ASTNode>,
    },
    /// A variable declaration node.
    VariableDeclaration {
        /// The variable name.
        name: String,
        /// The value assigned to the variable.
        value: Box<ASTNode>,
        /// The variable type.
        var_type: String,
    },
    /// A conditional node (if statement).
    Conditional {
        /// The condition.
        condition: Box<ASTNode>,
        /// The block of code to execute if the condition is true.
        if_block: Vec<ASTNode>,
        /// The block of code to execute if the condition is false (optional).
        else_block: Option<Vec<ASTNode>>,
    },
    /// A loop node (while loop).
    Loop {
        /// The loop condition.
        condition: Box<ASTNode>,
        /// The body of the loop.
        body: Vec<ASTNode>,
    },
    /// A function declaration node.
    FunctionDeclaration {
        /// The function name.
        name: String,
        /// The return type of the function.
        return_type: String,
        /// The parameters of the function (name and type).
        parameters: Vec<(String, String)>,
        /// The body of the function.
        body: Vec<ASTNode>,
    },
    /// A function call node.
    FunctionCall {
        /// The name of the function to call.
        name: String,
        /// The arguments to pass to the function.
        arguments: Vec<Box<ASTNode>>,
    },
    /// A return statement node.
    ReturnStatement(Box<ASTNode>),
    /// An assignment node.
    Assignment {
        /// The identifier (variable) being assigned to.
        identifier: String,
        /// The value being assigned.
        value: Box<ASTNode>,
    },
    /// An if-else node.
    IfElse {
        /// The condition.
        condition: Box<ASTNode>,
        /// The block of code to execute if the condition is true.
        if_block: Vec<ASTNode>,
        /// The block of code to execute if the condition is false (optional).
        else_block: Option<Vec<ASTNode>>,
    },
    /// An else statement node.
    ElseStatement {
        /// The body of the else statement.
        body: Vec<ASTNode>,
    },
    /// A print statement node.
    PrintStatement {
        /// The context to print.
        context: Box<ASTNode>,
    },
    /// A no-operation node (empty statement).
    NoOp,
}

impl Clone for ASTNode {
    /// Clones the ASTNode.
    fn clone(&self) -> Self {
        match self {
            ASTNode::IntegerLiteral(value) => ASTNode::IntegerLiteral(*value),
            ASTNode::BooleanLiteral(value) => ASTNode::BooleanLiteral(*value),
            ASTNode::StringLiteral(value) => ASTNode::StringLiteral(value.clone()),
            ASTNode::Identifier(name) => ASTNode::Identifier(name.clone()),
            ASTNode::BinaryOp { op, left, right } => ASTNode::BinaryOp {
                op: op.clone(),
                left: Box::new((**left).clone()),
                right: Box::new((**right).clone()),
            },
            ASTNode::VariableDeclaration { name, value, var_type } => ASTNode::VariableDeclaration {
                name: name.clone(),
                value: Box::new((**value).clone()),
                var_type: var_type.clone(),
            },
            ASTNode::Conditional { condition, if_block, else_block } => ASTNode::Conditional {
                condition: Box::new((**condition).clone()),
                if_block: if_block.clone(),
                else_block: else_block.clone().map_or(None, |block| Some(block.clone())),
            },
            ASTNode::Loop { condition, body } => ASTNode::Loop {
                condition: Box::new((**condition).clone()),
                body: body.clone(),
            },
            ASTNode::FunctionDeclaration { name, return_type, parameters, body } => ASTNode::FunctionDeclaration {
                name: name.clone(),
                return_type: return_type.clone(),
                parameters: parameters.clone(),
                body: body.clone(),
            },
            ASTNode::FunctionCall { name, arguments } => ASTNode::FunctionCall {
                name: name.clone(),
                arguments: arguments.iter().map(|arg| arg.clone()).collect(),
            },
            ASTNode::ReturnStatement(value) => ASTNode::ReturnStatement(Box::new((**value).clone())),
            ASTNode::Assignment { identifier, value } => ASTNode::Assignment {
                identifier: identifier.clone(),
                value: Box::new((**value).clone()),
            },
            ASTNode::IfElse { condition, if_block, else_block } => ASTNode::IfElse {
                condition: Box::new((**condition).clone()),
                if_block: if_block.clone(),
                else_block: else_block.clone().map_or(None, |block| Some(block.clone())),
            },
            ASTNode::ElseStatement { body } => ASTNode::ElseStatement {
                body: body.clone(),
            },
            ASTNode::PrintStatement { context } => ASTNode::PrintStatement {
                context: Box::new((**context).clone()),
            },
            ASTNode::NoOp => ASTNode::NoOp,
        }
    }
}
