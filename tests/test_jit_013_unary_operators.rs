// INTERP-065 (JIT-012): Unary Operators in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement unary operators in JIT compiler
//
// What we need to support:
// 1. Negation: -x, -42
// 2. Logical NOT: !x, !true
// 3. Unary plus: +x (identity)
// 4. Unary operators in complex expressions
//
// Why this is critical:
// - Essential for mathematical expressions
// - Required for boolean logic
// - Foundation for more complex operations
// - Common in real-world code
//
// Implementation strategy:
// - Add UnaryOp handler to compile_expr_with_context
// - Use Cranelift instructions: ineg, bnot, etc.
// - Map operators: Negate → ineg, Not → conditional logic
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator, UnaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Simple integer negation
///
/// Validates: return -42;
#[test]
fn test_compile_integer_negation() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return -42; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::UnaryOp {
            op: UnaryOperator::Negate,
            operand: Box::new(AstNode::IntegerLiteral(42)),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile integer negation");

    assert_eq!(main(), -42, "Negation of 42 should be -42");
}

/// Test: Variable negation
///
/// Validates: let x = 10; return -x;
#[test]
fn test_compile_variable_negation() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let x = 10; return -x; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(AstNode::Identifier("x".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile variable negation");

    assert_eq!(main(), -10, "Negation of x=10 should be -10");
}

/// Test: Logical NOT with boolean literal
///
/// Validates: return !true; (should return 0, as false)
#[test]
fn test_compile_logical_not_true() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return !true; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::UnaryOp {
            op: UnaryOperator::Not,
            operand: Box::new(AstNode::BooleanLiteral(true)),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile logical NOT");

    assert_eq!(main(), 0, "!true should be 0 (false)");
}

/// Test: Logical NOT with integer (treat non-zero as true)
///
/// Validates: let x = 5; return !x; (should return 0, as x is non-zero/true)
#[test]
fn test_compile_logical_not_integer() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let x = 5; return !x; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(5)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(AstNode::Identifier("x".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile logical NOT with integer");

    assert_eq!(main(), 0, "!5 should be 0 (5 is non-zero, so !5 is false)");
}

/// Test: Logical NOT with zero
///
/// Validates: let x = 0; return !x; (should return 1, as x is zero/false)
#[test]
fn test_compile_logical_not_zero() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let x = 0; return !x; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(AstNode::Identifier("x".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile logical NOT with zero");

    assert_eq!(main(), 1, "!0 should be 1 (0 is false, so !0 is true)");
}

/// Test: Double negation
///
/// Validates: return --42; (should be 42)
#[test]
fn test_compile_double_negation() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return --42; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::UnaryOp {
            op: UnaryOperator::Negate,
            operand: Box::new(AstNode::UnaryOp {
                op: UnaryOperator::Negate,
                operand: Box::new(AstNode::IntegerLiteral(42)),
            }),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile double negation");

    assert_eq!(main(), 42, "Double negation --42 should be 42");
}

/// Test: Negation in arithmetic expression
///
/// Validates: let x = 10; return -x + 5; (should be -5)
#[test]
fn test_compile_negation_in_expression() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let x = 10; return -x + 5; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::UnaryOp {
                        op: UnaryOperator::Negate,
                        operand: Box::new(AstNode::Identifier("x".to_string())),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::IntegerLiteral(5)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile negation in expression");

    assert_eq!(main(), -5, "-x + 5 where x=10 should be -5");
}

/// Test: Unary plus (identity operation)
///
/// Validates: return +42; (should be 42)
#[test]
fn test_compile_unary_plus() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return +42; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::UnaryOp {
            op: UnaryOperator::Plus,
            operand: Box::new(AstNode::IntegerLiteral(42)),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile unary plus");

    assert_eq!(main(), 42, "Unary plus +42 should be 42");
}

/// Test: Logical NOT in conditional
///
/// Validates: let x = 0; if (!x) { return 42; } else { return 0; }
#[test]
fn test_compile_logical_not_in_conditional() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 0;
    //     if (!x) { return 42; } else { return 0; }
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::IfExpr {
                condition: Box::new(AstNode::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(AstNode::Identifier("x".to_string())),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::IntegerLiteral(42))),
                }],
                else_branch: Some(vec![AstNode::Return {
                    value: Some(Box::new(AstNode::IntegerLiteral(0))),
                }]),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile logical NOT in conditional");

    assert_eq!(
        main(),
        42,
        "if (!x) where x=0 should take then branch (return 42)"
    );
}
