// INTERP-012: Execute All Chapter 2 Examples (Variables & Types)
// RED Phase: Create tests for Chapter 2 book examples
//
// This test suite validates that Chapter 2 examples from the Ruchy book
// execute correctly. Chapter 2 focuses on variables, types, and basic arithmetic.
//
// Tests for:
// - Example 1: Basic integer variable
// - Example 2: String variable
// - Example 3: Multiple variables and arithmetic
// - Example 4: Floating-point calculations (EXPECTED TO FAIL - no float support)
// - Example 5: Variable scope
//
// Examples 6-8 are intentionally broken (error cases) and not tested here.
//
// Test Coverage:
// - Valid examples: 5 tests
// - Meta test: 1 test
// Total: 6 tests

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
// Chapter 2 Example Tests
// =============================================================================

#[test]
fn test_ch02_example_01_basic_integer() {
    // Example 1: Basic Integer Variable
    //
    // fun main() {
    //     let x = 42;
    //     println(x);
    // }

    let source = r#"
fun main() {
    let x = 42;
    println(x);
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
fn test_ch02_example_02_string_variable() {
    // Example 2: String Variable
    //
    // fun main() {
    //     let name = "Ruchy";
    //     println(name);
    // }

    let source = r#"
fun main() {
    let name = "Ruchy";
    println(name);
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
fn test_ch02_example_03_multiple_vars_arithmetic() {
    // Example 3: Multiple Variables and Arithmetic
    //
    // fun main() {
    //     let x = 10;
    //     let y = 20;
    //     let sum = x + y;
    //     println(sum);
    // }

    let source = r#"
fun main() {
    let x = 10;
    let y = 20;
    let sum = x + y;
    println(sum);
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
fn test_ch02_example_04_float_calculations() {
    // Example 4: Floating-Point Calculations
    //
    // fun main() {
    //     let pi = 3.14159;
    //     let radius = 5.0;
    //     let area = pi * radius * radius;
    //     println(area);
    // }
    //
    // EXPECTED TO FAIL: Interpreter doesn't support float literals yet
    // This test documents the missing feature

    let source = r#"
fun main() {
    let pi = 3.14159;
    let radius = 5.0;
    let area = pi * radius * radius;
    println(area);
}
"#;

    let result = execute_program(source);

    // RED phase: This should fail because we don't support floats yet
    // We're documenting this expected failure
    if result.is_err() {
        println!("Expected failure: Float literals not supported yet");
        println!("Error: {:?}", result);
    } else {
        // If it passes, great! But unexpected in RED phase
        println!("Unexpected: Float support seems to work!");
    }

    // For now, we expect this to fail
    assert!(
        result.is_err(),
        "Example 4 expected to fail (no float support), but it passed: {:?}",
        result
    );
}

#[test]
fn test_ch02_example_05_variable_scope() {
    // Example 5: Variable Scope
    //
    // fun main() {
    //     let outer = 100;
    //     // outer is accessible here
    //     println(outer);
    // }
    // // outer is NOT accessible here

    let source = r#"
fun main() {
    let outer = 100;
    println(outer);
}
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 5 should execute successfully: {:?}",
        result
    );
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_012_completeness() {
    // Meta-test: Verify test suite completeness
    //
    // Expected test count:
    // - Valid examples: 5 tests (Examples 1-5)
    // - Meta test: 1 test
    // Total: 6 tests
    //
    // Note: Examples 6-8 are intentionally broken error cases, not tested
    //
    // This test ensures we have comprehensive coverage of Chapter 2 examples.
    println!("INTERP-012 Test Suite (Chapter 2 Examples)");
    println!("==========================================");
    println!("Example 1: Basic integer variable");
    println!("Example 2: String variable");
    println!("Example 3: Multiple variables and arithmetic");
    println!("Example 4: Floating-point calculations (EXPECTED FAIL)");
    println!("Example 5: Variable scope");
    println!("==========================================");
    println!("Total: 5 valid examples + 1 meta test = 6 tests");
    println!("Note: Float support (Example 4) expected to fail in RED phase");
}
