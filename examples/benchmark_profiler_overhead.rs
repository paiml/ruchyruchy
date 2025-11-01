// DEBUGGER-041: Benchmark profiler overhead
// Measures performance impact of profiling enabled vs disabled
// Target: <5% overhead

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use std::time::Instant;

fn main() {
    let code = r#"
        fun factorial(n) {
            if (n <= 1) { return 1; }
            return n * factorial(n - 1);
        }

        fun fibonacci(n) {
            if (n <= 1) { return n; }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }

        // Run multiple calls
        factorial(10);
        factorial(10);
        factorial(10);
        factorial(10);
        factorial(10);
        fibonacci(8);
        fibonacci(8);
        fibonacci(8);
        fibonacci(8);
        fibonacci(8);
    "#;

    const ITERATIONS: usize = 100;

    // Benchmark WITHOUT profiling
    println!("Benchmarking WITHOUT profiling...");
    let mut times_without = Vec::new();
    for _ in 0..ITERATIONS {
        let mut parser = Parser::new(code);
        let ast = parser.parse().unwrap();
        let mut eval = Evaluator::new();

        let start = Instant::now();
        for statement in ast.nodes() {
            eval.eval(statement).unwrap();
        }
        let duration = start.elapsed();
        times_without.push(duration);
    }

    // Benchmark WITH profiling
    println!("Benchmarking WITH profiling...");
    let mut times_with = Vec::new();
    for _ in 0..ITERATIONS {
        let mut parser = Parser::new(code);
        let ast = parser.parse().unwrap();
        let mut eval = Evaluator::new().with_profiling();

        let start = Instant::now();
        for statement in ast.nodes() {
            eval.eval(statement).unwrap();
        }
        let duration = start.elapsed();
        times_with.push(duration);

        // Show profiling stats from last run
        if times_with.len() == ITERATIONS {
            if let Some(profile) = eval.get_profiling_data() {
                println!("\nProfiling stats:");
                println!("  Max depth: {}", profile.max_depth);
                println!("  Total calls: {}", profile.total_calls);
            }
        }
    }

    // Calculate statistics
    let avg_without = times_without.iter().sum::<std::time::Duration>() / ITERATIONS as u32;
    let avg_with = times_with.iter().sum::<std::time::Duration>() / ITERATIONS as u32;

    let overhead = if avg_without.as_nanos() > 0 {
        ((avg_with.as_nanos() as f64 - avg_without.as_nanos() as f64)
            / avg_without.as_nanos() as f64)
            * 100.0
    } else {
        0.0
    };

    println!("\n=== Results ===");
    println!("Without profiling: {:?} avg", avg_without);
    println!("With profiling:    {:?} avg", avg_with);
    println!("Overhead:          {:.2}%", overhead);

    if overhead < 5.0 {
        println!("✅ PASS: Overhead <5% target");
    } else {
        println!("⚠️  WARNING: Overhead exceeds 5% target");
    }
}
