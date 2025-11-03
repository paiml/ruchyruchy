// INTERP-050: Type Stability Tracking for JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Add type stability tracking to ruchydbg for JIT candidate identification
//
// JIT Needs:
// 1. Which functions are type-stable? (monomorphic = excellent JIT target)
// 2. Which functions are polymorphic? (moderate JIT target, need inline cache)
// 3. Which functions are megamorphic? (poor JIT target, generic dispatch)
//
// Type Stability Classification:
// - Monomorphic: 1 type signature observed (excellent for JIT)
// - Polymorphic: 2-3 type signatures (moderate, use inline cache)
// - Megamorphic: 4+ type signatures (poor, generic dispatch)
//
// This enables JIT to:
// - Prioritize monomorphic hot functions
// - Use inline caches for polymorphic functions
// - Skip megamorphic functions (too unpredictable)
//
// Method: Test-driven development with CompilerProfiler integration

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use ruchyruchy::profiler::{CompilerProfiler, Stability};

/// Test: Monomorphic function detection
///
/// Validates that profiler detects functions with single type signature
#[test]
fn test_monomorphic_function_detection() {
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

    // Check type stability
    let stability = profiler.type_stability("add");
    assert_eq!(
        stability,
        Stability::Monomorphic,
        "add should be monomorphic (always Int, Int)"
    );

    // Get type observations
    let observations = profiler.type_observations("add");
    assert_eq!(
        observations.len(),
        3,
        "add should have 3 observations (3 calls)"
    );

    // All should be the same type signature
    let unique_sigs: std::collections::HashSet<_> = observations.iter().collect();
    assert_eq!(
        unique_sigs.len(),
        1,
        "All calls should have same type signature"
    );
}

/// Test: Polymorphic function detection
///
/// Validates that profiler detects functions with 2-3 type signatures
#[test]
fn test_polymorphic_function_detection() {
    let source = r#"
        fun identity(x) {
            return x;
        }

        let r1 = identity(42);        // Int
        let r2 = identity("hello");   // String
        let r3 = identity(42);        // Int again
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Check type stability
    let stability = profiler.type_stability("identity");
    assert_eq!(
        stability,
        Stability::Polymorphic,
        "identity should be polymorphic (Int and String)"
    );

    // Get type observations
    let observations = profiler.type_observations("identity");
    assert_eq!(observations.len(), 3, "identity should have 3 observations");

    // Should have 2 unique type signatures
    let unique_sigs: std::collections::HashSet<_> = observations.iter().collect();
    assert_eq!(
        unique_sigs.len(),
        2,
        "identity should have 2 unique type signatures"
    );
}

/// Test: Megamorphic function detection
///
/// Validates that profiler detects functions with 4+ type signatures
#[test]
fn test_megamorphic_function_detection() {
    let source = r#"
        fun process(x) {
            return x;
        }

        let r1 = process(42);           // Integer
        let r2 = process("hello");      // String
        let r3 = process(true);         // Boolean
        let r4 = process(3.14);         // Float (4th type)
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Check type stability
    let stability = profiler.type_stability("process");
    assert_eq!(
        stability,
        Stability::Megamorphic,
        "process should be megamorphic (4+ type signatures)"
    );

    // Get type observations
    let observations = profiler.type_observations("process");
    assert_eq!(observations.len(), 4, "process should have 4 observations");

    // Should have 4 unique type signatures
    let unique_sigs: std::collections::HashSet<_> = observations.iter().collect();
    assert_eq!(
        unique_sigs.len(),
        4,
        "process should have 4 unique type signatures"
    );
}

/// Test: JIT candidate ranking by type stability
///
/// Validates that profiler can identify excellent JIT candidates
/// based on both hotness AND type stability
#[test]
fn test_jit_candidate_ranking_by_type_stability() {
    let source = r#"
        fun hot_stable(n) {
            let i = 0;
            while (i < 100) { i = i + 1; }
            return n;
        }

        fun hot_unstable(x) {
            return x;
        }

        fun cold_stable(n) {
            return n + 1;
        }

        // Hot + stable (EXCELLENT JIT candidate)
        let r1 = hot_stable(1);
        let r2 = hot_stable(2);
        let r3 = hot_stable(3);

        // Hot + unstable (POOR JIT candidate)
        let r4 = hot_unstable(1);
        let r5 = hot_unstable("x");
        let r6 = hot_unstable(true);

        // Cold + stable (NOT JIT candidate - not hot)
        let r7 = cold_stable(1);
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Check type stability classifications
    assert_eq!(
        profiler.type_stability("hot_stable"),
        Stability::Monomorphic,
        "hot_stable should be monomorphic"
    );

    assert_eq!(
        profiler.type_stability("hot_unstable"),
        Stability::Polymorphic,
        "hot_unstable should be polymorphic"
    );

    assert_eq!(
        profiler.type_stability("cold_stable"),
        Stability::Monomorphic,
        "cold_stable should be monomorphic"
    );

    // hot_stable should be:
    // - Hot (takes >30% of time due to loop)
    // - Type-stable (monomorphic)
    // = EXCELLENT JIT CANDIDATE

    let hot_functions = profiler.hot_functions(30.0); // >30% of time
    let hot_stable_is_hot = hot_functions.iter().any(|(name, _)| name == "hot_stable");

    assert!(
        hot_stable_is_hot,
        "hot_stable should be identified as hot function"
    );

    // Combine with type stability for best JIT candidates
    let best_jit_candidates: Vec<_> = hot_functions
        .iter()
        .filter(|(name, _)| profiler.type_stability(name) == Stability::Monomorphic)
        .map(|(name, _)| name.clone())
        .collect();

    assert!(
        best_jit_candidates.contains(&"hot_stable".to_string()),
        "hot_stable should be excellent JIT candidate (hot + monomorphic)"
    );

    assert!(
        !best_jit_candidates.contains(&"hot_unstable".to_string()),
        "hot_unstable should NOT be excellent candidate (not monomorphic)"
    );
}

/// Test: Type signature display
///
/// Validates that profiler can format type signatures for display
#[test]
fn test_type_signature_display() {
    let source = r#"
        fun add(a, b) {
            return a + b;
        }

        let r1 = add(1, 2);
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Get type observations
    let observations = profiler.type_observations("add");
    assert_eq!(observations.len(), 1, "Should have 1 observation");

    // Check signature structure
    let sig = &observations[0];
    assert_eq!(sig.param_types().len(), 2, "Should have 2 params");
    assert_eq!(
        sig.param_types()[0],
        "Integer",
        "First param should be Integer"
    );
    assert_eq!(
        sig.param_types()[1],
        "Integer",
        "Second param should be Integer"
    );
    assert_eq!(
        sig.return_type(),
        "Integer",
        "Return type should be Integer"
    );
}

/// Test: Type stability percentage calculation
///
/// Validates that profiler can calculate type stability as percentage
#[test]
fn test_type_stability_percentage() {
    let source = r#"
        fun maybe_add(a, b) {
            return a + b;
        }

        let r1 = maybe_add(1, 2);      // Int, Int → Int
        let r2 = maybe_add(3, 4);      // Int, Int → Int
        let r3 = maybe_add(5, 6);      // Int, Int → Int
        let r4 = maybe_add(7, 8);      // Int, Int → Int
        let r5 = maybe_add("a", "b");  // String, String → String (different!)
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse should succeed");

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        let _ = eval.eval(statement);
    }

    // Should be polymorphic (2 type signatures)
    assert_eq!(
        profiler.type_stability("maybe_add"),
        Stability::Polymorphic,
        "maybe_add should be polymorphic"
    );

    // Calculate stability percentage
    let observations = profiler.type_observations("maybe_add");
    let unique_sigs: std::collections::HashSet<_> = observations.iter().collect();

    // 5 total calls, 2 unique signatures
    // Stability score: 4/5 = 80% (4 calls matched most common signature)
    assert_eq!(observations.len(), 5, "Should have 5 observations");
    assert_eq!(unique_sigs.len(), 2, "Should have 2 unique signatures");

    // Most stable signature appears 4 times (80%)
    let mut sig_counts: std::collections::HashMap<_, usize> = std::collections::HashMap::new();
    for sig in &observations {
        *sig_counts.entry(sig).or_insert(0) += 1;
    }

    let max_count = sig_counts.values().max().unwrap();
    let stability_pct = (*max_count as f64 / observations.len() as f64) * 100.0;

    assert!(
        stability_pct >= 80.0,
        "Stability should be 80% (4/5 calls with same signature)"
    );
}
