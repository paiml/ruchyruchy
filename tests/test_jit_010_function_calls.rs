// INTERP-062 (JIT-009): Function Calls within JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement function calls between JIT-compiled functions
//
// What we need to support:
// 1. Calling other JIT-compiled functions
// 2. Passing arguments to called functions
// 3. Returning values from called functions
// 4. Recursive function calls
//
// Why this is critical:
// - Enables modular JIT code
// - Required for real-world programs
// - Foundation for mixed-mode execution
//
// Implementation strategy:
// - Compile all functions first, store in registry
// - Generate call instructions with proper ABI
// - Handle return values via Cranelift calling convention
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Call simple helper function (no parameters)
///
/// Validates: fun helper() { return 42; } fun main() { return helper(); }
#[test]
fn test_compile_simple_function_call() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Helper function: fun helper() { return 42; }
    let helper_body = AstNode::Return {
        value: Some(Box::new(AstNode::IntegerLiteral(42))),
    };

    // Compile helper function first
    let helper: fn() -> i64 = jit
        .compile_function_with_params(&[], &helper_body)
        .expect("Should compile helper function");

    // Register the helper function for calls
    jit.register_function("helper".to_string(), helper as *const u8);

    // Main function: fun main() { return helper(); }
    let main_body = AstNode::Return {
        value: Some(Box::new(AstNode::FunctionCall {
            name: "helper".to_string(),
            args: vec![],
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &main_body)
        .expect("Should compile main with function call");

    assert_eq!(main(), 42, "Calling helper() should return 42");
}

/// Test: Call function with one parameter
///
/// Validates: fun double(x) { return x * 2; } fun main() { return double(21); }
#[test]
fn test_compile_function_call_with_parameter() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Helper: fun double(x) { return x * 2; }
    let double_body = AstNode::Return {
        value: Some(Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::Identifier("x".to_string())),
            op: BinaryOperator::Multiply,
            right: Box::new(AstNode::IntegerLiteral(2)),
        })),
    };

    let double: fn(i64) -> i64 = jit
        .compile_function_with_params(&["x".to_string()], &double_body)
        .expect("Should compile double function");

    jit.register_function("double".to_string(), double as *const u8);

    // Main: fun main() { return double(21); }
    let main_body = AstNode::Return {
        value: Some(Box::new(AstNode::FunctionCall {
            name: "double".to_string(),
            args: vec![AstNode::IntegerLiteral(21)],
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &main_body)
        .expect("Should compile main with function call");

    assert_eq!(main(), 42, "Calling double(21) should return 42");
}

/// Test: Call function with multiple parameters
///
/// Validates: fun add(a, b) { return a + b; } fun main() { return add(10, 32); }
#[test]
fn test_compile_function_call_multiple_params() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Helper: fun add(a, b) { return a + b; }
    let add_body = AstNode::Return {
        value: Some(Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::Identifier("a".to_string())),
            op: BinaryOperator::Add,
            right: Box::new(AstNode::Identifier("b".to_string())),
        })),
    };

    let add: fn(i64, i64) -> i64 = jit
        .compile_function_with_params(&["a".to_string(), "b".to_string()], &add_body)
        .expect("Should compile add function");

    jit.register_function("add".to_string(), add as *const u8);

    // Main: fun main() { return add(10, 32); }
    let main_body = AstNode::Return {
        value: Some(Box::new(AstNode::FunctionCall {
            name: "add".to_string(),
            args: vec![AstNode::IntegerLiteral(10), AstNode::IntegerLiteral(32)],
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &main_body)
        .expect("Should compile main with function call");

    assert_eq!(main(), 42, "Calling add(10, 32) should return 42");
}

/// Test: Recursive function call
///
/// Validates: fun factorial(n) { if (n <= 1) { return 1; } return n * factorial(n - 1); }
///
/// Note: Recursive functions require forward declaration support (planned for future release)
#[test]
#[ignore = "Recursive functions require forward declaration support"]
fn test_compile_recursive_function_call() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Factorial: fun factorial(n) { if (n <= 1) { return 1; } return n * factorial(n - 1); }
    let factorial_body = AstNode::Block {
        statements: vec![
            AstNode::IfExpr {
                condition: Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    op: BinaryOperator::LessEqual,
                    right: Box::new(AstNode::IntegerLiteral(1)),
                }),
                then_branch: vec![AstNode::Return {
                    value: Some(Box::new(AstNode::IntegerLiteral(1))),
                }],
                else_branch: None,
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("n".to_string())),
                    op: BinaryOperator::Multiply,
                    right: Box::new(AstNode::FunctionCall {
                        name: "factorial".to_string(),
                        args: vec![AstNode::BinaryOp {
                            left: Box::new(AstNode::Identifier("n".to_string())),
                            op: BinaryOperator::Subtract,
                            right: Box::new(AstNode::IntegerLiteral(1)),
                        }],
                    }),
                })),
            },
        ],
    };

    let factorial: fn(i64) -> i64 = jit
        .compile_function_with_params(&["n".to_string()], &factorial_body)
        .expect("Should compile factorial function");

    // Register for recursive calls
    jit.register_function("factorial".to_string(), factorial as *const u8);

    // Test factorial
    assert_eq!(factorial(5), 120, "factorial(5) should be 120");
    assert_eq!(factorial(10), 3628800, "factorial(10) should be 3628800");
}

/// Test: Nested function calls
///
/// Validates: fun square(x) { return x * x; } fun sum_of_squares(a, b) { return square(a) + square(b); }
#[test]
fn test_compile_nested_function_calls() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // square: fun square(x) { return x * x; }
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

    // sum_of_squares: fun sum_of_squares(a, b) { return square(a) + square(b); }
    let sum_body = AstNode::Return {
        value: Some(Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::FunctionCall {
                name: "square".to_string(),
                args: vec![AstNode::Identifier("a".to_string())],
            }),
            op: BinaryOperator::Add,
            right: Box::new(AstNode::FunctionCall {
                name: "square".to_string(),
                args: vec![AstNode::Identifier("b".to_string())],
            }),
        })),
    };

    let sum_of_squares: fn(i64, i64) -> i64 = jit
        .compile_function_with_params(&["a".to_string(), "b".to_string()], &sum_body)
        .expect("Should compile sum_of_squares function");

    // Test: 3^2 + 4^2 = 9 + 16 = 25
    assert_eq!(
        sum_of_squares(3, 4),
        25,
        "sum_of_squares(3, 4) should be 25"
    );

    // Test: 5^2 + 12^2 = 25 + 144 = 169
    assert_eq!(
        sum_of_squares(5, 12),
        169,
        "sum_of_squares(5, 12) should be 169"
    );
}
