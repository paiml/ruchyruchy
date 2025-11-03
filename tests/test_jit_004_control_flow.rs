// INTERP-055 (JIT-004): Control Flow Compilation (If/While)
//
// EXTREME TDD - RED Phase
//
// Mission: Compile conditional statements and loops to machine code
//
// What we need to support:
// 1. Comparison operators: ==, !=, <, >, <=, >=
// 2. Boolean operators: &&, ||
// 3. If statements: if (condition) { then } else { else }
// 4. While loops: while (condition) { body }
//
// Why this is critical for JIT:
// - Real programs need conditionals and loops
// - Control flow is where performance matters most
// - Enables compiling hot loops identified by profiler
//
// Cranelift concepts needed:
// - Basic blocks: Separate code blocks for branches
// - Branch instructions: brz (branch if zero), brnz (branch if not zero)
// - Comparison instructions: icmp (integer compare)
// - Phi nodes: Not needed (SSA form handles this)
//
// Method: Test-driven development with incremental complexity

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Compile comparison operators (equal)
///
/// Validates: fun(a, b) { return a == b; } returns 1 (true) or 0 (false)
#[test]
fn test_compile_comparison_equal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun equals(a, b) { return a == b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::Equal,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile equals function");

    // Execute: equals(5, 5) should return 1 (true)
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(5, 5), 1, "equals(5, 5) should return 1 (true)");
    assert_eq!(func(3, 7), 0, "equals(3, 7) should return 0 (false)");
}

/// Test: Compile comparison operators (not equal)
///
/// Validates: fun(a, b) { return a != b; }
#[test]
fn test_compile_comparison_not_equal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun not_equals(a, b) { return a != b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::NotEqual,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile not_equals function");

    // Execute
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(5, 5), 0, "not_equals(5, 5) should return 0 (false)");
    assert_eq!(func(3, 7), 1, "not_equals(3, 7) should return 1 (true)");
}

/// Test: Compile comparison operators (less than)
///
/// Validates: fun(a, b) { return a < b; }
#[test]
fn test_compile_comparison_less_than() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun less_than(a, b) { return a < b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::LessThan,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile less_than function");

    // Execute
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(3, 7), 1, "less_than(3, 7) should return 1 (true)");
    assert_eq!(func(7, 3), 0, "less_than(7, 3) should return 0 (false)");
    assert_eq!(func(5, 5), 0, "less_than(5, 5) should return 0 (false)");
}

/// Test: Compile comparison operators (greater than)
///
/// Validates: fun(a, b) { return a > b; }
#[test]
fn test_compile_comparison_greater_than() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun greater_than(a, b) { return a > b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::GreaterThan,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile greater_than function");

    // Execute
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(7, 3), 1, "greater_than(7, 3) should return 1 (true)");
    assert_eq!(func(3, 7), 0, "greater_than(3, 7) should return 0 (false)");
    assert_eq!(func(5, 5), 0, "greater_than(5, 5) should return 0 (false)");
}

/// Test: Compile comparison operators (less than or equal)
///
/// Validates: fun(a, b) { return a <= b; }
#[test]
fn test_compile_comparison_less_equal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun less_equal(a, b) { return a <= b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::LessEqual,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile less_equal function");

    // Execute
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(3, 7), 1, "less_equal(3, 7) should return 1 (true)");
    assert_eq!(func(5, 5), 1, "less_equal(5, 5) should return 1 (true)");
    assert_eq!(func(7, 3), 0, "less_equal(7, 3) should return 0 (false)");
}

/// Test: Compile comparison operators (greater than or equal)
///
/// Validates: fun(a, b) { return a >= b; }
#[test]
fn test_compile_comparison_greater_equal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun greater_equal(a, b) { return a >= b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::GreaterEqual,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile greater_equal function");

    // Execute
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(7, 3), 1, "greater_equal(7, 3) should return 1 (true)");
    assert_eq!(func(5, 5), 1, "greater_equal(5, 5) should return 1 (true)");
    assert_eq!(func(3, 7), 0, "greater_equal(3, 7) should return 0 (false)");
}

/// Test: Compile boolean AND operator
///
/// Validates: fun(a, b) { return a && b; }
#[test]
fn test_compile_boolean_and() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun and(a, b) { return a && b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::And,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile and function");

    // Execute (treating non-zero as true, zero as false)
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(1, 1), 1, "and(1, 1) should return 1 (true)");
    assert_eq!(func(1, 0), 0, "and(1, 0) should return 0 (false)");
    assert_eq!(func(0, 1), 0, "and(0, 1) should return 0 (false)");
    assert_eq!(func(0, 0), 0, "and(0, 0) should return 0 (false)");
}

/// Test: Compile boolean OR operator
///
/// Validates: fun(a, b) { return a || b; }
#[test]
fn test_compile_boolean_or() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun or(a, b) { return a || b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::Or,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile or function");

    // Execute
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(1, 1), 1, "or(1, 1) should return 1 (true)");
    assert_eq!(func(1, 0), 1, "or(1, 0) should return 1 (true)");
    assert_eq!(func(0, 1), 1, "or(0, 1) should return 1 (true)");
    assert_eq!(func(0, 0), 0, "or(0, 0) should return 0 (false)");
}

/// Test: Compile complex boolean expression
///
/// Validates: fun(a, b, c) { return (a > b) && (b < c); }
#[test]
fn test_compile_complex_boolean() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun complex(a, b, c) { return (a > b) && (b < c); }
    let param_names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::Identifier("a".to_string())),
            op: BinaryOperator::GreaterThan,
            right: Box::new(AstNode::Identifier("b".to_string())),
        }),
        op: BinaryOperator::And,
        right: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::Identifier("b".to_string())),
            op: BinaryOperator::LessThan,
            right: Box::new(AstNode::Identifier("c".to_string())),
        }),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile complex boolean function");

    // Execute: complex(10, 5, 8) -> (10 > 5) && (5 < 8) -> true && true -> true
    let func: fn(i64, i64, i64) -> i64 = compiled;
    assert_eq!(
        func(10, 5, 8),
        1,
        "complex(10, 5, 8) should return 1 (true)"
    );
    assert_eq!(func(3, 5, 8), 0, "complex(3, 5, 8) should return 0 (false)");
    assert_eq!(
        func(10, 5, 2),
        0,
        "complex(10, 5, 2) should return 0 (false)"
    );
}
