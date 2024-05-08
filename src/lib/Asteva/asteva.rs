use crate::lib::Enums::astnode::ASTNode;
use crate::lib::Asteva::context::Context;
use crate::lib::Enums::value::Value;
use crate::lib::Asteva::context::FunctionDefinition;

/// Evaluates a block of statements in the provided context.
///
/// # Arguments
///
/// * `block` - A reference to a vector of ASTNode representing the block of statements to evaluate.
/// * `context` - A mutable reference to the Context struct representing the evaluation context.
///
/// # Returns
///
/// A Value representing the result of the last evaluated statement in the block.
pub fn evaluate_block(block: &Vec<ASTNode>, context: &mut Context) -> Value {
    // Initialize the result to Void
    let mut result = Value::Void;
    // Iterate over each statement in the block
    for statement in block {
        // Evaluate the statement in the context
        result = evaluate(statement, context);
    }
    // Return the result of the last evaluated statement
    result
}



/// Evaluates an expression represented by an ASTNode in the provided context.
///
/// # Arguments
///
/// * `expr` - A reference to an ASTNode representing the expression to evaluate.
/// * `context` - A mutable reference to the Context struct representing the evaluation context.
///
/// # Returns
///
/// A Value representing the result of the evaluated expression.
pub fn evaluate_expression(expr: &ASTNode, context: &mut Context) -> Value {
    match expr {
        // Evaluate an integer literal to an Integer value
        ASTNode::IntegerLiteral(value) => Value::Integer(*value),
        // Evaluate a boolean literal to a Boolean value
        ASTNode::BooleanLiteral(value) => Value::Boolean(*value),
        // Evaluate an identifier to the corresponding value in the context
        ASTNode::Identifier(name) => match context.variables.get(name) {
            Some(value) => Value::Integer(value.clone()), // Clone the value to avoid ownership issues
            None => panic!("Variable '{}' not found", name), // Panic if the variable is not found
        },
        // Evaluate a function call
        ASTNode::FunctionCall { name, arguments } => {
            // Retrieve the function definition from the context
            let function_def = match context.functions.get(name) {
                Some(def) => def,
                None => panic!("Function '{}' not found", name), // Panic if the function is not found
            };
            // Create a new context to evaluate the function body
            let mut new_context = context.clone();
            // Iterate over the function parameters and arguments
            for (param, arg) in function_def.parameters.iter().zip(arguments.iter()) {
                // Evaluate the argument expression
                let value = evaluate_expression(arg, &mut new_context);
                // Insert the argument value into the new context
                if let Value::Integer(int_value) = value {
                    new_context.variables.insert(param.1.clone(), int_value);
                } else {
                    panic!("Expected integer value for parameter '{}'", param.1);
                }
            }
            // Evaluate the function body in the new context and return the result
            evaluate_block(&function_def.body, &mut new_context)
        }
        // Panic if the node type is not supported for evaluation
        _ => panic!("Evaluation for this node not implemented: {:?}", expr),
    }
}


/// Evaluates an ASTNode and returns the resulting Value.
///
/// # Arguments
///
/// * `node` - The ASTNode to evaluate.
/// * `contextparam` - The evaluation context containing variables and functions.
///
/// # Returns
///
/// The resulting Value of the evaluation.

pub fn evaluate(node: &ASTNode, contextparam: &mut Context) -> Value {
    match node {
        ASTNode::IntegerLiteral(value) => Value::Integer(*value),
        ASTNode::BooleanLiteral(value) => Value::Boolean(*value),
        ASTNode::StringLiteral(value) => Value::String(value.clone()),
        ASTNode::Identifier(name) => {
            if let Some(value) = contextparam.get_variable(name) {
                Value::Integer(value.clone())
            }  else {
                panic!("Variable '{}' not found", name);
            }
        },
        ASTNode::Assignment { identifier, value } => {
            let new_value = match value.as_ref() {
                ASTNode::BinaryOp { op, left, right } => {
                    let left_value = evaluate(left, contextparam);
                    let right_value = evaluate(right, contextparam);
                    match op.as_str() {
                        "Plus" => match (left_value, right_value) {
                            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                            _ => panic!("Invalid operands for +"),
                        },
                        _ => unimplemented!("Operator '{}' not implemented", op),
                    }
                },
                _ => unimplemented!("Evaluation for assignment value not implemented: {:?}", value),
            };
            contextparam.set_variable(identifier, new_value.clone());
            new_value
        },        
        ASTNode::BinaryOp { op, left, right } => {
            let left_value = evaluate(left, contextparam);
            let right_value = evaluate(right, contextparam);
            match op.as_str() {
                "Plus" => match (left_value, right_value) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                    _ => panic!("Invalid operands for +"),
                },
                "Minus" => match (left_value, right_value) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                    _ => panic!("Invalid operands for -"),
                },
                "Multiply" => match (left_value, right_value) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                    _ => panic!("Invalid operands for *"),
                },
                "Divide" => match (left_value, right_value) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
                    _ => panic!("Invalid operands for /"),
                },
                "Modulo" => match (left_value, right_value) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(a % b),
                    _ => panic!("Invalid operands for %"),
                },
                "Equals" => Value::Boolean(left_value == right_value),
                "NotEquals" => Value::Boolean(left_value != right_value),
                "LessThan" => match (left_value, right_value) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a < b),
                    _ => panic!("Invalid operands for <"),
                },
                "LessThanOrEqual" => match (left_value, right_value) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a <= b),
                    _ => panic!("Invalid operands for <="),
                },
                "GreaterThan" => match (left_value, right_value) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a > b),
                    _ => panic!("Invalid operands for >"),
                },
                "GreaterThanOrEqual" => match (left_value, right_value) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a >= b),
                    _ => panic!("Invalid operands for >="),
                },
                "And" => match (left_value, right_value) {
                    (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a && b),
                    _ => panic!("Invalid operands for 'and'"),
                },
                "Or" => match (left_value, right_value) {
                    (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a || b),
                    _ => panic!("Invalid operands for 'or'"),
                },
                "Not" => match left_value {
                    Value::Boolean(a) => Value::Boolean(!a),
                    _ => panic!("Invalid operand for 'not'"),
                },
                _ => unimplemented!("Operator '{}' not implemented", op),
            }
        }
        ASTNode::VariableDeclaration {
            name,
            value,
            var_type: _,
        } => {
            let value = evaluate(value, contextparam);
            contextparam.set_variable(name, value.clone());
            value
        },
        ASTNode::PrintStatement { context } => {
            let value = evaluate(context, contextparam);
            println!("here is the context");
            println!("{:?}",context);
            println!("{}", value); 
            Value::Void 
        }

        ASTNode::Loop { condition, body } => {
            let mut result = Value::Void; // Default result for an empty loop
            println!("This is inside evaluation loop");
            println!("{:?}",evaluate(condition, contextparam));
            while evaluate(condition, contextparam).as_bool() {
                // Evaluate the body of the loop
                for statement in body.iter() {
                    result = evaluate(statement, contextparam);
                }
            }
            result // Return the result of the last iteration
        },
        ASTNode::IfElse { condition, if_block, else_block } => {
            if evaluate(condition, contextparam).as_bool() {
                // Evaluate the if block
                let mut result = Value::Void;
                for statement in if_block.iter() {
                    result = evaluate(statement, contextparam);
                }
                result
            } else if let Some(else_block) = else_block {
                // Evaluate the else block
                let mut result = Value::Void;
                for statement in else_block.iter() {
                    result = evaluate(statement, contextparam);
                }
                result
            } else {
                // No else block, return Void
                Value::Void
            }
        },
        ASTNode::FunctionDeclaration { name, return_type, parameters, body } => {
            // Add the function definition to the context
            contextparam.functions.insert(name.clone(), FunctionDefinition {
                return_type: return_type.clone(),
                parameters: parameters.clone(),
                body: body.clone(),
            });
            println!("I am inside the function declaration section");
            // Evaluate the function body (optional, depends on your interpreter design)
            // for statement in body.iter() {
            //     evaluate(statement, contextparam); // Assuming evaluate_statement evaluates a single statement
            // }
        
            // Return Void since function declaration doesn't produce a value
            Value::Void
        },
        ASTNode::ReturnStatement(value) => {
            match &**value {
                ASTNode::BinaryOp { op, left, right } => {
                    let left_value = evaluate(&left, contextparam);
                    let right_value = evaluate(&right, contextparam);
        
                    match op.as_str() {
                        "Plus" => {
                            match (left_value, right_value) {
                                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
                                _ => panic!("Operands must be integers for addition"),
                            }
                        },
                        "Minus" => {
                            match (left_value, right_value) {
                                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left - right),
                                _ => panic!("Operands must be integers for subtraction"),
                            }
                        },
                        "Multiply" => {
                            match (left_value, right_value) {
                                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left * right),
                                _ => panic!("Operands must be integers for multiplication"),
                            }
                        },
                        "Divide" => {
                            match (left_value, right_value) {
                                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left / right),
                                _ => panic!("Operands must be integers for division"),
                            }
                        },
                        "Modulo" => {
                            match (left_value, right_value) {
                                (Value::Integer(left), Value::Integer(right)) => Value::Integer(left % right),
                                _ => panic!("Operands must be integers for modulo"),
                            }
                        },
                        "Equals" => Value::Boolean(match (left_value, right_value) {
                            (Value::Integer(left), Value::Integer(right)) => left == right,
                            (Value::Boolean(left), Value::Boolean(right)) => left == right,
                            _ => false,
                        }),
                        "NotEquals" => Value::Boolean(match (left_value, right_value) {
                            (Value::Integer(left), Value::Integer(right)) => left != right,
                            (Value::Boolean(left), Value::Boolean(right)) => left != right,
                            _ => false,
                        }),
                        "LessThan" => Value::Boolean(match (left_value, right_value) {
                            (Value::Integer(left), Value::Integer(right)) => left < right,
                            _ => false,
                        }),
                        "LessThanOrEqual" => Value::Boolean(match (left_value, right_value) {
                            (Value::Integer(left), Value::Integer(right)) => left <= right,
                            _ => false,
                        }),
                        "GreaterThan" => Value::Boolean(match (left_value, right_value) {
                            (Value::Integer(left), Value::Integer(right)) => left > right,
                            _ => false,
                        }),
                        "GreaterThanOrEqual" => Value::Boolean(match (left_value, right_value) {
                            (Value::Integer(left), Value::Integer(right)) => left >= right,
                            _ => false,
                        }),
                        
                        "And" => {
                            match (left_value, right_value) {
                                (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left && right),
                                _ => panic!("Operands must be booleans for logical AND"),
                            }
                        },
                        "Or" => {
                            match (left_value, right_value) {
                                (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left || right),
                                _ => panic!("Operands must be booleans for logical OR"),
                            }
                        },
                        "Not" => {
                            match right_value {
                                Value::Boolean(value) => Value::Boolean(!value),
                                _ => panic!("Operand must be a boolean for logical NOT"),
                            }
                        },
                        _ => unimplemented!("Operation not implemented: {}", op),
                    }
                    
                    
                },
                _ => unimplemented!("Return value must be a binary operation"),
            }
        },
        ASTNode::FunctionCall { name, arguments } => {
            // Find the function definition
            let function_def = match contextparam.functions.get(name) {
                Some(def) => def,
                None => panic!("Function '{}' not defined", name),
            };
        
            // Evaluate the function body with the arguments
            let mut new_context = contextparam.clone();
            new_context.enter_function(name.clone());
        
            // Borrow contextparam mutably here
            for (param, arg) in function_def.parameters.iter().zip(arguments.iter()) {
                let arg_value = evaluate_expression(arg, &mut new_context);
                new_context.set_variable(&param.1, arg_value);
            }
        
            evaluate_block(&function_def.body, &mut new_context)
        },
        
        _ => unimplemented!("Evaluation for this node not implemented: {:?}", node),
    }
}




/// Evaluates a list of ASTNodes in the given context.
///
/// # Arguments
///
/// * `ast` - The list of ASTNodes to evaluate.
/// * `context` - The evaluation context containing variables and functions.
///
/// # Returns
///
/// * `Ok(())` if evaluation succeeds, otherwise returns an error message as a String.
pub fn evaluate_ast(ast: Vec<ASTNode>, context: &mut Context) -> Result<(), String> {
    for node in ast {
        evaluate(&node, context); // Evaluate each ASTNode in the list
    }
    Ok(()) // Return Ok(()) if evaluation succeeds
}
