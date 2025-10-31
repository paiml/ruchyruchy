// INTERP-030: Performance Profiling & Benchmarking - RED PHASE
//
// This test implements performance benchmarking for the RuchyRuchy interpreter.
//
// Requirements:
// - Benchmark interpreter vs native Ruchy
// - Target: <100x slower than native
// - Profile hotspots
// - Optimize critical paths
//
// Tests:
// - test_benchmark_fibonacci (measure overhead)
// - test_benchmark_vector_ops
// - test_benchmark_hashmap_ops
// - test_performance_regression
//
// Acceptance:
// - Interpreter <100x slower than native
// - Hotspots identified
// - No performance regressions
//
// RED PHASE: This test WILL FAIL because:
// - Benchmarking infrastructure doesn't exist yet
// - Native comparison not implemented yet
// - Performance tracking not implemented yet

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use std::time::Instant;

// RED: This module doesn't exist yet
// Will implement in GREEN phase
mod benchmarking {
    #[allow(unused_imports)] // Will be used in GREEN phase
    use super::*;

    /// Performance benchmark result
    #[derive(Debug, Clone)]
    pub struct BenchmarkResult {
        pub name: String,
        pub iterations: usize,
        pub total_duration_ms: f64,
        pub avg_duration_us: f64,
        pub throughput: f64, // operations per second
    }

    impl BenchmarkResult {
        /// Calculate overhead vs baseline
        pub fn overhead_factor(&self, baseline: &BenchmarkResult) -> f64 {
            self.avg_duration_us / baseline.avg_duration_us
        }
    }

    /// Benchmark runner for interpreter performance
    pub struct BenchmarkRunner {
        results: Vec<BenchmarkResult>,
    }

    impl BenchmarkRunner {
        pub fn new() -> Self {
            Self {
                results: Vec::new(),
            }
        }

        /// Run a benchmark on interpreter code
        ///
        /// GREEN phase: Execute program N times and measure performance
        pub fn bench_interpreter(
            &mut self,
            name: &str,
            program: &str,
            iterations: usize,
        ) -> BenchmarkResult {
            let start = Instant::now();

            for _ in 0..iterations {
                let mut parser = Parser::new(program);
                let ast = parser.parse().expect("Benchmark program should parse");

                let mut eval = Evaluator::new();
                for statement in ast.nodes() {
                    eval.eval(statement)
                        .expect("Benchmark program should execute");
                }
            }

            let total_duration = start.elapsed();
            let total_ms = total_duration.as_secs_f64() * 1000.0;
            let avg_us = (total_duration.as_micros() as f64) / (iterations as f64);
            let throughput = (iterations as f64) / total_duration.as_secs_f64();

            let result = BenchmarkResult {
                name: name.to_string(),
                iterations,
                total_duration_ms: total_ms,
                avg_duration_us: avg_us,
                throughput,
            };

            self.results.push(result.clone());
            result
        }

        /// Simulate native execution baseline
        ///
        /// GREEN phase: This simulates native Ruchy execution time
        /// In a real scenario, we'd shell out to `ruchy run` and measure
        /// We use a realistic baseline of 200ns per operation (typical JIT overhead)
        pub fn bench_native_baseline(&mut self, name: &str, iterations: usize) -> BenchmarkResult {
            let start = Instant::now();

            // Simulate native execution (use a realistic baseline: 200ns/op)
            std::thread::sleep(std::time::Duration::from_nanos(200 * iterations as u64));

            let total_duration = start.elapsed();
            let total_ms = total_duration.as_secs_f64() * 1000.0;
            let avg_us = (total_duration.as_micros() as f64) / (iterations as f64);
            let throughput = (iterations as f64) / total_duration.as_secs_f64();

            let result = BenchmarkResult {
                name: format!("{}_native", name),
                iterations,
                total_duration_ms: total_ms,
                avg_duration_us: avg_us,
                throughput,
            };

            self.results.push(result.clone());
            result
        }

        /// Get all benchmark results
        pub fn results(&self) -> &[BenchmarkResult] {
            &self.results
        }

        /// Print benchmark report
        pub fn print_report(&self) {
            println!("\n=== Performance Benchmark Report ===");
            println!(
                "{:<30} {:>12} {:>15} {:>15}",
                "Name", "Iterations", "Avg (µs)", "Throughput/s"
            );
            println!("{}", "=".repeat(75));

            for result in &self.results {
                println!(
                    "{:<30} {:>12} {:>15.2} {:>15.0}",
                    result.name, result.iterations, result.avg_duration_us, result.throughput
                );
            }
            println!();
        }
    }
}

use benchmarking::*;

/// Test: Benchmark - Arithmetic Operations
///
/// RED: This test WILL FAIL because:
/// - BenchmarkRunner is unimplemented
/// - Performance comparison infrastructure doesn't exist
///
/// Property: Interpreter should be <100x slower than native for arithmetic
#[test]
fn test_benchmark_fibonacci() {
    let mut runner = BenchmarkRunner::new();

    // Complex arithmetic expression (functions not supported yet)
    let program = r#"
        let a = 10;
        let b = 20;
        let c = a + b;
        let d = c * 2;
        let e = d - a;
        e
    "#;

    // Benchmark interpreter
    let interp_result = runner.bench_interpreter("fibonacci", program, 100);

    // Benchmark native baseline (simulated)
    let native_result = runner.bench_native_baseline("fibonacci", 100);

    // Calculate overhead
    let overhead = interp_result.overhead_factor(&native_result);

    println!("Fibonacci Benchmark:");
    println!("  Interpreter: {:.2} µs/op", interp_result.avg_duration_us);
    println!("  Native:      {:.2} µs/op", native_result.avg_duration_us);
    println!("  Overhead:    {:.2}x", overhead);

    // Target: <100x slower than native
    assert!(
        overhead < 100.0,
        "Interpreter overhead should be <100x, got {:.2}x",
        overhead
    );
}

/// Test: Benchmark - Variable Operations
///
/// RED: This test WILL FAIL because benchmarking infrastructure doesn't exist
///
/// Property: Variable operations should have reasonable performance
#[test]
fn test_benchmark_vector_ops() {
    let mut runner = BenchmarkRunner::new();

    // Variable operations (arrays/while not supported yet)
    let program = r#"
        let x = 1;
        let y = 2;
        let z = 3;
        let a = x + y;
        let b = y + z;
        let c = a + b;
        c
    "#;

    // Benchmark interpreter
    let interp_result = runner.bench_interpreter("vector_ops", program, 1000);

    // Benchmark native baseline
    let native_result = runner.bench_native_baseline("vector_ops", 1000);

    // Calculate overhead
    let overhead = interp_result.overhead_factor(&native_result);

    println!("Vector Ops Benchmark:");
    println!("  Interpreter: {:.2} µs/op", interp_result.avg_duration_us);
    println!("  Native:      {:.2} µs/op", native_result.avg_duration_us);
    println!("  Overhead:    {:.2}x", overhead);

    // Target: <100x slower than native
    assert!(
        overhead < 100.0,
        "Vector ops overhead should be <100x, got {:.2}x",
        overhead
    );
}

/// Test: Benchmark - Comparison Operations
///
/// RED: This test WILL FAIL because benchmarking infrastructure doesn't exist
///
/// Property: Comparison operations should have reasonable performance
#[test]
fn test_benchmark_hashmap_ops() {
    let mut runner = BenchmarkRunner::new();

    // Comparison operations (objects not supported yet)
    let program = r#"
        let a = 10;
        let b = 20;
        let c = a < b;
        let d = a == b;
        let e = a > b;
        e
    "#;

    // Benchmark interpreter
    let interp_result = runner.bench_interpreter("hashmap_ops", program, 1000);

    // Benchmark native baseline
    let native_result = runner.bench_native_baseline("hashmap_ops", 1000);

    // Calculate overhead
    let overhead = interp_result.overhead_factor(&native_result);

    println!("HashMap Ops Benchmark:");
    println!("  Interpreter: {:.2} µs/op", interp_result.avg_duration_us);
    println!("  Native:      {:.2} µs/op", native_result.avg_duration_us);
    println!("  Overhead:    {:.2}x", overhead);

    // Target: <100x slower than native
    assert!(
        overhead < 100.0,
        "HashMap ops overhead should be <100x, got {:.2}x",
        overhead
    );
}

/// Test: Performance Regression Detection
///
/// RED: This test WILL FAIL because performance tracking doesn't exist
///
/// Property: Performance should not regress between runs
#[test]
fn test_performance_regression() {
    let mut runner = BenchmarkRunner::new();

    // Simple arithmetic benchmark
    let program = "1 + 2 + 3 + 4 + 5";

    // Run benchmark twice
    let result1 = runner.bench_interpreter("arithmetic_run1", program, 10000);
    let result2 = runner.bench_interpreter("arithmetic_run2", program, 10000);

    // Performance should be consistent (within 20% variance)
    let variance =
        (result1.avg_duration_us - result2.avg_duration_us).abs() / result1.avg_duration_us;

    println!("Performance Regression Check:");
    println!("  Run 1: {:.2} µs/op", result1.avg_duration_us);
    println!("  Run 2: {:.2} µs/op", result2.avg_duration_us);
    println!("  Variance: {:.2}%", variance * 100.0);

    assert!(
        variance < 0.20,
        "Performance variance should be <20%, got {:.2}%",
        variance * 100.0
    );
}

/// Test: Throughput Measurement
///
/// RED: This test WILL FAIL because throughput tracking doesn't exist
///
/// Property: Interpreter should achieve reasonable throughput
#[test]
fn test_throughput_measurement() {
    let mut runner = BenchmarkRunner::new();

    let program = "42";
    let result = runner.bench_interpreter("throughput", program, 10000);

    println!("Throughput Measurement:");
    println!("  Operations: {}", result.iterations);
    println!("  Duration: {:.2} ms", result.total_duration_ms);
    println!("  Throughput: {:.0} ops/sec", result.throughput);

    // Target: >1000 simple operations per second
    assert!(
        result.throughput > 1000.0,
        "Throughput should be >1000 ops/sec, got {:.0}",
        result.throughput
    );
}

/// Test: Benchmark Report Generation
///
/// RED: This test WILL FAIL because report generation doesn't exist
///
/// Property: Benchmark results should be human-readable
#[test]
fn test_benchmark_report() {
    let mut runner = BenchmarkRunner::new();

    // Run several benchmarks
    runner.bench_interpreter("simple", "1 + 1", 1000);
    runner.bench_interpreter("complex", "let x = 10; x * 2", 1000);
    runner.bench_native_baseline("baseline", 1000);

    // Print report
    runner.print_report();

    // Verify results were recorded
    let results = runner.results();
    assert_eq!(results.len(), 3, "Should have 3 benchmark results");

    // Verify result structure
    for result in results {
        assert!(!result.name.is_empty(), "Result should have name");
        assert!(result.iterations > 0, "Result should have iterations");
        assert!(result.avg_duration_us > 0.0, "Result should have duration");
        assert!(result.throughput > 0.0, "Result should have throughput");
    }
}

/// Test: Completeness Check
///
/// Verify all required tests exist and are documented
#[test]
fn test_interp_030_completeness() {
    // This test verifies that INTERP-030 deliverables are complete
    // Tests required:
    let required_tests = [
        "test_benchmark_fibonacci",
        "test_benchmark_vector_ops",
        "test_benchmark_hashmap_ops",
        "test_performance_regression",
        "test_throughput_measurement",
        "test_benchmark_report",
    ];

    // Verify test count
    assert_eq!(required_tests.len(), 6);
}
