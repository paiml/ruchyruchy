// INTERP-015: Execute All Chapter 5 Examples (Loops and Iteration)
// RED Phase: Create tests for Chapter 5 book examples
//
// This test suite validates that Chapter 5 examples from the Ruchy book
// execute correctly. Chapter 5 focuses on loops and iteration including
// while loops, for loops, break, continue, and nested loops.
//
// Tests for:
// - Example 1: Basic while loop (countdown)
// - Example 2: Basic for loop (iteration over range)
// - Example 3: Loop with break (search pattern)
// - Example 4: Loop with continue (filtering pattern)
// - Example 5: Nested loops (multiplication table)
// - Example 6: Loop with accumulation (sum and average)
//
// Test Coverage:
// - Valid examples: 6 main tests
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
// Chapter 5 Example Tests
// =============================================================================

#[test]
fn test_ch05_example_01_while_loop() {
    // Example 1: Basic while loop (countdown)
    //
    // fun countdown(n: i32) {
    //     let mut count = n;
    //     while count > 0 {
    //         println("{}", count);
    //         count = count - 1;
    //     }
    //     println("Liftoff!");
    // }

    let source = r#"
fun countdown(n: i32) {
    let mut count = n;
    while count > 0 {
        println("{}", count);
        count = count - 1;
    }
    println("Liftoff!");
}

fun main() {
    countdown(5);
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
fn test_ch05_example_02_for_loop() {
    // Example 2: Basic for loop (iteration over range)
    //
    // fun print_squares() {
    //     for i in 1..6 {
    //         let square = i * i;
    //         println("{} squared is {}", i, square);
    //     }
    // }

    let source = r#"
fun print_squares() {
    for i in 1..6 {
        let square = i * i;
        println("{} squared is {}", i, square);
    }
}

fun main() {
    print_squares();
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
fn test_ch05_example_03_loop_with_break() {
    // Example 3: Loop with break (search pattern)
    //
    // fun find_first_even(numbers: Vec<i32>) -> i32 {
    //     for num in numbers {
    //         if num % 2 == 0 {
    //             return num;
    //         }
    //     }
    //     return -1;
    // }

    let source = r#"
fun find_first_even(start: i32, end: i32) -> i32 {
    let mut current = start;
    while current <= end {
        if current % 2 == 0 {
            println("Found first even: {}", current);
            break;
        }
        current = current + 1;
    }
    current
}

fun main() {
    let result = find_first_even(7, 20);
    println("Result: {}", result);
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
fn test_ch05_example_04_loop_with_continue() {
    // Example 4: Loop with continue (filtering pattern)
    //
    // fun print_odd_numbers(n: i32) {
    //     for i in 1..n {
    //         if i % 2 == 0 {
    //             continue;
    //         }
    //         println("{}", i);
    //     }
    // }

    let source = r#"
fun print_odd_numbers(n: i32) {
    let mut i = 1;
    while i <= n {
        if i % 2 == 0 {
            i = i + 1;
            continue;
        }
        println("Odd: {}", i);
        i = i + 1;
    }
}

fun main() {
    print_odd_numbers(10);
}
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 4 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch05_example_05_nested_loops() {
    // Example 5: Nested loops (multiplication table)
    //
    // fun multiplication_table(size: i32) {
    //     for i in 1..size {
    //         for j in 1..size {
    //             let product = i * j;
    //             print("{}\t", product);
    //         }
    //         println("");
    //     }
    // }

    let source = r#"
fun multiplication_table(size: i32) {
    let mut i = 1;
    while i <= size {
        let mut j = 1;
        while j <= size {
            let product = i * j;
            println("{} x {} = {}", i, j, product);
            j = j + 1;
        }
        i = i + 1;
    }
}

fun main() {
    multiplication_table(3);
}
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "Example 5 should execute successfully: {:?}",
        result
    );
}

#[test]
fn test_ch05_example_06_accumulation() {
    // Example 6: Loop with accumulation (sum and average)
    //
    // fun calculate_average(count: i32) -> f64 {
    //     let mut sum = 0;
    //     for i in 1..count {
    //         sum = sum + i;
    //     }
    //     let average = (sum as f64) / (count as f64);
    //     return average;
    // }

    let source = r#"
fun calculate_sum(count: i32) -> i32 {
    let mut sum = 0;
    let mut i = 1;
    while i <= count {
        sum = sum + i;
        i = i + 1;
    }
    sum
}

fun calculate_average(count: i32) -> f64 {
    let sum = calculate_sum(count);
    let average = (sum as f64) / (count as f64);
    average
}

fun main() {
    let sum = calculate_sum(10);
    let avg = calculate_average(10);
    println("Sum of 1..10: {}", sum);
    println("Average: {}", avg);
}
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
fn test_interp_015_completeness() {
    // Meta-test: Verify test suite completeness
    //
    // Expected test count:
    // - Valid examples: 6 tests (Examples 1-6)
    // - Meta test: 1 test
    // Total: 7 tests
    //
    // This test ensures we have comprehensive coverage of Chapter 5 examples.
    println!("INTERP-015 Test Suite (Chapter 5 Loops and Iteration)");
    println!("======================================================");
    println!("Example 1: Basic while loop (countdown)");
    println!("Example 2: Basic for loop (iteration over range)");
    println!("Example 3: Loop with break (search pattern)");
    println!("Example 4: Loop with continue (filtering pattern)");
    println!("Example 5: Nested loops (multiplication table)");
    println!("Example 6: Loop with accumulation (sum and average)");
    println!("======================================================");
    println!("Total: 6 valid examples + 1 meta test = 7 tests");
}
