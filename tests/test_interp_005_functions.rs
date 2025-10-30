// INTERP-005: Function Calls & Recursion - RED Phase
// Tests for function call semantics, argument passing, return values, and recursion

use ruchyruchy::interpreter::evaluator::{Evaluator, EvalError};
use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::interpreter::value::Value;

// =============================================================================
// Simple Function Calls
// =============================================================================

#[test]
fn test_simple_function_call_no_args() {
    // fun get_five() { return 5; }
    // get_five()
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "get_five".to_string(),
        params: vec![],
        body: vec![AstNode::Return {
            value: Some(Box::new(AstNode::IntegerLiteral(5))),
        }],
    };

    let func_call = AstNode::FunctionCall {
        name: "get_five".to_string(),
        args: vec![],
    };

    // Register function
    eval.eval(&func_def).unwrap();

    // Call function
    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 5);
}

#[test]
fn test_function_call_with_one_arg() {
    // fun double(x) { return x * 2; }
    // double(21)
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "double".to_string(),
        params: vec!["x".to_string()],
        body: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(AstNode::Identifier("x".to_string())),
                right: Box::new(AstNode::IntegerLiteral(2)),
            })),
        }],
    };

    let func_call = AstNode::FunctionCall {
        name: "double".to_string(),
        args: vec![AstNode::IntegerLiteral(21)],
    };

    eval.eval(&func_def).unwrap();
    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 42);
}

#[test]
fn test_function_call_with_multiple_args() {
    // fun add(a, b) { return a + b; }
    // add(10, 32)
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "add".to_string(),
        params: vec!["a".to_string(), "b".to_string()],
        body: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(AstNode::Identifier("a".to_string())),
                right: Box::new(AstNode::Identifier("b".to_string())),
            })),
        }],
    };

    let func_call = AstNode::FunctionCall {
        name: "add".to_string(),
        args: vec![AstNode::IntegerLiteral(10), AstNode::IntegerLiteral(32)],
    };

    eval.eval(&func_def).unwrap();
    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 42);
}

#[test]
fn test_function_call_with_expression_args() {
    // fun multiply(x, y) { return x * y; }
    // multiply(2 + 3, 8)
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "multiply".to_string(),
        params: vec!["x".to_string(), "y".to_string()],
        body: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(AstNode::Identifier("x".to_string())),
                right: Box::new(AstNode::Identifier("y".to_string())),
            })),
        }],
    };

    let func_call = AstNode::FunctionCall {
        name: "multiply".to_string(),
        args: vec![
            AstNode::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(AstNode::IntegerLiteral(2)),
                right: Box::new(AstNode::IntegerLiteral(3)),
            },
            AstNode::IntegerLiteral(8),
        ],
    };

    eval.eval(&func_def).unwrap();
    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 40); // (2 + 3) * 8 = 40
}

// =============================================================================
// Return Values
// =============================================================================

#[test]
fn test_function_with_implicit_return() {
    // fun get_ten() { 10 }  // Last expression is implicit return
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "get_ten".to_string(),
        params: vec![],
        body: vec![AstNode::IntegerLiteral(10)],
    };

    let func_call = AstNode::FunctionCall {
        name: "get_ten".to_string(),
        args: vec![],
    };

    eval.eval(&func_def).unwrap();
    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 10);
}

#[test]
fn test_function_with_explicit_return() {
    // fun get_twenty() { return 20; }
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "get_twenty".to_string(),
        params: vec![],
        body: vec![AstNode::Return {
            value: Some(Box::new(AstNode::IntegerLiteral(20))),
        }],
    };

    let func_call = AstNode::FunctionCall {
        name: "get_twenty".to_string(),
        args: vec![],
    };

    eval.eval(&func_def).unwrap();
    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 20);
}

#[test]
fn test_function_with_early_return() {
    // fun early(x) {
    //     if (x > 5) { return 100; }
    //     return 1;
    // }
    // early(10) -> 100, early(3) -> 1
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "early".to_string(),
        params: vec!["x".to_string()],
        body: vec![
            AstNode::IfExpr {
                condition: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::GreaterThan,
                    left: Box::new(AstNode::Identifier("x".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(5)),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::IntegerLiteral(100))),
                }],
                else_branch: None,
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    // Test early(10)
    eval.eval(&func_def).unwrap();
    let func_call_10 = AstNode::FunctionCall {
        name: "early".to_string(),
        args: vec![AstNode::IntegerLiteral(10)],
    };
    let result_10 = eval.eval(&func_call_10).unwrap();
    assert_eq!(result_10.as_integer().unwrap(), 100);

    // Test early(3)
    let func_call_3 = AstNode::FunctionCall {
        name: "early".to_string(),
        args: vec![AstNode::IntegerLiteral(3)],
    };
    let result_3 = eval.eval(&func_call_3).unwrap();
    assert_eq!(result_3.as_integer().unwrap(), 1);
}

#[test]
fn test_function_with_no_return() {
    // fun no_return() { let x = 5; }  // Returns nil/unit
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "no_return".to_string(),
        params: vec![],
        body: vec![AstNode::LetDecl {
            name: "x".to_string(),
            value: Box::new(AstNode::IntegerLiteral(5)),
        }],
    };

    let func_call = AstNode::FunctionCall {
        name: "no_return".to_string(),
        args: vec![],
    };

    eval.eval(&func_def).unwrap();
    let result = eval.eval(&func_call).unwrap();
    // Should return nil/unit value
    assert!(result.is_nil());
}

// =============================================================================
// Recursion
// =============================================================================

#[test]
fn test_recursive_factorial() {
    // fun factorial(n) {
    //     if (n <= 1) { return 1; }
    //     return n * factorial(n - 1);
    // }
    // factorial(5) -> 120
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "factorial".to_string(),
        params: vec!["n".to_string()],
        body: vec![
            AstNode::IfExpr {
                condition: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::LessEqual,
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(1)),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::IntegerLiteral(1))),
                }],
                else_branch: None,
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    right: Box::new(AstNode::FunctionCall {
                        name: "factorial".to_string(),
                        args: vec![AstNode::BinaryOp {
                            op: BinaryOperator::Subtract,
                            left: Box::new(AstNode::Identifier("n".to_string())),
                            right: Box::new(AstNode::IntegerLiteral(1)),
                        }],
                    }),
                })),
            },
        ],
    };

    eval.eval(&func_def).unwrap();

    let func_call = AstNode::FunctionCall {
        name: "factorial".to_string(),
        args: vec![AstNode::IntegerLiteral(5)],
    };

    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 120); // 5! = 120
}

#[test]
fn test_recursive_fibonacci() {
    // fun fib(n) {
    //     if (n <= 1) { return n; }
    //     return fib(n - 1) + fib(n - 2);
    // }
    // fib(6) -> 8
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "fib".to_string(),
        params: vec!["n".to_string()],
        body: vec![
            AstNode::IfExpr {
                condition: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::LessEqual,
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(1)),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::Identifier("n".to_string()))),
                }],
                else_branch: None,
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(AstNode::FunctionCall {
                        name: "fib".to_string(),
                        args: vec![AstNode::BinaryOp {
                            op: BinaryOperator::Subtract,
                            left: Box::new(AstNode::Identifier("n".to_string())),
                            right: Box::new(AstNode::IntegerLiteral(1)),
                        }],
                    }),
                    right: Box::new(AstNode::FunctionCall {
                        name: "fib".to_string(),
                        args: vec![AstNode::BinaryOp {
                            op: BinaryOperator::Subtract,
                            left: Box::new(AstNode::Identifier("n".to_string())),
                            right: Box::new(AstNode::IntegerLiteral(2)),
                        }],
                    }),
                })),
            },
        ],
    };

    eval.eval(&func_def).unwrap();

    let func_call = AstNode::FunctionCall {
        name: "fib".to_string(),
        args: vec![AstNode::IntegerLiteral(6)],
    };

    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 8); // fib(6) = 8
}

#[test]
fn test_tail_recursion() {
    // fun sum_to(n, acc) {
    //     if (n == 0) { return acc; }
    //     return sum_to(n - 1, acc + n);
    // }
    // sum_to(10, 0) -> 55
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "sum_to".to_string(),
        params: vec!["n".to_string(), "acc".to_string()],
        body: vec![
            AstNode::IfExpr {
                condition: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Equal,
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(0)),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::Identifier("acc".to_string()))),
                }],
                else_branch: None,
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FunctionCall {
                    name: "sum_to".to_string(),
                    args: vec![
                        AstNode::BinaryOp {
                            op: BinaryOperator::Subtract,
                            left: Box::new(AstNode::Identifier("n".to_string())),
                            right: Box::new(AstNode::IntegerLiteral(1)),
                        },
                        AstNode::BinaryOp {
                            op: BinaryOperator::Add,
                            left: Box::new(AstNode::Identifier("acc".to_string())),
                            right: Box::new(AstNode::Identifier("n".to_string())),
                        },
                    ],
                })),
            },
        ],
    };

    eval.eval(&func_def).unwrap();

    let func_call = AstNode::FunctionCall {
        name: "sum_to".to_string(),
        args: vec![AstNode::IntegerLiteral(10), AstNode::IntegerLiteral(0)],
    };

    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 55); // 1+2+3+...+10 = 55
}

// =============================================================================
// Mutual Recursion
// =============================================================================

#[test]
fn test_mutual_recursion_is_even_is_odd() {
    // fun is_even(n) {
    //     if (n == 0) { return true; }
    //     return is_odd(n - 1);
    // }
    // fun is_odd(n) {
    //     if (n == 0) { return false; }
    //     return is_even(n - 1);
    // }
    // is_even(4) -> true, is_odd(4) -> false
    let mut eval = Evaluator::new();

    let is_even_def = AstNode::FunctionDef {
        name: "is_even".to_string(),
        params: vec!["n".to_string()],
        body: vec![
            AstNode::IfExpr {
                condition: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Equal,
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(0)),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::BooleanLiteral(true))),
                }],
                else_branch: None,
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FunctionCall {
                    name: "is_odd".to_string(),
                    args: vec![AstNode::BinaryOp {
                        op: BinaryOperator::Subtract,
                        left: Box::new(AstNode::Identifier("n".to_string())),
                        right: Box::new(AstNode::IntegerLiteral(1)),
                    }],
                })),
            },
        ],
    };

    let is_odd_def = AstNode::FunctionDef {
        name: "is_odd".to_string(),
        params: vec!["n".to_string()],
        body: vec![
            AstNode::IfExpr {
                condition: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Equal,
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(0)),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::BooleanLiteral(false))),
                }],
                else_branch: None,
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FunctionCall {
                    name: "is_even".to_string(),
                    args: vec![AstNode::BinaryOp {
                        op: BinaryOperator::Subtract,
                        left: Box::new(AstNode::Identifier("n".to_string())),
                        right: Box::new(AstNode::IntegerLiteral(1)),
                    }],
                })),
            },
        ],
    };

    eval.eval(&is_even_def).unwrap();
    eval.eval(&is_odd_def).unwrap();

    // Test is_even(4)
    let call_is_even_4 = AstNode::FunctionCall {
        name: "is_even".to_string(),
        args: vec![AstNode::IntegerLiteral(4)],
    };
    let result_even_4 = eval.eval(&call_is_even_4).unwrap();
    assert_eq!(result_even_4.as_boolean().unwrap(), true);

    // Test is_odd(4)
    let call_is_odd_4 = AstNode::FunctionCall {
        name: "is_odd".to_string(),
        args: vec![AstNode::IntegerLiteral(4)],
    };
    let result_odd_4 = eval.eval(&call_is_odd_4).unwrap();
    assert_eq!(result_odd_4.as_boolean().unwrap(), false);
}

// =============================================================================
// Stack Overflow Detection
// =============================================================================

#[test]
fn test_stack_overflow_detection() {
    // fun infinite(n) { return infinite(n + 1); }
    // infinite(0) should detect stack overflow
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "infinite".to_string(),
        params: vec!["n".to_string()],
        body: vec![AstNode::Return {
            value: Some(Box::new(AstNode::FunctionCall {
                name: "infinite".to_string(),
                args: vec![AstNode::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(1)),
                }],
            })),
        }],
    };

    eval.eval(&func_def).unwrap();

    let func_call = AstNode::FunctionCall {
        name: "infinite".to_string(),
        args: vec![AstNode::IntegerLiteral(0)],
    };

    let result = eval.eval(&func_call);
    assert!(result.is_err());

    match result {
        Err(EvalError::StackOverflow) => {} // Expected
        other => panic!("Expected StackOverflow error, got: {:?}", other),
    }
}

#[test]
fn test_deep_recursion_within_limit() {
    // Test that reasonable recursion depth works
    // fun count_down(n) {
    //     if (n <= 0) { return 0; }
    //     return count_down(n - 1);
    // }
    // count_down(100) should work
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "count_down".to_string(),
        params: vec!["n".to_string()],
        body: vec![
            AstNode::IfExpr {
                condition: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::LessEqual,
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(0)),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::IntegerLiteral(0))),
                }],
                else_branch: None,
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FunctionCall {
                    name: "count_down".to_string(),
                    args: vec![AstNode::BinaryOp {
                        op: BinaryOperator::Subtract,
                        left: Box::new(AstNode::Identifier("n".to_string())),
                        right: Box::new(AstNode::IntegerLiteral(1)),
                    }],
                })),
            },
        ],
    };

    eval.eval(&func_def).unwrap();

    let func_call = AstNode::FunctionCall {
        name: "count_down".to_string(),
        args: vec![AstNode::IntegerLiteral(100)],
    };

    let result = eval.eval(&func_call).unwrap();
    assert_eq!(result.as_integer().unwrap(), 0);
}

// =============================================================================
// Error Cases
// =============================================================================

#[test]
fn test_undefined_function() {
    let mut eval = Evaluator::new();

    let func_call = AstNode::FunctionCall {
        name: "undefined_func".to_string(),
        args: vec![],
    };

    let result = eval.eval(&func_call);
    assert!(result.is_err());

    match result {
        Err(EvalError::UndefinedFunction { name }) => {
            assert_eq!(name, "undefined_func");
        }
        other => panic!("Expected UndefinedFunction error, got: {:?}", other),
    }
}

#[test]
fn test_argument_count_mismatch() {
    // fun add(a, b) { return a + b; }
    // add(1) -> error (too few args)
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "add".to_string(),
        params: vec!["a".to_string(), "b".to_string()],
        body: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(AstNode::Identifier("a".to_string())),
                right: Box::new(AstNode::Identifier("b".to_string())),
            })),
        }],
    };

    eval.eval(&func_def).unwrap();

    // Too few arguments
    let func_call_too_few = AstNode::FunctionCall {
        name: "add".to_string(),
        args: vec![AstNode::IntegerLiteral(1)],
    };

    let result = eval.eval(&func_call_too_few);
    assert!(result.is_err());

    match result {
        Err(EvalError::ArgumentCountMismatch { expected, actual, .. }) => {
            assert_eq!(expected, 2);
            assert_eq!(actual, 1);
        }
        other => panic!("Expected ArgumentCountMismatch error, got: {:?}", other),
    }
}

#[test]
fn test_argument_count_mismatch_too_many() {
    // fun add(a, b) { return a + b; }
    // add(1, 2, 3) -> error (too many args)
    let mut eval = Evaluator::new();

    let func_def = AstNode::FunctionDef {
        name: "add".to_string(),
        params: vec!["a".to_string(), "b".to_string()],
        body: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(AstNode::Identifier("a".to_string())),
                right: Box::new(AstNode::Identifier("b".to_string())),
            })),
        }],
    };

    eval.eval(&func_def).unwrap();

    // Too many arguments
    let func_call_too_many = AstNode::FunctionCall {
        name: "add".to_string(),
        args: vec![
            AstNode::IntegerLiteral(1),
            AstNode::IntegerLiteral(2),
            AstNode::IntegerLiteral(3),
        ],
    };

    let result = eval.eval(&func_call_too_many);
    assert!(result.is_err());

    match result {
        Err(EvalError::ArgumentCountMismatch { expected, actual, .. }) => {
            assert_eq!(expected, 2);
            assert_eq!(actual, 3);
        }
        other => panic!("Expected ArgumentCountMismatch error, got: {:?}", other),
    }
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_005_completeness() {
    // This test documents what INTERP-005 should cover
    // - Simple function calls: 4 tests
    // - Return values: 4 tests
    // - Recursion: 3 tests
    // - Mutual recursion: 1 test
    // - Stack overflow: 2 tests
    // - Error cases: 3 tests
    // Total: 17 core tests + 1 meta test = 18 tests
    assert!(true);
}
