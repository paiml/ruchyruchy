// INTERP-059 (JIT-007): Return Statements - Early Function Exit
//
// EXTREME TDD - RED Phase
//
// Mission: Implement explicit return statements in JIT
//
// Note: Return value field is Option<Box<AstNode>>

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Compile simple return statement
#[test]
fn test_compile_simple_return() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    let param_names = vec![];
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::IntegerLiteral(42))),
    };

    let compiled: fn() -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile return statement");

    assert_eq!(compiled(), 42);
}

/// Test: Compile early return from if
#[test]
fn test_compile_early_return_if() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // if (x > 0) { return 1; } return 0;
    let param_names = vec!["x".to_string()];
    let body = AstNode::Block {
        statements: vec![
            AstNode::IfExpr {
                condition: Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("x".to_string())),
                    op: BinaryOperator::GreaterThan,
                    right: Box::new(AstNode::IntegerLiteral(0)),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::IntegerLiteral(1))),
                }],
                else_branch: None,
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(0))),
            },
        ],
    };

    let compiled: fn(i64) -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile early return from if");

    assert_eq!(compiled(5), 1);
    assert_eq!(compiled(-3), 0);
}

/// Test: Return from loop (early exit)
#[test]
fn test_compile_return_from_loop() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // let i = 0; while (true) { if (i >= 3) { return i; } i = i + 1; }
    let param_names = vec![];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "i".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::WhileLoop {
                condition: Box::new(AstNode::BooleanLiteral(true)),
                body: vec![
                    AstNode::IfExpr {
                        condition: Box::new(AstNode::BinaryOp {
                            left: Box::new(AstNode::Identifier("i".to_string())),
                            op: BinaryOperator::GreaterEqual,
                            right: Box::new(AstNode::IntegerLiteral(3)),
                        }),
                        then_branch: vec![AstNode::Return {
                            value: Some(Box::new(AstNode::Identifier("i".to_string()))),
                        }],
                        else_branch: None,
                    },
                    AstNode::Assignment {
                        name: "i".to_string(),
                        value: Box::new(AstNode::BinaryOp {
                            left: Box::new(AstNode::Identifier("i".to_string())),
                            op: BinaryOperator::Add,
                            right: Box::new(AstNode::IntegerLiteral(1)),
                        }),
                    },
                ],
            },
        ],
    };

    let compiled: fn() -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile return from loop");

    assert_eq!(compiled(), 3);
}
