// PERF-001A: Runtime Baseline Analysis
//
// EXTREME TDD Status:
// - RED Phase: ✅ Complete (Test written, no baseline exists yet)
// - GREEN Phase: 🔄 In Progress (Establishing baseline measurements)
// - REFACTOR Phase: ⏳ Pending (After baseline established)
// - TOOL Phase: ⏳ Pending (fmt, clippy, tests)
// - PMAT Phase: ⏳ Pending (Performance evaluation)
//
// Mission: Break down "Ruchy Compiled" 2.64ms runtime into measurable components
// to identify optimization targets using Amdahl's Law.
//
// Benchmarks (compute-bound):
// 1. Hello World (minimal, startup-dominated)
// 2. Fibonacci(35) recursive (CPU-bound, function call overhead)
// 3. Factorial(20) iterative (loop-bound)
// 4. Array Sum (1M elements) (memory-bound)
// 5. File I/O (I/O-bound)
//
// Measurements Required (per PERFORMANCE_ROADMAP_REFINED.md):
// - Process Startup: fork/exec, ELF loading, dynamic linking
// - Runtime Init: static constructors, stdlib initialization
// - Computation: actual user code execution
// - Shutdown: destructors, exit handlers
//
// Tools: perf stat, perf record, flamegraph, custom instrumentation
//
// Success Criteria (Amdahl's Law):
// - Identify phase >50% of runtime = dominant bottleneck
// - Generate flamegraphs for all 5 benchmarks
// - Document in PERF_001A_RUNTIME_ANALYSIS.md
// - Clear statement: "To beat Julia (2.03ms), optimize [X] by [Y]%"

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

/// PERF-001A Test Suite: Runtime Baseline Profiling
///
/// This test establishes ground truth for runtime performance optimization.
/// BLOCKING: All optimization work depends on this baseline.

#[test]
fn test_perf_001a_hello_world_baseline() {
    let test_name = "hello_world";
    let ruchy_code = r#"fun main() {
    println("Hello, World!");
}
"#;

    let result = run_baseline_benchmark(test_name, ruchy_code, 100);

    println!("\n📊 PERF-001A: Hello World Baseline");
    println!("Total iterations: {}", result.iterations);
    println!("Average runtime: {:.3}ms", result.avg_runtime_ms);
    println!("Min runtime: {:.3}ms", result.min_runtime_ms);
    println!("Max runtime: {:.3}ms", result.max_runtime_ms);
    println!("Std deviation: {:.3}ms", result.std_dev_ms);

    // Hello World should be startup-dominated
    println!("\n🎯 Expected: Startup-dominated (<1ms computation)");

    assert!(result.avg_runtime_ms > 0.0, "Benchmark should complete");
}

#[test]
#[ignore] // BLOCKED by Ruchy Compiler Bug #128: if-else in functions fails to compile
fn test_perf_001a_fibonacci_baseline() {
    let test_name = "fibonacci";
    let ruchy_code = r#"fun fib(n) {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fun main() {
    let result = fib(35)
    println(result)
}
"#;

    let result = run_baseline_benchmark(test_name, ruchy_code, 10);

    println!("\n📊 PERF-001A: Fibonacci(35) Baseline");
    println!("Total iterations: {}", result.iterations);
    println!("Average runtime: {:.3}ms", result.avg_runtime_ms);
    println!("Min runtime: {:.3}ms", result.min_runtime_ms);
    println!("Max runtime: {:.3}ms", result.max_runtime_ms);
    println!("Std deviation: {:.3}ms", result.std_dev_ms);

    // Fibonacci should be computation-dominated
    println!("\n🎯 Expected: Computation-dominated (function call overhead)");
    println!("🎯 Target: <2.10ms (approach Julia's 2.03ms)");

    // This is our primary performance target
    if result.avg_runtime_ms < 2.10 {
        println!("✅ BEATING Julia JIT target!");
    } else if result.avg_runtime_ms < 3.02 {
        println!("✅ Beating C ({:.2}ms)", 3.02);
    } else {
        println!("⚠️ Slower than C (3.02ms)");
    }

    assert!(result.avg_runtime_ms > 0.0, "Benchmark should complete");
}

#[test]
#[ignore] // BLOCKED by Ruchy Compiler Bug #128: while loops in functions fail to compile
fn test_perf_001a_factorial_baseline() {
    let test_name = "factorial";
    let ruchy_code = r#"fun factorial(n) {
    let result = 1
    let i = 2
    while i <= n {
        result = result * i
        i = i + 1
    }
    result
}

fun main() {
    let result = factorial(20)
    println(result)
}
"#;

    let result = run_baseline_benchmark(test_name, ruchy_code, 50);

    println!("\n📊 PERF-001A: Factorial(20) Baseline");
    println!("Total iterations: {}", result.iterations);
    println!("Average runtime: {:.3}ms", result.avg_runtime_ms);
    println!("Min runtime: {:.3}ms", result.min_runtime_ms);
    println!("Max runtime: {:.3}ms", result.max_runtime_ms);
    println!("Std deviation: {:.3}ms", result.std_dev_ms);

    println!("\n🎯 Expected: Loop-bound (tight iteration)");

    assert!(result.avg_runtime_ms > 0.0, "Benchmark should complete");
}

#[test]
#[ignore] // BLOCKED by Ruchy Compiler Bug #128: while loops in functions fail to compile
fn test_perf_001a_array_sum_baseline() {
    let test_name = "array_sum";
    let ruchy_code = r#"fun main() {
    let sum = 0
    let n = 1000000
    let i = 0
    while i < n {
        sum = sum + i
        i = i + 1
    }
    println(sum)
}
"#;

    let result = run_baseline_benchmark(test_name, ruchy_code, 10);

    println!("\n📊 PERF-001A: Array Sum (1M elements) Baseline");
    println!("Total iterations: {}", result.iterations);
    println!("Average runtime: {:.3}ms", result.avg_runtime_ms);
    println!("Min runtime: {:.3}ms", result.min_runtime_ms);
    println!("Max runtime: {:.3}ms", result.max_runtime_ms);
    println!("Std deviation: {:.3}ms", result.std_dev_ms);

    println!("\n🎯 Expected: Memory-bound (cache/register usage)");

    assert!(result.avg_runtime_ms > 0.0, "Benchmark should complete");
}

#[test]
#[ignore] // Requires file I/O implementation in Ruchy (not yet available)
fn test_perf_001a_file_io_baseline() {
    let test_name = "file_io";
    let ruchy_code = r#"fun main() {
    let i = 0
    while i < 1000 {
        // File I/O would go here
        i = i + 1
    }
    println("File I/O complete")
}
"#;

    let result = run_baseline_benchmark(test_name, ruchy_code, 10);

    println!("\n📊 PERF-001A: File I/O Baseline");
    println!("Total iterations: {}", result.iterations);
    println!("Average runtime: {:.3}ms", result.avg_runtime_ms);

    println!("\n🎯 Expected: I/O-bound (syscall overhead)");

    assert!(result.avg_runtime_ms > 0.0, "Benchmark should complete");
}

#[test]
fn test_perf_001a_generate_summary_report() {
    println!("\n============================================================");
    println!("PERF-001A: Runtime Baseline Summary");
    println!("============================================================");

    println!("\n🎯 Objective: Break down runtime to identify optimization target");
    println!("🎯 Target: <2.10ms (approach Julia's 2.03ms)");
    println!("🎯 Comparison: Julia JIT (2.03ms), C (3.02ms), Rust (3.04ms)");

    println!("\n📊 Benchmarks:");
    println!("1. Hello World     - Startup-dominated baseline");
    println!("2. Fibonacci(35)   - CPU-bound, function call overhead");
    println!("3. Factorial(20)   - Loop-bound, tight iteration");
    println!("4. Array Sum (1M)  - Memory-bound, cache usage");
    println!("5. File I/O        - I/O-bound, syscall overhead [IGNORED]");

    println!("\n📝 Next Steps (MANDATORY - BLOCKING):");
    println!("1. Run: cargo test --test test_perf_001a_runtime_baseline --release -- --nocapture");
    println!("2. Profile with perf: perf stat -d cargo test --release");
    println!("3. Generate flamegraph: cargo flamegraph --test test_perf_001a_runtime_baseline");
    println!("4. Analyze breakdown: Process Startup vs Runtime Init vs Computation");
    println!("5. Apply Amdahl's Law: Identify phase >50% of runtime");
    println!("6. Document in: docs/PERF_001A_RUNTIME_ANALYSIS.md");
    println!("7. Clear statement: 'To beat Julia, optimize [X] by [Y]%'");

    println!("\n⚠️ CRITICAL: No optimization work proceeds until PERF-001A complete!");
    println!("============================================================");
}

// Helper Structures

struct BenchmarkResult {
    iterations: usize,
    avg_runtime_ms: f64,
    min_runtime_ms: f64,
    max_runtime_ms: f64,
    std_dev_ms: f64,
}

// Helper Functions

fn run_baseline_benchmark(name: &str, ruchy_code: &str, iterations: usize) -> BenchmarkResult {
    // Setup: Create temporary directory for benchmark
    let temp_dir = setup_benchmark_dir(name);

    // Step 1: Write Ruchy source file
    let ruchy_file = temp_dir.join(format!("{}.ruchy", name));
    fs::write(&ruchy_file, ruchy_code).expect("Failed to write Ruchy source file");

    // Step 2: Compile with ruchy (if available)
    // For now, we'll compile as Rust directly since we're in the ruchyruchy repo
    // In production, this would call: ruchy build {name}.ruchy
    let binary_path = compile_ruchy_program(&ruchy_file, &temp_dir);

    if !binary_path.exists() {
        eprintln!(
            "⚠️ Benchmark '{}' requires ruchy compiler - skipping for now",
            name
        );
        return BenchmarkResult {
            iterations: 0,
            avg_runtime_ms: 0.0,
            min_runtime_ms: 0.0,
            max_runtime_ms: 0.0,
            std_dev_ms: 0.0,
        };
    }

    // Step 3: Run benchmark multiple times
    let mut runtimes = Vec::new();

    for _ in 0..iterations {
        let start = Instant::now();

        let output = Command::new(&binary_path)
            .output()
            .expect("Failed to run benchmark binary");

        let duration = start.elapsed();

        if !output.status.success() {
            eprintln!("⚠️ Benchmark failed: {:?}", output.stderr);
            continue;
        }

        runtimes.push(duration.as_secs_f64() * 1000.0); // Convert to milliseconds
    }

    // Step 4: Calculate statistics
    calculate_statistics(runtimes, iterations)
}

fn setup_benchmark_dir(name: &str) -> PathBuf {
    let temp_dir = std::env::temp_dir().join(format!("perf_001a_{}", name));

    // Clean up old runs
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }

    fs::create_dir_all(&temp_dir).expect("Failed to create benchmark directory");

    temp_dir
}

fn compile_ruchy_program(ruchy_file: &Path, output_dir: &Path) -> PathBuf {
    let binary_name = ruchy_file.file_stem().unwrap().to_str().unwrap();
    let binary_path = output_dir.join(binary_name);

    // Check if ruchy compiler is available
    let ruchy_available = Command::new("ruchy")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !ruchy_available {
        eprintln!("⚠️ ruchy compiler not found - PERF-001A requires production ruchy compiler");
        eprintln!("   Install from: https://github.com/paiml/ruchy");
        return binary_path; // Return non-existent path
    }

    // Compile with ruchy compile (standalone binary, not project build)
    let output = Command::new("ruchy")
        .arg("compile")
        .arg(ruchy_file)
        .arg("-o")
        .arg(&binary_path)
        .arg("-O")
        .arg("3") // Maximum optimization level
        .output()
        .expect("Failed to execute ruchy compile");

    if !output.status.success() {
        eprintln!(
            "⚠️ Compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    binary_path
}

fn calculate_statistics(runtimes: Vec<f64>, _expected_iterations: usize) -> BenchmarkResult {
    if runtimes.is_empty() {
        return BenchmarkResult {
            iterations: 0,
            avg_runtime_ms: 0.0,
            min_runtime_ms: 0.0,
            max_runtime_ms: 0.0,
            std_dev_ms: 0.0,
        };
    }

    let iterations = runtimes.len();
    let avg_runtime_ms = runtimes.iter().sum::<f64>() / iterations as f64;
    let min_runtime_ms = runtimes.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_runtime_ms = runtimes.iter().cloned().fold(0.0, f64::max);

    // Calculate standard deviation
    let variance = runtimes
        .iter()
        .map(|&x| (x - avg_runtime_ms).powi(2))
        .sum::<f64>()
        / iterations as f64;
    let std_dev_ms = variance.sqrt();

    BenchmarkResult {
        iterations,
        avg_runtime_ms,
        min_runtime_ms,
        max_runtime_ms,
        std_dev_ms,
    }
}

// Instrumentation helpers for phase breakdown
// (Would be integrated into ruchy runtime for production measurements)

#[allow(dead_code)]
fn measure_process_startup() -> f64 {
    // Measure time from process start to runtime init
    // This would require instrumentation in the ruchy runtime
    // For now, placeholder
    0.0
}

#[allow(dead_code)]
fn measure_runtime_init() -> f64 {
    // Measure time for static constructors, stdlib initialization
    // This would require instrumentation in the ruchy runtime
    0.0
}

#[allow(dead_code)]
fn measure_computation() -> f64 {
    // Measure actual user code execution time
    // This is what we're measuring with the outer timer
    0.0
}

#[allow(dead_code)]
fn measure_shutdown() -> f64 {
    // Measure destructors, exit handlers
    0.0
}
