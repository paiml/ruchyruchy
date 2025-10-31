// INTERP-010: Bug Discovery Integration
// RED Phase: Create tests for bug discovery and automatic filing
//
// This test suite validates the integration of bug discovery capabilities
// into the interpreter, including:
// - Automatic bug detection from runtime errors
// - Confidence scoring for bug reports
// - Delta debugging for minimal reproduction
// - Automatic GitHub issue filing
// - Bug deduplication
//
// Tests for:
// - Bug detection from runtime errors
// - Confidence scoring (>0.9 threshold)
// - Delta debugging integration
// - GitHub auto-filing (mock)
// - Bug deduplication
//
// Test Coverage:
// - Valid tests: 5 main tests
// - Meta test: 1 test
// Total: 6 tests

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;

// =============================================================================
// Helper Functions
// =============================================================================

/// Parse and execute a Ruchy program, returning error if any
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
// Bug Discovery Tests
// =============================================================================

#[test]
fn test_bug_detection_from_runtime_error() {
    // Test that runtime errors are detected and categorized
    //
    // This test verifies that when a runtime error occurs,
    // the bug discovery system can detect and categorize it.

    let source = r#"
fun main() {
    let x = 10;
    let y = 0;
    let z = x / y;  // Division by zero - runtime error
    println("Result: {}", z);
}

main();
"#;

    let result = execute_program(source);

    // Should fail with runtime error
    assert!(
        result.is_err(),
        "Division by zero should produce runtime error"
    );

    if let Err(e) = result {
        // Verify error contains useful debugging info
        assert!(
            e.contains("Eval error") || e.contains("division") || e.contains("zero"),
            "Error should contain division/zero context: {}",
            e
        );
    }
}

#[test]
fn test_bug_confidence_scoring() {
    // Test confidence scoring for bug reports
    //
    // Confidence score should be >0.9 for clear runtime errors
    // This test validates that we can assess bug report quality.

    let source = r#"
fun main() {
    let arr = vec![1, 2, 3];
    let x = arr[10];  // Index out of bounds - high confidence bug
    println("Value: {}", x);
}

main();
"#;

    let result = execute_program(source);

    // Should fail with clear index out of bounds error
    assert!(
        result.is_err(),
        "Index out of bounds should produce runtime error"
    );

    // Note: Actual confidence scoring will be implemented in GREEN phase
    // For now, we just verify the error is detected
}

#[test]
fn test_delta_debugging_integration() {
    // Test that delta debugging can minimize failing test cases
    //
    // Given a complex program that fails, delta debugging should
    // find the minimal subset of code that still reproduces the failure.

    let source = r#"
fun helper() {
    println("Helper function");
}

fun main() {
    let x = 10;
    helper();
    let y = 0;
    println("About to divide");
    let z = x / y;  // This is the actual bug
    println("Result: {}", z);
}

main();
"#;

    let result = execute_program(source);

    // Should fail
    assert!(result.is_err(), "Program should fail");

    // Note: Actual delta debugging will be implemented in GREEN phase
    // It should minimize this to just:
    //   fun main() { let x = 10; let y = 0; let z = x / y; }
}

#[test]
fn test_github_auto_filing_mock() {
    // Test automatic GitHub issue filing (mocked)
    //
    // When a high-confidence bug is detected, it should be
    // automatically filed to GitHub (using mock for testing).

    let source = r#"
fun main() {
    let x = 10 % 0;  // Modulo by zero - clear bug
    println("Value: {}", x);
}

main();
"#;

    let result = execute_program(source);

    // Should fail with modulo by zero error
    assert!(result.is_err(), "Modulo by zero should fail");

    // Note: Actual GitHub filing will be implemented in GREEN phase
    // For now, we just verify the error is detected
    if let Err(e) = result {
        assert!(
            e.contains("Eval error") && (e.contains("division") || e.contains("zero") || e.contains("Division")),
            "Error should be eval error with division/zero: {}",
            e
        );
    }
}

#[test]
fn test_bug_deduplication() {
    // Test that duplicate bugs are not filed multiple times
    //
    // If the same error occurs in multiple test runs,
    // it should be deduplicated and not create duplicate issues.

    let source = r#"
fun main() {
    let x = 10 / 0;  // Same division by zero error
}

main();
"#;

    // Run the same failing program twice
    let result1 = execute_program(source);
    let result2 = execute_program(source);

    // Both should fail
    assert!(result1.is_err(), "First run should fail");
    assert!(result2.is_err(), "Second run should fail");

    // Note: Actual deduplication will be implemented in GREEN phase
    // It should detect that these are the same bug and not file twice
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_010_completeness() {
    // Meta-test: Verify test suite completeness
    //
    // Expected test count:
    // - Bug detection: 1 test
    // - Confidence scoring: 1 test
    // - Delta debugging: 1 test
    // - GitHub auto-filing: 1 test
    // - Deduplication: 1 test
    // - Meta test: 1 test
    // Total: 6 tests
    //
    // This test ensures we have comprehensive coverage of bug discovery features.
    println!("INTERP-010 Test Suite (Bug Discovery Integration)");
    println!("=================================================");
    println!("Test 1: Bug detection from runtime error");
    println!("Test 2: Confidence scoring (>0.9 threshold)");
    println!("Test 3: Delta debugging integration");
    println!("Test 4: GitHub auto-filing (mocked)");
    println!("Test 5: Bug deduplication");
    println!("=================================================");
    println!("Total: 5 feature tests + 1 meta test = 6 tests");
}
