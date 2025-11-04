// INTERP-049: Enhanced Function-Level Profiling for JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Enhance profiler to show function-level hotspots for JIT decisions
//
// JIT Needs:
// 1. Which functions are hot? (>30% of time OR >1000 calls)
// 2. What's the call count for each function?
// 3. What's the time per function?
// 4. What percentage of total execution time?
//
// This data feeds JIT compilation decisions:
// - Hot functions → JIT compile
// - Type-stable functions → excellent JIT candidates
// - Cold functions → keep in interpreter
//
// Method: Test-driven development with CompilerProfiler integration

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use ruchyruchy::profiler::CompilerProfiler;

/// Test: Track function call counts
///
/// Validates that profiler counts each function invocation
#[test]
fn test_function_call_counts() {
    let source = r#"
        fun fibonacci(n) {
            if (n <= 1) {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        let result = fibonacci(5);
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Should have tracked fibonacci calls
    let function_profile = profiler.function_profile("fibonacci");
    assert!(
        function_profile.is_some(),
        "Should have profile for fibonacci"
    );

    let profile = function_profile.unwrap();
    assert!(
        profile.call_count > 0,
        "Should have call count for fibonacci"
    );

    // Fibonacci(5) makes 15 calls total (1+2+3+5+4 = 15)
    assert_eq!(
        profile.call_count, 15,
        "Fibonacci(5) should make 15 recursive calls"
    );
}

/// Test: Track function execution time
///
/// Validates that profiler measures time per function
#[test]
fn test_function_execution_time() {
    let source = r#"
        fun slow_function() {
            let sum = 0;
            let i = 0;
            while (i < 100) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
        let result = slow_function();
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    let function_profile = profiler.function_profile("slow_function");
    assert!(function_profile.is_some(), "Should have profile");

    let profile = function_profile.unwrap();
    assert!(
        profile.total_time_us > 0.0,
        "Should have non-zero execution time"
    );
    assert_eq!(profile.call_count, 1, "Should be called once");
}

/// Test: Identify hot functions (>30% threshold)
///
/// Validates that profiler can identify JIT candidates
#[test]
fn test_hot_function_identification() {
    let source = r#"
        fun hot_function() {
            let sum = 0;
            let i = 0;
            while (i < 1000) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }

        fun cold_function() {
            return 42;
        }

        let result1 = hot_function();
        let result2 = cold_function();
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Get hot functions (>30% of total time)
    let hot_functions = profiler.hot_functions(0.30);

    assert!(
        !hot_functions.is_empty(),
        "Should identify at least one hot function"
    );

    // hot_function should be in the list
    assert!(
        hot_functions.iter().any(|(name, _)| name == "hot_function"),
        "hot_function should be identified as hot"
    );

    // cold_function should NOT be in the list
    assert!(
        !hot_functions
            .iter()
            .any(|(name, _)| name == "cold_function"),
        "cold_function should not be identified as hot"
    );
}

/// Test: Function profiling with multiple calls
///
/// Validates that profiler aggregates data across multiple calls
#[test]
fn test_function_multiple_calls() {
    let source = r#"
        fun add(a, b) {
            return a + b;
        }

        let r1 = add(1, 2);
        let r2 = add(3, 4);
        let r3 = add(5, 6);
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    let function_profile = profiler.function_profile("add");
    assert!(function_profile.is_some(), "Should have profile for add");

    let profile = function_profile.unwrap();
    assert_eq!(profile.call_count, 3, "add should be called 3 times");
    assert!(
        profile.total_time_us > 0.0,
        "Should have non-zero total time"
    );
}

/// Test: Percentage calculation
///
/// Validates that profiler calculates percentage of total execution time
#[test]
fn test_function_percentage_calculation() {
    let source = r#"
        fun fast() { return 1; }
        fun slow() {
            let i = 0;
            while (i < 100) { i = i + 1; }
            return i;
        }

        let r1 = fast();
        let r2 = slow();
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    let fast_profile = profiler.function_profile("fast").unwrap();
    let slow_profile = profiler.function_profile("slow").unwrap();

    // slow should take more time than fast
    assert!(
        slow_profile.total_time_us > fast_profile.total_time_us,
        "slow should take more time than fast"
    );

    // Get percentages
    let fast_pct = profiler.function_percentage("fast");
    let slow_pct = profiler.function_percentage("slow");

    assert!(fast_pct > 0.0, "fast should have non-zero percentage");
    assert!(slow_pct > 0.0, "slow should have non-zero percentage");

    // slow should have higher percentage
    assert!(
        slow_pct > fast_pct,
        "slow should have higher percentage than fast"
    );

    // Percentages should sum to ~100% (allowing for rounding errors and timing variance)
    assert!(
        fast_pct + slow_pct <= 101.0, // Allow 1% tolerance for timing variance/rounding
        "Percentages should not significantly exceed 100%: got {}%",
        fast_pct + slow_pct
    );
}

/// Test: Get all function profiles sorted by time
///
/// Validates that we can get ranked list of hot functions
#[test]
fn test_get_all_function_profiles_sorted() {
    let source = r#"
        fun fast() { return 1; }
        fun medium() { let i = 0; while (i < 10) { i = i + 1; } return i; }
        fun slow() { let i = 0; while (i < 100) { i = i + 1; } return i; }

        let r1 = fast();
        let r2 = medium();
        let r3 = slow();
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Get all profiles sorted by total time (descending)
    let profiles = profiler.all_function_profiles_sorted();

    assert_eq!(profiles.len(), 3, "Should have 3 function profiles");

    // Should be sorted by time (descending)
    assert_eq!(profiles[0].0, "slow", "slow should be first (hottest)");
    assert_eq!(profiles[1].0, "medium", "medium should be second");
    assert_eq!(profiles[2].0, "fast", "fast should be last (coldest)");
}

/// Test: JIT candidate identification
///
/// Validates that profiler can identify excellent JIT candidates based on:
/// - High call count (>1000 calls) OR
/// - High percentage (>30% of time)
#[test]
fn test_jit_candidate_identification() {
    let source = r#"
        fun many_calls(n) {
            if (n <= 0) { return 0; }
            return 1 + many_calls(n - 1);
        }

        fun big_loop() {
            let i = 0;
            while (i < 1000) { i = i + 1; }
            return i;
        }

        fun cold() { return 42; }

        let r1 = many_calls(20);  // Many calls (21 total)
        let r2 = big_loop();       // Hot (takes >30% of time)
        let r3 = cold();           // Cold (few calls, fast)
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Get JIT candidates (>1000 calls OR >30% time)
    let jit_candidates = profiler.jit_candidates(1000, 0.30);

    assert!(
        !jit_candidates.is_empty(),
        "Should identify at least one JIT candidate"
    );

    // big_loop should be a candidate (>30% time)
    assert!(
        jit_candidates.iter().any(|name| name == "big_loop"),
        "big_loop should be a JIT candidate (hot)"
    );

    // cold should NOT be a candidate
    assert!(
        !jit_candidates.iter().any(|name| name == "cold"),
        "cold should not be a JIT candidate"
    );
}
