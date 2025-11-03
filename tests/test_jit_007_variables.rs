// INTERP-058 (JIT-006): Variable Storage - Let Declarations and Assignments
//
// EXTREME TDD - RED Phase
//
// Mission: Implement variable storage in JIT to enable mutable variables
//
// What we need to support:
// 1. Let declarations: let x = 5;
// 2. Variable assignments: x = x + 1;
// 3. Variables in loops: while (i < n) { sum = sum + i; i = i + 1; }
//
// Why this is critical:
// - Without variables, loops are useless (can't have counters)
// - Can't accumulate values
// - Can't have local state
//
// Cranelift approach:
// - Use Cranelift Variable class (not stack slots)
// - declare_var() creates a variable
// - def_var() assigns a value to a variable
// - use_var() reads a variable's current value
// - Variables are automatically converted to SSA form
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Compile simple let declaration and usage
///
/// Validates: let x = 42; return x;
#[test]
fn test_compile_let_declaration() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun() { let x = 42; return x; }
    // Need to support sequence of statements
    let param_names = vec![];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(42)),
            },
            AstNode::Identifier("x".to_string()),
        ],
    };

    let compiled: fn() -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile let declaration");

    let result = compiled();
    assert_eq!(result, 42, "let x = 42; return x should return 42");
}

/// Test: Compile variable assignment
///
/// Validates: let x = 5; x = 10; return x;
#[test]
fn test_compile_variable_assignment() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun() { let x = 5; x = 10; return x; }
    let param_names = vec![];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(5)),
            },
            AstNode::Assignment {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::Identifier("x".to_string()),
        ],
    };

    let compiled: fn() -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile variable assignment");

    let result = compiled();
    assert_eq!(result, 10, "After assignment, x should be 10");
}

/// Test: Compile variable increment
///
/// Validates: let x = 5; x = x + 1; return x;
#[test]
fn test_compile_variable_increment() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun() { let x = 5; x = x + 1; return x; }
    let param_names = vec![];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(5)),
            },
            AstNode::Assignment {
                name: "x".to_string(),
                value: Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("x".to_string())),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::IntegerLiteral(1)),
                }),
            },
            AstNode::Identifier("x".to_string()),
        ],
    };

    let compiled: fn() -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile variable increment");

    let result = compiled();
    assert_eq!(result, 6, "x = x + 1 should result in 6");
}

/// Test: Compile while loop with counter
///
/// Validates: let i = 0; while (i < 3) { i = i + 1; } return i;
/// This is the killer app for variables - makes loops useful!
#[test]
fn test_compile_while_loop_with_counter() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun() { let i = 0; while (i < 3) { i = i + 1; } return i; }
    let param_names = vec![];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "i".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::WhileLoop {
                condition: Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("i".to_string())),
                    op: BinaryOperator::LessThan,
                    right: Box::new(AstNode::IntegerLiteral(3)),
                }),
                body: vec![AstNode::Assignment {
                    name: "i".to_string(),
                    value: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("i".to_string())),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::IntegerLiteral(1)),
                    }),
                }],
            },
            AstNode::Identifier("i".to_string()),
        ],
    };

    let compiled: fn() -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile while loop with counter");

    let result = compiled();
    assert_eq!(result, 3, "Loop should execute 3 times, final i = 3");
}

/// Test: Compile while loop with accumulator
///
/// Validates: Sum from 1 to n
/// fun(n) { let sum = 0; let i = 1; while (i <= n) { sum = sum + i; i = i + 1; } return sum; }
#[test]
fn test_compile_while_loop_sum() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun(n) { let sum = 0; let i = 1; while (i <= n) { sum = sum + i; i = i + 1; } return sum; }
    let param_names = vec!["n".to_string()];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "sum".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::LetDecl {
                name: "i".to_string(),
                value: Box::new(AstNode::IntegerLiteral(1)),
            },
            AstNode::WhileLoop {
                condition: Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("i".to_string())),
                    op: BinaryOperator::LessEqual,
                    right: Box::new(AstNode::Identifier("n".to_string())),
                }),
                body: vec![
                    AstNode::Assignment {
                        name: "sum".to_string(),
                        value: Box::new(AstNode::BinaryOp {
                            left: Box::new(AstNode::Identifier("sum".to_string())),
                            op: BinaryOperator::Add,
                            right: Box::new(AstNode::Identifier("i".to_string())),
                        }),
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
            AstNode::Identifier("sum".to_string()),
        ],
    };

    let compiled: fn(i64) -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile while loop with accumulator");

    // Sum from 1 to 5: 1+2+3+4+5 = 15
    let result = compiled(5);
    assert_eq!(result, 15, "Sum from 1 to 5 should be 15");

    // Sum from 1 to 10: 1+2+...+10 = 55
    let result = compiled(10);
    assert_eq!(result, 55, "Sum from 1 to 10 should be 55");
}

/// Test: Multiple variables with different scopes
///
/// Validates: let x = 1; let y = 2; return x + y;
#[test]
fn test_compile_multiple_variables() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun() { let x = 1; let y = 2; return x + y; }
    let param_names = vec![];
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(1)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::IntegerLiteral(2)),
            },
            AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("x".to_string())),
                op: BinaryOperator::Add,
                right: Box::new(AstNode::Identifier("y".to_string())),
            },
        ],
    };

    let compiled: fn() -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile multiple variables");

    let result = compiled();
    assert_eq!(result, 3, "x + y should be 1 + 2 = 3");
}
