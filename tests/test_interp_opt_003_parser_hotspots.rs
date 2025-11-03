// INTERP-OPT-003: Parser Hotspot Analysis
//
// EXTREME TDD - RED Phase
//
// Mission: Identify THE specific parser operations that dominate parsing time
//
// Hypothesis to VALIDATE:
// - Tokenization (lexer): ?% of parse time
// - Expression parsing (operators): ?% of parse time
// - Statement parsing (let, if, while): ?% of parse time
// - AST node allocation: ?% of parse time
//
// Method: Micro-benchmarks for each parser operation type
// Target: Find operation >30% of parse time = THE bottleneck
//
// Current State: Parser is 52.9% of total time (0.77 µs for simple program)
// Goal: Identify which parser operation to optimize first

use ruchyruchy::interpreter::parser::Parser;
use std::time::Instant;

/// Test: Tokenization Performance
///
/// Measures: Lexer tokenization speed (string → tokens)
#[test]
fn test_tokenization_performance() {
    // Program with various token types
    let program = r#"
        let a = 1;
        let b = 2;
        let c = 3;
        let result = a + b + c;
    "#;

    let iterations = 10000;

    println!("\n=== Tokenization Performance (10000 iterations) ===");

    let start = Instant::now();
    for _ in 0..iterations {
        let parser = Parser::new(program);
        // Parser tokenizes on construction
        drop(parser);
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Tokens: ~20 tokens (let, identifier, =, number, ;, operators)");
    println!("Time per token: {:.3} µs", per_iter_us / 20.0);
    println!();
}

/// Test: Expression Parsing Performance
///
/// Measures: Binary operator parsing with precedence
#[test]
fn test_expression_parsing_performance() {
    let program = r#"1 + 2 * 3 - 4 / 2"#;

    let iterations = 10000;

    println!("\n=== Expression Parsing Performance (10000 iterations) ===");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut parser = Parser::new(program);
        let _ = parser.parse().expect("Parse should succeed");
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Operations: 4 binary operators with precedence");
    println!("Time per operator: {:.2} µs", per_iter_us / 4.0);
    println!();
}

/// Test: Statement Parsing Performance
///
/// Measures: Let declarations and simple statements
#[test]
fn test_statement_parsing_performance() {
    let program = r#"
        let a = 10;
        let b = 20;
        let c = 30;
    "#;

    let iterations = 10000;

    println!("\n=== Statement Parsing Performance (10000 iterations) ===");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut parser = Parser::new(program);
        let _ = parser.parse().expect("Parse should succeed");
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Operations: 3 let declarations");
    println!("Time per statement: {:.2} µs", per_iter_us / 3.0);
    println!();
}

/// Test: Control Flow Parsing Performance
///
/// Measures: If-else and while loop parsing
#[test]
fn test_control_flow_parsing_performance() {
    let program = r#"
        if (x > 0) {
            let a = 1;
        } else {
            let b = 2;
        }
    "#;

    let iterations = 10000;

    println!("\n=== Control Flow Parsing Performance (10000 iterations) ===");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut parser = Parser::new(program);
        let _ = parser.parse().expect("Parse should succeed");
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Operations: 1 if-else with 2 branches");
    println!();
}

/// Test: Function Definition Parsing Performance
///
/// Measures: Function signature and body parsing
#[test]
fn test_function_parsing_performance() {
    let program = r#"
        fun add(a, b) {
            return a + b;
        }
    "#;

    let iterations = 10000;

    println!("\n=== Function Parsing Performance (10000 iterations) ===");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut parser = Parser::new(program);
        let _ = parser.parse().expect("Parse should succeed");
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Operations: 1 function definition with 2 params");
    println!();
}

/// Test: String Literal Parsing Performance
///
/// Measures: String tokenization and parsing
#[test]
fn test_string_parsing_performance() {
    let program = r#"
        let s1 = "hello";
        let s2 = "world";
        let s3 = "testing";
    "#;

    let iterations = 10000;

    println!("\n=== String Parsing Performance (10000 iterations) ===");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut parser = Parser::new(program);
        let _ = parser.parse().expect("Parse should succeed");
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Operations: 3 string literals");
    println!("Time per string: {:.2} µs", per_iter_us / 3.0);
    println!();
}

/// Test: Comparative Analysis - Find THE Parser Bottleneck
///
/// Compares all parser operation types to identify the hottest
#[test]
fn test_comparative_parser_hotspot_analysis() {
    println!("\n============================================================");
    println!("Parser Hotspot Analysis");
    println!("============================================================");
    println!();

    let benchmarks = vec![
        ("Empty", "", 10000),
        ("Tokenization", "let a = 1; let b = 2; let c = 3;", 10000),
        ("Expressions", "let x = 1 + 2 * 3 - 4 / 2; x", 10000),
        ("Statements", "let a = 10; let b = 20; let c = 30;", 10000),
        (
            "Control flow",
            "if (x > 0) { let a = 1; } else { let b = 2; }",
            10000,
        ),
        ("Functions", "fun add(a, b) { return a + b; }", 10000),
        ("Strings", r#"let s1 = "hello"; let s2 = "world";"#, 10000),
    ];

    println!("{:<20} {:>12} {:>15}", "Operation", "Time (µs)", "Ops/sec");
    println!("{}", "=".repeat(50));

    let mut results = Vec::new();

    for (name, program, iterations) in benchmarks {
        let start = Instant::now();
        for _ in 0..iterations {
            let mut parser = Parser::new(program);
            let _ = parser.parse().expect("Parse should succeed");
        }
        let duration = start.elapsed();

        let per_iter_us = duration.as_micros() as f64 / iterations as f64;
        let ops_per_sec = iterations as f64 / duration.as_secs_f64();

        println!("{:<20} {:>12.2} {:>15.0}", name, per_iter_us, ops_per_sec);
        results.push((name, per_iter_us));
    }

    println!();
    println!("=== Amdahl's Law Analysis ===");

    // Find the slowest operation (excluding empty)
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
