// INTERP-072 (JIT-019): Method Call Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement method call support in JIT compiler
//
// What we need to support:
// 1. Method calls: receiver.method(args)
// 2. Desugaring to function calls: method(receiver, args)
// 3. Method calls on variables, literals, and expressions
// 4. Chained method calls (if methods return values)
//
// Why this is critical:
// - Method call syntax is ergonomic and intuitive
// - Common pattern in object-oriented and functional code
// - Enables fluent APIs and method chaining
// - Foundation for more complex OOP features
//
// Implementation strategy:
// - Desugar MethodCall to FunctionCall
// - receiver.method(arg1, arg2) → method(receiver, arg1, arg2)
// - Compile receiver expression first
// - Pass as first argument to method function
// - Compile remaining args and pass in order
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Simple method call with no arguments
///
/// Validates: x.increment() where increment(x) { return x + 1; }
#[test]
fn test_compile_method_call_no_args() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define increment function: fun increment(x) { return x + 1; }
    let increment_fn = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("x".to_string())),
                op: BinaryOperator::Add,
                right: Box::new(AstNode::IntegerLiteral(1)),
            })),
        }],
    };

    let increment: fn(i64) -> i64 = jit
        .compile_function_with_params(&["x".to_string()], &increment_fn)
        .expect("Should compile increment function");

    // Register the increment function for calls
    jit.register_function("increment".to_string(), increment as *const u8);

    // Function: fun main() {
    //     let x = 5;
    //     return x.increment();
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(5)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MethodCall {
                    receiver: Box::new(AstNode::Identifier("x".to_string())),
                    method: "increment".to_string(),
                    args: vec![],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile method call with no args");

    assert_eq!(main(), 6, "x.increment() should return 6");
}

/// Test: Method call with one argument
///
/// Validates: x.add(y) where add(a, b) { return a + b; }
#[test]
fn test_compile_method_call_one_arg() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define add function: fun add(a, b) { return a + b; }
    let add_fn = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("a".to_string())),
                op: BinaryOperator::Add,
                right: Box::new(AstNode::Identifier("b".to_string())),
            })),
        }],
    };

    let add: fn(i64, i64) -> i64 = jit
        .compile_function_with_params(&["a".to_string(), "b".to_string()], &add_fn)
        .expect("Should compile add function");

    // Register the add function for calls
    jit.register_function("add".to_string(), add as *const u8);

    // Function: fun main() {
    //     let x = 10;
    //     let y = 20;
    //     return x.add(y);
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::IntegerLiteral(20)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MethodCall {
                    receiver: Box::new(AstNode::Identifier("x".to_string())),
                    method: "add".to_string(),
                    args: vec![AstNode::Identifier("y".to_string())],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile method call with one arg");

    assert_eq!(main(), 30, "x.add(y) should return 30");
}

/// Test: Method call with multiple arguments
///
/// Validates: x.sum3(y, z) where sum3(a, b, c) { return a + b + c; }
#[test]
fn test_compile_method_call_multiple_args() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define sum3 function: fun sum3(a, b, c) { return a + b + c; }
    let sum3_fn = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                left: Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("a".to_string())),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::Identifier("b".to_string())),
                }),
                op: BinaryOperator::Add,
                right: Box::new(AstNode::Identifier("c".to_string())),
            })),
        }],
    };

    let sum3: fn(i64, i64, i64) -> i64 = jit
        .compile_function_with_params(
            &["a".to_string(), "b".to_string(), "c".to_string()],
            &sum3_fn,
        )
        .expect("Should compile sum3 function");

    // Register the sum3 function for calls
    jit.register_function("sum3".to_string(), sum3 as *const u8);

    // Function: fun main() {
    //     let x = 10;
    //     let y = 20;
    //     let z = 30;
    //     return x.sum3(y, z);
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::IntegerLiteral(20)),
            },
            AstNode::LetDecl {
                name: "z".to_string(),
                value: Box::new(AstNode::IntegerLiteral(30)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MethodCall {
                    receiver: Box::new(AstNode::Identifier("x".to_string())),
                    method: "sum3".to_string(),
                    args: vec![
                        AstNode::Identifier("y".to_string()),
                        AstNode::Identifier("z".to_string()),
                    ],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile method call with multiple args");

    assert_eq!(main(), 60, "x.sum3(y, z) should return 60");
}

/// Test: Method call on literal
///
/// Validates: 5.add(3) where add(a, b) { return a + b; }
#[test]
fn test_compile_method_call_on_literal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define add function: fun add(a, b) { return a + b; }
    let add_fn = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("a".to_string())),
                op: BinaryOperator::Add,
                right: Box::new(AstNode::Identifier("b".to_string())),
            })),
        }],
    };

    let add: fn(i64, i64) -> i64 = jit
        .compile_function_with_params(&["a".to_string(), "b".to_string()], &add_fn)
        .expect("Should compile add function");

    // Register the add function for calls
    jit.register_function("add".to_string(), add as *const u8);

    // Function: fun main() {
    //     return 5.add(3);
    // }
    let body = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::MethodCall {
                receiver: Box::new(AstNode::IntegerLiteral(5)),
                method: "add".to_string(),
                args: vec![AstNode::IntegerLiteral(3)],
            })),
        }],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile method call on literal");

    assert_eq!(main(), 8, "5.add(3) should return 8");
}

/// Test: Method call on expression
///
/// Validates: (x + y).double() where double(a) { return a * 2; }
#[test]
fn test_compile_method_call_on_expression() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define double function: fun double(a) { return a * 2; }
    let double_fn = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("a".to_string())),
                op: BinaryOperator::Multiply,
                right: Box::new(AstNode::IntegerLiteral(2)),
            })),
        }],
    };

    let double: fn(i64) -> i64 = jit
        .compile_function_with_params(&["a".to_string()], &double_fn)
        .expect("Should compile double function");

    // Register the double function for calls
    jit.register_function("double".to_string(), double as *const u8);

    // Function: fun main() {
    //     let x = 10;
    //     let y = 5;
    //     return (x + y).double();
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::IntegerLiteral(5)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MethodCall {
                    receiver: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("x".to_string())),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::Identifier("y".to_string())),
                    }),
                    method: "double".to_string(),
                    args: vec![],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile method call on expression");

    assert_eq!(main(), 30, "(x + y).double() should return 30");
}

/// Test: Chained method calls
///
/// Validates: x.add(y).double() where add/double return values
#[test]
fn test_compile_chained_method_calls() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define add function: fun add(a, b) { return a + b; }
    let add_fn = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("a".to_string())),
                op: BinaryOperator::Add,
                right: Box::new(AstNode::Identifier("b".to_string())),
            })),
        }],
    };

    let add: fn(i64, i64) -> i64 = jit
        .compile_function_with_params(&["a".to_string(), "b".to_string()], &add_fn)
        .expect("Should compile add function");

    // Register the add function for calls
    jit.register_function("add".to_string(), add as *const u8);

    // Define double function: fun double(a) { return a * 2; }
    let double_fn = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("a".to_string())),
                op: BinaryOperator::Multiply,
                right: Box::new(AstNode::IntegerLiteral(2)),
            })),
        }],
    };

    let double: fn(i64) -> i64 = jit
        .compile_function_with_params(&["a".to_string()], &double_fn)
        .expect("Should compile double function");

    // Register the double function for calls
    jit.register_function("double".to_string(), double as *const u8);

    // Function: fun main() {
    //     let x = 10;
    //     let y = 5;
    //     return x.add(y).double();
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::IntegerLiteral(5)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MethodCall {
                    receiver: Box::new(AstNode::MethodCall {
                        receiver: Box::new(AstNode::Identifier("x".to_string())),
                        method: "add".to_string(),
                        args: vec![AstNode::Identifier("y".to_string())],
                    }),
                    method: "double".to_string(),
                    args: vec![],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile chained method calls");

    assert_eq!(main(), 30, "x.add(y).double() should return 30");
}

/// Test: Method call in arithmetic expression
///
/// Validates: x.double() + y.double() where double(a) { return a * 2; }
#[test]
fn test_compile_method_call_in_expression() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define double function: fun double(a) { return a * 2; }
    let double_fn = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("a".to_string())),
                op: BinaryOperator::Multiply,
                right: Box::new(AstNode::IntegerLiteral(2)),
            })),
        }],
    };

    let double: fn(i64) -> i64 = jit
        .compile_function_with_params(&["a".to_string()], &double_fn)
        .expect("Should compile double function");

    // Register the double function for calls
    jit.register_function("double".to_string(), double as *const u8);

    // Function: fun main() {
    //     let x = 10;
    //     let y = 5;
    //     return x.double() + y.double();
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::IntegerLiteral(5)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::MethodCall {
                        receiver: Box::new(AstNode::Identifier("x".to_string())),
                        method: "double".to_string(),
                        args: vec![],
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::MethodCall {
                        receiver: Box::new(AstNode::Identifier("y".to_string())),
                        method: "double".to_string(),
                        args: vec![],
                    }),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile method calls in expression");

    assert_eq!(main(), 30, "x.double() + y.double() should return 30");
}
