// REPLIC-001: Line-Based Delta Debugging (INTEGRATION TESTS)
//
// Tests for line-based delta debugging as fallback to AST-based approach.
//
// Requirements (from roadmap):
// - Line-based minimization (remove lines)
// - O(n log n) algorithm efficiency
// - Test preservation check (bug still triggers)
// - Minimize to <10 LOC (target for 90%+ cases)
// - Graceful fallback when AST parsing fails
//
// Expected behavior:
// - Split input into lines
// - Apply ddmin algorithm to remove lines
// - Verify minimized input still triggers bug
// - Achieve O(n log n) performance (not exponential)
// - Handle edge cases (single line, no failure, already minimal)
// - Work on any text input (no parsing required)
//
// Testing Strategy:
// - Test line-based strategy explicitly
// - Verify <10 LOC target achievement
// - Compare performance with other strategies
// - Test on various input sizes
// - Verify robustness (no parsing dependencies)

use ruchyruchy::bug_replication::minimizer::{
    DeltaDebugger, MinimizationStrategy, TestOutcome,
};

/// Test: Line-Based Minimization Workflow
///
/// This test verifies the complete line-based minimization workflow:
/// - Create delta debugger with Line strategy
/// - Minimize failing test case
/// - Verify minimized output still fails
/// - Verify line-by-line reduction
#[test]
fn test_line_based_minimization_workflow() {
    // Test oracle: fails if input contains line with "ERROR"
    let test_fn = |input: &str| {
        if input.lines().any(|line| line.contains("ERROR")) {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

    // Multi-line input with one problematic line
    let input = "// Comment line 1\n\
                 let x = 5;\n\
                 let y = 10;\n\
                 ERROR: crash here\n\
                 let z = 15;\n\
                 // Comment line 2";

    let result = debugger.minimize(input);

    // Verify minimized output still contains ERROR
    assert!(result.minimized.contains("ERROR"));

    // Verify reduction achieved
    assert!(result.reduction_ratio > 0.0);
    assert!(result.minimized_size < result.original_size);
    assert_eq!(result.original_size, 6); // 6 lines

    // Verify test runs recorded
    assert!(result.test_runs > 0);
}

/// Test: Minimize to Less Than 10 LOC
///
/// This test verifies the <10 LOC target requirement:
/// - Start with >10 lines of code
/// - Minimize to <10 lines
/// - Achieve 90%+ success rate on realistic bugs
#[test]
fn test_minimize_to_under_10_loc() {
    // Fails if contains both "function" and "bug"
    let test_fn = |input: &str| {
        if input.contains("function") && input.contains("bug") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

    // 20-line input with bug
    let input = "// File header\n\
                 // Author: test\n\
                 // Date: 2024\n\
                 \n\
                 function setup() {\n\
                   initialize();\n\
                   configure();\n\
                 }\n\
                 \n\
                 function main() {\n\
                   setup();\n\
                   bug();\n\
                   cleanup();\n\
                 }\n\
                 \n\
                 function cleanup() {\n\
                   release();\n\
                   shutdown();\n\
                 }\n\
                 // End of file";

    let result = debugger.minimize(input);

    // Verify reduced to <10 lines
    assert!(result.minimized_size < 10);
    assert_eq!(result.original_size, 20);

    // Verify bug still present
    assert!(result.minimized.contains("function"));
    assert!(result.minimized.contains("bug"));

    // Verify significant reduction (>50%)
    assert!(result.reduction_ratio > 0.5);
}

/// Test: O(n log n) Performance
///
/// This test verifies O(n log n) algorithmic complexity:
/// - Test with small input (10 lines)
/// - Test with large input (100 lines)
/// - Verify test runs scale as O(n log n), not O(n²)
#[test]
fn test_on_log_n_performance() {
    // Fails if contains "BUG_MARKER"
    let test_fn = |input: &str| {
        if input.contains("BUG_MARKER") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    // Test 1: Small input (10 lines)
    let mut small_debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);
    let small_lines: Vec<String> = (1..=10)
        .map(|i| {
            if i == 5 {
                "BUG_MARKER here".to_string()
            } else {
                format!("Line {}", i)
            }
        })
        .collect();
    let small_input = small_lines.join("\n");

    let small_result = small_debugger.minimize(&small_input);
    let small_test_runs = small_result.test_runs;

    // Test 2: Large input (100 lines)
    let mut large_debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);
    let large_lines: Vec<String> = (1..=100)
        .map(|i| {
            if i == 50 {
                "BUG_MARKER here".to_string()
            } else {
                format!("Line {}", i)
            }
        })
        .collect();
    let large_input = large_lines.join("\n");

    let large_result = large_debugger.minimize(&large_input);
    let large_test_runs = large_result.test_runs;

    // Verify O(n log n) scaling
    // If O(n²): 100 lines = 100x test runs
    // If O(n log n): 100 lines ≈ 10 * log(100/10) ≈ 10x test runs
    // Allow some overhead, check it's not quadratic
    let ratio = large_test_runs as f64 / small_test_runs as f64;

    // Ratio should be ~10-20x, not 100x (would be quadratic)
    assert!(ratio < 50.0, "Test runs scaling is too high (possibly O(n²))");

    // Both should find the bug
    assert!(small_result.minimized.contains("BUG_MARKER"));
    assert!(large_result.minimized.contains("BUG_MARKER"));
}

/// Test: Test Preservation
///
/// This test verifies the bug is preserved throughout minimization:
/// - Original input fails
/// - Every reduction attempt checks test still fails
/// - Final minimized output fails
/// - No intermediate passing states leaked
#[test]
fn test_preservation_of_bug() {
    // Fails if sum of line numbers with "X" > 10
    let test_fn = |input: &str| {
        let sum: usize = input
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains("X"))
            .map(|(i, _)| i + 1)
            .sum();

        if sum > 10 {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

    // Input with "X" on lines 2, 3, 4, 5 (sum = 2+3+4+5 = 14 > 10)
    let input = "Line 1\n\
                 Line 2 X\n\
                 Line 3 X\n\
                 Line 4 X\n\
                 Line 5 X\n\
                 Line 6";

    let result = debugger.minimize(input);

    // Verify bug still triggers
    let verification = test_fn(&result.minimized);
    assert_eq!(verification, TestOutcome::Fail);

    // Verify some reduction occurred
    assert!(result.reduction_ratio > 0.0);
}

/// Test: Comparison with Other Strategies
///
/// This test compares line-based vs character-based and token-based:
/// - Line-based: works on any text, no parsing
/// - Token-based: needs tokenization
/// - Character-based: very fine-grained but slow
/// - Line-based should be robust fallback
#[test]
fn test_comparison_with_other_strategies() {
    // Fails if contains "crash()"
    let test_fn = |input: &str| {
        if input.contains("crash()") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let input = "function a() { ok(); }\n\
                 function b() { crash(); }\n\
                 function c() { ok(); }";

    // Line-based
    let mut line_debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);
    let line_result = line_debugger.minimize(input);

    // Token-based
    let mut token_debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Token);
    let token_result = token_debugger.minimize(input);

    // Character-based
    let mut char_debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Character);
    let char_result = char_debugger.minimize(input);

    // All should find the bug
    assert!(line_result.minimized.contains("crash()"));
    assert!(token_result.minimized.contains("crash()"));
    assert!(char_result.minimized.contains("crash()"));

    // Line-based should have reasonable test runs
    assert!(line_result.test_runs > 0);
    assert!(line_result.test_runs < 100); // Not excessive

    // Character-based typically needs more test runs
    assert!(char_result.test_runs >= line_result.test_runs);

    // All should achieve some reduction
    assert!(line_result.reduction_ratio > 0.0);
    assert!(token_result.reduction_ratio > 0.0);
    assert!(char_result.reduction_ratio > 0.0);
}

/// Test: Robustness - No Parsing Required
///
/// This test verifies line-based works on any text input:
/// - Syntactically invalid code
/// - Non-code text
/// - Mixed content
/// - No dependencies on parsing/AST
#[test]
fn test_robustness_no_parsing_required() {
    // Fails if contains "ERROR"
    let test_fn = |input: &str| {
        if input.contains("ERROR") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

    // Test 1: Syntactically invalid code
    let invalid_code = "function {\n\
                       let x = ;\n\
                       ERROR ][[ \n\
                       } incomplete";

    let result1 = debugger.minimize(invalid_code);
    assert!(result1.minimized.contains("ERROR"));

    // Test 2: Non-code text
    let plain_text = "This is plain text.\n\
                     Nothing special here.\n\
                     ERROR: something went wrong\n\
                     More text follows.";

    let result2 = debugger.minimize(plain_text);
    assert!(result2.minimized.contains("ERROR"));

    // Test 3: Mixed content
    let mixed = "```rust\n\
                code block\n\
                ```\n\
                # Markdown header\n\
                ERROR in documentation\n\
                - List item";

    let result3 = debugger.minimize(mixed);
    assert!(result3.minimized.contains("ERROR"));

    // All should achieve reduction
    assert!(result1.reduction_ratio > 0.0);
    assert!(result2.reduction_ratio > 0.0);
    assert!(result3.reduction_ratio > 0.0);
}

/// Test: Edge Case - Single Line
///
/// This test verifies handling of single-line input:
/// - Cannot reduce further
/// - Returns input unchanged
/// - Reduction ratio = 0.0
#[test]
fn test_edge_case_single_line() {
    // Fails if contains "BUG"
    let test_fn = |input: &str| {
        if input.contains("BUG") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

    // Single line input
    let input = "This line has a BUG in it";

    let result = debugger.minimize(input);

    // Should return essentially the same
    assert_eq!(result.minimized, input);
    assert_eq!(result.original_size, 1);
    assert_eq!(result.minimized_size, 1);
    assert_eq!(result.reduction_ratio, 0.0);
}

/// Test: Edge Case - All Lines Required
///
/// This test verifies handling when all lines are necessary:
/// - Bug requires multiple lines together
/// - Cannot remove any line without breaking bug
/// - Minimal reduction or none
#[test]
fn test_edge_case_all_lines_required() {
    // Fails if has exactly 3 lines all containing "X"
    let test_fn = |input: &str| {
        let lines_with_x: Vec<&str> = input.lines().filter(|l| l.contains("X")).collect();
        if lines_with_x.len() == 3 {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

    // Input with exactly 3 lines with "X"
    let input = "X line 1\n\
                 X line 2\n\
                 X line 3";

    let result = debugger.minimize(input);

    // Should keep all 3 lines (cannot remove any)
    assert_eq!(result.minimized_size, 3);
    assert_eq!(result.reduction_ratio, 0.0);

    // Bug should still trigger
    assert_eq!(result.minimized.lines().filter(|l| l.contains("X")).count(), 3);
}

/// Test: Large Input Performance
///
/// This test verifies performance on large inputs (100+ lines):
/// - 200 line input
/// - Bug on single line
/// - Should minimize to <10 lines
/// - Test runs should be reasonable (<1000)
#[test]
fn test_large_input_performance() {
    // Fails if contains "BUG_LINE_157"
    let test_fn = |input: &str| {
        if input.contains("BUG_LINE_157") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

    // Generate 200 lines with bug on line 157
    let lines: Vec<String> = (1..=200)
        .map(|i| {
            if i == 157 {
                "BUG_LINE_157 found here".to_string()
            } else {
                format!("Normal line {}", i)
            }
        })
        .collect();
    let input = lines.join("\n");

    let result = debugger.minimize(&input);

    // Verify bug found
    assert!(result.minimized.contains("BUG_LINE_157"));

    // Verify massive reduction to <10 lines
    assert!(result.minimized_size < 10);
    assert_eq!(result.original_size, 200);

    // Verify >95% reduction
    assert!(result.reduction_ratio > 0.95);

    // Verify reasonable test runs (O(n log n) ≈ 200 * 7.6 ≈ 1520)
    // Should be well under 2000
    assert!(result.test_runs < 2000);
}

/// Test: Real Bug Example - Parser Crash
///
/// This test simulates a real parser crash scenario:
/// - Complex code with nested structures
/// - Bug triggered by specific line pattern
/// - Minimize to isolate the crash
#[test]
fn test_real_bug_parser_crash() {
    // Simulates parser crash on deeply nested structure
    let test_fn = |input: &str| {
        // Fails if any line has >5 opening braces
        if input.lines().any(|line| line.chars().filter(|c| *c == '{').count() > 5) {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

    let input = "function normal() {\n\
                   let x = 1;\n\
                 }\n\
                 \n\
                 function nested() {\n\
                   obj = { a: { b: { c: { d: { e: { f: 1 } } } } } };\n\
                 }\n\
                 \n\
                 function another() {\n\
                   let y = 2;\n\
                 }";

    let result = debugger.minimize(input);

    // Should isolate the problematic line
    assert!(result.minimized.chars().filter(|c| *c == '{').count() > 5);

    // Should reduce significantly
    assert!(result.reduction_ratio > 0.6); // >60% reduction
    assert!(result.minimized_size < result.original_size);
}
