// INTERP-063 (JIT-010): Array Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement basic array/vector support in JIT compiler
//
// What we need to support:
// 1. Array literals: [1, 2, 3]
// 2. Array indexing: arr[0]
// 3. Array length (basic support)
// 4. Arrays as function parameters
//
// Why this is critical:
// - Arrays are fundamental data structures
// - Required for real-world programs
// - Foundation for more complex collections
//
// Implementation strategy:
// - Start with fixed-size arrays (stack-allocated)
// - Support integer arrays only (MVP)
// - Add heap allocation in future iterations
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Create simple array literal
///
/// Validates: let arr = [1, 2, 3]; return arr[0];
#[test]
fn test_compile_array_literal_and_index() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let arr = [1, 2, 3]; return arr[0]; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "arr".to_string(),
                value: Box::new(AstNode::VectorLiteral {
                    elements: vec![
                        AstNode::IntegerLiteral(1),
                        AstNode::IntegerLiteral(2),
                        AstNode::IntegerLiteral(3),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("arr".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(0)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile array literal and indexing");

    assert_eq!(main(), 1, "arr[0] should return 1");
}

/// Test: Access multiple array elements
///
/// Validates: let arr = [10, 20, 30]; return arr[1] + arr[2];
#[test]
fn test_compile_array_multiple_access() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let arr = [10, 20, 30]; return arr[1] + arr[2]; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "arr".to_string(),
                value: Box::new(AstNode::VectorLiteral {
                    elements: vec![
                        AstNode::IntegerLiteral(10),
                        AstNode::IntegerLiteral(20),
                        AstNode::IntegerLiteral(30),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::IndexAccess {
                        expr: Box::new(AstNode::Identifier("arr".to_string())),
                        index: Box::new(AstNode::IntegerLiteral(1)),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::IndexAccess {
                        expr: Box::new(AstNode::Identifier("arr".to_string())),
                        index: Box::new(AstNode::IntegerLiteral(2)),
                    }),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile multiple array accesses");

    assert_eq!(main(), 50, "arr[1] + arr[2] should be 20 + 30 = 50");
}

/// Test: Array with computed index
///
/// Validates: let arr = [5, 10, 15, 20]; let i = 2; return arr[i];
#[test]
fn test_compile_array_computed_index() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let arr = [5, 10, 15, 20]; let i = 2; return arr[i]; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "arr".to_string(),
                value: Box::new(AstNode::VectorLiteral {
                    elements: vec![
                        AstNode::IntegerLiteral(5),
                        AstNode::IntegerLiteral(10),
                        AstNode::IntegerLiteral(15),
                        AstNode::IntegerLiteral(20),
                    ],
                }),
            },
            AstNode::LetDecl {
                name: "i".to_string(),
                value: Box::new(AstNode::IntegerLiteral(2)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("arr".to_string())),
                    index: Box::new(AstNode::Identifier("i".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile array with computed index");

    assert_eq!(main(), 15, "arr[i] where i=2 should return 15");
}

/// Test: Array in loop
///
/// Validates: let arr = [1, 2, 3, 4]; let sum = 0; for i in 0..4 { sum = sum + arr[i]; } return sum;
#[test]
fn test_compile_array_in_loop() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let arr = [1, 2, 3, 4];
    //     let sum = 0;
    //     for i in 0..4 { sum = sum + arr[i]; }
    //     return sum;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "arr".to_string(),
                value: Box::new(AstNode::VectorLiteral {
                    elements: vec![
                        AstNode::IntegerLiteral(1),
                        AstNode::IntegerLiteral(2),
                        AstNode::IntegerLiteral(3),
                        AstNode::IntegerLiteral(4),
                    ],
                }),
            },
            AstNode::LetDecl {
                name: "sum".to_string(),
                value: Box::new(AstNode::IntegerLiteral(0)),
            },
            AstNode::ForLoop {
                var: "i".to_string(),
                iterable: Box::new(AstNode::Range {
                    start: Box::new(AstNode::IntegerLiteral(0)),
                    end: Box::new(AstNode::IntegerLiteral(4)),
                }),
                body: vec![AstNode::Assignment {
                    name: "sum".to_string(),
                    value: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("sum".to_string())),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::IndexAccess {
                            expr: Box::new(AstNode::Identifier("arr".to_string())),
                            index: Box::new(AstNode::Identifier("i".to_string())),
                        }),
                    }),
                }],
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("sum".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile array iteration");

    assert_eq!(main(), 10, "Sum of [1,2,3,4] should be 10");
}

/// Test: Empty array
///
/// Validates: let arr = []; (for future - currently marked as ignored)
#[test]
#[ignore = "Empty arrays require special handling - future work"]
fn test_compile_empty_array() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let arr = []; return 0; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "arr".to_string(),
                value: Box::new(AstNode::VectorLiteral { elements: vec![] }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(0))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile empty array");

    assert_eq!(main(), 0);
}
