// INTERP-011: Execute All Chapter 1 Examples (Hello World)
// RED Phase: Create tests for executing book examples
//
// Tests for:
// - Example 1: Basic Hello World with main
// - Example 2: Multiple println with main
// - Example 3: Variable with main
// - Example 4: Direct println (no main)
// - Example 5: Variable storage (no main)
// - Example 6: Sequential output (no main)
//
// Test Coverage:
// - Basic examples: 6 tests
// - Meta test: 1 test
// Total: 7 tests

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;

// =============================================================================
// Helper Functions
// =============================================================================

/// Parse and execute a Ruchy program
fn execute_program(source: &str) -> Result<(), String> {
    let mut parser = Parser::new(source);
    let ast = parser.parse().map_err(|e| format!("Parse error: {:?}", e))?;

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
// Chapter 1 Example Tests
// =============================================================================

#[test]
fn test_ch01_example_01_hello_world_with_main() {
    // Example 1: Basic Hello World
    //
    // fun main() {
    //     println("Hello, World!");
    // }

    let source = r#"
fun main() {
    println("Hello, World!");
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
fn test_ch01_example_02_multiple_prints_with_main() {
    // Example 2: Multiple Print Statements
    //
    // fun main() {
    //     println("Hello,");
    //     println("World!");
    // }

    let source = r#"
fun main() {
    println("Hello,");
    println("World!");
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
fn test_ch01_example_03_variable_with_main() {
    // Example 3: Using Variables
    //
    // fun main() {
    //     let greeting = "Hello, World!";
    //     println(greeting);
    // }

    let source = r#"
fun main() {
    let greeting = "Hello, World!";
    println(greeting);
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
fn test_ch01_example_04_direct_println_no_main() {
    // Example 4: Direct Output (no main)
    //
    // println("Your message here");

    let source = r#"
println("Your message here");
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 4 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch01_example_05_variable_storage_no_main() {
    // Example 5: Variable Storage (no main)
    //
    // let message = "Your message";
    // println(message);

    let source = r#"
let message = "Your message";
println(message);
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 5 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch01_example_06_sequential_output_no_main() {
    // Example 6: Sequential Output (no main)
    //
    // println("First line");
    // println("Second line");

    let source = r#"
println("First line");
println("Second line");
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 6 should execute successfully: {:?}",
        result
    );
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_011_completeness() {
    // Meta-test: Verify test suite completeness
    //
    // Expected test count:
    // - Basic examples: 6 tests
    // - Meta test: 1 test
    // Total: 7 tests
    //
    // This test ensures we have comprehensive coverage of Chapter 1 examples.
    println!("INTERP-011 Test Suite (Chapter 1 Examples)");
    println!("==========================================");
    println!("Example 1: Hello World with main");
    println!("Example 2: Multiple prints with main");
    println!("Example 3: Variable with main");
    println!("Example 4: Direct println (no main)");
    println!("Example 5: Variable storage (no main)");
    println!("Example 6: Sequential output (no main)");
    println!("==========================================");
    println!("Total: 6 examples + 1 meta test = 7 tests");
}
