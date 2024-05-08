use std::collections::HashMap;
use crate::lib::Enums::astnode::ASTNode;
use crate::lib::Enums::value::Value;

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub return_type: String,
    pub parameters: Vec<(String, String)>, // (parameter_type, parameter_name)
    pub body: Vec<ASTNode>,
}

#[derive(Clone)]
pub struct Context {
    pub variables: HashMap<String, i32>, // Map of variable names to their integer values
    pub functions: HashMap<String, FunctionDefinition>,
    pub current_function: Option<String>,
}

impl Context {
    /// Create a new context.
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_function: Some(String::new()),
        }
    }

    /// Clone the context.
    pub fn clone(&self) -> Context {
        Context {
            variables: self.variables.clone(),
            functions: self.functions.clone(),
            current_function: self.current_function.clone(),
        }
    }

    /// Enter a function context.
    pub fn enter_function(&mut self, function_name: String) {
        self.current_function = Some(function_name); // Set the current function
    }

    /// Exit a function context.
    pub fn exit_function(&mut self) {
        self.current_function = None; // Reset the current function after exiting
    }

    /// Add a variable binding to the context.
    pub fn add_variable(&mut self, name: &str, value: i32) {
        self.variables.insert(name.to_string(), value);
    }

    /// Set the value of a variable in the context.
    pub fn set_variable(&mut self, name: &str, value: Value) {
        match value {
            Value::Integer(i) => {
                self.variables.insert(name.to_string(), i);
            }
            Value::Boolean(b) => {
                let int_value = if b { 1 } else { 0 }; // Convert bool to i32
                self.variables.insert(name.to_string(), int_value);
            }
            Value::Void => {
                // Do nothing or handle Void as a special case
            }
            _ => panic!("Unexpected value type"),
        }
    }

    /// Get the value of a variable from the context.
    pub fn get_variable(&self, name: &str) -> Option<&i32> {
        self.variables.get(name)
    }

    /// Update the value of a variable in the context.
    pub fn update_variable(&mut self, name: &str, value: i32) {
        self.variables.insert(name.to_string(), value);
    }

    /// Remove a variable from the context.
    pub fn remove_variable(&mut self, name: &str) {
        self.variables.remove(name);
    }

    /// Set a function definition in the context.
    pub fn set_function(&mut self, name: String, definition: FunctionDefinition) {
        self.functions.insert(name, definition);
    }

    /// Get the value of a function parameter from the context.
    pub fn get_parameter(&self, name: &str) -> Option<Value> {
        if let Some(function) = self.functions.get("calculate") {
            if let Some((_param_type, _)) = function.parameters.iter().find(|(_param_type, param_name)| *param_name == name) {
                if let Some(param_value) = self.variables.get(name) {
                    return Some(Value::Integer(*param_value));
                }
            }
        }
        None
    }
}
