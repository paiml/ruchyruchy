// INTERP-067 (JIT-014): Float Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement floating-point number support in JIT compiler
//
// What we need to support:
// 1. Float literals: 3.14, 2.718, 0.5
// 2. Float arithmetic: +, -, *, /
// 3. Float comparisons: ==, !=, <, >, <=, >=
// 4. Float variables
// 5. Mixed int/float operations (type promotion)
//
// Why this is critical:
// - Floats are fundamental for scientific/mathematical computation
// - Required for real-world numeric applications
// - Completes basic numeric type support
// - Foundation for more complex math operations
//
// Implementation strategy:
// - Use Cranelift F64 type for floating-point values
// - Float literals compile to f64 constants
// - Arithmetic uses fadd, fsub, fmul, fdiv instructions
// - Comparisons use fcmp instruction
// - Type promotion: convert i64 to f64 when needed
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Helper to compare floats with small epsilon for floating-point precision
fn assert_float_eq(actual: i64, expected: f64, msg: &str) {
    let actual_f64 = f64::from_bits(actual as u64);
    let epsilon = 0.0001;
    assert!(
        (actual_f64 - expected).abs() < epsilon,
        "{}: expected {}, got {}",
        msg,
        expected,
        actual_f64
    );
}

/// Test: Simple float literal
///
/// Validates: return 3.14;
#[test]
fn test_compile_float_literal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return 3.5; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::FloatLiteral(3.5))),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float literal");

    let result = main();
    assert_float_eq(result, 3.5, "Float literal 3.5");
}

/// Test: Float in variable
///
/// Validates: let x = 2.718; return x;
#[test]
fn test_compile_float_in_variable() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let x = 2.75; return x; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::FloatLiteral(2.75)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("x".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float in variable");

    let result = main();
    assert_float_eq(result, 2.75, "Float variable x=2.75");
}

/// Test: Float addition
///
/// Validates: return 1.5 + 2.5; (should be 4.0)
#[test]
fn test_compile_float_addition() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return 1.5 + 2.5; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::FloatLiteral(1.5)),
            op: BinaryOperator::Add,
            right: Box::new(AstNode::FloatLiteral(2.5)),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float addition");

    let result = main();
    assert_float_eq(result, 4.0, "1.5 + 2.5 should be 4.0");
}

/// Test: Float subtraction
///
/// Validates: return 5.5 - 2.2; (should be 3.3)
#[test]
fn test_compile_float_subtraction() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return 5.5 - 2.2; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::FloatLiteral(5.5)),
            op: BinaryOperator::Subtract,
            right: Box::new(AstNode::FloatLiteral(2.2)),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float subtraction");

    let result = main();
    assert_float_eq(result, 3.3, "5.5 - 2.2 should be 3.3");
}

/// Test: Float multiplication
///
/// Validates: return 2.5 * 4.0; (should be 10.0)
#[test]
fn test_compile_float_multiplication() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return 2.5 * 4.0; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::FloatLiteral(2.5)),
            op: BinaryOperator::Multiply,
            right: Box::new(AstNode::FloatLiteral(4.0)),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float multiplication");

    let result = main();
    assert_float_eq(result, 10.0, "2.5 * 4.0 should be 10.0");
}

/// Test: Float division
///
/// Validates: return 10.0 / 4.0; (should be 2.5)
#[test]
fn test_compile_float_division() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return 10.0 / 4.0; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::FloatLiteral(10.0)),
            op: BinaryOperator::Divide,
            right: Box::new(AstNode::FloatLiteral(4.0)),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float division");

    let result = main();
    assert_float_eq(result, 2.5, "10.0 / 4.0 should be 2.5");
}

/// Test: Float comparison (equality)
///
/// Validates: if (3.14 == 3.14) { return 1; } else { return 0; }
#[test]
fn test_compile_float_equality() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     if (4.25 == 4.25) { return 1; } else { return 0; }
    // }
    let body = AstNode::IfExpr {
        condition: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::FloatLiteral(4.25)),
            op: BinaryOperator::Equal,
            right: Box::new(AstNode::FloatLiteral(4.25)),
        }),
        then_branch: vec![AstNode::Return {
            value: Some(Box::new(AstNode::IntegerLiteral(1))),
        }],
        else_branch: Some(vec![AstNode::Return {
            value: Some(Box::new(AstNode::IntegerLiteral(0))),
        }]),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float equality");

    assert_eq!(main(), 1, "4.25 == 4.25 should be true");
}

/// Test: Float comparison (less than)
///
/// Validates: if (1.5 < 2.5) { return 1; } else { return 0; }
#[test]
fn test_compile_float_less_than() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     if (1.5 < 2.5) { return 1; } else { return 0; }
    // }
    let body = AstNode::IfExpr {
        condition: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::FloatLiteral(1.5)),
            op: BinaryOperator::LessThan,
            right: Box::new(AstNode::FloatLiteral(2.5)),
        }),
        then_branch: vec![AstNode::Return {
            value: Some(Box::new(AstNode::IntegerLiteral(1))),
        }],
        else_branch: Some(vec![AstNode::Return {
            value: Some(Box::new(AstNode::IntegerLiteral(0))),
        }]),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float less than");

    assert_eq!(main(), 1, "1.5 < 2.5 should be true");
}

/// Test: Float with arithmetic expression
///
/// Validates: let x = 2.0; let y = 3.0; return x * y + 1.0; (should be 7.0)
#[test]
fn test_compile_float_expression() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 2.0;
    //     let y = 3.0;
    //     return x * y + 1.0;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::FloatLiteral(2.0)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::FloatLiteral(3.0)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::BinaryOp {
                        left: Box::new(AstNode::Identifier("x".to_string())),
                        op: BinaryOperator::Multiply,
                        right: Box::new(AstNode::Identifier("y".to_string())),
                    }),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::FloatLiteral(1.0)),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile float expression");

    let result = main();
    assert_float_eq(result, 7.0, "x * y + 1.0 where x=2.0, y=3.0 should be 7.0");
}

/// Test: Negative float literal
///
/// Validates: return -5.25;
#[test]
fn test_compile_negative_float() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return -5.25; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::UnaryOp {
            op: ruchyruchy::interpreter::parser::UnaryOperator::Negate,
            operand: Box::new(AstNode::FloatLiteral(5.25)),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile negative float");

    let result = main();
    assert_float_eq(result, -5.25, "Negation of 5.25 should be -5.25");
}

/// Test: Float as function parameter
///
/// Validates: fun square(x) { return x * x; } fun main() { return square(3.5); }
///
/// Note: Float parameters require F64 function signatures (not yet implemented in MVP)
#[test]
#[ignore = "Float parameters require F64 function signatures - future work"]
fn test_compile_float_parameter() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Helper: fun square(x) { return x * x; }
    let square_body = AstNode::Return {
        value: Some(Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::Identifier("x".to_string())),
            op: BinaryOperator::Multiply,
            right: Box::new(AstNode::Identifier("x".to_string())),
        })),
    };

    let square: fn(i64) -> i64 = jit
        .compile_function_with_params(&["x".to_string()], &square_body)
        .expect("Should compile square function");

    jit.register_function("square".to_string(), square as *const u8);

    // Main: fun main() { return square(3.5); }
    let main_body = AstNode::Return {
        value: Some(Box::new(AstNode::FunctionCall {
            name: "square".to_string(),
            args: vec![AstNode::FloatLiteral(3.5)],
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &main_body)
        .expect("Should compile main with float parameter");

    let result = main();
    assert_float_eq(result, 12.25, "square(3.5) should be 12.25");
}
