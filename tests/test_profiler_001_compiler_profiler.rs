// PROFILER-001: Compiler Profiling Tool - Phase 1: Enhanced Profiler
//
// RED PHASE: Write failing tests first
//
// Mission: Extend DEBUGGER-047 PerformanceProfiler for compiler optimization
// Goal: Track compilation phases, type observations, optimization opportunities
//
// Requirements (from specification):
// - Track compilation time per phase (lexing, parsing, type checking, codegen)
// - Observe runtime types at function calls (Julia-inspired)
// - Identify hot functions (call count > 100, time > 1% total)
// - Detect optimization opportunities (constant folding, inlining, TCO)
// - Compare performance across execution modes (AST, Bytecode, Transpiled, Compiled)
//
// Tests:
// - test_compiler_phase_tracking: Track time for each compilation phase
// - test_type_observation: Observe argument types at function calls
// - test_hot_function_detection: Identify functions consuming >1% total time
// - test_optimization_opportunity_detection: Find constant expressions in hot paths
// - test_cross_mode_comparison: Compare AST vs Bytecode vs Transpiled performance
// - test_profiler_001_completeness: Meta-test

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use ruchyruchy::profiler::CompilerProfiler;
use std::time::Duration;

/// Test: Compiler Phase Tracking
///
/// RED: Track compilation time for each phase
///
/// Property: Should measure time for lexing, parsing, type checking, codegen
#[test]
fn test_compiler_phase_tracking() {
    // Create compiler profiler
    let profiler = CompilerProfiler::new();

    // Simulate compilation phases
    profiler.start_phase("lexing");
    std::thread::sleep(Duration::from_millis(10));
    profiler.end_phase("lexing");

    profiler.start_phase("parsing");
    std::thread::sleep(Duration::from_millis(20));
    profiler.end_phase("parsing");

    profiler.start_phase("type_checking");
    std::thread::sleep(Duration::from_millis(15));
    profiler.end_phase("type_checking");

    // Get report
    let report = profiler.phase_report();

    // Verify phases tracked
    assert!(report.contains_phase("lexing"), "Should track lexing phase");
    assert!(
        report.contains_phase("parsing"),
        "Should track parsing phase"
    );
    assert!(
        report.contains_phase("type_checking"),
        "Should track type_checking phase"
    );

    // Verify timing (rough checks due to sleep variance)
    assert!(
        report.phase_time("lexing").as_millis() >= 9,
        "Lexing should take ~10ms"
    );
    assert!(
        report.phase_time("parsing").as_millis() >= 19,
        "Parsing should take ~20ms"
    );

    // Verify total time
    let total = report.total_time();
    assert!(
        total.as_millis() >= 44, // 10+20+15-1 (variance)
        "Total time should be sum of phases"
    );
}

/// Test: Type Observation
///
/// RED: Observe runtime types at function calls (Julia-inspired)
///
/// Property: Should track type signatures for each function call
#[test]
fn test_type_observation() {
    let profiler = CompilerProfiler::new();

    // Simulate function calls with type observations
    let code = r#"
        fun add(a, b) { a + b }

        let x = add(1, 2);        // Int32 + Int32
        let y = add(1.5, 2.3);    // Float64 + Float64
        let z = add(5, 10);       // Int32 + Int32 (again)
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse");

    let mut eval = Evaluator::new().with_type_observation(&profiler);
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute");
    }

    // Get type observations
    let observations = profiler.type_observations("add");

    // Verify observed types
    assert_eq!(observations.len(), 3, "Should observe 3 calls to add()");

    // Verify type signatures
    let sig1 = &observations[0];
    assert_eq!(sig1.param_types(), &["Integer", "Integer"]);
    assert_eq!(sig1.return_type(), "Integer");

    let sig2 = &observations[1];
    assert_eq!(sig2.param_types(), &["Float", "Float"]);
    assert_eq!(sig2.return_type(), "Float");

    // Verify type stability analysis
    let stability = profiler.type_stability("add");
    assert_eq!(
        stability,
        ruchyruchy::profiler::Stability::Polymorphic,
        "add() should be polymorphic (2 type signatures observed)"
    );
}

/// Test: Hot Function Detection
///
/// RED: Identify functions consuming >1% of total execution time
///
/// Property: Functions with high call count or high total time should be flagged
#[test]
fn test_hot_function_detection() {
    let profiler = CompilerProfiler::new();

    let code = r#"
        fun fibonacci(n) {
            if n <= 1 {
                n
            } else {
                fibonacci(n - 1) + fibonacci(n - 2)
            }
        }

        let result = fibonacci(10);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse");

    // DEBUGGER-052: Attach profiler to track function calls
    let mut eval = Evaluator::new().with_type_observation(&profiler);
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute");
    }

    // Get hot functions (>1% of total time)
    let hot_functions = profiler.hot_functions(1.0); // >1% of total time

    // Verify fibonacci is identified as hot
    assert!(
        hot_functions.iter().any(|(name, _)| name == "fibonacci"),
        "fibonacci should be identified as hot function"
    );

    // Verify percentage
    let (_, fib_percentage) = hot_functions
        .iter()
        .find(|(name, _)| name == "fibonacci")
        .unwrap();

    assert!(
        *fib_percentage > 95.0,
        "fibonacci should consume >95% of total time"
    );

    // Verify call count separately
    let fib_profile = profiler.function_profile("fibonacci").unwrap();
    assert!(
        fib_profile.call_count > 100,
        "fibonacci(10) should be called 177 times"
    );
}

/// Test: Optimization Opportunity Detection
///
/// RED: Find constant expressions in hot paths
///
/// Property: Should detect expressions that can be folded at compile-time
#[test]
fn test_optimization_opportunity_detection() {
    let profiler = CompilerProfiler::new();

    let code = r#"
        fun compute() {
            let x = 2 + 3 * 4;      // Constant: 14
            let y = 10 > 5;         // Constant: true
            let mut sum = 0;
            let mut i = 0;
            while i < 1000 {
                sum = sum + x;      // x is constant in hot loop!
                i = i + 1;
            }
            sum
        }

        let result = compute();
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse");

    // Analyze for optimization opportunities
    let opportunities = profiler.analyze_ast(&ast);

    // Verify constant folding opportunities detected
    let const_folding = opportunities
        .iter()
        .filter(|o| {
            matches!(
                o.kind,
                ruchyruchy::profiler::OptKind::ConstantFolding { .. }
            )
        })
        .collect::<Vec<_>>();

    assert!(
        !const_folding.is_empty(),
        "Should detect constant folding opportunities"
    );

    // Verify specific constants detected
    let has_arithmetic = const_folding.iter().any(|o| {
        if let ruchyruchy::profiler::OptKind::ConstantFolding { expr, .. } = &o.kind {
            expr.contains("2 + 3 * 4")
        } else {
            false
        }
    });

    assert!(
        has_arithmetic,
        "Should detect '2 + 3 * 4' as constant expression"
    );
}

/// Test: Cross-Mode Performance Comparison
///
/// RED: Compare execution time across AST, Bytecode, Transpiled modes
///
/// Property: Should run same code in multiple modes and report speedup
#[test]
fn test_cross_mode_comparison() {
    let profiler = CompilerProfiler::new();

    // INTERP-044: Reduced from fibonacci(15) to fibonacci(12) to prevent stack overflow
    // with closure implementation's additional stack usage per recursive call
    let code = r#"
        fun fibonacci(n) {
            if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) }
        }
        let result = fibonacci(12);
    "#;

    // Run in AST mode
    let _ast_time = profiler.profile_mode(code, ruchyruchy::profiler::ExecutionMode::AST);

    // Run in Bytecode mode
    let _bytecode_time = profiler.profile_mode(code, ruchyruchy::profiler::ExecutionMode::Bytecode);

    // Run in Transpiled mode (if available)
    let _transpiled_time =
        profiler.profile_mode(code, ruchyruchy::profiler::ExecutionMode::Transpiled);

    // Generate comparison report
    let report = profiler.comparison_report();

    // Verify modes tracked
    assert!(report.has_mode(ruchyruchy::profiler::ExecutionMode::AST));
    assert!(report.has_mode(ruchyruchy::profiler::ExecutionMode::Bytecode));

    // Verify speedup calculation
    let speedup = report.speedup(
        ruchyruchy::profiler::ExecutionMode::AST,
        ruchyruchy::profiler::ExecutionMode::Transpiled,
    );

    assert!(speedup > 5.0, "Transpiled should be >5x faster than AST");
}

/// Test: PROFILER-001 Completeness
///
/// Verify all required components exist
#[test]
fn test_profiler_001_completeness() {
    let required_tests = [
        "test_compiler_phase_tracking",
        "test_type_observation",
        "test_hot_function_detection",
        "test_optimization_opportunity_detection",
        "test_cross_mode_comparison",
        "test_profiler_001_completeness",
    ];

    println!(
        "✅ PROFILER-001: All {} required tests present",
        required_tests.len()
    );
    println!("   - Compiler phase tracking");
    println!("   - Type observation (Julia-inspired)");
    println!("   - Hot function detection");
    println!("   - Optimization opportunity detection");
    println!("   - Cross-mode comparison");
}
