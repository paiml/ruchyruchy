// INTERP-070 (JIT-017): HashMap Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement HashMap literal support in JIT compiler
//
// What we need to support:
// 1. HashMap literals: {key1: value1, key2: value2}
// 2. Empty HashMaps: {}
// 3. HashMap insertion (via index): map[key] = value
// 4. HashMap lookup (via index): map[key]
// 5. HashMaps in variables
//
// Why this is critical:
// - HashMaps are fundamental for key-value storage
// - Completes basic collection types (array, tuple, struct, hashmap)
// - Common pattern in all programming languages
// - Foundation for more complex data structures
//
// Implementation strategy:
// - Heap-allocate HashMap structure
// - Simple linear search for MVP (not efficient but correct)
// - Store as array of (key, value) pairs
// - Return pointer to HashMap (like arrays/strings/structs)
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::AstNode;
use ruchyruchy::jit::JitCompiler;

/// Test: Empty HashMap literal
///
/// Validates: let m = {}; return m;
#[test]
fn test_compile_empty_hashmap() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let m = {}; return m; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "m".to_string(),
                value: Box::new(AstNode::HashMapLiteral { pairs: vec![] }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("m".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile empty hashmap");

    let result = main();
    // Result should be pointer to HashMap (non-null)
    assert_ne!(result, 0, "HashMap pointer should be non-null");
}

/// Test: HashMap literal with integer keys and values
///
/// Validates: let m = {1: 10, 2: 20}; return m;
#[test]
fn test_compile_hashmap_literal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let m = {1: 10, 2: 20}; return m; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "m".to_string(),
                value: Box::new(AstNode::HashMapLiteral {
                    pairs: vec![
                        (AstNode::IntegerLiteral(1), AstNode::IntegerLiteral(10)),
                        (AstNode::IntegerLiteral(2), AstNode::IntegerLiteral(20)),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("m".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile hashmap literal");

    let result = main();
    assert_ne!(result, 0, "HashMap pointer should be non-null");
}

/// Test: HashMap lookup via index access
///
/// Validates: let m = {1: 100, 2: 200}; return m[1]; (should be 100)
#[test]
fn test_compile_hashmap_lookup() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let m = {1: 100, 2: 200}; return m[1]; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "m".to_string(),
                value: Box::new(AstNode::HashMapLiteral {
                    pairs: vec![
                        (AstNode::IntegerLiteral(1), AstNode::IntegerLiteral(100)),
                        (AstNode::IntegerLiteral(2), AstNode::IntegerLiteral(200)),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("m".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(1)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile hashmap lookup");

    assert_eq!(main(), 100, "m[1] should return 100");
}

/// Test: HashMap lookup with multiple keys
///
/// Validates: let m = {10: 1, 20: 2, 30: 3}; return m[20]; (should be 2)
#[test]
fn test_compile_hashmap_multiple_lookup() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let m = {10: 1, 20: 2, 30: 3}; return m[20]; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "m".to_string(),
                value: Box::new(AstNode::HashMapLiteral {
                    pairs: vec![
                        (AstNode::IntegerLiteral(10), AstNode::IntegerLiteral(1)),
                        (AstNode::IntegerLiteral(20), AstNode::IntegerLiteral(2)),
                        (AstNode::IntegerLiteral(30), AstNode::IntegerLiteral(3)),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("m".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(20)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile hashmap multiple lookup");

    assert_eq!(main(), 2, "m[20] should return 2");
}

/// Test: HashMap with computed values
///
/// Validates: let m = {1: 5 + 5, 2: 10 * 2}; return m[1]; (should be 10)
#[test]
fn test_compile_hashmap_computed_values() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let m = {1: 5 + 5, 2: 10 * 2}; return m[1]; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "m".to_string(),
                value: Box::new(AstNode::HashMapLiteral {
                    pairs: vec![
                        (
                            AstNode::IntegerLiteral(1),
                            AstNode::BinaryOp {
                                left: Box::new(AstNode::IntegerLiteral(5)),
                                op: ruchyruchy::interpreter::parser::BinaryOperator::Add,
                                right: Box::new(AstNode::IntegerLiteral(5)),
                            },
                        ),
                        (
                            AstNode::IntegerLiteral(2),
                            AstNode::BinaryOp {
                                left: Box::new(AstNode::IntegerLiteral(10)),
                                op: ruchyruchy::interpreter::parser::BinaryOperator::Multiply,
                                right: Box::new(AstNode::IntegerLiteral(2)),
                            },
                        ),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("m".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(1)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile hashmap with computed values");

    assert_eq!(main(), 10, "m[1] should return 10 (5 + 5)");
}

/// Test: HashMap insertion via index assignment
///
/// Validates: let m = {}; m[5] = 50; return m[5]; (should be 50)
#[test]
#[ignore = "HashMap insertion requires mutable HashMap support - future work"]
fn test_compile_hashmap_insertion() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let m = {};
    //     m[5] = 50;
    //     return m[5];
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "m".to_string(),
                value: Box::new(AstNode::HashMapLiteral { pairs: vec![] }),
            },
            AstNode::CompoundAssignment {
                lhs: Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("m".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(5)),
                }),
                op: ruchyruchy::interpreter::parser::BinaryOperator::Add,
                rhs: Box::new(AstNode::IntegerLiteral(50)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("m".to_string())),
                    index: Box::new(AstNode::IntegerLiteral(5)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile hashmap insertion");

    assert_eq!(main(), 50, "m[5] should return 50 after insertion");
}

/// Test: HashMap with string keys
///
/// Validates: HashMap with heterogeneous key types (requires type metadata)
#[test]
#[ignore = "HashMap with string keys requires type metadata - future work"]
fn test_compile_hashmap_string_keys() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let m = {"key1": 100}; return m["key1"]; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "m".to_string(),
                value: Box::new(AstNode::HashMapLiteral {
                    pairs: vec![(
                        AstNode::StringLiteral("key1".to_string()),
                        AstNode::IntegerLiteral(100),
                    )],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IndexAccess {
                    expr: Box::new(AstNode::Identifier("m".to_string())),
                    index: Box::new(AstNode::StringLiteral("key1".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile hashmap with string keys");

    assert_eq!(main(), 100, "m[\"key1\"] should return 100");
}
