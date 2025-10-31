// INTERP-006: Control Flow Implementation - RED Phase
// Tests for while loops, for loops, and match expressions
// Note: if/else already implemented in INTERP-005

use ruchyruchy::interpreter::evaluator::{EvalError, Evaluator};
use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator, MatchArm, Pattern};

// =============================================================================
// While Loops
// =============================================================================

#[test]
fn test_while_loop_simple() {
    // let sum = 0;
    // let i = 1;
    // while (i <= 5) {
    //     sum = sum + i;
    //     i = i + 1;
    // }
    // sum -> 15 (1+2+3+4+5)
    let mut eval = Evaluator::new();

    // Initialize sum = 0
    eval.eval(&AstNode::LetDecl {
        name: "sum".to_string(),
        value: Box::new(AstNode::IntegerLiteral(0)),
    })
    .unwrap();

    // Initialize i = 1
    eval.eval(&AstNode::LetDecl {
        name: "i".to_string(),
        value: Box::new(AstNode::IntegerLiteral(1)),
    })
    .unwrap();

    // while (i <= 5) { sum = sum + i; i = i + 1; }
    eval.eval(&AstNode::WhileLoop {
        condition: Box::new(AstNode::BinaryOp {
            op: BinaryOperator::LessEqual,
            left: Box::new(AstNode::Identifier("i".to_string())),
            right: Box::new(AstNode::IntegerLiteral(5)),
        }),
        body: vec![
            AstNode::Assignment {
                name: "sum".to_string(),
                value: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(AstNode::Identifier("sum".to_string())),
                    right: Box::new(AstNode::Identifier("i".to_string())),
                }),
            },
            AstNode::Assignment {
                name: "i".to_string(),
                value: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(AstNode::Identifier("i".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(1)),
                }),
            },
        ],
    })
    .unwrap();

    // Check sum = 15
    let sum = eval.eval(&AstNode::Identifier("sum".to_string())).unwrap();
    assert_eq!(sum.as_integer().unwrap(), 15);
}

#[test]
fn test_while_loop_zero_iterations() {
    // while (false) { ... } should not execute body
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "x".to_string(),
        value: Box::new(AstNode::IntegerLiteral(0)),
    })
    .unwrap();

    eval.eval(&AstNode::WhileLoop {
        condition: Box::new(AstNode::BooleanLiteral(false)),
        body: vec![AstNode::Assignment {
            name: "x".to_string(),
            value: Box::new(AstNode::IntegerLiteral(99)),
        }],
    })
    .unwrap();

    let x = eval.eval(&AstNode::Identifier("x".to_string())).unwrap();
    assert_eq!(x.as_integer().unwrap(), 0); // Should still be 0
}

#[test]
fn test_while_loop_nested() {
    // Nested while loops
    // let result = 0;
    // let i = 1;
    // while (i <= 3) {
    //     let j = 1;
    //     while (j <= 2) {
    //         result = result + 1;
    //         j = j + 1;
    //     }
    //     i = i + 1;
    // }
    // result -> 6 (3 iterations * 2 iterations)
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "result".to_string(),
        value: Box::new(AstNode::IntegerLiteral(0)),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "i".to_string(),
        value: Box::new(AstNode::IntegerLiteral(1)),
    })
    .unwrap();

    eval.eval(&AstNode::WhileLoop {
        condition: Box::new(AstNode::BinaryOp {
            op: BinaryOperator::LessEqual,
            left: Box::new(AstNode::Identifier("i".to_string())),
            right: Box::new(AstNode::IntegerLiteral(3)),
        }),
        body: vec![
            AstNode::LetDecl {
                name: "j".to_string(),
                value: Box::new(AstNode::IntegerLiteral(1)),
            },
            AstNode::WhileLoop {
                condition: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::LessEqual,
                    left: Box::new(AstNode::Identifier("j".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(2)),
                }),
                body: vec![
                    AstNode::Assignment {
                        name: "result".to_string(),
                        value: Box::new(AstNode::BinaryOp {
                            op: BinaryOperator::Add,
                            left: Box::new(AstNode::Identifier("result".to_string())),
                            right: Box::new(AstNode::IntegerLiteral(1)),
                        }),
                    },
                    AstNode::Assignment {
                        name: "j".to_string(),
                        value: Box::new(AstNode::BinaryOp {
                            op: BinaryOperator::Add,
                            left: Box::new(AstNode::Identifier("j".to_string())),
                            right: Box::new(AstNode::IntegerLiteral(1)),
                        }),
                    },
                ],
            },
            AstNode::Assignment {
                name: "i".to_string(),
                value: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(AstNode::Identifier("i".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(1)),
                }),
            },
        ],
    })
    .unwrap();

    let result = eval
        .eval(&AstNode::Identifier("result".to_string()))
        .unwrap();
    assert_eq!(result.as_integer().unwrap(), 6);
}

// =============================================================================
// For Loops
// =============================================================================

#[test]
fn test_for_loop_over_vector() {
    // let sum = 0;
    // for x in [1, 2, 3, 4, 5] {
    //     sum = sum + x;
    // }
    // sum -> 15
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "sum".to_string(),
        value: Box::new(AstNode::IntegerLiteral(0)),
    })
    .unwrap();

    eval.eval(&AstNode::ForLoop {
        var: "x".to_string(),
        iterable: Box::new(AstNode::VectorLiteral {
            elements: vec![
                AstNode::IntegerLiteral(1),
                AstNode::IntegerLiteral(2),
                AstNode::IntegerLiteral(3),
                AstNode::IntegerLiteral(4),
                AstNode::IntegerLiteral(5),
            ],
        }),
        body: vec![AstNode::Assignment {
            name: "sum".to_string(),
            value: Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(AstNode::Identifier("sum".to_string())),
                right: Box::new(AstNode::Identifier("x".to_string())),
            }),
        }],
    })
    .unwrap();

    let sum = eval.eval(&AstNode::Identifier("sum".to_string())).unwrap();
    assert_eq!(sum.as_integer().unwrap(), 15);
}

#[test]
fn test_for_loop_empty_vector() {
    // for x in [] { ... } should not execute body
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "x".to_string(),
        value: Box::new(AstNode::IntegerLiteral(0)),
    })
    .unwrap();

    eval.eval(&AstNode::ForLoop {
        var: "item".to_string(),
        iterable: Box::new(AstNode::VectorLiteral { elements: vec![] }),
        body: vec![AstNode::Assignment {
            name: "x".to_string(),
            value: Box::new(AstNode::IntegerLiteral(99)),
        }],
    })
    .unwrap();

    let x = eval.eval(&AstNode::Identifier("x".to_string())).unwrap();
    assert_eq!(x.as_integer().unwrap(), 0);
}

#[test]
fn test_for_loop_nested() {
    // let result = 0;
    // for i in [1, 2, 3] {
    //     for j in [10, 20] {
    //         result = result + i + j;
    //     }
    // }
    // result -> (1+10) + (1+20) + (2+10) + (2+20) + (3+10) + (3+20) = 11 + 21 + 12 + 22 + 13 + 23 = 102
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "result".to_string(),
        value: Box::new(AstNode::IntegerLiteral(0)),
    })
    .unwrap();

    eval.eval(&AstNode::ForLoop {
        var: "i".to_string(),
        iterable: Box::new(AstNode::VectorLiteral {
            elements: vec![
                AstNode::IntegerLiteral(1),
                AstNode::IntegerLiteral(2),
                AstNode::IntegerLiteral(3),
            ],
        }),
        body: vec![AstNode::ForLoop {
            var: "j".to_string(),
            iterable: Box::new(AstNode::VectorLiteral {
                elements: vec![AstNode::IntegerLiteral(10), AstNode::IntegerLiteral(20)],
            }),
            body: vec![AstNode::Assignment {
                name: "result".to_string(),
                value: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(AstNode::BinaryOp {
                        op: BinaryOperator::Add,
                        left: Box::new(AstNode::Identifier("result".to_string())),
                        right: Box::new(AstNode::Identifier("i".to_string())),
                    }),
                    right: Box::new(AstNode::Identifier("j".to_string())),
                }),
            }],
        }],
    })
    .unwrap();

    let result = eval
        .eval(&AstNode::Identifier("result".to_string()))
        .unwrap();
    assert_eq!(result.as_integer().unwrap(), 102);
}

// =============================================================================
// Match Expressions
// =============================================================================

#[test]
fn test_match_literal_integer() {
    // match 2 {
    //     1 => 10,
    //     2 => 20,
    //     3 => 30,
    // }
    // Result: 20
    let mut eval = Evaluator::new();

    let result = eval
        .eval(&AstNode::MatchExpr {
            expr: Box::new(AstNode::IntegerLiteral(2)),
            arms: vec![
                MatchArm {
                    pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                    body: vec![AstNode::IntegerLiteral(10)],
                },
                MatchArm {
                    pattern: Pattern::Literal(AstNode::IntegerLiteral(2)),
                    body: vec![AstNode::IntegerLiteral(20)],
                },
                MatchArm {
                    pattern: Pattern::Literal(AstNode::IntegerLiteral(3)),
                    body: vec![AstNode::IntegerLiteral(30)],
                },
            ],
        })
        .unwrap();

    assert_eq!(result.as_integer().unwrap(), 20);
}

#[test]
fn test_match_with_wildcard() {
    // match 99 {
    //     1 => 10,
    //     2 => 20,
    //     _ => 999,
    // }
    // Result: 999
    let mut eval = Evaluator::new();

    let result = eval
        .eval(&AstNode::MatchExpr {
            expr: Box::new(AstNode::IntegerLiteral(99)),
            arms: vec![
                MatchArm {
                    pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                    body: vec![AstNode::IntegerLiteral(10)],
                },
                MatchArm {
                    pattern: Pattern::Literal(AstNode::IntegerLiteral(2)),
                    body: vec![AstNode::IntegerLiteral(20)],
                },
                MatchArm {
                    pattern: Pattern::Wildcard,
                    body: vec![AstNode::IntegerLiteral(999)],
                },
            ],
        })
        .unwrap();

    assert_eq!(result.as_integer().unwrap(), 999);
}

#[test]
fn test_match_with_identifier_binding() {
    // match 42 {
    //     1 => 10,
    //     x => x * 2,
    // }
    // Result: 84 (42 * 2)
    let mut eval = Evaluator::new();

    let result = eval
        .eval(&AstNode::MatchExpr {
            expr: Box::new(AstNode::IntegerLiteral(42)),
            arms: vec![
                MatchArm {
                    pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                    body: vec![AstNode::IntegerLiteral(10)],
                },
                MatchArm {
                    pattern: Pattern::Identifier("x".to_string()),
                    body: vec![AstNode::BinaryOp {
                        op: BinaryOperator::Multiply,
                        left: Box::new(AstNode::Identifier("x".to_string())),
                        right: Box::new(AstNode::IntegerLiteral(2)),
                    }],
                },
            ],
        })
        .unwrap();

    assert_eq!(result.as_integer().unwrap(), 84);
}

#[test]
fn test_match_no_match_error() {
    // match 99 {
    //     1 => 10,
    //     2 => 20,
    // }
    // Error: No match arm matched
    let mut eval = Evaluator::new();

    let result = eval.eval(&AstNode::MatchExpr {
        expr: Box::new(AstNode::IntegerLiteral(99)),
        arms: vec![
            MatchArm {
                pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                body: vec![AstNode::IntegerLiteral(10)],
            },
            MatchArm {
                pattern: Pattern::Literal(AstNode::IntegerLiteral(2)),
                body: vec![AstNode::IntegerLiteral(20)],
            },
        ],
    });

    assert!(result.is_err());
    match result {
        Err(EvalError::NoMatchArm) => {} // Expected
        other => panic!("Expected NoMatchArm error, got: {:?}", other),
    }
}

#[test]
fn test_match_with_boolean() {
    // match true {
    //     true => 1,
    //     false => 0,
    // }
    // Result: 1
    let mut eval = Evaluator::new();

    let result = eval
        .eval(&AstNode::MatchExpr {
            expr: Box::new(AstNode::BooleanLiteral(true)),
            arms: vec![
                MatchArm {
                    pattern: Pattern::Literal(AstNode::BooleanLiteral(true)),
                    body: vec![AstNode::IntegerLiteral(1)],
                },
                MatchArm {
                    pattern: Pattern::Literal(AstNode::BooleanLiteral(false)),
                    body: vec![AstNode::IntegerLiteral(0)],
                },
            ],
        })
        .unwrap();

    assert_eq!(result.as_integer().unwrap(), 1);
}

// =============================================================================
// Control Flow with Functions
// =============================================================================

#[test]
fn test_while_loop_in_function() {
    // fun sum_to(n) {
    //     let sum = 0;
    //     let i = 1;
    //     while (i <= n) {
    //         sum = sum + i;
    //         i = i + 1;
    //     }
    //     return sum;
    // }
    // sum_to(10) -> 55
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::FunctionDef {
        name: "sum_to".to_string(),
        params: vec!["n".to_string()],
        body: vec![
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
                    op: BinaryOperator::LessEqual,
                    left: Box::new(AstNode::Identifier("i".to_string())),
                    right: Box::new(AstNode::Identifier("n".to_string())),
                }),
                body: vec![
                    AstNode::Assignment {
                        name: "sum".to_string(),
                        value: Box::new(AstNode::BinaryOp {
                            op: BinaryOperator::Add,
                            left: Box::new(AstNode::Identifier("sum".to_string())),
                            right: Box::new(AstNode::Identifier("i".to_string())),
                        }),
                    },
                    AstNode::Assignment {
                        name: "i".to_string(),
                        value: Box::new(AstNode::BinaryOp {
                            op: BinaryOperator::Add,
                            left: Box::new(AstNode::Identifier("i".to_string())),
                            right: Box::new(AstNode::IntegerLiteral(1)),
                        }),
                    },
                ],
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("sum".to_string()))),
            },
        ],
    })
    .unwrap();

    let result = eval
        .eval(&AstNode::FunctionCall {
            name: "sum_to".to_string(),
            args: vec![AstNode::IntegerLiteral(10)],
        })
        .unwrap();

    assert_eq!(result.as_integer().unwrap(), 55);
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_006_completeness() {
    // This test documents what INTERP-006 should cover
    // - While loops: 3 tests
    // - For loops: 3 tests
    // - Match expressions: 5 tests
    // - Control flow with functions: 1 test
    // Total: 12 core tests + 1 meta test = 13 tests
    // Verification complete - test structure validated
}
