// DEBUGGER-053: Differential Testing Framework (Interpreter vs JIT)
//
// EXTREME TDD - RED Phase
//
// Jidoka Stop-the-Line Policy (CRITICAL):
// Any mismatch between interpreter and JIT represents a fundamental break in compiler
// correctness and MUST be treated as a line-stopping failure.
//
// Policy:
// 1. Blocking Failures: Interpreter/JIT mismatch = CRITICAL failure blocks ALL merges
// 2. Zero Tolerance: Known discrepancies NEVER passed forward
// 3. Immediate Resolution: Failures trigger immediate investigation
// 4. CI/CD Integration: Pre-commit hooks FAIL HARD on mismatch
//
// Rationale (Hoare Logic):
// Interpreter = formal "specification" of correct behavior
// JIT = "implementation" that must provably produce equivalent results
// Any deviation = proof failure that invalidates correctness guarantees
//
// Tests follow Toyota Way: Each test enforces Jidoka correctness policy

/// Test 1: Simple arithmetic - interpreter and JIT must match
///
/// Jidoka: Any mismatch in basic arithmetic is a line-stopping failure
#[test]
fn test_differential_simple_arithmetic() {
    let source = "fun main() { return 10 + 5; }";

    // Run through interpreter
    let interp_result = ruchyruchy::debugger::differential::run_interpreter(source, "main", &[]);

    // Run through JIT
    let jit_result = ruchyruchy::debugger::differential::run_jit(source, "main", &[]);

    // CRITICAL: Results must match exactly (Jidoka policy)
    assert!(
        interp_result.is_ok(),
        "Interpreter must execute successfully"
    );
    assert!(jit_result.is_ok(), "JIT must execute successfully");

    let interp_val = interp_result.unwrap();
    let jit_val = jit_result.unwrap();

    assert_eq!(
        interp_val, jit_val,
        "JIDOKA VIOLATION: Interpreter returned {}, JIT returned {}. This is a line-stopping failure!",
        interp_val, jit_val
    );

    assert_eq!(interp_val, 15, "Expected result is 15");
}

/// Test 2: Detect JIT bugs - must catch when results differ
///
/// Jidoka: Framework must detect mismatches (defensive quality)
#[test]
fn test_differential_catches_jit_bug() {
    let source = "fun main() { return 42; }";

    // Simulate a comparison where results differ
    let result = ruchyruchy::debugger::differential::compare_results(
        source,
        "main",
        &[],
        Some(42), // Interpreter result
        Some(99), // JIT result (simulated bug)
    );

    // Must detect the mismatch
    assert!(
        result.is_err(),
        "Must detect interpreter/JIT mismatch (Jidoka)"
    );

    let error = result.unwrap_err();
    assert!(
        error.contains("mismatch") || error.contains("differ"),
        "Error should explain mismatch: {}",
        error
    );
}

/// Test 3: Functions with parameters - compare with arguments
///
/// Jidoka: Parameterized functions must match across all argument values
#[test]
fn test_differential_with_params() {
    let source = "fun add(a: i64, b: i64) { return a + b; }";

    // Test with various argument combinations
    let test_cases = vec![(1, 2), (10, 20), (100, 200), (-5, 10)];

    for (a, b) in test_cases {
        let args = vec![a, b];

        let interp_result =
            ruchyruchy::debugger::differential::run_interpreter(source, "add", &args);
        let jit_result = ruchyruchy::debugger::differential::run_jit(source, "add", &args);

        assert!(
            interp_result.is_ok(),
            "Interpreter must execute with args {:?}",
            args
        );
        assert!(jit_result.is_ok(), "JIT must execute with args {:?}", args);

        let interp_val = interp_result.unwrap();
        let jit_val = jit_result.unwrap();

        assert_eq!(
            interp_val, jit_val,
            "JIDOKA VIOLATION with args {:?}: Interpreter={}, JIT={}",
            args, interp_val, jit_val
        );

        assert_eq!(
            interp_val,
            a + b,
            "Expected result is {} + {} = {}",
            a,
            b,
            a + b
        );
    }
}

/// Test 4: Fuzzing - random inputs must all match
///
/// Jidoka: Comprehensive testing to find discrepancies
#[test]
fn test_differential_fuzzing() {
    let source = "fun multiply(x: i64, y: i64) { return x * y; }";

    // Fuzz with 100 random input combinations
    let fuzz_results = ruchyruchy::debugger::differential::fuzz_test(
        source, "multiply", 100, 2, /* num_args */
    );

    assert!(fuzz_results.is_ok(), "Fuzzing must complete without errors");

    let stats = fuzz_results.unwrap();

    // All iterations must match (Jidoka zero-tolerance)
    assert_eq!(
        stats.mismatches, 0,
        "JIDOKA VIOLATION: Found {} mismatches in fuzzing (zero tolerance!)",
        stats.mismatches
    );

    assert_eq!(
        stats.total_iterations, 100,
        "Must run all 100 fuzz iterations"
    );

    assert_eq!(stats.matches, 100, "All 100 iterations must match (Jidoka)");
}

/// Test 5: Performance comparison - timing both paths
///
/// Jidoka: While JIT should be faster, correctness comes first
#[test]
fn test_differential_performance_comparison() {
    let source = "fun loop_sum(n: i64) { let sum = 0; let i = 0; while i < n { sum = sum + i; i = i + 1; } return sum; }";

    let perf = ruchyruchy::debugger::differential::compare_performance(source, "loop_sum", &[100]);

    assert!(perf.is_ok(), "Performance comparison must succeed");

    let stats = perf.unwrap();

    // Both must return same result (Jidoka)
    assert_eq!(
        stats.interp_result, stats.jit_result,
        "JIDOKA VIOLATION: Results differ in performance test"
    );

    // Both must have measurable time
    assert!(
        stats.interp_time_ms > 0.0,
        "Interpreter time must be positive"
    );
    assert!(stats.jit_time_ms > 0.0, "JIT time must be positive");

    // JIT should typically be faster (informational, not Jidoka-critical)
    println!(
        "Performance: Interpreter={:.3}ms, JIT={:.3}ms, Speedup={:.2}x",
        stats.interp_time_ms,
        stats.jit_time_ms,
        stats.interp_time_ms / stats.jit_time_ms
    );
}

/// Test 6: Coverage - ensure all AST node types are tested
///
/// Jidoka: Comprehensive coverage to prevent gaps in correctness validation
#[test]
fn test_differential_coverage() {
    // Test various AST node types
    let test_programs = vec![
        ("fun int_literal() { return 42; }", "int_literal"),
        ("fun arithmetic() { return 10 + 5 * 2; }", "arithmetic"),
        ("fun comparison() { return 10 > 5; }", "comparison"),
        (
            "fun conditional() { if 1 > 0 { return 1; } else { return 0; } }",
            "conditional",
        ),
        (
            "fun loop_test() { let x = 0; while x < 5 { x = x + 1; } return x; }",
            "loop_test",
        ),
    ];

    let coverage = ruchyruchy::debugger::differential::check_coverage(&test_programs);

    assert!(coverage.is_ok(), "Coverage check must succeed");

    let stats = coverage.unwrap();

    // All test programs must pass differential testing (Jidoka)
    assert_eq!(
        stats.mismatches, 0,
        "JIDOKA VIOLATION: Found {} mismatches in coverage testing",
        stats.mismatches
    );

    assert!(
        stats.ast_nodes_covered >= 5,
        "Must cover at least 5 AST node types (found {})",
        stats.ast_nodes_covered
    );

    println!(
        "Coverage: {}/{} programs passed, {} AST nodes covered",
        stats.passed, stats.total, stats.ast_nodes_covered
    );
}

// Data structures for test assertions
// These will be implemented in src/debugger/differential.rs

#[allow(dead_code)]
struct FuzzStats {
    total_iterations: usize,
    matches: usize,
    mismatches: usize,
}

#[allow(dead_code)]
struct PerformanceStats {
    interp_result: i64,
    jit_result: i64,
    interp_time_ms: f64,
    jit_time_ms: f64,
}

#[allow(dead_code)]
struct CoverageStats {
    total: usize,
    passed: usize,
    mismatches: usize,
    ast_nodes_covered: usize,
}
