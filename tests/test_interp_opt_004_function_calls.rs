// INTERP-OPT-004: Function Call Optimization
//
// EXTREME TDD - RED Phase
//
// Mission: Optimize function call performance
//
// Hypothesis to VALIDATE:
// - Function parameter binding clones values unnecessarily
// - arg_values.iter() + value.clone() can be arg_values.into_iter()
// - Vec::new() for arg_values can use with_capacity
//
// Method: Micro-benchmark function calls
// Target: Reduce cloning in call_function hot path

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use std::time::Instant;

/// Test: Function Call Performance
///
/// Measures: Function definition + call overhead
#[test]
fn test_function_call_performance() {
    let program = r#"
        fun add(a, b) {
            return a + b;
        }
        let result = add(5, 3);
    "#;

    let iterations = 10000;

    println!("\n=== Function Call Performance (10000 iterations) ===");

    let mut parser = Parser::new(program);
    let ast = parser.parse().expect("Parse should succeed");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement);
        }
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Operations: 1 function definition + 1 function call");
    println!();
}

/// Test: Multi-parameter Function Call
///
/// Measures: Parameter binding overhead
#[test]
fn test_multi_param_function_call() {
    let program = r#"
        fun sum_five(a, b, c, d, e) {
            return a + b + c + d + e;
        }
        let result = sum_five(1, 2, 3, 4, 5);
    "#;

    let iterations = 10000;

    println!("\n=== Multi-Parameter Function Call (10000 iterations) ===");

    let mut parser = Parser::new(program);
    let ast = parser.parse().expect("Parse should succeed");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement);
        }
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Operations: 5 parameter bindings (each clones value)");
    println!("Time per parameter: {:.2} µs", per_iter_us / 5.0);
    println!();
}

/// Test: Recursive Function Call
///
/// Measures: Function call overhead with recursion
#[test]
fn test_recursive_function_call() {
    let program = r#"
        fun factorial(n) {
            if (n <= 1) {
                return 1;
            }
            return n * factorial(n - 1);
        }
        let result = factorial(5);
    "#;

    let iterations = 10000;

    println!("\n=== Recursive Function Call (10000 iterations) ===");

    let mut parser = Parser::new(program);
    let ast = parser.parse().expect("Parse should succeed");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement);
        }
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Operations: 5 recursive calls (factorial(5))");
    println!("Time per call: {:.2} µs", per_iter_us / 5.0);
    println!();
}
