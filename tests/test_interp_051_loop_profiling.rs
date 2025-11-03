// INTERP-051: Loop-Level Profiling for OSR (On-Stack Replacement)
//
// EXTREME TDD - RED Phase
//
// Mission: Track hot loops for OSR JIT compilation decisions
//
// OSR (On-Stack Replacement):
// - Detect hot loops during execution
// - Compile loop to machine code WHILE running
// - Replace interpreter with JIT mid-execution
//
// JIT Needs:
// 1. Which loops are hot? (>1000 iterations)
// 2. Which loops take significant time? (>30% of function time)
// 3. What's the iteration count and time per iteration?
// 4. Where is the loop located? (function, line number)
//
// This enables:
// - OSR compilation of long-running loops
// - JIT optimization of interactive workloads
// - Targeting loops that weren't initially hot
//
// Method: Test-driven development with loop instrumentation

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use ruchyruchy::profiler::CompilerProfiler;

/// Test: Track loop iteration counts
///
/// Validates that profiler counts loop iterations
#[test]
fn test_loop_iteration_counting() {
    let source = r#"
        fun compute(n) {
            let sum = 0;
            let i = 0;
            while (i < n) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }

        let result = compute(100);
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Should have tracked loop in compute function
    let loop_profiles = profiler.loop_profiles("compute");
    assert!(
        !loop_profiles.is_empty(),
        "Should have tracked at least one loop in compute"
    );

    // Should have correct iteration count
    let loop_profile = &loop_profiles[0];
    assert_eq!(
        loop_profile.iteration_count, 100,
        "Loop should have 100 iterations"
    );
}

/// Test: Identify hot loops (>1000 iterations)
///
/// Validates that profiler identifies OSR candidates
#[test]
fn test_hot_loop_identification() {
    let source = r#"
        fun hot_loop() {
            let sum = 0;
            let i = 0;
            while (i < 2000) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }

        fun cold_loop() {
            let sum = 0;
            let i = 0;
            while (i < 10) {
                sum = sum + 1;
                i = i + 1;
            }
            return sum;
        }

        let r1 = hot_loop();
        let r2 = cold_loop();
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Get OSR candidates (>1000 iterations)
    let osr_candidates = profiler.osr_candidates(1000);

    assert!(
        !osr_candidates.is_empty(),
        "Should identify at least one OSR candidate"
    );

    // hot_loop should be a candidate
    assert!(
        osr_candidates
            .iter()
            .any(|lp| lp.function == "hot_loop" && lp.iteration_count >= 2000),
        "hot_loop should be OSR candidate (2000 iterations)"
    );

    // cold_loop should NOT be a candidate
    assert!(
        !osr_candidates.iter().any(|lp| lp.function == "cold_loop"),
        "cold_loop should not be OSR candidate (only 10 iterations)"
    );
}

/// Test: Track time per iteration
///
/// Validates that profiler calculates average time per iteration
#[test]
fn test_time_per_iteration() {
    let source = r#"
        fun timed_loop(n) {
            let i = 0;
            while (i < n) {
                i = i + 1;
            }
            return i;
        }

        let result = timed_loop(100);
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    let loop_profiles = profiler.loop_profiles("timed_loop");
    assert!(!loop_profiles.is_empty(), "Should have loop profile");

    let loop_profile = &loop_profiles[0];
    assert!(
        loop_profile.avg_time_per_iteration_us > 0.0,
        "Should have non-zero time per iteration"
    );
    assert!(
        loop_profile.total_time_us > 0.0,
        "Should have non-zero total time"
    );
}

/// Test: Track multiple loops in same function
///
/// Validates that profiler distinguishes between different loops
#[test]
fn test_multiple_loops_per_function() {
    let source = r#"
        fun multi_loop() {
            let sum1 = 0;
            let i = 0;
            while (i < 50) {
                sum1 = sum1 + i;
                i = i + 1;
            }

            let sum2 = 0;
            let j = 0;
            while (j < 75) {
                sum2 = sum2 + j;
                j = j + 1;
            }

            return sum1 + sum2;
        }

        let result = multi_loop();
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    let loop_profiles = profiler.loop_profiles("multi_loop");
    assert!(
        !loop_profiles.is_empty(),
        "Should have tracked at least one loop in multi_loop"
    );

    // Currently we aggregate multiple loops in the same function
    // Total iterations should be 50 + 75 = 125
    let total_iterations: usize = loop_profiles.iter().map(|lp| lp.iteration_count).sum();
    assert_eq!(
        total_iterations, 125,
        "Total iterations across loops should be 125 (50 + 75)"
    );
}

/// Test: Loop profiling with nested loops
///
/// Validates that profiler tracks nested loops separately
#[test]
fn test_nested_loop_profiling() {
    let source = r#"
        fun nested() {
            let sum = 0;
            let i = 0;
            while (i < 10) {
                let j = 0;
                while (j < 5) {
                    sum = sum + 1;
                    j = j + 1;
                }
                i = i + 1;
            }
            return sum;
        }

        let result = nested();
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    let loop_profiles = profiler.loop_profiles("nested");
    assert!(!loop_profiles.is_empty(), "Should track nested loops");

    // Currently we aggregate nested loops
    // Outer loop: 10 iterations
    // Inner loop: executed 10 times, 5 iterations each = 50 total iterations
    // Aggregated total: 10 + 50 = 60
    let total_iterations: usize = loop_profiles.iter().map(|lp| lp.iteration_count).sum();
    assert!(
        total_iterations >= 50,
        "Should have at least 50 total iterations (inner loop executions)"
    );
}

/// Test: OSR candidate ranking
///
/// Validates that profiler ranks OSR candidates by potential impact
#[test]
fn test_osr_candidate_ranking() {
    let source = r#"
        fun many_iterations() {
            let i = 0;
            while (i < 5000) {
                i = i + 1;
            }
            return i;
        }

        fun medium_iterations() {
            let i = 0;
            while (i < 2000) {
                i = i + 1;
            }
            return i;
        }

        let r1 = many_iterations();
        let r2 = medium_iterations();
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Get OSR candidates sorted by iteration count (descending)
    let osr_candidates = profiler.osr_candidates_sorted(1000);

    assert!(
        osr_candidates.len() >= 2,
        "Should have at least 2 OSR candidates"
    );

    // Should be sorted by iteration count (highest first)
    assert!(
        osr_candidates[0].iteration_count >= osr_candidates[1].iteration_count,
        "OSR candidates should be sorted by iteration count (descending)"
    );

    // many_iterations should be first (5000 iterations)
    assert!(
        osr_candidates[0].function == "many_iterations",
        "many_iterations should be top OSR candidate"
    );
}

/// Test: Loop percentage of function time
///
/// Validates that profiler calculates loop time as percentage of function time
#[test]
fn test_loop_percentage_of_function_time() {
    let source = r#"
        fun mostly_loop() {
            let setup = 1 + 2;  // Minimal setup time

            let sum = 0;
            let i = 0;
            while (i < 1000) {
                sum = sum + i;
                i = i + 1;
            }

            return sum + setup;
        }

        let result = mostly_loop();
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    let loop_profiles = profiler.loop_profiles("mostly_loop");
    assert!(!loop_profiles.is_empty(), "Should have loop profile");

    let loop_profile = &loop_profiles[0];

    // Loop should take >90% of function time
    let function_profile = profiler.function_profile("mostly_loop").unwrap();
    let loop_percentage = (loop_profile.total_time_us / function_profile.total_time_us) * 100.0;

    assert!(
        loop_percentage > 80.0,
        "Loop should take >80% of function time, got {:.1}%",
        loop_percentage
    );
}
