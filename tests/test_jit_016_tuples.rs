// INTERP-068 (JIT-015): Tuple Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement tuple support in JIT compiler
//
// What we need to support:
// 1. Tuple literals: (1, 2, 3)
// 2. Tuple field access: tuple.0, tuple.1
// 3. Tuples in variables
// 4. Heterogeneous tuples: (42, 3.14, "hello")
// 5. Tuple assignment
//
// Why this is critical:
// - Tuples enable multiple return values
// - Foundation for more complex data structures
// - Common pattern in functional programming
// - Simpler than full structs
//
// Implementation strategy:
// - Stack-allocate tuples like arrays
// - Field access via computed offsets
// - Return pointer to tuple (like arrays/strings)
// - Support mixed types in single tuple
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::AstNode;
use ruchyruchy::jit::JitCompiler;

/// Test: Simple tuple literal with integers
///
/// Validates: let t = (1, 2, 3); return t;
#[test]
fn test_compile_tuple_literal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let t = (1, 2, 3); return t; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "t".to_string(),
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![
                        AstNode::IntegerLiteral(1),
                        AstNode::IntegerLiteral(2),
                        AstNode::IntegerLiteral(3),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("t".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile tuple literal");

    let result = main();
    // Result should be pointer to tuple (non-null)
    assert_ne!(result, 0, "Tuple pointer should be non-null");
}

/// Test: Tuple field access
///
/// Validates: let t = (10, 20, 30); return t.1; (should be 20)
#[test]
fn test_compile_tuple_field_access() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let t = (10, 20, 30); return t.1; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "t".to_string(),
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![
                        AstNode::IntegerLiteral(10),
                        AstNode::IntegerLiteral(20),
                        AstNode::IntegerLiteral(30),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FieldAccess {
                    expr: Box::new(AstNode::Identifier("t".to_string())),
                    field: "1".to_string(), // Field access by index
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile tuple field access");

    assert_eq!(main(), 20, "t.1 should return 20");
}

/// Test: Tuple with multiple field accesses
///
/// Validates: let t = (5, 10, 15); return t.0 + t.2; (should be 20)
#[test]
fn test_compile_tuple_multiple_access() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let t = (5, 10, 15); return t.0 + t.2; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "t".to_string(),
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
                    left: Box::new(AstNode::FieldAccess {
                        expr: Box::new(AstNode::Identifier("t".to_string())),
                        field: "0".to_string(),
                    }),
                    op: ruchyruchy::interpreter::parser::BinaryOperator::Add,
                    right: Box::new(AstNode::FieldAccess {
                        expr: Box::new(AstNode::Identifier("t".to_string())),
                        field: "2".to_string(),
                    }),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile multiple tuple access");

    assert_eq!(main(), 20, "t.0 + t.2 should be 5 + 15 = 20");
}

/// Test: Two-element tuple (pair)
///
/// Validates: let pair = (42, 100); return pair.0; (should be 42)
#[test]
fn test_compile_tuple_pair() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let pair = (42, 100); return pair.0; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "pair".to_string(),
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![AstNode::IntegerLiteral(42), AstNode::IntegerLiteral(100)],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FieldAccess {
                    expr: Box::new(AstNode::Identifier("pair".to_string())),
                    field: "0".to_string(),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile tuple pair");

    assert_eq!(main(), 42, "pair.0 should be 42");
}

/// Test: Heterogeneous tuple (mixed types)
///
/// Validates: let mixed = (42, 3.5); return mixed.0; (should be 42)
///
/// Note: This requires storing different types in the same tuple
#[test]
#[ignore = "Heterogeneous tuples require type metadata - future work"]
fn test_compile_heterogeneous_tuple() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let mixed = (42, 3.5); return mixed.0; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "mixed".to_string(),
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![AstNode::IntegerLiteral(42), AstNode::FloatLiteral(3.5)],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FieldAccess {
                    expr: Box::new(AstNode::Identifier("mixed".to_string())),
                    field: "0".to_string(),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile heterogeneous tuple");

    assert_eq!(main(), 42, "mixed.0 should be 42");
}

/// Test: Nested tuple access
///
/// Validates: let t = ((1, 2), (3, 4)); return t.0.1; (should be 2)
#[test]
#[ignore = "Nested tuples require recursive field access - future work"]
fn test_compile_nested_tuple() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let t = ((1, 2), (3, 4)); return t.0.1; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "t".to_string(),
                value: Box::new(AstNode::TupleLiteral {
                    elements: vec![
                        AstNode::TupleLiteral {
                            elements: vec![AstNode::IntegerLiteral(1), AstNode::IntegerLiteral(2)],
                        },
                        AstNode::TupleLiteral {
                            elements: vec![AstNode::IntegerLiteral(3), AstNode::IntegerLiteral(4)],
                        },
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FieldAccess {
                    expr: Box::new(AstNode::FieldAccess {
                        expr: Box::new(AstNode::Identifier("t".to_string())),
                        field: "0".to_string(),
                    }),
                    field: "1".to_string(),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile nested tuple");

    assert_eq!(main(), 2, "t.0.1 should be 2");
}
