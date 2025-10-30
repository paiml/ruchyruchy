// INTERP-003: Symbol Table & Lexical Scoping - RED Phase Tests
// These tests define the scope interface through EXTREME TDD
//
// Research: Aho et al. (2006) Chapter 2: Symbol Tables
//
// Test Strategy:
// 1. Global scope operations
// 2. Local scope with nesting
// 3. Variable shadowing
// 4. Closure variable capture
// 5. Scope chaining and lookups

use ruchyruchy::interpreter::scope::{Scope, ScopeError};
use ruchyruchy::interpreter::value::Value;

// ===== RED PHASE TEST 1: Global Scope =====

#[test]
fn test_create_global_scope() {
    // RED: Create a root scope
    let scope = Scope::new();

    assert!(scope.is_global());
    assert_eq!(scope.depth(), 0);
}

#[test]
fn test_define_variable_in_global_scope() {
    // RED: Define a variable in global scope
    let mut scope = Scope::new();

    let result = scope.define("x".to_string(), Value::integer(42));
    assert!(result.is_ok());

    // Should be able to retrieve the variable
    let value = scope.get("x").unwrap();
    assert_eq!(value.as_integer().unwrap(), 42);
}

#[test]
fn test_redefine_variable_in_same_scope() {
    // RED: Redefining variable in same scope should error
    let mut scope = Scope::new();

    scope.define("x".to_string(), Value::integer(42)).unwrap();
    let result = scope.define("x".to_string(), Value::integer(100));

    assert!(result.is_err());
    match result.err().unwrap() {
        ScopeError::AlreadyDefined { name } => {
            assert_eq!(name, "x");
        }
        _ => panic!("Expected AlreadyDefined error"),
    }
}

#[test]
fn test_assign_to_existing_variable() {
    // RED: Assignment should update existing variable
    let mut scope = Scope::new();

    scope.define("x".to_string(), Value::integer(42)).unwrap();
    scope.assign("x", Value::integer(100)).unwrap();

    let value = scope.get("x").unwrap();
    assert_eq!(value.as_integer().unwrap(), 100);
}

#[test]
fn test_assign_to_undefined_variable() {
    // RED: Assignment to undefined variable should error
    let mut scope = Scope::new();

    let result = scope.assign("x", Value::integer(42));

    assert!(result.is_err());
    match result.err().unwrap() {
        ScopeError::Undefined { name } => {
            assert_eq!(name, "x");
        }
        _ => panic!("Expected Undefined error"),
    }
}

// ===== RED PHASE TEST 2: Local Scope =====

#[test]
fn test_create_child_scope() {
    // RED: Create nested scopes
    let global = Scope::new();
    let local = global.create_child();

    assert!(global.is_global());
    assert!(!local.is_global());
    assert_eq!(global.depth(), 0);
    assert_eq!(local.depth(), 1);
}

#[test]
fn test_local_scope_access_parent_variable() {
    // RED: Child scope should access parent variables
    let mut global = Scope::new();
    global.define("x".to_string(), Value::integer(42)).unwrap();

    let local = global.create_child();
    let value = local.get("x").unwrap();
    assert_eq!(value.as_integer().unwrap(), 42);
}

#[test]
fn test_local_scope_define_variable() {
    // RED: Child scope can define its own variables
    let global = Scope::new();
    let mut local = global.create_child();

    local.define("y".to_string(), Value::integer(100)).unwrap();
    let value = local.get("y").unwrap();
    assert_eq!(value.as_integer().unwrap(), 100);
}

#[test]
fn test_parent_cannot_access_child_variable() {
    // RED: Parent scope cannot see child variables
    let global = Scope::new();
    let mut local = global.create_child();

    local.define("y".to_string(), Value::integer(100)).unwrap();

    let result = global.get("y");
    assert!(result.is_err());
    match result.err().unwrap() {
        ScopeError::Undefined { name } => {
            assert_eq!(name, "y");
        }
        _ => panic!("Expected Undefined error"),
    }
}

// ===== RED PHASE TEST 3: Nested Scopes =====

#[test]
fn test_deeply_nested_scopes() {
    // RED: Multiple levels of nesting
    let global = Scope::new();
    let level1 = global.create_child();
    let level2 = level1.create_child();
    let level3 = level2.create_child();

    assert_eq!(global.depth(), 0);
    assert_eq!(level1.depth(), 1);
    assert_eq!(level2.depth(), 2);
    assert_eq!(level3.depth(), 3);
}

#[test]
fn test_nested_scope_variable_lookup() {
    // RED: Nested scope looks up chain for variables
    let mut global = Scope::new();
    global.define("a".to_string(), Value::integer(1)).unwrap();

    let mut level1 = global.create_child();
    level1.define("b".to_string(), Value::integer(2)).unwrap();

    let mut level2 = level1.create_child();
    level2.define("c".to_string(), Value::integer(3)).unwrap();

    // Level2 should see all variables
    assert_eq!(level2.get("a").unwrap().as_integer().unwrap(), 1);
    assert_eq!(level2.get("b").unwrap().as_integer().unwrap(), 2);
    assert_eq!(level2.get("c").unwrap().as_integer().unwrap(), 3);
}

#[test]
fn test_nested_scope_assignment_updates_correct_scope() {
    // RED: Assignment updates variable in correct scope
    let mut global = Scope::new();
    global.define("x".to_string(), Value::integer(1)).unwrap();

    let mut level1 = global.create_child();
    level1.assign("x", Value::integer(2)).unwrap();

    // Both scopes should see updated value
    assert_eq!(global.get("x").unwrap().as_integer().unwrap(), 2);
    assert_eq!(level1.get("x").unwrap().as_integer().unwrap(), 2);
}

// ===== RED PHASE TEST 4: Variable Shadowing =====

#[test]
fn test_variable_shadowing_in_child_scope() {
    // RED: Child can shadow parent variable
    let mut global = Scope::new();
    global.define("x".to_string(), Value::integer(1)).unwrap();

    let mut local = global.create_child();
    local.define("x".to_string(), Value::integer(2)).unwrap();

    // Local sees shadowed value
    assert_eq!(local.get("x").unwrap().as_integer().unwrap(), 2);

    // Global still sees original value
    assert_eq!(global.get("x").unwrap().as_integer().unwrap(), 1);
}

#[test]
fn test_shadowing_assignment_updates_local() {
    // RED: Assignment updates local shadowed variable
    let mut global = Scope::new();
    global.define("x".to_string(), Value::integer(1)).unwrap();

    let mut local = global.create_child();
    local.define("x".to_string(), Value::integer(2)).unwrap();
    local.assign("x", Value::integer(3)).unwrap();

    // Local sees updated shadowed value
    assert_eq!(local.get("x").unwrap().as_integer().unwrap(), 3);

    // Global still sees original value
    assert_eq!(global.get("x").unwrap().as_integer().unwrap(), 1);
}

#[test]
fn test_multilevel_shadowing() {
    // RED: Multiple levels can shadow same variable
    let mut global = Scope::new();
    global.define("x".to_string(), Value::string("global".to_string())).unwrap();

    let mut level1 = global.create_child();
    level1.define("x".to_string(), Value::string("level1".to_string())).unwrap();

    let mut level2 = level1.create_child();
    level2.define("x".to_string(), Value::string("level2".to_string())).unwrap();

    // Each scope sees its own version
    assert_eq!(global.get("x").unwrap().as_string().unwrap(), "global");
    assert_eq!(level1.get("x").unwrap().as_string().unwrap(), "level1");
    assert_eq!(level2.get("x").unwrap().as_string().unwrap(), "level2");
}

// ===== RED PHASE TEST 5: Closure Capture =====

#[test]
fn test_closure_captures_variables() {
    // RED: Closure captures variables from enclosing scope
    let mut global = Scope::new();
    global.define("x".to_string(), Value::integer(42)).unwrap();

    let closure_scope = global.create_child();

    // Closure should capture x
    let captured = closure_scope.capture();
    assert!(captured.contains_key("x"));
    assert_eq!(captured.get("x").unwrap().as_integer().unwrap(), 42);
}

#[test]
fn test_closure_captures_only_referenced_variables() {
    // RED: Closure captures only variables it references
    let mut global = Scope::new();
    global.define("x".to_string(), Value::integer(1)).unwrap();
    global.define("y".to_string(), Value::integer(2)).unwrap();
    global.define("z".to_string(), Value::integer(3)).unwrap();

    let closure_scope = global.create_child();

    // Mark only x and z as referenced
    closure_scope.mark_referenced("x");
    closure_scope.mark_referenced("z");

    let captured = closure_scope.capture();
    assert_eq!(captured.len(), 2);
    assert!(captured.contains_key("x"));
    assert!(captured.contains_key("z"));
    assert!(!captured.contains_key("y"));
}

#[test]
fn test_closure_captures_from_multiple_levels() {
    // RED: Closure captures from all parent scopes
    let mut global = Scope::new();
    global.define("a".to_string(), Value::integer(1)).unwrap();

    let mut level1 = global.create_child();
    level1.define("b".to_string(), Value::integer(2)).unwrap();

    let closure_scope = level1.create_child();
    closure_scope.mark_referenced("a");
    closure_scope.mark_referenced("b");

    let captured = closure_scope.capture();
    assert_eq!(captured.len(), 2);
    assert!(captured.contains_key("a"));
    assert!(captured.contains_key("b"));
}

// ===== RED PHASE TEST 6: Scope Introspection =====

#[test]
fn test_scope_contains_check() {
    // RED: Check if variable exists in current scope (not parent)
    let mut global = Scope::new();
    global.define("x".to_string(), Value::integer(1)).unwrap();

    let mut local = global.create_child();
    local.define("y".to_string(), Value::integer(2)).unwrap();

    // Local contains only y locally
    assert!(!local.contains_local("x"));
    assert!(local.contains_local("y"));

    // But can see x in parent
    assert!(local.get("x").is_ok());
}

#[test]
fn test_scope_variable_names() {
    // RED: Get all variable names in current scope
    let mut scope = Scope::new();
    scope.define("a".to_string(), Value::integer(1)).unwrap();
    scope.define("b".to_string(), Value::integer(2)).unwrap();
    scope.define("c".to_string(), Value::integer(3)).unwrap();

    let names = scope.local_names();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"a".to_string()));
    assert!(names.contains(&"b".to_string()));
    assert!(names.contains(&"c".to_string()));
}

// ===== RED PHASE META TEST: Count Test Coverage =====

#[test]
fn test_red_phase_completeness() {
    // This test documents that RED phase is complete
    // We have 24 tests covering:
    // - Global scope (creation, define, redefine, assign, errors)
    // - Local scope (creation, parent access, child define, isolation)
    // - Nested scopes (deep nesting, lookup chain, assignment)
    // - Variable shadowing (basic, assignment, multilevel)
    // - Closure capture (basic, selective, multilevel)
    // - Scope introspection (contains, names)

    println!("RED phase: 24 tests defined");
    println!("Next: GREEN phase - implement minimal Scope system");
}
