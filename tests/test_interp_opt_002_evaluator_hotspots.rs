// INTERP-OPT-002: Evaluator Hotspot Analysis
//
// EXTREME TDD - RED Phase
//
// Mission: Identify THE specific evaluator operations that dominate the 94.4%
//
// Hypothesis to VALIDATE:
// - Variable lookup (scope.get): ?% of eval time
// - Binary operations (+, -, *, /): ?% of eval time
// - Function calls (call stack): ?% of eval time
// - Cloning/allocations: ?% of eval time
//
// Method: Micro-benchmarks for each operation type
// Target: Find operation >30% of eval time = THE bottleneck

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use std::time::Instant;

/// Test: Variable Lookup Performance
///
/// Measures: scope.get() performance with many variables
#[test]
fn test_variable_lookup_performance() {
    // Program with many variable lookups
    let program = r#"
        let a = 1;
        let b = 2;
        let c = 3;
        let d = 4;
        let e = 5;
        let result = a + b + c + d + e;
        result
    "#;

    let iterations = 10000;

    println!("\n=== Variable Lookup Performance (10000 iterations) ===");

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
    println!("Operations: 5 let bindings + 5 lookups + 4 additions");
    println!("Time per operation: {:.2} µs", per_iter_us / 14.0);
    println!();
}

/// Test: Binary Operation Performance
///
/// Measures: Addition, subtraction, multiplication, division
#[test]
fn test_binary_operation_performance() {
    let program = r#"
        let result = 10 + 20 - 5 * 2 / 2;
        result
    "#;

    let iterations = 10000;

    println!("\n=== Binary Operation Performance (10000 iterations) ===");

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
    println!("Operations: 4 binary ops (+ - * /)");
    println!("Time per operation: {:.2} µs", per_iter_us / 4.0);
    println!();
}

/// Test: Function Call Performance
///
/// Measures: Function definition + call overhead
/// Note: Skipped - function syntax not yet supported in simple interpreter
#[test]
#[ignore]
fn test_function_call_performance() {
    println!("Function calls not yet supported - skipped");
}

/// Test: Scope Creation Performance
///
/// Measures: Creating new scopes for blocks/functions
#[test]
fn test_scope_creation_performance() {
    let program = r#"
        let x = 1;
        {
            let y = 2;
            {
                let z = 3;
                z
            }
        }
    "#;

    let iterations = 10000;

    println!("\n=== Scope Creation Performance (10000 iterations) ===");

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
    println!("Operations: 3 scope creations (nested blocks)");
    println!("Time per scope: {:.2} µs", per_iter_us / 3.0);
    println!();
}

/// Test: Value Cloning Performance
///
/// Measures: Cost of cloning values during operations
#[test]
fn test_value_cloning_performance() {
    // Many operations that require value clones
    let program = r#"
        let a = 10;
        let b = a;
        let c = b;
        let d = c;
        let e = d;
        e
    "#;

    let iterations = 10000;

    println!("\n=== Value Cloning Performance (10000 iterations) ===");

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
    println!("Operations: 5 variable bindings (5 clones)");
    println!("Time per clone: {:.2} µs", per_iter_us / 5.0);
    println!();
}

/// Test: println Performance
///
/// Measures: Built-in function call overhead (println)
#[test]
fn test_println_performance() {
    let program = r#"println("Hello, World!")"#;

    let iterations = 1000; // Fewer iterations due to I/O

    println!("\n=== println Performance (1000 iterations) ===");

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
    println!("Note: Includes actual println I/O overhead");
    println!();
}

/// Test: Comparative Analysis - Find THE Bottleneck
///
/// Compares all operation types to identify the hottest
#[test]
fn test_comparative_hotspot_analysis() {
    println!("\n============================================================");
    println!("Evaluator Hotspot Analysis");
    println!("============================================================");
    println!();

    let benchmarks = vec![
        ("Baseline (empty)", "", 10000),
        (
            "Variable lookup",
            "let a = 1; let b = 2; let c = a + b; c",
            10000,
        ),
        ("Binary ops", "let x = 10 + 20 - 5 * 2 / 2; x", 10000),
        (
            "Scope creation",
            "let x = 1; { let y = 2; { let z = 3; z } }",
            10000,
        ),
        (
            "Value cloning",
            "let a = 10; let b = a; let c = b; let d = c; d",
            10000,
        ),
    ];

    println!("{:<20} {:>12} {:>15}", "Operation", "Time (µs)", "Ops/sec");
    println!("{}", "=".repeat(50));

    let mut results = Vec::new();

    for (name, program, iterations) in benchmarks {
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
        let ops_per_sec = iterations as f64 / duration.as_secs_f64();

        println!("{:<20} {:>12.2} {:>15.0}", name, per_iter_us, ops_per_sec);
        results.push((name, per_iter_us));
    }

    println!();
    println!("=== Amdahl's Law Analysis ===");

    // Find the slowest operation (excluding baseline)
    let baseline = results[0].1;
    let mut max_overhead = 0.0f64;
    let mut hottest_op = "";

    for (name, time) in &results[1..] {
        let overhead = time - baseline;
        if overhead > max_overhead {
            max_overhead = overhead;
            hottest_op = name;
        }
    }

    println!("Baseline (empty): {:.2} µs", baseline);
    println!();
    println!("🎯 HOTTEST OPERATION: {}", hottest_op);
    println!("   Overhead: {:.2} µs above baseline", max_overhead);
    println!("   This is THE bottleneck to optimize first");
    println!();
    println!("============================================================");
}
