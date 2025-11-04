// INTERP-075 (JIT-022): vec![] Macro Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement vec![] macro syntax in JIT compiler
//
// What we need to support:
// 1. List form: vec![1, 2, 3] - creates vector with listed elements
// 2. Repeat form: vec![42; 5] - creates vector with value repeated count times
// 3. Empty vector: vec![]
// 4. Using vec![] in expressions and variables
// 5. Index access on vec![] results
//
// Why this is critical:
// - vec![] is idiomatic Rust/Ruchy syntax
// - More ergonomic than [1, 2, 3] for some use cases
// - Repeat form enables convenient initialization
// - Common pattern in modern code
//
// Implementation strategy:
// - List form: Desugar to VectorLiteral (same as [1, 2, 3])
// - Repeat form: Loop to fill array with repeated value
// - Heap-allocate vector like arrays
// - Return pointer to vector
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Empty vec![]
///
/// Validates: let v = vec![]; return v;
#[test]
fn test_compile_vec_macro_empty() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let v = vec![];
    //     return v;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "v".to_string(),
                value: Box::new(AstNode::VecMacro {
                    elements: vec![],
                    repeat_count: None,
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("v".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile empty vec![]");

    let result = main();
    assert_ne!(result, 0, "vec![] should return non-null pointer");
}

/// Test: vec![] list form with elements
///
/// Validates: let v = vec![10, 20, 30]; return v[1];
#[test]
fn test_compile_vec_macro_list_form() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let v = vec![10, 20, 30];
    //     return v[1];
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "v".to_string(),
                value: Box::new(AstNode::VecMacro {
                    elements: vec![
                        AstNode::IntegerLiteral(10),
                        AstNode::IntegerLiteral(20),
                        AstNode::IntegerLiteral(30),
                    ],
                    repeat_count: None,
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("v".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(1)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile vec![] list form");

    assert_eq!(main(), 20, "vec![10, 20, 30][1] should return 20");
}

/// Test: vec![] repeat form
///
/// Validates: let v = vec![42; 5]; return v[3];
#[test]
fn test_compile_vec_macro_repeat_form() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let v = vec![42; 5];
    //     return v[3];
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "v".to_string(),
                value: Box::new(AstNode::VecMacro {
                    elements: vec![AstNode::IntegerLiteral(42)],
                    repeat_count: Some(Box::new(AstNode::IntegerLiteral(5))),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("v".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(3)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile vec![] repeat form");

    assert_eq!(main(), 42, "vec![42; 5][3] should return 42");
}

/// Test: vec![] repeat form with variable count
///
/// Validates: let n = 3; let v = vec![100; n]; return v[2];
#[test]
fn test_compile_vec_macro_repeat_variable_count() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let n = 3;
    //     let v = vec![100; n];
    //     return v[2];
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "n".to_string(),
                value: Box::new(AstNode::IntegerLiteral(3)),
            },
            AstNode::LetDecl {
                name: "v".to_string(),
                value: Box::new(AstNode::VecMacro {
                    elements: vec![AstNode::IntegerLiteral(100)],
                    repeat_count: Some(Box::new(AstNode::Identifier("n".to_string()))),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("v".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(2)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile vec![] repeat with variable count");

    assert_eq!(main(), 100, "vec![100; n][2] should return 100");
}

/// Test: vec![] with computed elements
///
/// Validates: let v = vec![5 + 5, 10 * 2]; return v[0] + v[1];
#[test]
fn test_compile_vec_macro_computed_elements() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let v = vec![5 + 5, 10 * 2];
    //     return v[0] + v[1];
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "v".to_string(),
                value: Box::new(AstNode::VecMacro {
                    elements: vec![
                        AstNode::BinaryOp {
                            left: Box::new(AstNode::IntegerLiteral(5)),
                            op: BinaryOperator::Add,
                            right: Box::new(AstNode::IntegerLiteral(5)),
                        },
                        AstNode::BinaryOp {
                            left: Box::new(AstNode::IntegerLiteral(10)),
                            op: BinaryOperator::Multiply,
                            right: Box::new(AstNode::IntegerLiteral(2)),
                        },
                    ],
                    repeat_count: None,
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::IndexAccess {
                        expr: Box::new(AstNode::Identifier("v".to_string())),
                        index: Box::new(AstNode::IntegerLiteral(0)),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::IndexAccess {
                        expr: Box::new(AstNode::Identifier("v".to_string())),
                        index: Box::new(AstNode::IntegerLiteral(1)),
                    }),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile vec![] with computed elements");

    assert_eq!(main(), 30, "v[0] + v[1] should be 10 + 20 = 30");
}

/// Test: vec![] single element
///
/// Validates: let v = vec![42]; return v[0];
#[test]
fn test_compile_vec_macro_single_element() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let v = vec![42];
    //     return v[0];
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "v".to_string(),
                value: Box::new(AstNode::VecMacro {
                    elements: vec![AstNode::IntegerLiteral(42)],
                    repeat_count: None,
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("v".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(0)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile vec![] with single element");

    assert_eq!(main(), 42, "vec![42][0] should return 42");
}

/// Test: vec![] repeat form with expression
///
/// Validates: let v = vec![10 + 5; 3]; return v[0] + v[1] + v[2];
#[test]
fn test_compile_vec_macro_repeat_expression() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let v = vec![10 + 5; 3];
    //     return v[0] + v[1] + v[2];
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "v".to_string(),
                value: Box::new(AstNode::VecMacro {
                    elements: vec![AstNode::BinaryOp {
                        left: Box::new(AstNode::IntegerLiteral(10)),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::IntegerLiteral(5)),
                    }],
                    repeat_count: Some(Box::new(AstNode::IntegerLiteral(3))),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::IndexAccess {
                            expr: Box::new(AstNode::Identifier("v".to_string())),
                            index: Box::new(AstNode::IntegerLiteral(0)),
                        }),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::IndexAccess {
                            expr: Box::new(AstNode::Identifier("v".to_string())),
                            index: Box::new(AstNode::IntegerLiteral(1)),
                        }),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::IndexAccess {
                        expr: Box::new(AstNode::Identifier("v".to_string())),
                        index: Box::new(AstNode::IntegerLiteral(2)),
                    }),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile vec![] repeat with expression");

    assert_eq!(main(), 45, "v[0] + v[1] + v[2] should be 15 + 15 + 15 = 45");
}

/// Test: vec![] repeat form with zero count
///
/// Validates: let v = vec![42; 0]; return v;
#[test]
fn test_compile_vec_macro_repeat_zero() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let v = vec![42; 0];
    //     return v;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "v".to_string(),
                value: Box::new(AstNode::VecMacro {
                    elements: vec![AstNode::IntegerLiteral(42)],
                    repeat_count: Some(Box::new(AstNode::IntegerLiteral(0))),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("v".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile vec![] repeat with zero count");

    let result = main();
    assert_ne!(result, 0, "vec![42; 0] should return non-null pointer");
}
