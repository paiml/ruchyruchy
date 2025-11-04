// INTERP-044: Closure Support with Environment Capture
//
// EXTREME TDD - RED Phase
//
// Mission: Implement closures with proper environment capture
// - Basic closure syntax: |params| { body }
// - Environment capture: Access variables from outer scope
// - Move semantics: move keyword for taking ownership
// - Return closures from functions
//
// Test Strategy:
// - 7 tests covering major closure features
// - Simple execution validation (RED phase - closures not implemented)
// - All tests WILL FAIL initially due to unimplemented closure support

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;

/// Helper function to execute a program
fn execute_program(source: &str) -> Result<(), String> {
    let mut parser = Parser::new(source);
    let ast = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    let mut evaluator = Evaluator::new();

    for node in ast.nodes() {
        evaluator
            .eval(node)
            .map_err(|e| format!("Eval error: {:?}", e))?;
    }

    Ok(())
}

/// Test 1: Basic closure syntax
#[test]
fn test_closure_basic() {
    let source = r#"
fun main() {
    let add_one = |x| { x + 1 };
    let result = add_one(5);
    println(result);
}

main();
"#;

    let result = execute_program(source);
    // Expected: Closure captures nothing, simple parameter
    assert!(result.is_ok(), "Basic closure should work: {:?}", result);
}

/// Test 2: Closure with environment capture
#[test]
fn test_closure_capture() {
    let source = r#"
fun main() {
    let y = 10;
    let add_y = |x| { x + y };
    let result = add_y(5);
    println(result);
}

main();
"#;

    let result = execute_program(source);
    // Expected: Closure captures variable 'y' from outer scope
    // Result should be 15 (5 + 10)
    assert!(result.is_ok(), "Closure capture should work: {:?}", result);
}

/// Test 3: Closure with move semantics
#[test]
fn test_closure_move() {
    let source = r#"
fun main() {
    let x = vec![1, 2, 3];
    let use_x = move |i| { x[i] };
    let result = use_x(1);
    println(result);
}

main();
"#;

    let result = execute_program(source);
    // Expected: Closure takes ownership of x
    // Result should be 2 (x[1])
    assert!(result.is_ok(), "Closure move should work: {:?}", result);
}

/// Test 4: Closure returning from function
#[test]
fn test_closure_return() {
    let source = r#"
fun make_adder(n) {
    |x| { x + n }
}

fun main() {
    let add_5 = make_adder(5);
    let result = add_5(10);
    println(result);
}

main();
"#;

    let result = execute_program(source);
    // Expected: make_adder returns closure that captures n
    // Result should be 15 (10 + 5)
    assert!(result.is_ok(), "Closure return should work: {:?}", result);
}

/// Test 5: Closure with multiple parameters
#[test]
fn test_closure_multi_params() {
    let source = r#"
fun main() {
    let multiply = |x, y| { x * y };
    let result = multiply(3, 4);
    println(result);
}

main();
"#;

    let result = execute_program(source);
    // Expected: Closure with multiple parameters
    // Result should be 12 (3 * 4)
    assert!(
        result.is_ok(),
        "Closure multi-params should work: {:?}",
        result
    );
}

/// Test 6: Closure mutating captured variable
#[test]
fn test_closure_mut_capture() {
    let source = r#"
fun main() {
    let mut count = 0;
    let mut increment = || { count = count + 1; count };
    increment();
    increment();
    let result = increment();
    println(result);
}

main();
"#;

    let result = execute_program(source);
    // Expected: Closure mutates captured variable count
    // Result should be 3 (after 3 calls)
    assert!(
        result.is_ok(),
        "Closure mut capture should work: {:?}",
        result
    );
}

/// Test 7: Nested closures
#[test]
fn test_closure_nested() {
    let source = r#"
fun main() {
    let x = 10;
    let outer = |y| {
        let inner = |z| { x + y + z };
        inner(5)
    };
    let result = outer(20);
    println(result);
}

main();
"#;

    let result = execute_program(source);
    // Expected: Nested closures with multiple captures
    // Result should be 35 (10 + 20 + 5)
    assert!(result.is_ok(), "Nested closures should work: {:?}", result);
}
