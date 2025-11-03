// INTERP-060 (JIT-008): For Loops over Ranges
//
// EXTREME TDD - RED Phase
//
// Mission: Implement for loops in JIT for iteration over ranges
//
// What we need to support:
// 1. For loops over literal ranges: for i in 0..5 { ... }
// 2. For loops over variable ranges: for i in 0..n { ... }
// 3. For loops with accumulator pattern (sum, count, etc.)
// 4. For loops with early return
//
// Why this is critical:
// - For loops are more ergonomic than while loops
// - Common pattern in all languages
// - Enables cleaner iteration code
//
// Implementation strategy:
// - Desugar ForLoop to WhileLoop internally
// - for i in start..end { body } becomes:
//   let i = start; while (i < end) { body; i = i + 1; }
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Compile simple for loop over literal range
///
/// Validates: for i in 0..5 { sum = sum + i; } return sum;
/// Should compute: 0 + 1 + 2 + 3 + 4 = 10
#[test]
fn test_compile_for_loop_sum() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun() { let sum = 0; for i in 0..5 { sum = sum + i; } return sum; }
    let param_names = vec![];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "sum".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::ForLoop {
                var: "i".to_string(),
                iterable: Box::new(AstNode::Range {
                    start: Box::new(AstNode::IntegerLiteral(0)),
                    end: Box::new(AstNode::IntegerLiteral(5)),
                }),
                body: vec![AstNode::Assignment {
                    name: "sum".to_string(),
                    value: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("sum".to_string())),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::Identifier("i".to_string())),
                    }),
                }],
            },
            AstNode::Identifier("sum".to_string()),
        ],
    };

    let compiled: fn() -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile for loop");

    let result = compiled();
    assert_eq!(result, 10, "for i in 0..5 should sum to 10");
}

/// Test: Compile for loop over variable range
///
/// Validates: for i in 0..n { sum = sum + i; } return sum;
#[test]
fn test_compile_for_loop_variable_range() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun(n) { let sum = 0; for i in 0..n { sum = sum + i; } return sum; }
    let param_names = vec!["n".to_string()];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "sum".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::ForLoop {
                var: "i".to_string(),
                iterable: Box::new(AstNode::Range {
                    start: Box::new(AstNode::IntegerLiteral(0)),
                    end: Box::new(AstNode::Identifier("n".to_string())),
                }),
                body: vec![AstNode::Assignment {
                    name: "sum".to_string(),
                    value: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("sum".to_string())),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::Identifier("i".to_string())),
                    }),
                }],
            },
            AstNode::Identifier("sum".to_string()),
        ],
    };

    let compiled: fn(i64) -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile for loop with variable range");

    // Sum 0..5: 0 + 1 + 2 + 3 + 4 = 10
    assert_eq!(compiled(5), 10, "for i in 0..5 should sum to 10");

    // Sum 0..10: 0 + 1 + ... + 9 = 45
    assert_eq!(compiled(10), 45, "for i in 0..10 should sum to 45");
}

/// Test: Compile for loop with non-zero start
///
/// Validates: for i in 1..n { sum = sum + i; } return sum;
#[test]
fn test_compile_for_loop_nonzero_start() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun(n) { let sum = 0; for i in 1..n { sum = sum + i; } return sum; }
    let param_names = vec!["n".to_string()];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "sum".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::ForLoop {
                var: "i".to_string(),
                iterable: Box::new(AstNode::Range {
                    start: Box::new(AstNode::IntegerLiteral(1)),
                    end: Box::new(AstNode::Identifier("n".to_string())),
                }),
                body: vec![AstNode::Assignment {
                    name: "sum".to_string(),
                    value: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("sum".to_string())),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::Identifier("i".to_string())),
                    }),
                }],
            },
            AstNode::Identifier("sum".to_string()),
        ],
    };

    let compiled: fn(i64) -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile for loop with non-zero start");

    // Sum 1..6: 1 + 2 + 3 + 4 + 5 = 15
    assert_eq!(compiled(6), 15, "for i in 1..6 should sum to 15");

    // Sum 1..11: 1 + 2 + ... + 10 = 55
    assert_eq!(compiled(11), 55, "for i in 1..11 should sum to 55");
}

/// Test: Compile for loop with early return
///
/// Validates: for i in 0..n { if (i >= 3) { return i; } } return -1;
#[test]
fn test_compile_for_loop_early_return() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun(n) { for i in 0..n { if (i >= 3) { return i; } } return -1; }
    let param_names = vec!["n".to_string()];
    let body = AstNode::Block {
        statements: vec![
            AstNode::ForLoop {
                var: "i".to_string(),
                iterable: Box::new(AstNode::Range {
                    start: Box::new(AstNode::IntegerLiteral(0)),
                    end: Box::new(AstNode::Identifier("n".to_string())),
                }),
                body: vec![AstNode::IfExpr {
                    condition: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("i".to_string())),
                        op: BinaryOperator::GreaterEqual,
                        right: Box::new(AstNode::IntegerLiteral(3)),
                    }),
                    then_branch: vec![AstNode::Return {
                        value: Some(Box::new(AstNode::Identifier("i".to_string()))),
                    }],
                    else_branch: None,
                }],
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(-1))),
            },
        ],
    };

    let compiled: fn(i64) -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile for loop with early return");

    assert_eq!(compiled(10), 3, "Should return 3 when reaching i=3");
    assert_eq!(compiled(2), -1, "Should return -1 when loop doesn't reach 3");
}

/// Test: Compile for loop counting down
///
/// Validates: for i in (10-n)..10 { count = count + 1; } return count;
#[test]
fn test_compile_for_loop_countdown() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun(n) { let count = 0; for i in (10-n)..10 { count = count + 1; } return count; }
    let param_names = vec!["n".to_string()];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "count".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::ForLoop {
                var: "i".to_string(),
                iterable: Box::new(AstNode::Range {
                    start: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::IntegerLiteral(10)),
                        op: BinaryOperator::Subtract,
                        right: Box::new(AstNode::Identifier("n".to_string())),
                    }),
                    end: Box::new(AstNode::IntegerLiteral(10)),
                }),
                body: vec![AstNode::Assignment {
                    name: "count".to_string(),
                    value: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("count".to_string())),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::IntegerLiteral(1)),
                    }),
                }],
            },
            AstNode::Identifier("count".to_string()),
        ],
    };

    let compiled: fn(i64) -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile for loop with arithmetic in range");

    // for i in 7..10: iterations = 3
    assert_eq!(compiled(3), 3, "for i in 7..10 should iterate 3 times");

    // for i in 5..10: iterations = 5
    assert_eq!(compiled(5), 5, "for i in 5..10 should iterate 5 times");
}
