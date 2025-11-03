// INTERP-073 (JIT-020): Tuple Destructuring Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement tuple destructuring in JIT compiler
//
// What we need to support:
// 1. Tuple destructuring: let (a, b, c) = tuple_expr;
// 2. Destructuring from tuple literals: let (x, y) = (1, 2);
// 3. Destructuring from tuple variables: let t = (1, 2); let (a, b) = t;
// 4. Using destructured values in expressions
// 5. Two-element, three-element, and larger tuples
//
// Why this is critical:
// - Pattern matching is fundamental in modern languages
// - Natural extension of tuple support
// - Ergonomic way to extract tuple values
// - Foundation for more complex destructuring patterns
//
// Implementation strategy:
// - Load tuple pointer from value expression
// - Create variable for each pattern name
// - Load tuple field at offset i*8 for pattern name i
// - Bind each variable to its tuple field value
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Simple two-element tuple destructuring from literal
///
/// Validates: let (a, b) = (10, 20); return a + b;
#[test]
fn test_compile_tuple_destruct_two_elements() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let (a, b) = (10, 20);
    //     return a + b;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::TupleDestruct {
                names: vec!["a".to_string(), "b".to_string()],
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![AstNode::IntegerLiteral(10), AstNode::IntegerLiteral(20)],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("a".to_string())),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::Identifier("b".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile two-element tuple destructuring");

    assert_eq!(main(), 30, "a + b should be 10 + 20 = 30");
}

/// Test: Three-element tuple destructuring
///
/// Validates: let (x, y, z) = (5, 10, 15); return x + y + z;
#[test]
fn test_compile_tuple_destruct_three_elements() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let (x, y, z) = (5, 10, 15);
    //     return x + y + z;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::TupleDestruct {
                names: vec!["x".to_string(), "y".to_string(), "z".to_string()],
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![
                        AstNode::IntegerLiteral(5),
                        AstNode::IntegerLiteral(10),
                        AstNode::IntegerLiteral(15),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("x".to_string())),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::Identifier("y".to_string())),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::Identifier("z".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile three-element tuple destructuring");

    assert_eq!(main(), 30, "x + y + z should be 5 + 10 + 15 = 30");
}

/// Test: Tuple destructuring from variable
///
/// Validates: let t = (100, 200); let (a, b) = t; return a + b;
#[test]
fn test_compile_tuple_destruct_from_variable() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let t = (100, 200);
    //     let (a, b) = t;
    //     return a + b;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "t".to_string(),
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![AstNode::IntegerLiteral(100), AstNode::IntegerLiteral(200)],
                }),
            },
            AstNode::TupleDestruct {
                names: vec!["a".to_string(), "b".to_string()],
                value: Box::new(AstNode::Identifier("t".to_string())),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("a".to_string())),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::Identifier("b".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile tuple destructuring from variable");

    assert_eq!(main(), 300, "a + b should be 100 + 200 = 300");
}

/// Test: Using individual destructured values
///
/// Validates: let (x, y) = (7, 3); return x * y;
#[test]
fn test_compile_tuple_destruct_usage() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let (x, y) = (7, 3);
    //     return x * y;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::TupleDestruct {
                names: vec!["x".to_string(), "y".to_string()],
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![AstNode::IntegerLiteral(7), AstNode::IntegerLiteral(3)],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("x".to_string())),
                    op: BinaryOperator::Multiply,
                    right: Box::new(AstNode::Identifier("y".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile tuple destructuring usage");

    assert_eq!(main(), 21, "x * y should be 7 * 3 = 21");
}

/// Test: Tuple destructuring with reassignment
///
/// Validates: let (a, b) = (10, 20); a = a + 5; return a + b;
#[test]
fn test_compile_tuple_destruct_with_reassignment() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let (a, b) = (10, 20);
    //     a = a + 5;
    //     return a + b;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::TupleDestruct {
                names: vec!["a".to_string(), "b".to_string()],
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![AstNode::IntegerLiteral(10), AstNode::IntegerLiteral(20)],
                }),
            },
            AstNode::Assignment {
                name: "a".to_string(),
                value: Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("a".to_string())),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::IntegerLiteral(5)),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("a".to_string())),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::Identifier("b".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile tuple destructuring with reassignment");

    assert_eq!(main(), 35, "(a + 5) + b should be 15 + 20 = 35");
}

/// Test: Multiple tuple destructurings
///
/// Validates: let (a, b) = (1, 2); let (c, d) = (3, 4); return a + b + c + d;
#[test]
fn test_compile_multiple_tuple_destructs() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let (a, b) = (1, 2);
    //     let (c, d) = (3, 4);
    //     return a + b + c + d;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::TupleDestruct {
                names: vec!["a".to_string(), "b".to_string()],
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![AstNode::IntegerLiteral(1), AstNode::IntegerLiteral(2)],
                }),
            },
            AstNode::TupleDestruct {
                names: vec!["c".to_string(), "d".to_string()],
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![AstNode::IntegerLiteral(3), AstNode::IntegerLiteral(4)],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::BinaryOp {
                            left: Box::new(AstNode::Identifier("a".to_string())),
                            op: BinaryOperator::Add,
                            right: Box::new(AstNode::Identifier("b".to_string())),
                        }),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::Identifier("c".to_string())),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::Identifier("d".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile multiple tuple destructurings");

    assert_eq!(main(), 10, "a + b + c + d should be 1 + 2 + 3 + 4 = 10");
}

/// Test: Tuple destructuring with swapping pattern
///
/// Validates: let (a, b) = (10, 20); let (b, a) = (a, b); return a - b;
/// This tests that new bindings shadow old ones correctly
#[test]
fn test_compile_tuple_destruct_swap() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let (a, b) = (10, 20);
    //     let (x, y) = (b, a);  // Swap values
    //     return x - y;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::TupleDestruct {
                names: vec!["a".to_string(), "b".to_string()],
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![AstNode::IntegerLiteral(10), AstNode::IntegerLiteral(20)],
                }),
            },
            AstNode::TupleDestruct {
                names: vec!["x".to_string(), "y".to_string()],
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![
                        AstNode::Identifier("b".to_string()),
                        AstNode::Identifier("a".to_string()),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("x".to_string())),
                    op: BinaryOperator::Subtract,
                    right: Box::new(AstNode::Identifier("y".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile tuple destructuring with swap");

    assert_eq!(main(), 10, "x - y should be 20 - 10 = 10 (after swap)");
}

/// Test: Four-element tuple destructuring
///
/// Validates: let (a, b, c, d) = (1, 2, 3, 4); return a * b * c * d;
#[test]
fn test_compile_tuple_destruct_four_elements() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let (a, b, c, d) = (2, 3, 4, 5);
    //     return a * b * c * d;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::TupleDestruct {
                names: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                ],
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![
                        AstNode::IntegerLiteral(2),
                        AstNode::IntegerLiteral(3),
                        AstNode::IntegerLiteral(4),
                        AstNode::IntegerLiteral(5),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::BinaryOp {
                            left: Box::new(AstNode::Identifier("a".to_string())),
                            op: BinaryOperator::Multiply,
                            right: Box::new(AstNode::Identifier("b".to_string())),
                        }),
                        op: BinaryOperator::Multiply,
                        right: Box::new(AstNode::Identifier("c".to_string())),
                    }),
                    op: BinaryOperator::Multiply,
                    right: Box::new(AstNode::Identifier("d".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile four-element tuple destructuring");

    assert_eq!(main(), 120, "a * b * c * d should be 2 * 3 * 4 * 5 = 120");
}
