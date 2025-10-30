// INTERP-002: Value Representation System - RED Phase Tests
// These tests define the runtime value interface through EXTREME TDD
//
// Research: Ierusalimschy et al. (2007) Section 3: Value Representation
//
// Test Strategy:
// 1. Create all 6 value types (Integer, String, Boolean, Vector, HashMap, Function)
// 2. Test type checking at runtime
// 3. Test basic operations on each type
// 4. Test memory safety (ownership/borrowing)

use ruchyruchy::interpreter::value::{Value, ValueError};

// ===== RED PHASE TEST 1: Create Integer Value =====

#[test]
fn test_create_integer_value() {
    // RED: This test will fail because Value doesn't exist yet
    let val = Value::integer(42);

    assert!(val.is_integer());
    assert_eq!(val.as_integer().unwrap(), 42);
}

#[test]
fn test_integer_arithmetic() {
    // RED: Test integer operations
    let a = Value::integer(10);
    let b = Value::integer(5);

    // Addition
    let sum = a.add(&b).unwrap();
    assert_eq!(sum.as_integer().unwrap(), 15);

    // Subtraction
    let diff = a.subtract(&b).unwrap();
    assert_eq!(diff.as_integer().unwrap(), 5);

    // Multiplication
    let prod = a.multiply(&b).unwrap();
    assert_eq!(prod.as_integer().unwrap(), 50);

    // Division
    let quot = a.divide(&b).unwrap();
    assert_eq!(quot.as_integer().unwrap(), 2);
}

// ===== RED PHASE TEST 2: Create String Value =====

#[test]
fn test_create_string_value() {
    // RED: Test string value creation
    let val = Value::string("hello".to_string());

    assert!(val.is_string());
    assert_eq!(val.as_string().unwrap(), "hello");
}

#[test]
fn test_string_concatenation() {
    // RED: Test string operations
    let a = Value::string("hello".to_string());
    let b = Value::string(" world".to_string());

    let result = a.add(&b).unwrap();
    assert_eq!(result.as_string().unwrap(), "hello world");
}

// ===== RED PHASE TEST 3: Create Boolean Value =====

#[test]
fn test_create_boolean_value() {
    // RED: Test boolean value creation
    let val_true = Value::boolean(true);
    let val_false = Value::boolean(false);

    assert!(val_true.is_boolean());
    assert!(val_false.is_boolean());

    assert_eq!(val_true.as_boolean().unwrap(), true);
    assert_eq!(val_false.as_boolean().unwrap(), false);
}

#[test]
fn test_boolean_logical_operations() {
    // RED: Test logical operations
    let t = Value::boolean(true);
    let f = Value::boolean(false);

    // AND
    let and_result = t.logical_and(&f).unwrap();
    assert_eq!(and_result.as_boolean().unwrap(), false);

    // OR
    let or_result = t.logical_or(&f).unwrap();
    assert_eq!(or_result.as_boolean().unwrap(), true);

    // NOT
    let not_result = t.logical_not().unwrap();
    assert_eq!(not_result.as_boolean().unwrap(), false);
}

// ===== RED PHASE TEST 4: Create Vector Value =====

#[test]
fn test_create_vector_value() {
    // RED: Test vector (array) creation
    let elements = vec![
        Value::integer(1),
        Value::integer(2),
        Value::integer(3),
    ];

    let val = Value::vector(elements);

    assert!(val.is_vector());
    assert_eq!(val.as_vector().unwrap().len(), 3);
}

#[test]
fn test_vector_indexing() {
    // RED: Test vector element access
    let elements = vec![
        Value::integer(10),
        Value::integer(20),
        Value::integer(30),
    ];

    let vec = Value::vector(elements);

    let elem = vec.index(1).unwrap();
    assert_eq!(elem.as_integer().unwrap(), 20);
}

#[test]
fn test_vector_push() {
    // RED: Test vector mutation
    let mut vec = Value::vector(vec![]);

    vec.push(Value::integer(42)).unwrap();
    vec.push(Value::string("hello".to_string())).unwrap();

    assert_eq!(vec.as_vector().unwrap().len(), 2);
}

// ===== RED PHASE TEST 5: Create HashMap Value =====

#[test]
fn test_create_hashmap_value() {
    // RED: Test hashmap creation
    let mut map = Value::hashmap();

    assert!(map.is_hashmap());
    assert_eq!(map.as_hashmap().unwrap().len(), 0);
}

#[test]
fn test_hashmap_insert_and_get() {
    // RED: Test hashmap operations
    let mut map = Value::hashmap();

    map.insert(
        Value::string("name".to_string()),
        Value::string("Alice".to_string())
    ).unwrap();

    map.insert(
        Value::string("age".to_string()),
        Value::integer(30)
    ).unwrap();

    let name = map.get(&Value::string("name".to_string())).unwrap();
    assert_eq!(name.as_string().unwrap(), "Alice");

    let age = map.get(&Value::string("age".to_string())).unwrap();
    assert_eq!(age.as_integer().unwrap(), 30);
}

// ===== RED PHASE TEST 6: Create Function Value =====

#[test]
fn test_create_function_value() {
    // RED: Test function value (closure)
    let params = vec!["x".to_string(), "y".to_string()];
    let body = vec![]; // Empty AST for now

    let func = Value::function(params, body);

    assert!(func.is_function());
    assert_eq!(func.as_function_params().unwrap().len(), 2);
}

// ===== RED PHASE TEST 7: Type Checking =====

#[test]
fn test_type_mismatch_errors() {
    // RED: Test runtime type errors
    let int_val = Value::integer(42);
    let str_val = Value::string("hello".to_string());

    // Adding integer to string should fail
    let result = int_val.add(&str_val);
    assert!(result.is_err());

    match result.err().unwrap() {
        ValueError::TypeMismatch { expected, found, .. } => {
            assert_eq!(expected, "Integer");
            assert_eq!(found, "String");
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_comparison_operations() {
    // RED: Test comparison operators
    let a = Value::integer(10);
    let b = Value::integer(20);

    assert_eq!(a.less_than(&b).unwrap().as_boolean().unwrap(), true);
    assert_eq!(a.greater_than(&b).unwrap().as_boolean().unwrap(), false);
    assert_eq!(a.equals(&b).unwrap().as_boolean().unwrap(), false);
    assert_eq!(a.equals(&a).unwrap().as_boolean().unwrap(), true);
}

// ===== RED PHASE TEST 8: Display and Debug =====

#[test]
fn test_value_display() {
    // RED: Test string representation
    let int_val = Value::integer(42);
    assert_eq!(format!("{}", int_val), "42");

    let str_val = Value::string("hello".to_string());
    assert_eq!(format!("{}", str_val), "\"hello\"");

    let bool_val = Value::boolean(true);
    assert_eq!(format!("{}", bool_val), "true");
}

// ===== RED PHASE TEST 9: Cloning Values =====

#[test]
fn test_value_cloning() {
    // RED: Test that values can be cloned
    let original = Value::integer(42);
    let cloned = original.clone();

    assert_eq!(cloned.as_integer().unwrap(), 42);

    // Verify they're equal but independent
    assert_eq!(original.equals(&cloned).unwrap().as_boolean().unwrap(), true);
}

// ===== RED PHASE TEST 10: Type Name Introspection =====

#[test]
fn test_type_name() {
    // RED: Test type introspection
    assert_eq!(Value::integer(42).type_name(), "Integer");
    assert_eq!(Value::string("hi".to_string()).type_name(), "String");
    assert_eq!(Value::boolean(true).type_name(), "Boolean");
    assert_eq!(Value::vector(vec![]).type_name(), "Vector");
    assert_eq!(Value::hashmap().type_name(), "HashMap");
}

// ===== RED PHASE META TEST: Count Test Coverage =====

#[test]
fn test_red_phase_completeness() {
    // This test documents that RED phase is complete
    // We have 18 tests covering:
    // - Integer values (creation, arithmetic)
    // - String values (creation, concatenation)
    // - Boolean values (creation, logical ops)
    // - Vector values (creation, indexing, mutation)
    // - HashMap values (creation, insert, get)
    // - Function values (creation, params)
    // - Type checking (errors, comparisons)
    // - Display formatting
    // - Cloning
    // - Type introspection

    println!("RED phase: 18 tests defined");
    println!("Next: GREEN phase - implement minimal Value system");
}
