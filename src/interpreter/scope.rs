// INTERP-003: Symbol Table & Lexical Scoping
// REFACTOR Phase: Clean implementation with enhanced documentation
//
// Research: Aho et al. (2006) Chapter 2: Symbol Tables
//
// This module implements lexical scoping and symbol tables for the Ruchy interpreter.
// Supports nested scopes, variable shadowing, and closure capture.
//
// Key Features:
// - Nested scopes with parent chain lookup
// - Variable shadowing (local variables hide parent variables)
// - Closure capture (all variables or only referenced ones)
// - Interior mutability via Rc<RefCell<>> for shared parent references
//
// Design Pattern:
// Uses interior mutability (RefCell) wrapped in reference counting (Rc) to allow
// multiple scopes to share parent scope references while maintaining mutability.
// This enables assignment to update variables in parent scopes when not shadowed.

use crate::interpreter::value::Value;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

// Type aliases for complex types
type Variables = Rc<RefCell<HashMap<String, Value>>>;
type ParentScope = Option<Rc<RefCell<Scope>>>;
type ReferencedVars = Rc<RefCell<HashSet<String>>>;

/// Scope represents a lexical scope with variable bindings
///
/// Scopes form a tree structure where each scope has at most one parent.
/// Variable lookup traverses the parent chain from child to root.
/// Variable shadowing occurs when a child scope defines a variable with
/// the same name as a parent scope variable.
#[derive(Debug, Clone)]
pub struct Scope {
    /// Local variable bindings in this scope
    variables: Variables,
    /// Parent scope (None for global scope)
    parent: ParentScope,
    /// Depth of this scope (0 = global, 1+ = nested)
    depth: usize,
    /// Variables referenced in this scope (for closure capture)
    referenced: ReferencedVars,
}

/// Scope-related errors
#[derive(Debug, Clone)]
pub enum ScopeError {
    /// Variable already defined in current scope
    AlreadyDefined {
        /// Name of the variable
        name: String,
    },
    /// Variable not defined in any scope
    Undefined {
        /// Name of the variable
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
        Scope {
            variables: Rc::new(RefCell::new(HashMap::new())),
            parent: None,
            depth: 0,
            referenced: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    /// Create a child scope
    pub fn create_child(&self) -> Self {
        Scope {
            variables: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(Rc::new(RefCell::new(self.clone()))),
            depth: self.depth + 1,
            referenced: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    // ===== Scope Properties =====

    /// Check if this is the global scope
    pub fn is_global(&self) -> bool {
        self.parent.is_none()
    }

    /// Get the depth of this scope (0 = global, 1+ = nested)
    pub fn depth(&self) -> usize {
        self.depth
    }

    // ===== Variable Operations =====

    /// Define a new variable in this scope
    pub fn define(&mut self, name: String, value: Value) -> Result<(), ScopeError> {
        let mut vars = self.variables.borrow_mut();
        if vars.contains_key(&name) {
            return Err(ScopeError::AlreadyDefined { name });
        }
        vars.insert(name, value);
        Ok(())
    }

    /// Assign to an existing variable (searches parent scopes)
    ///
    /// If the variable is defined in the current scope, updates it locally.
    /// Otherwise, searches parent scopes recursively and updates the first match.
    /// Returns an error if the variable is not defined in any scope.
    ///
    /// # Example
    /// ```ignore
    /// let mut global = Scope::new();
    /// global.define("x".to_string(), Value::integer(1)).unwrap();
    ///
    /// let mut local = global.create_child();
    /// local.assign("x", Value::integer(2)).unwrap(); // Updates global's x
    /// ```
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), ScopeError> {
        // Check if variable exists in current scope
        if self.variables.borrow().contains_key(name) {
            self.variables.borrow_mut().insert(name.to_string(), value);
            return Ok(());
        }

        // Search parent scopes recursively
        if let Some(parent_rc) = &self.parent {
            parent_rc.borrow_mut().assign(name, value)
        } else {
            Err(ScopeError::Undefined {
                name: name.to_string(),
            })
        }
    }

    /// Get a variable's value by cloning (searches parent scopes)
    ///
    /// Returns a clone of the variable's value. Searches the current scope first,
    /// then recursively searches parent scopes. Returns an error if the variable
    /// is not defined in any scope.
    ///
    /// Note: Returns a clone rather than a reference due to RefCell borrow rules.
    ///
    /// # Example
    /// ```ignore
    /// let mut scope = Scope::new();
    /// scope.define("x".to_string(), Value::integer(42)).unwrap();
    /// let value = scope.get_cloned("x").unwrap();
    /// assert_eq!(value.as_integer().unwrap(), 42);
    /// ```
    pub fn get_cloned(&self, name: &str) -> Result<Value, ScopeError> {
        // Check local scope first
        if let Some(value) = self.variables.borrow().get(name) {
            return Ok(value.clone());
        }

        // Recursively search parent scopes
        if let Some(parent_rc) = &self.parent {
            parent_rc.borrow().get_cloned(name)
        } else {
            Err(ScopeError::Undefined {
                name: name.to_string(),
            })
        }
    }

    // ===== Scope Introspection =====

    /// Check if variable exists in this scope (not parent)
    pub fn contains_local(&self, name: &str) -> bool {
        self.variables.borrow().contains_key(name)
    }

    /// Get all variable names in this scope (not parent)
    pub fn local_names(&self) -> Vec<String> {
        self.variables.borrow().keys().cloned().collect()
    }

    // ===== Closure Support =====

    /// Mark a variable as referenced (for closure capture)
    pub fn mark_referenced(&self, name: &str) {
        self.referenced.borrow_mut().insert(name.to_string());
    }

    /// Capture all referenced variables (for closure)
    /// If no variables are explicitly marked as referenced, captures all accessible variables
    pub fn capture(&self) -> HashMap<String, Value> {
        let mut captured = HashMap::new();
        let referenced_set = self.referenced.borrow();

        // If specific variables are marked, capture only those
        if !referenced_set.is_empty() {
            for name in referenced_set.iter() {
                if let Ok(value) = self.get_cloned(name) {
                    captured.insert(name.clone(), value);
                }
            }
        } else {
            // Otherwise, capture all variables from parent scopes
            self.capture_all_from_parent(&mut captured);
        }

        captured
    }

    /// Helper to capture all variables from parent scopes
    fn capture_all_from_parent(&self, captured: &mut HashMap<String, Value>) {
        // Capture from parent first (recursively)
        if let Some(parent_rc) = &self.parent {
            parent_rc.borrow().capture_all_from_parent(captured);
        }

        // Then capture from current scope (overriding parent if shadowing)
        for (name, value) in self.variables.borrow().iter() {
            captured.insert(name.clone(), value.clone());
        }
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
    fn test_scope_implemented() {
        // GREEN: Verify basic implementation works
        let scope = Scope::new();
        assert!(scope.is_global());
        assert_eq!(scope.depth(), 0);

        let child = scope.create_child();
        assert!(!child.is_global());
        assert_eq!(child.depth(), 1);
    }
}
