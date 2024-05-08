use std::fmt;

/// Represents a value in the custom language.
#[derive(Debug, Clone)] // Add Clone trait here
pub enum Value {
    /// An integer value.
    Integer(i32),
    /// A boolean value.
    Boolean(bool),
    /// A string value.
    String(String),
    /// Represents the absence of a value (for statements like VariableDeclaration).
    Void,
}

impl fmt::Display for Value {
    /// Formats the value for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(value) => write!(f, "{}", value),
            Value::Boolean(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Void => write!(f, "void"),
        }
    }
}

impl PartialEq for Value {
    /// Implements the equality comparison for values.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Void, Value::Void) => true,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl Value {
    /// Converts the value to a boolean.
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0, 
            _ => panic!("Value cannot be converted to boolean"),
        }
    }
}
