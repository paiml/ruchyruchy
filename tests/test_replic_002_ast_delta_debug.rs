// REPLIC-002: AST-Based Delta Debugging (INTEGRATION TESTS)
//
// Tests for tree-based delta debugging that preserves syntactic correctness.
//
// Requirements (from roadmap):
// - AST parsing for hierarchical minimization
// - Syntax preservation during reduction
// - Hierarchical delta debugging (HDD)
// - O(n log n) performance target
// - Minimize to <10 LOC
//
// Expected behavior:
// - Parse input into AST structure
// - Apply delta debugging at AST level (not line level)
// - Preserve syntactic correctness throughout minimization
// - Use hierarchical approach (remove functions, then statements, then expressions)
// - Verify minimized test still triggers bug
// - Achieve better reduction than line-based approach
//
// Testing Strategy:
// - Test with semantic-aware minimization strategy
// - Verify syntax preservation
// - Compare with line-based reduction
// - Test on real code examples
// - Measure reduction efficiency

use ruchyruchy::bug_replication::minimizer::{
    DeltaDebugger, MinimizationStrategy, TestOutcome,
};

/// Test: AST-Based Minimization Workflow
///
/// This test verifies the complete AST-based minimization workflow:
/// - Create delta debugger with Semantic strategy
/// - Minimize failing test case
/// - Verify minimized output still fails
/// - Verify significant reduction achieved
#[test]
fn test_ast_based_minimization_workflow() {
    // Test oracle: fails if code contains both "function" and "crash"
    let test_fn = |input: &str| {
        if input.contains("function") && input.contains("crash") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Semantic);

    // Large input with irrelevant code
    let input = "// Header comment\n\
                 function setup() {\n\
                   let x = 1;\n\
                   return x;\n\
                 }\n\
                 \n\
                 function main() {\n\
                   crash();\n\
                 }\n\
                 \n\
                 function cleanup() {\n\
                   // Cleanup code\n\
                 }";

    let result = debugger.minimize(input);

    // Verify minimized output still triggers bug
    assert!(result.minimized.contains("function"));
    assert!(result.minimized.contains("crash"));

    // Verify reduction achieved
    assert!(result.reduction_ratio > 0.0);
    assert!(result.minimized_size < result.original_size);

    // Verify test runs recorded
    assert!(result.test_runs > 0);
}

/// Test: Syntax Preservation
///
/// This test verifies that AST-based minimization preserves syntactic correctness:
/// - Minimized code remains syntactically valid
/// - No partial lines or broken structures
/// - Semantic strategy preserves structure
#[test]
fn test_syntax_preservation() {
    // Fails if input has more than 3 lines
    let test_fn = |input: &str| {
        if input.lines().filter(|l| !l.trim().is_empty()).count() > 3 {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Semantic);

    let input = "function test() {\n\
                   line1();\n\
                   line2();\n\
                   line3();\n\
                   line4();\n\
                 }";

    let result = debugger.minimize(input);

    // Should reduce to 4 non-empty lines (minimal failing case)
    let non_empty_lines = result
        .minimized
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count();
    assert_eq!(non_empty_lines, 4);

    // Verify output looks structurally sound (has function structure)
    assert!(
        result.minimized.contains("function") || result.minimized.contains("line"),
        "Minimized output should preserve some structure"
    );
}

/// Test: Hierarchical Minimization
///
/// This test verifies hierarchical delta debugging approach:
/// - Remove large chunks first (functions)
/// - Then smaller chunks (statements)
/// - More efficient than character-by-character
#[test]
fn test_hierarchical_minimization() {
    // Fails if code contains "bug()" function call
    let test_fn = |input: &str| {
        if input.contains("bug()") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut semantic_debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Semantic);
    let mut line_debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

    let input = "function a() { x(); }\n\
                 function b() { y(); }\n\
                 function c() { bug(); }\n\
                 function d() { z(); }";

    let semantic_result = semantic_debugger.minimize(input);
    let line_result = line_debugger.minimize(input);

    // Both should find the bug
    assert!(semantic_result.minimized.contains("bug()"));
    assert!(line_result.minimized.contains("bug()"));

    // Semantic (AST-based) might be more efficient in test runs
    // (though actual performance depends on the algorithm)
    assert!(semantic_result.test_runs > 0);
    assert!(line_result.test_runs > 0);

    // Both should achieve reduction
    assert!(semantic_result.reduction_ratio > 0.0);
    assert!(line_result.reduction_ratio > 0.0);
}

/// Test: Reduction Efficiency
///
/// This test verifies reduction efficiency metrics:
/// - Reduction ratio calculation (1.0 = complete, 0.0 = none)
/// - Test run count tracking
/// - Original vs minimized size comparison
#[test]
fn test_reduction_efficiency() {
    // Fails if contains "ERROR" keyword
    let test_fn = |input: &str| {
        if input.contains("ERROR") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Semantic);

    let input = "// Comment line 1\n\
                 // Comment line 2\n\
                 // Comment line 3\n\
                 ERROR found here\n\
                 // Comment line 5\n\
                 // Comment line 6";

    let result = debugger.minimize(input);

    // Verify metrics
    assert_eq!(result.original_size, 6); // 6 lines
    assert!(result.minimized_size < result.original_size);

    // Reduction ratio: should be significant (removed most comments)
    assert!(result.reduction_ratio > 0.5); // At least 50% reduction

    // Test runs: should have made multiple attempts
    assert!(result.test_runs >= 3);

    // Verify ERROR still present
    assert!(result.minimized.contains("ERROR"));
}

/// Test: Real Code Minimization
///
/// This test verifies minimization on realistic code examples:
/// - Parser bug: deeply nested expressions
/// - Type error: complex type annotations
/// - Runtime crash: function call chain
#[test]
fn test_real_code_minimization() {
    // Test Case 1: Parser bug with nested expressions
    let parser_bug_fn = |input: &str| {
        // Fails if more than 5 opening parens
        if input.chars().filter(|c| *c == '(').count() > 5 {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut parser_debugger = DeltaDebugger::new(parser_bug_fn, MinimizationStrategy::Semantic);

    let parser_input = "let x = 1;\n\
                       let y = (((((((a + b) * c) / d) - e) + f) * g) + h);\n\
                       let z = 3;";

    let parser_result = parser_debugger.minimize(parser_input);

    // Should reduce to just the problematic line
    assert!(parser_result.minimized.chars().filter(|c| *c == '(').count() > 5);
    assert!(parser_result.reduction_ratio > 0.0);

    // Test Case 2: Type error with complex annotations
    let type_error_fn = |input: &str| {
        // Fails if contains both "Vec" and "HashMap"
        if input.contains("Vec") && input.contains("HashMap") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut type_debugger = DeltaDebugger::new(type_error_fn, MinimizationStrategy::Semantic);

    let type_input = "let a: i32 = 5;\n\
                     let b: String = \"test\";\n\
                     let c: Vec<HashMap<String, i32>> = Vec::new();\n\
                     let d: bool = true;";

    let type_result = type_debugger.minimize(type_input);

    // Should reduce to just the problematic line
    assert!(type_result.minimized.contains("Vec"));
    assert!(type_result.minimized.contains("HashMap"));
    assert!(type_result.reduction_ratio > 0.0);
}

/// Test: Edge Case - Already Minimal
///
/// This test verifies handling of already minimal test cases:
/// - Input that cannot be reduced further
/// - Single line failure
/// - Reduction ratio should be 0.0
#[test]
fn test_edge_case_already_minimal() {
    // Fails if contains "BUG"
    let test_fn = |input: &str| {
        if input.contains("BUG") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Semantic);

    // Already minimal input
    let input = "BUG";

    let result = debugger.minimize(input);

    // Should return essentially the same (maybe with different whitespace)
    assert!(result.minimized.contains("BUG"));
    assert_eq!(result.original_size, 1); // 1 line
    assert_eq!(result.minimized_size, 1); // Still 1 line

    // Reduction ratio should be 0.0 (no reduction possible)
    assert_eq!(result.reduction_ratio, 0.0);
}

/// Test: Edge Case - No Failure
///
/// This test verifies handling when input doesn't trigger bug:
/// - Input that passes all tests
/// - Should return input unchanged
/// - Reduction ratio should be 0.0
#[test]
fn test_edge_case_no_failure() {
    // Never fails
    let test_fn = |_: &str| TestOutcome::Pass;

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Semantic);

    let input = "function test() {\n\
                   code();\n\
                 }";

    let result = debugger.minimize(input);

    // Should return input unchanged
    assert_eq!(result.minimized, input);
    assert_eq!(result.reduction_ratio, 0.0);
    assert_eq!(result.original_size, result.minimized_size);
}

/// Test: Complex Multi-Function Code
///
/// This test verifies minimization on code with multiple functions:
/// - Multiple function definitions
/// - Only one triggers bug
/// - Should isolate the problematic function
#[test]
fn test_complex_multi_function_code() {
    // Fails if code contains function named "problematic"
    let test_fn = |input: &str| {
        if input.contains("function problematic") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Semantic);

    let input = "function good1() {\n\
                   return 1;\n\
                 }\n\
                 \n\
                 function good2() {\n\
                   return 2;\n\
                 }\n\
                 \n\
                 function problematic() {\n\
                   crash();\n\
                 }\n\
                 \n\
                 function good3() {\n\
                   return 3;\n\
                 }";

    let result = debugger.minimize(input);

    // Should reduce to just the problematic function
    assert!(result.minimized.contains("function problematic"));
    assert!(!result.minimized.contains("good1") || !result.minimized.contains("good2"));

    // Should achieve significant reduction
    assert!(result.reduction_ratio > 0.5); // At least 50% reduction
    assert!(result.minimized_size < result.original_size);
}

/// Test: Performance - Large Input
///
/// This test verifies performance on larger inputs:
/// - 50+ lines of code
/// - Efficient delta debugging (O(n log n) ideally)
/// - Reasonable number of test runs
#[test]
fn test_performance_large_input() {
    // Fails if line contains "BUG_LINE"
    let test_fn = |input: &str| {
        if input.lines().any(|line| line.contains("BUG_LINE")) {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Semantic);

    // Generate 50 lines, with BUG_LINE at line 25
    let mut lines = Vec::new();
    for i in 1..=50 {
        if i == 25 {
            lines.push("function test() { BUG_LINE(); }".to_string());
        } else {
            lines.push(format!("function test{}() {{ ok(); }}", i));
        }
    }
    let input = lines.join("\n");

    let result = debugger.minimize(&input);

    // Should find the bug line
    assert!(result.minimized.contains("BUG_LINE"));

    // Should achieve massive reduction
    assert!(result.reduction_ratio > 0.9); // >90% reduction

    // Test runs should be reasonable (not exponential)
    // With 50 lines, O(n log n) ≈ 50 * 6 = 300
    // Delta debugging typically does better
    assert!(result.test_runs < 500); // Should be well under this
}
