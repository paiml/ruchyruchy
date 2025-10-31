// INTERP-002: Value Representation System
// REFACTOR Phase: Clean implementation with all tests passing
//
// Research: Ierusalimschy et al. (2007) Section 3: Value Representation
//
// This module implements runtime values for the Ruchy interpreter.
// All values are dynamically typed at runtime with type safety enforced
// through Result types. Supports 6 value types: Integer, String, Boolean,
// Vector, HashMap, and Function.

use crate::interpreter::parser::AstNode;
use std::collections::HashMap;
use std::fmt;

/// Runtime value types
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// String value
    String(String),
    /// Boolean value
    Boolean(bool),
    /// Vector (array) value
    Vector(Vec<Value>),
    /// HashMap (dictionary) value
    HashMap(HashMap<String, Value>),
    /// Function value (closure)
    Function {
        /// Function parameter names
        params: Vec<String>,
        /// Function body statements
        body: Vec<AstNode>,
    },
    /// Tuple value (ordered collection of heterogeneous values)
    Tuple(Vec<Value>),
    /// Nil/Unit value (represents absence of value)
    Nil,
}

/// Runtime type errors
#[derive(Debug, Clone)]
pub enum ValueError {
    /// Type mismatch error
    TypeMismatch {
        /// Expected type name
        expected: String,
        /// Found type name
        found: String,
        /// Operation that failed
        operation: String,
    },
    /// Division by zero
    DivisionByZero,
    /// Index out of bounds
    IndexOutOfBounds {
        /// Index attempted
        index: usize,
        /// Collection length
        len: usize
    },
    /// Key not found in HashMap
    KeyNotFound {
        /// Key that was not found
        key: String
    },
    /// Invalid operation
    InvalidOperation {
        /// Operation description
        operation: String,
        /// Error message
        message: String
    },
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueError::TypeMismatch {
                expected,
                found,
                operation,
            } => {
                write!(
                    f,
                    "Type mismatch in {}: expected {}, found {}",
                    operation, expected, found
                )
            }
            ValueError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            ValueError::IndexOutOfBounds { index, len } => {
                write!(f, "Index {} out of bounds for length {}", index, len)
            }
            ValueError::KeyNotFound { key } => {
                write!(f, "Key '{}' not found in HashMap", key)
            }
            ValueError::InvalidOperation { operation, message } => {
                write!(f, "Invalid operation {}: {}", operation, message)
            }
        }
    }
}

impl std::error::Error for ValueError {}

impl Value {
    // ===== Constructors =====

    /// Create an integer value
    pub fn integer(n: i64) -> Self {
        Value::Integer(n)
    }

    /// Create a float value
    pub fn float(f: f64) -> Self {
        Value::Float(f)
    }

    /// Create a string value
    pub fn string(s: String) -> Self {
        Value::String(s)
    }

    /// Create a boolean value
    pub fn boolean(b: bool) -> Self {
        Value::Boolean(b)
    }

    /// Create a vector value
    pub fn vector(elements: Vec<Value>) -> Self {
        Value::Vector(elements)
    }

    /// Create a tuple value
    pub fn tuple(elements: Vec<Value>) -> Self {
        Value::Tuple(elements)
    }

    /// Create a hashmap value
    pub fn hashmap() -> Self {
        Value::HashMap(HashMap::new())
    }

    /// Create a function value
    pub fn function(params: Vec<String>, body: Vec<AstNode>) -> Self {
        Value::Function { params, body }
    }

    /// Create a nil value
    pub fn nil() -> Self {
        Value::Nil
    }

    // ===== Type Checking =====

    /// Check if value is an integer
    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    /// Check if value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Check if value is a boolean
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Check if value is a vector
    pub fn is_vector(&self) -> bool {
        matches!(self, Value::Vector(_))
    }

    /// Check if value is a hashmap
    pub fn is_hashmap(&self) -> bool {
        matches!(self, Value::HashMap(_))
    }

    /// Check if value is a function
    pub fn is_function(&self) -> bool {
        matches!(self, Value::Function { .. })
    }

    /// Check if value is nil
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    /// Get type name as string
    pub fn type_name(&self) -> &str {
        match self {
            Value::Integer(_) => "Integer",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Boolean(_) => "Boolean",
            Value::Vector(_) => "Vector",
            Value::Tuple(_) => "Tuple",
            Value::HashMap(_) => "HashMap",
            Value::Function { .. } => "Function",
            Value::Nil => "Nil",
        }
    }

    // ===== Type Conversion =====

    /// Extract integer value
    pub fn as_integer(&self) -> Result<i64, ValueError> {
        match self {
            Value::Integer(n) => Ok(*n),
            _ => Err(ValueError::TypeMismatch {
                expected: "Integer".to_string(),
                found: self.type_name().to_string(),
                operation: "as_integer".to_string(),
            }),
        }
    }

    /// Extract float value
    pub fn as_float(&self) -> Result<f64, ValueError> {
        match self {
            Value::Float(f) => Ok(*f),
            _ => Err(ValueError::TypeMismatch {
                expected: "Float".to_string(),
                found: self.type_name().to_string(),
                operation: "as_float".to_string(),
            }),
        }
    }

    /// Extract string value
    pub fn as_string(&self) -> Result<&str, ValueError> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(ValueError::TypeMismatch {
                expected: "String".to_string(),
                found: self.type_name().to_string(),
                operation: "as_string".to_string(),
            }),
        }
    }

    /// Extract boolean value
    pub fn as_boolean(&self) -> Result<bool, ValueError> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err(ValueError::TypeMismatch {
                expected: "Boolean".to_string(),
                found: self.type_name().to_string(),
                operation: "as_boolean".to_string(),
            }),
        }
    }

    /// Extract vector value
    pub fn as_vector(&self) -> Result<&Vec<Value>, ValueError> {
        match self {
            Value::Vector(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: "Vector".to_string(),
                found: self.type_name().to_string(),
                operation: "as_vector".to_string(),
            }),
        }
    }

    /// Extract hashmap value (mutable)
    pub fn as_hashmap(&mut self) -> Result<&mut HashMap<String, Value>, ValueError> {
        match self {
            Value::HashMap(m) => Ok(m),
            _ => Err(ValueError::TypeMismatch {
                expected: "HashMap".to_string(),
                found: self.type_name().to_string(),
                operation: "as_hashmap".to_string(),
            }),
        }
    }

    /// Extract function parameters
    pub fn as_function_params(&self) -> Result<&Vec<String>, ValueError> {
        match self {
            Value::Function { params, .. } => Ok(params),
            _ => Err(ValueError::TypeMismatch {
                expected: "Function".to_string(),
                found: self.type_name().to_string(),
                operation: "as_function_params".to_string(),
            }),
        }
    }

    // ===== Arithmetic Operations =====

    /// Add two values
    pub fn add(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(ValueError::TypeMismatch {
                expected: self.type_name().to_string(),
                found: other.type_name().to_string(),
                operation: "add".to_string(),
            }),
        }
    }

    /// Subtract two values
    pub fn subtract(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err(ValueError::TypeMismatch {
                expected: "Integer or Float".to_string(),
                found: format!("{} - {}", self.type_name(), other.type_name()),
                operation: "subtract".to_string(),
            }),
        }
    }

    /// Multiply two values
    pub fn multiply(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err(ValueError::TypeMismatch {
                expected: "Integer or Float".to_string(),
                found: format!("{} * {}", self.type_name(), other.type_name()),
                operation: "multiply".to_string(),
            }),
        }
    }

    /// Divide two values
    pub fn divide(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(_), Value::Integer(0)) => Err(ValueError::DivisionByZero),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(ValueError::DivisionByZero)
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            _ => Err(ValueError::TypeMismatch {
                expected: "Integer or Float".to_string(),
                found: format!("{} / {}", self.type_name(), other.type_name()),
                operation: "divide".to_string(),
            }),
        }
    }

    // ===== Logical Operations =====

    /// Logical AND
    pub fn logical_and(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a && *b)),
            _ => Err(ValueError::TypeMismatch {
                expected: "Boolean".to_string(),
                found: format!("{} && {}", self.type_name(), other.type_name()),
                operation: "logical_and".to_string(),
            }),
        }
    }

    /// Logical OR
    pub fn logical_or(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a || *b)),
            _ => Err(ValueError::TypeMismatch {
                expected: "Boolean".to_string(),
                found: format!("{} || {}", self.type_name(), other.type_name()),
                operation: "logical_or".to_string(),
            }),
        }
    }

    /// Logical NOT
    pub fn logical_not(&self) -> Result<Value, ValueError> {
        match self {
            Value::Boolean(b) => Ok(Value::Boolean(!b)),
            _ => Err(ValueError::TypeMismatch {
                expected: "Boolean".to_string(),
                found: self.type_name().to_string(),
                operation: "logical_not".to_string(),
            }),
        }
    }

    // ===== Comparison Operations =====

    /// Less than comparison
    pub fn less_than(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a < b)),
            _ => Err(ValueError::TypeMismatch {
                expected: "Integer".to_string(),
                found: format!("{} < {}", self.type_name(), other.type_name()),
                operation: "less_than".to_string(),
            }),
        }
    }

    /// Greater than comparison
    pub fn greater_than(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a > b)),
            _ => Err(ValueError::TypeMismatch {
                expected: "Integer".to_string(),
                found: format!("{} > {}", self.type_name(), other.type_name()),
                operation: "greater_than".to_string(),
            }),
        }
    }

    /// Equality comparison
    pub fn equals(&self, other: &Value) -> Result<Value, ValueError> {
        // Use PartialEq implementation
        Ok(Value::Boolean(self == other))
    }

    // ===== Collection Operations =====

    /// Index into vector
    pub fn index(&self, idx: usize) -> Result<&Value, ValueError> {
        match self {
            Value::Vector(v) => v.get(idx).ok_or(ValueError::IndexOutOfBounds {
                index: idx,
                len: v.len(),
            }),
            _ => Err(ValueError::TypeMismatch {
                expected: "Vector".to_string(),
                found: self.type_name().to_string(),
                operation: "index".to_string(),
            }),
        }
    }

    /// Push value to vector
    pub fn push(&mut self, value: Value) -> Result<(), ValueError> {
        match self {
            Value::Vector(v) => {
                v.push(value);
                Ok(())
            }
            _ => Err(ValueError::TypeMismatch {
                expected: "Vector".to_string(),
                found: self.type_name().to_string(),
                operation: "push".to_string(),
            }),
        }
    }

    /// Insert key-value pair into hashmap
    pub fn insert(&mut self, key: Value, value: Value) -> Result<(), ValueError> {
        match self {
            Value::HashMap(m) => {
                let key_str = match key {
                    Value::String(s) => s,
                    _ => {
                        return Err(ValueError::TypeMismatch {
                            expected: "String".to_string(),
                            found: key.type_name().to_string(),
                            operation: "insert key".to_string(),
                        })
                    }
                };
                m.insert(key_str, value);
                Ok(())
            }
            _ => Err(ValueError::TypeMismatch {
                expected: "HashMap".to_string(),
                found: self.type_name().to_string(),
                operation: "insert".to_string(),
            }),
        }
    }

    /// Get value from hashmap by key
    pub fn get(&self, key: &Value) -> Result<&Value, ValueError> {
        match self {
            Value::HashMap(m) => {
                let key_str = match key {
                    Value::String(s) => s,
                    _ => {
                        return Err(ValueError::TypeMismatch {
                            expected: "String".to_string(),
                            found: key.type_name().to_string(),
                            operation: "get key".to_string(),
                        })
                    }
                };
                m.get(key_str).ok_or_else(|| ValueError::KeyNotFound {
                    key: key_str.clone(),
                })
            }
            _ => Err(ValueError::TypeMismatch {
                expected: "HashMap".to_string(),
                found: self.type_name().to_string(),
                operation: "get".to_string(),
            }),
        }
    }
}

impl Value {
    /// Format value for println output (strings without quotes)
    pub fn to_println_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(), // Strings without quotes for println
            Value::Vector(v) => {
                let elements: Vec<String> = v.iter().map(|val| val.to_println_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Tuple(t) => {
                let elements: Vec<String> = t.iter().map(|val| val.to_println_string()).collect();
                format!("({})", elements.join(", "))
            }
            Value::HashMap(m) => {
                let pairs: Vec<String> = m
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_println_string()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
            // For all other types, use Display trait
            other => format!("{}", other),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Vector(v) => {
                write!(f, "[")?;
                for (i, val) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Tuple(t) => {
                write!(f, "(")?;
                for (i, val) in t.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, ")")
            }
            Value::HashMap(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Function { params, .. } => {
                write!(f, "fun({})", params.join(", "))
            }
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_implemented() {
        // GREEN: Verify basic implementation works
        let val = Value::integer(42);
        assert_eq!(val.as_integer().unwrap(), 42);
        assert!(val.is_integer());
    }
}
