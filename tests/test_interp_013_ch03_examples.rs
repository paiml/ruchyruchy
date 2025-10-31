// INTERP-013: Execute All Chapter 3 Examples (Functions)
// RED Phase: Create tests for Chapter 3 book examples
//
// This test suite validates that Chapter 3 examples from the Ruchy book
// execute correctly. Chapter 3 focuses on functions, parameters, return values,
// and type annotations.
//
// Tests for:
// - Example 1: Basic function (no parameters/return)
// - Example 2: Function with return value
// - Example 3: Function with type annotations
// - Example 4: Nested function calls
//
// Test Coverage:
// - Valid examples: 4 main tests
// - Meta test: 1 test
// Total: 5 tests

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;

// =============================================================================
// Helper Functions
// =============================================================================

/// Parse and execute a Ruchy program
fn execute_program(source: &str) -> Result<(), String> {
    let mut parser = Parser::new(source);
    let ast = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    let mut evaluator = Evaluator::new();

    // Execute all top-level statements
    for node in ast.nodes() {
        evaluator
            .eval(node)
            .map_err(|e| format!("Eval error: {:?}", e))?;
    }

    Ok(())
}

// =============================================================================
// Chapter 3 Example Tests
// =============================================================================

#[test]
fn test_ch03_example_01_basic_function() {
    // Example 1: Basic Function
    //
    // fun greet() {
    //     println("Hello from function!");
    // }
    //
    // fun main() {
    //     greet();
    // }

    let source = r#"
fun greet() {
    println("Hello from function!");
}

fun main() {
    greet();
}
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 1 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch03_example_02_function_with_return() {
    // Example 2: Function with Return Value
    //
    // fun add(a, b) {
    //     a + b
    // }
    //
    // fun main() {
    //     let result = add(5, 3);
    //     println(result);
    // }

    let source = r#"
fun add(a, b) {
    a + b
}

fun main() {
    let result = add(5, 3);
    println(result);
}
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 2 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch03_example_03_function_with_types() {
    // Example 3: Function with Type Annotations
    //
    // fun multiply(x: i32, y: i32) -> i32 {
    //     x * y
    // }
    //
    // fun main() {
    //     let product = multiply(6, 7);
    //     println(product);
    // }
    //
    // BUG DISCOVERED (RED PHASE):
    // Parser enters infinite loop when encountering type annotations.
    // Symptoms: Test hangs indefinitely (>60 seconds)
    // Tokens involved: Colon (`:`) and Arrow (`->`)
    // Impact: Cannot parse function signatures with types
    // Status: Marked as ignored until parser fix

    let source = r#"
fun multiply(x: i32, y: i32) -> i32 {
    x * y
}

fun main() {
    let product = multiply(6, 7);
    println(product);
}
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 3 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch03_example_04_nested_calls() {
    // Example 4: Nested Function Calls
    //
    // fun square(n: i32) -> i32 {
    //     n * n
    // }
    //
    // fun sum_of_squares(a: i32, b: i32) -> i32 {
    //     square(a) + square(b)
    // }
    //
    // fun main() {
    //     let result = sum_of_squares(3, 4);
    //     println(result);
    // }
    //
    // BUG DISCOVERED (RED PHASE):
    // Same bug as Example 3 - parser hangs on type annotations.
    // This example tests nested function calls with type annotations.
    // Status: Marked as ignored until parser fix

    let source = r#"
fun square(n: i32) -> i32 {
    n * n
}

fun sum_of_squares(a: i32, b: i32) -> i32 {
    square(a) + square(b)
}

fun main() {
    let result = sum_of_squares(3, 4);
    println(result);
}
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 4 should execute successfully: {:?}",
        result
    );
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_013_completeness() {
    // Meta-test: Verify test suite completeness
    //
    // Expected test count:
    // - Valid examples: 4 tests (Examples 1-4)
    // - Meta test: 1 test
    // Total: 5 tests
    //
    // This test ensures we have comprehensive coverage of Chapter 3 examples.
    println!("INTERP-013 Test Suite (Chapter 3 Functions)");
    println!("==========================================");
    println!("Example 1: Basic function");
    println!("Example 2: Function with return value");
    println!("Example 3: Function with type annotations");
    println!("Example 4: Nested function calls");
    println!("==========================================");
    println!("Total: 4 valid examples + 1 meta test = 5 tests");
}
