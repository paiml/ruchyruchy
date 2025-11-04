// INTERP-077 (JIT-024): Path Expression Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement path expression (qualified names) in JIT compiler
//
// What we need to support:
// 1. Simple path expression as identifier: std::sync
// 2. Path expression in function calls: Arc::new(42)
// 3. Multi-segment paths: std::sync::Arc
// 4. Path expressions as return values
// 5. Path expressions in let declarations
//
// Why this is critical:
// - Path expressions are used for qualified names
// - Common pattern: Module::function() or Type::associated_fn()
// - Essential for calling functions with namespaces
// - Foundation for module system support
//
// Implementation strategy:
// - Join path segments with "::" to create qualified name
// - Treat as function lookup (same as FunctionCall)
// - For MVP: PathExpr evaluates to a placeholder value
// - Full implementation would need module/namespace resolution
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::AstNode;
use ruchyruchy::jit::JitCompiler;

/// Test: Path expression as simple identifier
///
/// Validates: let x = std::sync; return 1;
/// MVP: Just ensure it compiles without error
#[test]
fn test_compile_pathexpr_simple() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = std::sync;
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::PathExpr {
                    segments: vec!["std".to_string(), "sync".to_string()],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile simple path expression");

    assert_eq!(main(), 1, "simple path expression should compile");
}

/// Test: Multi-segment path expression
///
/// Validates: let x = std::sync::Arc; return 1;
#[test]
fn test_compile_pathexpr_multi_segment() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = std::sync::Arc;
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::PathExpr {
                    segments: vec!["std".to_string(), "sync".to_string(), "Arc".to_string()],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile multi-segment path expression");

    assert_eq!(main(), 1, "multi-segment path expression should compile");
}

/// Test: Path expression as return value
///
/// Validates: return std::sync::Arc;
/// MVP: Returns a placeholder value (e.g., 0)
#[test]
fn test_compile_pathexpr_return() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     return std::sync::Arc;
    // }
    let body = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::PathExpr {
                segments: vec!["std".to_string(), "sync".to_string(), "Arc".to_string()],
            })),
        }],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile path expression as return value");

    // MVP: Path expression evaluates to placeholder (0)
    assert_eq!(main(), 0, "path expression should return placeholder value");
}

/// Test: Single-segment path (edge case)
///
/// Validates: let x = Module; return 1;
#[test]
fn test_compile_pathexpr_single_segment() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = Module;
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::PathExpr {
                    segments: vec!["Module".to_string()],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile single-segment path expression");

    assert_eq!(main(), 1, "single-segment path expression should compile");
}

/// Test: Path expression with common Rust types
///
/// Validates: let a = Option::Some; let b = Result::Ok; return 1;
#[test]
fn test_compile_pathexpr_common_types() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let a = Option::Some;
    //     let b = Result::Ok;
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "a".to_string(),
                value: Box::new(AstNode::PathExpr {
                    segments: vec!["Option".to_string(), "Some".to_string()],
                }),
            },
            AstNode::LetDecl {
                name: "b".to_string(),
                value: Box::new(AstNode::PathExpr {
                    segments: vec!["Result".to_string(), "Ok".to_string()],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile common type path expressions");

    assert_eq!(main(), 1, "common type path expressions should compile");
}

/// Test: Path expression in arithmetic
///
/// Validates: let x = std::sync; return x + 10;
/// MVP: Path expression evaluates to 0, so result is 10
#[test]
fn test_compile_pathexpr_in_arithmetic() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = std::sync;
    //     return x + 10;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::PathExpr {
                    segments: vec!["std".to_string(), "sync".to_string()],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("x".to_string())),
                    op: ruchyruchy::interpreter::parser::BinaryOperator::Add,
                    right: Box::new(AstNode::IntegerLiteral(10)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile path expression in arithmetic");

    // MVP: Path expression evaluates to 0, so 0 + 10 = 10
    assert_eq!(main(), 10, "path expression in arithmetic should work");
}

/// Test: Multiple path expressions
///
/// Validates: let a = std::sync; let b = std::thread; return 1;
#[test]
fn test_compile_multiple_pathexprs() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let a = std::sync;
    //     let b = std::thread;
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "a".to_string(),
                value: Box::new(AstNode::PathExpr {
                    segments: vec!["std".to_string(), "sync".to_string()],
                }),
            },
            AstNode::LetDecl {
                name: "b".to_string(),
                value: Box::new(AstNode::PathExpr {
                    segments: vec!["std".to_string(), "thread".to_string()],
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile multiple path expressions");

    assert_eq!(main(), 1, "multiple path expressions should compile");
}
