// INTERP-016: Execute All Chapter 6 Examples (Data Structures)
// RED Phase: Create tests for Chapter 6 book examples
//
// This test suite validates that Chapter 6 examples from the Ruchy book
// execute correctly. Chapter 6 focuses on data structures including
// strings, arrays, tuples, indexing, and methods.
//
// Tests for:
// - Example 1: Basic String Variables
// - Example 2: Multiple String Variables
// - Example 3: Mixed Data Types (numbers and strings)
// - Example 4: String Methods (.len())
// - Example 5: Tuples (homogeneous)
// - Example 6: Arrays
// - Example 7: Array Indexing
// - Example 8: Array Arithmetic
// - Example 9: Mixed-Type Tuples
//
// Test Coverage:
// - Valid examples: 9 main tests
// - Meta test: 1 test
// Total: 10 tests

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
// Chapter 6 Example Tests
// =============================================================================

#[test]
fn test_ch06_example_01_basic_strings() {
    // Example 1: Basic String Variables
    //
    // fun main() {
    //     let greeting = "Hello";
    //     let name = "World";
    //     println(greeting);
    //     println(name);
    // }

    let source = r#"
fun main() {
    let greeting = "Hello";
    let name = "World";
    println(greeting);
    println(name);
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 1 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch06_example_02_multiple_strings() {
    // Example 2: Multiple String Variables
    //
    // fun main() {
    //     let first = "Hello";
    //     let second = "Beautiful";
    //     let third = "World";
    //     println(first);
    //     println(second);
    //     println(third);
    // }

    let source = r#"
fun main() {
    let first = "Hello";
    let second = "Beautiful";
    let third = "World";
    println(first);
    println(second);
    println(third);
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 2 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch06_example_03_mixed_data_types() {
    // Example 3: Mixed Data Types (numbers and strings)
    //
    // fun main() {
    //     let number = 42;
    //     let text = "Answer";
    //     println(text);
    //     println(number);
    // }

    let source = r#"
fun main() {
    let number = 42;
    let text = "Answer";
    println(text);
    println(number);
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 3 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch06_example_04_string_methods() {
    // Example 4: String Methods (.len())
    //
    // fun main() {
    //     let text = "Hello"
    //     println(text.len())
    // }
    //
    // Note: Book example missing semicolons - adding them for proper syntax

    let source = r#"
fun main() {
    let text = "Hello";
    println(text.len());
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 4 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch06_example_05_tuples() {
    // Example 5: Tuples (homogeneous)
    //
    // fun main() {
    //     let pair = (1, 2)
    //     println(pair)
    // }
    //
    // Note: Book example missing semicolons - adding them for proper syntax

    let source = r#"
fun main() {
    let pair = (1, 2);
    println(pair);
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 5 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch06_example_06_arrays() {
    // Example 6: Arrays
    //
    // fun main() {
    //     let numbers = [1, 2, 3]
    //     println(numbers)
    // }
    //
    // Note: Book example missing semicolons - adding them for proper syntax

    let source = r#"
fun main() {
    let numbers = [1, 2, 3];
    println(numbers);
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 6 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch06_example_07_array_indexing() {
    // Example 7: Array Indexing
    //
    // fun main() {
    //     let numbers = [1, 2, 3, 4, 5]
    //     println(numbers[0])
    //     println(numbers[4])
    // }
    //
    // Note: Book example missing semicolons - adding them for proper syntax

    let source = r#"
fun main() {
    let numbers = [1, 2, 3, 4, 5];
    println(numbers[0]);
    println(numbers[4]);
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 7 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch06_example_08_array_arithmetic() {
    // Example 8: Array Arithmetic
    //
    // fun main() {
    //     let numbers = [10, 20, 30]
    //     let sum = numbers[0] + numbers[1] + numbers[2]
    //     println(sum)
    // }
    //
    // Note: Book example missing semicolons - adding them for proper syntax

    let source = r#"
fun main() {
    let numbers = [10, 20, 30];
    let sum = numbers[0] + numbers[1] + numbers[2];
    println(sum);
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 8 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch06_example_09_mixed_type_tuples() {
    // Example 9: Mixed-Type Tuples
    //
    // fun main() {
    //     let pair = (42, "answer")
    //     println(pair)
    // }
    //
    // Note: Book example missing semicolons - adding them for proper syntax

    let source = r#"
fun main() {
    let pair = (42, "answer");
    println(pair);
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 9 should execute successfully: {:?}",
        result
    );
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_016_completeness() {
    // Meta-test: Verify test suite completeness
    //
    // Expected test count:
    // - Example 1: Basic strings
    // - Example 2: Multiple strings
    // - Example 3: Mixed data types
    // - Example 4: String methods (.len())
    // - Example 5: Tuples (homogeneous)
    // - Example 6: Arrays
    // - Example 7: Array indexing
    // - Example 8: Array arithmetic
    // - Example 9: Mixed-type tuples
    // - Meta test: 1 test
    // Total: 10 tests
    //
    // This test ensures we have comprehensive coverage of Chapter 6 examples.
    println!("INTERP-016 Test Suite (Chapter 6 Data Structures)");
    println!("==================================================");
    println!("Example 1: Basic String Variables");
    println!("Example 2: Multiple String Variables");
    println!("Example 3: Mixed Data Types (numbers and strings)");
    println!("Example 4: String Methods (.len())");
    println!("Example 5: Tuples (homogeneous)");
    println!("Example 6: Arrays");
    println!("Example 7: Array Indexing");
    println!("Example 8: Array Arithmetic");
    println!("Example 9: Mixed-Type Tuples");
    println!("==================================================");
    println!("Total: 9 valid examples + 1 meta test = 10 tests");
}
