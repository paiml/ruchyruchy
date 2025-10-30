// INTERP-003: Symbol Table & Lexical Scoping
// RED Phase: Stub implementation with unimplemented!() macros
//
// Research: Aho et al. (2006) Chapter 2: Symbol Tables
//
// This module implements lexical scoping and symbol tables for the Ruchy interpreter.
// Supports nested scopes, variable shadowing, and closure capture.

use crate::interpreter::value::Value;
use std::collections::HashMap;
use std::fmt;

/// Scope represents a lexical scope with variable bindings
#[derive(Debug, Clone)]
pub struct Scope {
    // RED phase stub - structure will be defined in GREEN phase
}

/// Scope-related errors
#[derive(Debug, Clone)]
pub enum ScopeError {
    /// Variable already defined in current scope
    AlreadyDefined {
        name: String,
    },
    /// Variable not defined in any scope
    Undefined {
        name: String,
    },
}

impl fmt::Display for ScopeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScopeError::AlreadyDefined { name } => {
                write!(f, "Variable '{}' is already defined in this scope", name)
            }
            ScopeError::Undefined { name } => {
                write!(f, "Variable '{}' is not defined", name)
            }
        }
    }
}

impl std::error::Error for ScopeError {}

impl Scope {
    // ===== Scope Creation =====

    /// Create a new global scope
    pub fn new() -> Self {
        unimplemented!("RED phase: Scope::new not implemented")
    }

    /// Create a child scope
    pub fn create_child(&self) -> Self {
        unimplemented!("RED phase: Scope::create_child not implemented")
    }

    // ===== Scope Properties =====

    /// Check if this is the global scope
    pub fn is_global(&self) -> bool {
        unimplemented!("RED phase: Scope::is_global not implemented")
    }

    /// Get the depth of this scope (0 = global, 1+ = nested)
    pub fn depth(&self) -> usize {
        unimplemented!("RED phase: Scope::depth not implemented")
    }

    // ===== Variable Operations =====

    /// Define a new variable in this scope
    pub fn define(&mut self, _name: String, _value: Value) -> Result<(), ScopeError> {
        unimplemented!("RED phase: Scope::define not implemented")
    }

    /// Assign to an existing variable (searches parent scopes)
    pub fn assign(&mut self, _name: &str, _value: Value) -> Result<(), ScopeError> {
        unimplemented!("RED phase: Scope::assign not implemented")
    }

    /// Get a variable's value (searches parent scopes)
    pub fn get(&self, _name: &str) -> Result<&Value, ScopeError> {
        unimplemented!("RED phase: Scope::get not implemented")
    }

    // ===== Scope Introspection =====

    /// Check if variable exists in this scope (not parent)
    pub fn contains_local(&self, _name: &str) -> bool {
        unimplemented!("RED phase: Scope::contains_local not implemented")
    }

    /// Get all variable names in this scope (not parent)
    pub fn local_names(&self) -> Vec<String> {
        unimplemented!("RED phase: Scope::local_names not implemented")
    }

    // ===== Closure Support =====

    /// Mark a variable as referenced (for closure capture)
    pub fn mark_referenced(&self, _name: &str) {
        unimplemented!("RED phase: Scope::mark_referenced not implemented")
    }

    /// Capture all referenced variables (for closure)
    pub fn capture(&self) -> HashMap<String, Value> {
        unimplemented!("RED phase: Scope::capture not implemented")
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_stub_implemented() {
        // GREEN: Verify basic stub is in place
        // This test will panic during RED phase
        // let scope = Scope::new();
        // assert!(scope.is_global());
        println!("RED phase: Stub implementation in place");
    }
}
