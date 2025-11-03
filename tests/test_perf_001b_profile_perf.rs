// PERF-001B: ruchydbg profile --perf Command
//
// EXTREME TDD - RED Phase
//
// Mission: Add performance profiling to ruchydbg
//
// Command: ruchydbg profile --perf <file>
// Output:
// - Parse time (ms, %)
// - Eval time (ms, %)
// - Total time
// - Bottleneck identification (Amdahl's Law)
// - Optimization recommendations
//
// Method: Test-driven development

use std::process::Command;

/// Helper: Run ruchydbg command
fn run_ruchydbg(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("ruchydbg")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to run ruchydbg");

    // Combine stdout and stderr for testing
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!("{}{}", stdout, stderr)
}

/// Test: ruchydbg profile --perf hello.ruchy
///
/// Should show phase-by-phase breakdown
#[test]
fn test_profile_perf_hello_world() {
    // Create test file
    let test_file = "/tmp/test_profile_hello.ruchy";
    std::fs::write(test_file, r#"println("Hello World");"#).unwrap();

    let output = run_ruchydbg(&["profile", "--perf", test_file]);

    // Should show phase breakdown
    assert!(output.contains("Parse"), "Should show parse phase");
    assert!(output.contains("Eval"), "Should show eval phase");
    assert!(output.contains("Total"), "Should show total time");

    // Should show percentages
    assert!(output.contains("%"), "Should show percentages");

    // Clean up
    let _ = std::fs::remove_file(test_file);
}

/// Test: ruchydbg profile --perf fibonacci.ruchy
///
/// Should identify bottleneck
#[test]
fn test_profile_perf_fibonacci() {
    // Create test file with compute-heavy code
    let test_file = "/tmp/test_profile_fib.ruchy";
    std::fs::write(
        test_file,
        r#"
fun fib(n) {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
let result = fib(10);
println(result);
    "#,
    )
    .unwrap();

    let output = run_ruchydbg(&["profile", "--perf", test_file]);

    // Should identify eval as bottleneck
    assert!(
        output.contains("BOTTLENECK") || output.contains("dominant"),
        "Should identify bottleneck"
    );

    // Should show Amdahl's Law analysis
    assert!(
        output.contains("Amdahl") || output.contains("speedup") || output.contains("optimize"),
        "Should show optimization potential"
    );

    // Clean up
    let _ = std::fs::remove_file(test_file);
}

/// Test: ruchydbg profile --perf (missing file)
///
/// Should show error
#[test]
fn test_profile_perf_missing_file() {
    let output = run_ruchydbg(&["profile", "--perf"]);

    assert!(
        output.contains("Error") || output.contains("Missing") || output.contains("Usage"),
        "Should show error for missing file"
    );
}

/// Test: ruchydbg profile --perf with iterations
///
/// Should support statistical rigor
#[test]
fn test_profile_perf_iterations() {
    let test_file = "/tmp/test_profile_iter.ruchy";
    std::fs::write(test_file, "let x = 1 + 2;").unwrap();

    let output = run_ruchydbg(&["profile", "--perf", test_file, "--iterations", "1000"]);

    // Should run multiple iterations
    assert!(
        output.contains("iteration") || output.contains("1000") || output.contains("mean"),
        "Should show statistical analysis"
    );

    // Clean up
    let _ = std::fs::remove_file(test_file);
}

/// Test: ruchydbg profile --perf output format
///
/// Should be machine-readable and human-readable
#[test]
fn test_profile_perf_output_format() {
    let test_file = "/tmp/test_profile_format.ruchy";
    std::fs::write(test_file, "let x = 42;").unwrap();

    let output = run_ruchydbg(&["profile", "--perf", test_file]);

    // Should have clear sections
    assert!(
        output.contains("=") || output.contains("-"),
        "Should have section separators"
    );

    // Should show times in consistent units
    assert!(
        output.contains("ms") || output.contains("µs") || output.contains("us"),
        "Should show time units"
    );

    // Clean up
    let _ = std::fs::remove_file(test_file);
}
