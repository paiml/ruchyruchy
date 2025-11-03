// INTERP-069 (JIT-016): Struct Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement basic struct support in JIT compiler
//
// What we need to support:
// 1. Struct definitions: struct Point { x, y }
// 2. Struct literals: Point { x: 10, y: 20 }
// 3. Struct field access: point.x, point.y
// 4. Structs in variables
// 5. Struct field mutation
//
// Why this is critical:
// - Structs are fundamental for data modeling
// - Natural progression from tuples (named vs indexed fields)
// - Foundation for more complex types
// - Common pattern in all programming
//
// Implementation strategy:
// - Store struct type definitions in registry (name -> field offsets)
// - Stack-allocate struct instances like tuples
// - Field access via name lookup to offset
// - Return pointer to struct (like tuples/arrays)
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, StructField};
use ruchyruchy::jit::JitCompiler;

/// Test: Simple struct definition and literal
///
/// Validates: struct Point { x, y } let p = Point { x: 10, y: 20 }; return p;
#[test]
fn test_compile_struct_literal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define struct Point { x, y }
    let struct_def = AstNode::StructDef {
        name: "Point".to_string(),
        fields: vec![
            StructField {
                name: "x".to_string(),
                type_annotation: None,
            },
            StructField {
                name: "y".to_string(),
                type_annotation: None,
            },
        ],
    };

    // Function: fun main() {
    //     struct Point { x, y }
    //     let p = Point { x: 10, y: 20 };
    //     return p;
    // }
    let body = AstNode::Block {
        statements: vec![
            struct_def,
            AstNode::LetDecl {
                name: "p".to_string(),
                value: Box::new(AstNode::StructLiteral {
                    name: "Point".to_string(),
                    fields: vec![
                        ("x".to_string(), AstNode::IntegerLiteral(10)),
                        ("y".to_string(), AstNode::IntegerLiteral(20)),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("p".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile struct literal");

    let result = main();
    // Result should be pointer to struct (non-null)
    assert_ne!(result, 0, "Struct pointer should be non-null");
}

/// Test: Struct field access
///
/// Validates: struct Point { x, y } let p = Point { x: 10, y: 20 }; return p.x;
#[test]
fn test_compile_struct_field_access() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define struct Point { x, y }
    let struct_def = AstNode::StructDef {
        name: "Point".to_string(),
        fields: vec![
            StructField {
                name: "x".to_string(),
                type_annotation: None,
            },
            StructField {
                name: "y".to_string(),
                type_annotation: None,
            },
        ],
    };

    // Function: fun main() {
    //     struct Point { x, y }
    //     let p = Point { x: 10, y: 20 };
    //     return p.x;
    // }
    let body = AstNode::Block {
        statements: vec![
            struct_def,
            AstNode::LetDecl {
                name: "p".to_string(),
                value: Box::new(AstNode::StructLiteral {
                    name: "Point".to_string(),
                    fields: vec![
                        ("x".to_string(), AstNode::IntegerLiteral(10)),
                        ("y".to_string(), AstNode::IntegerLiteral(20)),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FieldAccess {
                    expr: Box::new(AstNode::Identifier("p".to_string())),
                    field: "x".to_string(),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile struct field access");

    assert_eq!(main(), 10, "p.x should return 10");
}

/// Test: Struct multiple field accesses
///
/// Validates: struct Point { x, y } let p = Point { x: 5, y: 15 }; return p.x + p.y;
#[test]
fn test_compile_struct_multiple_access() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define struct Point { x, y }
    let struct_def = AstNode::StructDef {
        name: "Point".to_string(),
        fields: vec![
            StructField {
                name: "x".to_string(),
                type_annotation: None,
            },
            StructField {
                name: "y".to_string(),
                type_annotation: None,
            },
        ],
    };

    // Function: fun main() {
    //     struct Point { x, y }
    //     let p = Point { x: 5, y: 15 };
    //     return p.x + p.y;
    // }
    let body = AstNode::Block {
        statements: vec![
            struct_def,
            AstNode::LetDecl {
                name: "p".to_string(),
                value: Box::new(AstNode::StructLiteral {
                    name: "Point".to_string(),
                    fields: vec![
                        ("x".to_string(), AstNode::IntegerLiteral(5)),
                        ("y".to_string(), AstNode::IntegerLiteral(15)),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::FieldAccess {
                        expr: Box::new(AstNode::Identifier("p".to_string())),
                        field: "x".to_string(),
                    }),
                    op: ruchyruchy::interpreter::parser::BinaryOperator::Add,
                    right: Box::new(AstNode::FieldAccess {
                        expr: Box::new(AstNode::Identifier("p".to_string())),
                        field: "y".to_string(),
                    }),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile multiple struct access");

    assert_eq!(main(), 20, "p.x + p.y should be 5 + 15 = 20");
}

/// Test: Struct with single field
///
/// Validates: struct Wrapper { value } let w = Wrapper { value: 42 }; return w.value;
#[test]
fn test_compile_struct_single_field() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define struct Wrapper { value }
    let struct_def = AstNode::StructDef {
        name: "Wrapper".to_string(),
        fields: vec![StructField {
            name: "value".to_string(),
            type_annotation: None,
        }],
    };

    // Function: fun main() {
    //     struct Wrapper { value }
    //     let w = Wrapper { value: 42 };
    //     return w.value;
    // }
    let body = AstNode::Block {
        statements: vec![
            struct_def,
            AstNode::LetDecl {
                name: "w".to_string(),
                value: Box::new(AstNode::StructLiteral {
                    name: "Wrapper".to_string(),
                    fields: vec![("value".to_string(), AstNode::IntegerLiteral(42))],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FieldAccess {
                    expr: Box::new(AstNode::Identifier("w".to_string())),
                    field: "value".to_string(),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile struct with single field");

    assert_eq!(main(), 42, "w.value should be 42");
}

/// Test: Struct with three fields
///
/// Validates: struct Triple { a, b, c } let t = Triple { a: 1, b: 2, c: 3 }; return t.b;
#[test]
fn test_compile_struct_three_fields() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define struct Triple { a, b, c }
    let struct_def = AstNode::StructDef {
        name: "Triple".to_string(),
        fields: vec![
            StructField {
                name: "a".to_string(),
                type_annotation: None,
            },
            StructField {
                name: "b".to_string(),
                type_annotation: None,
            },
            StructField {
                name: "c".to_string(),
                type_annotation: None,
            },
        ],
    };

    // Function: fun main() {
    //     struct Triple { a, b, c }
    //     let t = Triple { a: 1, b: 2, c: 3 };
    //     return t.b;
    // }
    let body = AstNode::Block {
        statements: vec![
            struct_def,
            AstNode::LetDecl {
                name: "t".to_string(),
                value: Box::new(AstNode::StructLiteral {
                    name: "Triple".to_string(),
                    fields: vec![
                        ("a".to_string(), AstNode::IntegerLiteral(1)),
                        ("b".to_string(), AstNode::IntegerLiteral(2)),
                        ("c".to_string(), AstNode::IntegerLiteral(3)),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FieldAccess {
                    expr: Box::new(AstNode::Identifier("t".to_string())),
                    field: "b".to_string(),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile struct with three fields");

    assert_eq!(main(), 2, "t.b should be 2");
}

/// Test: Multiple struct instances
///
/// Validates: Creating two instances of the same struct type
#[test]
fn test_compile_multiple_struct_instances() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define struct Point { x, y }
    let struct_def = AstNode::StructDef {
        name: "Point".to_string(),
        fields: vec![
            StructField {
                name: "x".to_string(),
                type_annotation: None,
            },
            StructField {
                name: "y".to_string(),
                type_annotation: None,
            },
        ],
    };

    // Function: fun main() {
    //     struct Point { x, y }
    //     let p1 = Point { x: 10, y: 20 };
    //     let p2 = Point { x: 30, y: 40 };
    //     return p1.x + p2.y;
    // }
    let body = AstNode::Block {
        statements: vec![
            struct_def,
            AstNode::LetDecl {
                name: "p1".to_string(),
                value: Box::new(AstNode::StructLiteral {
                    name: "Point".to_string(),
                    fields: vec![
                        ("x".to_string(), AstNode::IntegerLiteral(10)),
                        ("y".to_string(), AstNode::IntegerLiteral(20)),
                    ],
                }),
            },
            AstNode::LetDecl {
                name: "p2".to_string(),
                value: Box::new(AstNode::StructLiteral {
                    name: "Point".to_string(),
                    fields: vec![
                        ("x".to_string(), AstNode::IntegerLiteral(30)),
                        ("y".to_string(), AstNode::IntegerLiteral(40)),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::FieldAccess {
                        expr: Box::new(AstNode::Identifier("p1".to_string())),
                        field: "x".to_string(),
                    }),
                    op: ruchyruchy::interpreter::parser::BinaryOperator::Add,
                    right: Box::new(AstNode::FieldAccess {
                        expr: Box::new(AstNode::Identifier("p2".to_string())),
                        field: "y".to_string(),
                    }),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile multiple struct instances");

    assert_eq!(main(), 50, "p1.x + p2.y should be 10 + 40 = 50");
}

/// Test: Struct with heterogeneous fields (mixed types)
///
/// Validates: struct Mixed { integer, float } (requires type metadata)
#[test]
#[ignore = "Heterogeneous struct fields require type metadata - future work"]
fn test_compile_heterogeneous_struct() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Define struct Mixed { integer, float }
    let struct_def = AstNode::StructDef {
        name: "Mixed".to_string(),
        fields: vec![
            StructField {
                name: "integer".to_string(),
                type_annotation: None,
            },
            StructField {
                name: "float".to_string(),
                type_annotation: None,
            },
        ],
    };

    // Function: fun main() {
    //     struct Mixed { integer, float }
    //     let m = Mixed { integer: 42, float: 3.14 };
    //     return m.integer;
    // }
    let body = AstNode::Block {
        statements: vec![
            struct_def,
            AstNode::LetDecl {
                name: "m".to_string(),
                value: Box::new(AstNode::StructLiteral {
                    name: "Mixed".to_string(),
                    fields: vec![
                        ("integer".to_string(), AstNode::IntegerLiteral(42)),
                        ("float".to_string(), AstNode::FloatLiteral(3.5)),
                    ],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::FieldAccess {
                    expr: Box::new(AstNode::Identifier("m".to_string())),
                    field: "integer".to_string(),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile heterogeneous struct");

    assert_eq!(main(), 42, "m.integer should be 42");
}
