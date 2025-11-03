// INTERP-OPT-005: String Operation Analysis
//
// EXTREME TDD - RED Phase
//
// Mission: Measure string allocation/cloning overhead
//
// Hypothesis to VALIDATE:
// - Identifier strings are cloned frequently (scope.define, scope.get)
// - String literals are cloned on every evaluation
// - Function names are cloned on every call
// - HashMap<String, Value> in Scope uses String keys (cloned on insert/lookup)
//
// Method: Micro-benchmark string-heavy operations
// Target: Identify if string operations warrant Arc<str> interning
//
// Current baseline (from previous work):
// - Variable lookup: 0.33 µs (includes String clone for name)
// - Function call: 0.69 µs (includes String clone for name)
//
// Goal: Determine what % of operation time is string overhead

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use std::time::Instant;

/// Test: Variable Declaration Performance
///
/// Measures: String allocation overhead in scope.define()
#[test]
fn test_variable_declaration_performance() {
    let program = r#"
        let variable_with_long_name_one = 1;
        let variable_with_long_name_two = 2;
        let variable_with_long_name_three = 3;
        let variable_with_long_name_four = 4;
        let variable_with_long_name_five = 5;
    "#;

    let iterations = 10000;

    println!("\n=== Variable Declaration Performance (10000 iterations) ===");

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
    println!("Operations: 5 variable declarations with long names");
    println!("Time per declaration: {:.2} µs", per_iter_us / 5.0);
    println!("Note: Each declaration clones identifier string");
    println!();
}

/// Test: Variable Lookup Performance
///
/// Measures: String comparison overhead in scope.get()
#[test]
fn test_variable_lookup_performance() {
    let program = r#"
        let var_a = 1;
        let var_b = 2;
        let var_c = 3;
        let result = var_a + var_b + var_c + var_a + var_b + var_c;
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
    println!("Operations: 3 declarations + 6 lookups + 5 additions");
    println!("Time per operation: {:.2} µs", per_iter_us / 14.0);
    println!("Note: Each lookup involves String comparison in HashMap");
    println!();
}

/// Test: String Literal Performance
///
/// Measures: String cloning overhead for string literals
#[test]
fn test_string_literal_performance() {
    let program = r#"
        let s1 = "This is a somewhat long string literal for testing";
        let s2 = "Another moderately long string for comparison";
        let s3 = "Yet another string to measure cloning overhead";
    "#;

    let iterations = 10000;

    println!("\n=== String Literal Performance (10000 iterations) ===");

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
    println!("Operations: 3 string literal evaluations");
    println!("Time per string: {:.2} µs", per_iter_us / 3.0);
    println!("Note: Each evaluation clones the string content");
    println!();
}

/// Test: Short vs Long Identifier Names
///
/// Measures: Impact of identifier length on performance
#[test]
fn test_short_vs_long_identifiers() {
    let short_program = r#"
        let a = 1;
        let b = 2;
        let c = 3;
        let d = a + b + c;
    "#;

    let long_program = r#"
        let very_long_variable_name_one = 1;
        let very_long_variable_name_two = 2;
        let very_long_variable_name_three = 3;
        let result = very_long_variable_name_one + very_long_variable_name_two + very_long_variable_name_three;
    "#;

    let iterations = 10000;

    println!("\n=== Short vs Long Identifiers (10000 iterations) ===");

    // Short identifiers
    let mut parser = Parser::new(short_program);
    let ast = parser.parse().expect("Parse should succeed");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement);
        }
    }
    let short_duration = start.elapsed();
    let short_per_iter_us = short_duration.as_micros() as f64 / iterations as f64;

    // Long identifiers
    let mut parser = Parser::new(long_program);
    let ast = parser.parse().expect("Parse should succeed");

    let start = Instant::now();
    for _ in 0..iterations {
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement);
        }
    }
    let long_duration = start.elapsed();
    let long_per_iter_us = long_duration.as_micros() as f64 / iterations as f64;

    println!(
        "Short identifiers (1 char): {:.2} µs/iter",
        short_per_iter_us
    );
    println!(
        "Long identifiers (25+ chars): {:.2} µs/iter",
        long_per_iter_us
    );
    println!(
        "Overhead from long names: {:.2} µs ({:.1}%)",
        long_per_iter_us - short_per_iter_us,
        ((long_per_iter_us - short_per_iter_us) / short_per_iter_us) * 100.0
    );
    println!("Note: Difference indicates string allocation/comparison cost");
    println!();
}

/// Test: Comparative Analysis - String Operations
///
/// Determines if string operations are significant enough to optimize
#[test]
fn test_comparative_string_analysis() {
    println!("\n============================================================");
    println!("String Operation Analysis");
    println!("============================================================");
    println!();

    let benchmarks = vec![
        ("Baseline (integers)", "let a = 1; let b = 2; let c = a + b; c", 10000),
        ("Short identifiers", "let x = 1; let y = 2; let z = x + y; z", 10000),
        ("Long identifiers", "let variable_one = 1; let variable_two = 2; let result = variable_one + variable_two; result", 10000),
        ("String literals", r#"let s1 = "hello"; let s2 = "world"; s1"#, 10000),
        ("Function calls", "fun f(x) { return x + 1; } let r = f(5); r", 10000),
    ];

    println!("{:<25} {:>12} {:>15}", "Operation", "Time (µs)", "Ops/sec");
    println!("{}", "=".repeat(55));

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

        println!("{:<25} {:>12.2} {:>15.0}", name, per_iter_us, ops_per_sec);
        results.push((name, per_iter_us));
    }

    println!();
    println!("=== String Overhead Analysis ===");

    let baseline = results[0].1;
    let short_idents = results[1].1;
    let long_idents = results[2].1;
    let string_literals = results[3].1;

    println!("Baseline (integers): {:.2} µs", baseline);
    println!(
        "Short identifiers: {:.2} µs ({:.1}% overhead)",
        short_idents,
        ((short_idents - baseline) / baseline) * 100.0
    );
    println!(
        "Long identifiers: {:.2} µs ({:.1}% overhead)",
        long_idents,
        ((long_idents - baseline) / baseline) * 100.0
    );
    println!(
        "String literals: {:.2} µs ({:.1}% overhead)",
        string_literals,
        ((string_literals - baseline) / baseline) * 100.0
    );
    println!();

    let identifier_overhead = long_idents - baseline;
    if identifier_overhead > 0.1 {
        println!(
            "🎯 STRING OVERHEAD IS SIGNIFICANT: {:.2} µs",
            identifier_overhead
        );
        println!("   Recommendation: Implement string interning (Arc<str>)");
        println!("   Potential savings: 10-15% with interned identifiers");
    } else {
        println!(
            "✓ String overhead is minimal: {:.2} µs",
            identifier_overhead
        );
        println!("   String interning may not be worthwhile");
    }
    println!();
    println!("============================================================");
}
