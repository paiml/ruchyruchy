// INTERP-064 (JIT-011): Array Mutation in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement array element mutation via CompoundAssignment in JIT compiler
//
// What we need to support:
// 1. Compound assignment: arr[0] += 1
// 2. Computed index: arr[i] *= 2
// 3. Various operators: +=, -=, *=, /=
// 4. Array mutations in loops
//
// Why this is critical:
// - Enables in-place array modifications
// - Required for sorting, filtering, transformations
// - Foundation for mutable data structures
//
// Implementation strategy:
// - Implement CompoundAssignment with IndexAccess on LHS
// - Reuse array address calculation from IndexAccess
// - Load current value, apply operation, store result
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Simple array element compound assignment
///
/// Validates: let arr = [1, 2, 3]; arr[1] += 40; return arr[1];
#[test]
fn test_compile_array_element_compound_assignment() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let arr = [1, 2, 3];
    //     arr[1] += 40;  // 2 + 40 = 42
    //     return arr[1];
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
                    ],
                }),
            },
            // arr[1] += 40
            AstNode::CompoundAssignment {
                lhs: Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("arr".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(1)),
                }),
                op: BinaryOperator::Add,
                rhs: Box::new(AstNode::IntegerLiteral(40)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("arr".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(1)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile array element compound assignment");

    assert_eq!(main(), 42, "arr[1] should be 42 (2 + 40)");
}

/// Test: Array element compound assignment with computed index
///
/// Validates: let arr = [10, 20, 30]; let i = 2; arr[i] += 70; return arr[2];
#[test]
fn test_compile_array_compound_assignment_computed_index() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let arr = [10, 20, 30];
    //     let i = 2;
    //     arr[i] += 70;  // 30 + 70 = 100
    //     return arr[2];
    // }
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
            AstNode::LetDecl {
                name: "i".to_string(),
                value: Box::new(AstNode::IntegerLiteral(2)),
            },
            // arr[i] += 70
            AstNode::CompoundAssignment {
                lhs: Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("arr".to_string())),
                    index: Box::new(AstNode::Identifier("i".to_string())),
                }),
                op: BinaryOperator::Add,
                rhs: Box::new(AstNode::IntegerLiteral(70)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("arr".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(2)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile array compound assignment with computed index");

    assert_eq!(main(), 100, "arr[2] should be 100 (30 + 70)");
}

/// Test: Array element multiply assignment
///
/// Validates: let arr = [5, 10, 15]; arr[1] *= 11; return arr[1];
#[test]
fn test_compile_array_multiply_assignment() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let arr = [5, 10, 15];
    //     arr[1] *= 11;  // 10 * 11 = 110
    //     return arr[1];
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "arr".to_string(),
                value: Box::new(AstNode::VectorLiteral {
                    elements: vec![
                        AstNode::IntegerLiteral(5),
                        AstNode::IntegerLiteral(10),
                        AstNode::IntegerLiteral(15),
                    ],
                }),
            },
            // arr[1] *= 11
            AstNode::CompoundAssignment {
                lhs: Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("arr".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(1)),
                }),
                op: BinaryOperator::Multiply,
                rhs: Box::new(AstNode::IntegerLiteral(11)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("arr".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(1)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile array multiply assignment");

    assert_eq!(main(), 110, "arr[1] should be 110 (10 * 11)");
}

/// Test: Array mutation in loop with compound assignment
///
/// Validates: let arr = [1, 2, 3]; for i in 0..3 { arr[i] *= 2; } return arr[0] + arr[1] + arr[2];
#[test]
fn test_compile_array_mutation_in_loop() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let arr = [1, 2, 3];
    //     for i in 0..3 {
    //         arr[i] *= 2;  // Double each element
    //     }
    //     return arr[0] + arr[1] + arr[2];
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
                    ],
                }),
            },
            AstNode::ForLoop {
                var: "i".to_string(),
                iterable: Box::new(AstNode::Range {
                    start: Box::new(AstNode::IntegerLiteral(0)),
                    end: Box::new(AstNode::IntegerLiteral(3)),
                }),
                body: vec![
                    // arr[i] *= 2
                    AstNode::CompoundAssignment {
                        lhs: Box::new(AstNode::IndexAccess {
                            expr: Box::new(AstNode::Identifier("arr".to_string())),
                            index: Box::new(AstNode::Identifier("i".to_string())),
                        }),
                        op: BinaryOperator::Multiply,
                        rhs: Box::new(AstNode::IntegerLiteral(2)),
                    },
                ],
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::IndexAccess {
                            expr: Box::new(AstNode::Identifier("arr".to_string())),
                            index: Box::new(AstNode::IntegerLiteral(0)),
                        }),
                        op: BinaryOperator::Add,
                        right: Box::new(AstNode::IndexAccess {
                            expr: Box::new(AstNode::Identifier("arr".to_string())),
                            index: Box::new(AstNode::IntegerLiteral(1)),
                        }),
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
        .expect("Should compile array mutation in loop");

    // After loop: arr = [2, 4, 6], sum = 12
    assert_eq!(
        main(),
        12,
        "Sum should be 12 after doubling each element ([2,4,6])"
    );
}
