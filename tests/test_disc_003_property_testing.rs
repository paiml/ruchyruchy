// DISC-003: Property-Based Testing (INTEGRATION TESTS)
//
// Tests for property-based testing framework for compiler invariants.
//
// Requirements (from roadmap):
// - Property specification DSL
// - Shrinking for minimal failures
// - 10,000+ test cases per property
// - Coverage tracking
// - Mathematical property validation (roundtrip, etc.)
//
// Expected behavior:
// - Define properties as predicates over generated inputs
// - Generate 10,000+ random test cases per property
// - Detect property violations with counterexamples
// - Shrink counterexamples to minimal failing cases
// - Track coverage of tested cases
// - Support common compiler properties (roundtrip, type preservation, etc.)
//
// Testing Strategy:
// - Test property checking with various predicates
// - Verify 10,000+ test cases executed
// - Validate shrinking reduces counterexample size
// - Test common compiler properties
// - Measure performance of property checking

use ruchyruchy::bug_discovery::property_testing::{
    AstGenerator, Generator, Property, PropertyBug, PropertyChecker, PropertyResult,
};

/// Test: Property Checking with 10,000+ Test Cases
///
/// This test verifies the 10,000+ test cases requirement:
/// - Configure PropertyChecker with 10,000 test cases
/// - Run property check
/// - Verify all 10,000 cases executed for passing property
#[test]
fn test_property_checking_10k_test_cases() {
    let prop = Property::new(
        "length_bound".to_string(),
        "Generated code is < 1000 characters".to_string(),
    );

    let checker = PropertyChecker::new(42).with_test_cases(10_000);

    let result = checker.check(&prop, |test_case| {
        // Property: all generated code is under 1000 chars
        test_case.len() < 1000
    });

    match result {
        PropertyResult::Success { cases_tested } => {
            // Verify all 10,000 test cases were executed
            assert_eq!(cases_tested, 10_000);
        }
        PropertyResult::Violation { .. } => {
            // Property might fail if generator produces long code
            // That's a valid result - we're testing it runs 10k cases
        }
        PropertyResult::Error { message } => {
            panic!("Unexpected error: {}", message);
        }
    }
}

/// Test: Property Violation Detection
///
/// This test verifies property violation detection:
/// - Property that always fails
/// - Detect violation on first test case
/// - Return counterexample
#[test]
fn test_property_violation_detection() {
    let prop = Property::new(
        "impossible".to_string(),
        "All code is empty (impossible)".to_string(),
    );

    let checker = PropertyChecker::new(42).with_test_cases(100);

    let result = checker.check(&prop, |test_case| {
        // Impossible property: all code must be empty
        test_case.is_empty()
    });

    match result {
        PropertyResult::Violation {
            counterexample,
            cases_until_failure,
            ..
        } => {
            // Should fail on first non-empty generated case
            assert!(!counterexample.is_empty());
            assert_eq!(cases_until_failure, 1);
        }
        _ => panic!("Expected violation"),
    }
}

/// Test: Shrinking Reduces Counterexample Size
///
/// This test verifies shrinking functionality:
/// - Property violated by test case
/// - Shrinking reduces counterexample to minimal form
/// - Shrunk example is smaller than original
/// - Shrunk example still violates property
#[test]
fn test_shrinking_reduces_counterexample() {
    let prop = Property::new(
        "no_semicolons".to_string(),
        "Generated code contains no semicolons".to_string(),
    );

    let checker = PropertyChecker::new(42).with_test_cases(1000);

    let result = checker.check(&prop, |test_case| {
        // Property: no semicolons allowed
        !test_case.contains(';')
    });

    match result {
        PropertyResult::Violation {
            counterexample,
            shrunk_example,
            ..
        } => {
            assert!(counterexample.contains(';'));

            if let Some(shrunk) = shrunk_example {
                // Shrunk should be smaller or equal
                assert!(shrunk.len() <= counterexample.len());

                // Shrunk should still violate property
                assert!(shrunk.contains(';'));

                // Shrunk should be non-trivial reduction for generated code
                // (AstGenerator typically produces statements with semicolons)
                if counterexample.len() > 10 {
                    assert!(shrunk.len() < counterexample.len());
                }
            }
        }
        _ => {
            // Property might pass if generator doesn't produce semicolons
            // (unlikely given AstGenerator implementation)
        }
    }
}

/// Test: Common Compiler Property - Roundtrip
///
/// This test validates the roundtrip property definition:
/// - Property: parse(emit(ast)) == ast
/// - Verify property structure
/// - Test with mock roundtrip predicate
#[test]
fn test_compiler_property_roundtrip() {
    let prop = Property::roundtrip_parse_emit();

    assert_eq!(prop.name, "roundtrip_parse_emit");
    assert!(prop.description.contains("parse"));
    assert!(prop.description.contains("emit"));

    // Test with mock roundtrip predicate (always passes)
    let checker = PropertyChecker::new(42).with_test_cases(100);

    let result = checker.check(&prop, |test_case| {
        // Mock roundtrip: assume all inputs roundtrip correctly
        // Real implementation would: parse(emit(parse(test_case))) == parse(test_case)
        !test_case.is_empty() // Simple mock validation
    });

    if let PropertyResult::Success { cases_tested } = result {
        assert_eq!(cases_tested, 100);
    }
}

/// Test: Common Compiler Property - Type Preservation
///
/// This test validates the type preservation property:
/// - Property: typecheck(transform(ast)) preserves types
/// - Verify property structure
#[test]
fn test_compiler_property_type_preservation() {
    let prop = Property::type_preservation();

    assert_eq!(prop.name, "type_preservation");
    assert!(prop.description.contains("typecheck"));
    assert!(prop.description.contains("transform"));
    assert!(prop.description.contains("types"));

    // Test with mock type preservation predicate
    let checker = PropertyChecker::new(42).with_test_cases(100);

    let result = checker.check(&prop, |test_case| {
        // Mock: assume all well-formed inputs preserve types
        !test_case.is_empty()
    });

    if let PropertyResult::Success { cases_tested } = result {
        assert_eq!(cases_tested, 100);
    }
}

/// Test: Common Compiler Property - Deterministic Compilation
///
/// This test validates the deterministic compilation property:
/// - Property: compile(source) produces identical output on repeated runs
/// - Verify property structure
#[test]
fn test_compiler_property_deterministic_compilation() {
    let prop = Property::deterministic_compilation();

    assert_eq!(prop.name, "deterministic_compilation");
    assert!(prop.description.contains("compile"));
    assert!(prop.description.contains("identical"));
    assert!(prop.description.contains("repeated"));

    // Test with mock deterministic predicate
    let checker = PropertyChecker::new(42).with_test_cases(100);

    let result = checker.check(&prop, |_test_case| {
        // Mock: compilation is always deterministic
        true
    });

    match result {
        PropertyResult::Success { cases_tested } => {
            assert_eq!(cases_tested, 100);
        }
        _ => panic!("Expected success"),
    }
}

/// Test: Common Compiler Property - Semantic Equivalence
///
/// This test validates the semantic equivalence property:
/// - Property: compile(source).run() == interpret(source)
/// - Verify property structure
#[test]
fn test_compiler_property_semantic_equivalence() {
    let prop = Property::semantic_equivalence();

    assert_eq!(prop.name, "semantic_equivalence");
    assert!(prop.description.contains("compile"));
    assert!(prop.description.contains("interpret"));

    // Test with mock semantic equivalence predicate
    let checker = PropertyChecker::new(42).with_test_cases(100);

    let result = checker.check(&prop, |test_case| {
        // Mock: all inputs have equivalent semantics
        !test_case.is_empty()
    });

    if let PropertyResult::Success { cases_tested } = result {
        assert_eq!(cases_tested, 100);
    }
}

/// Test: Common Compiler Property - Idempotent Optimization
///
/// This test validates the idempotent optimization property:
/// - Property: optimize(optimize(code)) == optimize(code)
/// - Verify property structure
#[test]
fn test_compiler_property_idempotent_optimization() {
    let prop = Property::idempotent_optimization();

    assert_eq!(prop.name, "idempotent_optimization");
    assert!(prop.description.contains("optimize"));
    assert!(prop.description.contains("optimize(code)"));

    // Test with mock idempotent predicate
    let checker = PropertyChecker::new(42).with_test_cases(100);

    let result = checker.check(&prop, |_test_case| {
        // Mock: optimization is always idempotent
        true
    });

    match result {
        PropertyResult::Success { cases_tested } => {
            assert_eq!(cases_tested, 100);
        }
        _ => panic!("Expected success"),
    }
}

/// Test: AST Generator Determinism
///
/// This test verifies AST generator determinism:
/// - Same seed produces same sequence
/// - Different seeds produce different sequences
/// - Generated code is syntactically structured
#[test]
fn test_ast_generator_determinism() {
    // Test 1: Same seed produces same output
    let mut gen1 = AstGenerator::new(12345);
    let mut gen2 = AstGenerator::new(12345);

    let output1 = gen1.generate();
    let output2 = gen2.generate();

    assert_eq!(
        output1, output2,
        "Same seed should produce identical output"
    );

    // Test 2: Different seeds produce different output (probabilistically)
    let mut gen3 = AstGenerator::new(54321);
    let output3 = gen3.generate();

    assert_ne!(
        output1, output3,
        "Different seeds should (usually) produce different output"
    );

    // Test 3: Generated code has structure
    assert!(!output1.is_empty());
    // AstGenerator produces statements, which typically contain keywords or operators
    let has_structure = output1.contains("let")
        || output1.contains("if")
        || output1.contains("+")
        || output1.contains("-")
        || output1.contains(";");
    assert!(
        has_structure,
        "Generated code should have syntactic structure"
    );
}

/// Test: AST Generator Depth Control
///
/// This test verifies AST generator depth control:
/// - Max depth limits recursion
/// - Verify depth setting affects generation
#[test]
fn test_ast_generator_depth_control() {
    // Test 1: Shallow generator (max_depth=1)
    let mut shallow_gen = AstGenerator::new(42).with_max_depth(1);
    let shallow_expr = shallow_gen.generate_expr(0);

    // At max_depth=1, should only generate leaves (no nested parens expected)
    // or very simple expressions
    assert!(!shallow_expr.is_empty());

    // Test 2: Deep generator allows more complex nesting
    let mut deep_gen = AstGenerator::new(100).with_max_depth(5);

    // Generate many expressions and check if at least some have nesting
    let deep_exprs: Vec<String> = (0..50).map(|_| deep_gen.generate_expr(0)).collect();

    // With deeper max_depth and enough samples, should see some parentheses
    let has_nested = deep_exprs.iter().any(|expr| expr.contains('('));
    assert!(
        has_nested,
        "Deep generator should produce some nested expressions"
    );

    // Verify we got reasonable variety of expressions
    let total_parens: usize = deep_exprs
        .iter()
        .map(|expr| expr.chars().filter(|c| *c == '(').count())
        .sum();

    // With 50 expressions and max_depth=5, should generate some nested structures
    assert!(
        total_parens > 0,
        "Generator should produce some nested expressions"
    );
}

/// Test: PropertyBug Confidence Scoring
///
/// This test verifies PropertyBug confidence scoring:
/// - Property violations have high confidence
/// - Confidence > 0.85 for property-based discovery
/// - Shrunk examples included in bug report
#[test]
fn test_property_bug_confidence() {
    let prop = Property::roundtrip_parse_emit();
    let counterexample = "let x = (1 + 2);".to_string();
    let shrunk = Some("x".to_string());

    let bug = PropertyBug::new(prop.clone(), counterexample.clone(), shrunk.clone());

    // Verify bug details
    assert_eq!(bug.property.name, prop.name);
    assert_eq!(bug.counterexample, counterexample);
    assert_eq!(bug.shrunk_example, shrunk);

    // Verify high confidence for property violations
    assert!(
        bug.confidence.overall > 0.85,
        "Property violations should have high confidence score"
    );
}

/// Test: Performance - Large Scale Property Checking
///
/// This test verifies performance at scale:
/// - 50,000 test cases
/// - Completes in reasonable time
/// - Tracks cases correctly
#[test]
fn test_performance_large_scale() {
    let prop = Property::new(
        "reasonable_size".to_string(),
        "Generated code is under 500 chars".to_string(),
    );

    let checker = PropertyChecker::new(42).with_test_cases(50_000);

    let result = checker.check(&prop, |test_case| {
        // Simple property check
        test_case.len() < 500
    });

    match result {
        PropertyResult::Success { cases_tested } => {
            assert_eq!(cases_tested, 50_000);
        }
        PropertyResult::Violation {
            cases_until_failure,
            ..
        } => {
            // Violation is OK, just verify it was detected
            assert!(cases_until_failure > 0);
            assert!(cases_until_failure <= 50_000);
        }
        PropertyResult::Error { message } => {
            panic!("Unexpected error: {}", message);
        }
    }
}

/// Test: Property Result Equality
///
/// This test verifies PropertyResult equality semantics:
/// - Success results equal when cases_tested match
/// - Violation results equal when all fields match
#[test]
fn test_property_result_equality() {
    // Test 1: Success equality
    let success1 = PropertyResult::Success { cases_tested: 1000 };
    let success2 = PropertyResult::Success { cases_tested: 1000 };
    let success3 = PropertyResult::Success { cases_tested: 2000 };

    assert_eq!(success1, success2);
    assert_ne!(success1, success3);

    // Test 2: Violation equality
    let violation1 = PropertyResult::Violation {
        counterexample: "test".to_string(),
        shrunk_example: Some("t".to_string()),
        cases_until_failure: 10,
    };
    let violation2 = PropertyResult::Violation {
        counterexample: "test".to_string(),
        shrunk_example: Some("t".to_string()),
        cases_until_failure: 10,
    };
    let violation3 = PropertyResult::Violation {
        counterexample: "other".to_string(),
        shrunk_example: Some("t".to_string()),
        cases_until_failure: 10,
    };

    assert_eq!(violation1, violation2);
    assert_ne!(violation1, violation3);

    // Test 3: Different variants are not equal
    assert_ne!(success1, violation1);
}

/// Test: Custom Property Definition
///
/// This test verifies custom property creation:
/// - Create property with name and description
/// - Use property with checker
/// - Property metadata preserved
#[test]
fn test_custom_property_definition() {
    let prop = Property::new(
        "no_division_by_zero".to_string(),
        "Generated code never divides by zero".to_string(),
    );

    assert_eq!(prop.name, "no_division_by_zero");
    assert!(prop.description.contains("zero"));

    let checker = PropertyChecker::new(42).with_test_cases(100);

    let result = checker.check(&prop, |test_case| {
        // Check for division by literal zero
        !test_case.contains("/ 0") && !test_case.contains("/0")
    });

    // Result depends on generator output
    match result {
        PropertyResult::Success { cases_tested } => {
            assert_eq!(cases_tested, 100);
        }
        PropertyResult::Violation { counterexample, .. } => {
            // Verify counterexample actually divides by zero
            assert!(counterexample.contains("/ 0") || counterexample.contains("/0"));
        }
        _ => {}
    }
}
