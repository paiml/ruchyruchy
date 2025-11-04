// INTERP-074 (JIT-021): Type Cast Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement explicit type casting in JIT compiler
//
// What we need to support:
// 1. Integer to float: (x as f64) where x is i64
// 2. Float to integer: (x as i64) where x is f64 (truncates)
// 3. Type casts in expressions: (x as f64) + 3.14
// 4. Type casts on literals: (42 as f64)
// 5. Type casts on variables and expressions
//
// Why this is critical:
// - Essential for mixed-type arithmetic
// - Common pattern in Rust/Ruchy
// - Enables precise control over numeric types
// - Foundation for type-safe numeric operations
//
// Implementation strategy:
// - i64 → f64: Use fcvt_from_sint (signed int to float)
// - f64 → i64: Use fcvt_to_sint_sat (float to signed int, saturating)
// - Check target_type string ("i64", "f64")
// - Compile expression, then apply conversion instruction
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Cast integer literal to float
///
/// Validates: let x = 42 as f64; return x;
#[test]
fn test_compile_int_to_float_literal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 42 as f64;
    //     return x;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::TypeCast {
                    expr: Box::new(AstNode::IntegerLiteral(42)),
                    target_type: "f64".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("x".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile int to float cast");

    let result = main();
    // Result is bit pattern of f64(42.0) reinterpreted as i64
    let float_value = f64::from_bits(result as u64);
    assert_eq!(float_value, 42.0, "42 as f64 should be 42.0");
}

/// Test: Cast integer variable to float
///
/// Validates: let x = 10; let y = x as f64; return y;
#[test]
fn test_compile_int_to_float_variable() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 10;
    //     let y = x as f64;
    //     return y;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::TypeCast {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    target_type: "f64".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("y".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile int to float cast from variable");

    let result = main();
    let float_value = f64::from_bits(result as u64);
    assert_eq!(float_value, 10.0, "x as f64 should be 10.0");
}

/// Test: Cast float literal to integer (truncation)
///
/// Validates: let x = 42.7 as i64; return x;
#[test]
fn test_compile_float_to_int_literal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 42.7 as i64;
    //     return x;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::TypeCast {
                    expr: Box::new(AstNode::FloatLiteral(42.7)),
                    target_type: "i64".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("x".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float to int cast");

    assert_eq!(main(), 42, "42.7 as i64 should truncate to 42");
}

/// Test: Cast float variable to integer
///
/// Validates: let x = 99.9; let y = x as i64; return y;
#[test]
fn test_compile_float_to_int_variable() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 99.9;
    //     let y = x as i64;
    //     return y;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::FloatLiteral(99.9)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::TypeCast {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    target_type: "i64".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("y".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float to int cast from variable");

    assert_eq!(main(), 99, "99.9 as i64 should truncate to 99");
}

/// Test: Type cast in arithmetic expression (int to float)
///
/// Validates: let x = 10; return (x as f64) + 3.5;
#[test]
fn test_compile_cast_in_float_arithmetic() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 10;
    //     return (x as f64) + 3.5;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::TypeCast {
                        expr: Box::new(AstNode::Identifier("x".to_string())),
                        target_type: "f64".to_string(),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::FloatLiteral(3.5)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile cast in float arithmetic");

    let result = main();
    let float_value = f64::from_bits(result as u64);
    assert_eq!(float_value, 13.5, "(x as f64) + 3.5 should be 13.5");
}

/// Test: Type cast on expression result
///
/// Validates: return ((10 + 5) as f64);
#[test]
fn test_compile_cast_expression_result() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     return ((10 + 5) as f64);
    // }
    let body = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::TypeCast {
                expr: Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::IntegerLiteral(10)),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::IntegerLiteral(5)),
                }),
                target_type: "f64".to_string(),
            })),
        }],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile cast on expression result");

    let result = main();
    let float_value = f64::from_bits(result as u64);
    assert_eq!(float_value, 15.0, "(10 + 5) as f64 should be 15.0");
}

/// Test: Round-trip cast (int → float → int)
///
/// Validates: let x = 42; return ((x as f64) as i64);
#[test]
fn test_compile_roundtrip_cast() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 42;
    //     return ((x as f64) as i64);
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(42)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::TypeCast {
                    expr: Box::new(AstNode::TypeCast {
                        expr: Box::new(AstNode::Identifier("x".to_string())),
                        target_type: "f64".to_string(),
                    }),
                    target_type: "i64".to_string(),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile roundtrip cast");

    assert_eq!(main(), 42, "((x as f64) as i64) should preserve value 42");
}

/// Test: Negative number cast (int to float)
///
/// Validates: let x = -10; return (x as f64);
#[test]
fn test_compile_negative_int_to_float() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = -10;
    //     return (x as f64);
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::UnaryOp {
                    op: ruchyruchy::interpreter::parser::UnaryOperator::Negate,
                    operand: Box::new(AstNode::IntegerLiteral(10)),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::TypeCast {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    target_type: "f64".to_string(),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile negative int to float");

    let result = main();
    let float_value = f64::from_bits(result as u64);
    assert_eq!(float_value, -10.0, "-10 as f64 should be -10.0");
}

/// Test: Zero cast
///
/// Validates: return (0 as f64);
#[test]
fn test_compile_zero_cast() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     return (0 as f64);
    // }
    let body = AstNode::Block {
        statements: vec![AstNode::Return {
            value: Some(Box::new(AstNode::TypeCast {
                expr: Box::new(AstNode::IntegerLiteral(0)),
                target_type: "f64".to_string(),
            })),
        }],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile zero cast");

    let result = main();
    let float_value = f64::from_bits(result as u64);
    assert_eq!(float_value, 0.0, "0 as f64 should be 0.0");
}
