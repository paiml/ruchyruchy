// INTERP-054 (JIT-003): Variable and Parameter Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Compile functions with parameters and local variables
//
// What we need to support:
// 1. Function parameters: fun add(a, b) { return a + b; }
// 2. Local variables: let x = 10; return x + 5;
// 3. Variable references: a + b where a and b are parameters
//
// Why this is critical for JIT:
// - Real functions take parameters (not just constants)
// - Variable access is fundamental to any useful computation
// - Enables compiling actual Ruchy functions, not just expressions
//
// Cranelift concepts needed:
// - Function parameters: AbiParam in signature
// - Block parameters: append_block_params_for_function_params
// - Variable storage: Stack slots or SSA values
// - Variable lookup: Map variable names to Cranelift values
//
// Method: Test-driven development with incremental complexity

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Compile function with single parameter
///
/// Validates: fun(a) { return a; } called with 42 returns 42
#[test]
fn test_compile_single_parameter() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun identity(x) { return x; }
    // Simplified AST: parameter reference
    let param_names = vec!["x".to_string()];
    let body = AstNode::Identifier("x".to_string());

    // Compile function with parameters
    let compiled = jit.compile_function_with_params(&param_names, &body);

    assert!(
        compiled.is_ok(),
        "Should compile function with single parameter"
    );

    // Execute: identity(42) should return 42
    let func: fn(i64) -> i64 = compiled.unwrap();
    let result = func(42);
    assert_eq!(result, 42, "identity(42) should return 42");
}

/// Test: Compile function with two parameters (addition)
///
/// Validates: fun(a, b) { return a + b; }
#[test]
fn test_compile_two_parameters_add() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun add(a, b) { return a + b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::Add,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile add function");

    // Execute: add(10, 20) should return 30
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(10, 20), 30, "add(10, 20) should return 30");
    assert_eq!(func(5, 7), 12, "add(5, 7) should return 12");
}

/// Test: Compile function with parameters (subtraction)
///
/// Validates: fun(a, b) { return a - b; }
#[test]
fn test_compile_two_parameters_subtract() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun sub(a, b) { return a - b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::Subtract,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile subtract function");

    // Execute: sub(10, 3) should return 7
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(10, 3), 7, "sub(10, 3) should return 7");
    assert_eq!(func(100, 42), 58, "sub(100, 42) should return 58");
}

/// Test: Compile function with parameters (multiplication)
///
/// Validates: fun(a, b) { return a * b; }
#[test]
fn test_compile_two_parameters_multiply() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun mul(a, b) { return a * b; }
    let param_names = vec!["a".to_string(), "b".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("a".to_string())),
        op: BinaryOperator::Multiply,
        right: Box::new(AstNode::Identifier("b".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile multiply function");

    // Execute: mul(6, 7) should return 42
    let func: fn(i64, i64) -> i64 = compiled;
    assert_eq!(func(6, 7), 42, "mul(6, 7) should return 42");
    assert_eq!(func(5, 8), 40, "mul(5, 8) should return 40");
}

/// Test: Compile function mixing parameters and constants
///
/// Validates: fun(a) { return a + 10; }
#[test]
fn test_compile_parameter_with_constant() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun add_ten(x) { return x + 10; }
    let param_names = vec!["x".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("x".to_string())),
        op: BinaryOperator::Add,
        right: Box::new(AstNode::IntegerLiteral(10)),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile function with parameter and constant");

    // Execute: add_ten(5) should return 15
    let func: fn(i64) -> i64 = compiled;
    assert_eq!(func(5), 15, "add_ten(5) should return 15");
    assert_eq!(func(32), 42, "add_ten(32) should return 42");
}

/// Test: Compile function with complex expression using parameters
///
/// Validates: fun(a, b, c) { return (a + b) * c; }
#[test]
fn test_compile_three_parameters_complex() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun calc(a, b, c) { return (a + b) * c; }
    let param_names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::Identifier("a".to_string())),
            op: BinaryOperator::Add,
            right: Box::new(AstNode::Identifier("b".to_string())),
        }),
        op: BinaryOperator::Multiply,
        right: Box::new(AstNode::Identifier("c".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile complex function");

    // Execute: calc(2, 3, 4) should return 20
    let func: fn(i64, i64, i64) -> i64 = compiled;
    assert_eq!(func(2, 3, 4), 20, "calc(2, 3, 4) should return 20");
    assert_eq!(func(5, 5, 2), 20, "calc(5, 5, 2) should return 20");
}

/// Test: Compile function using same parameter multiple times
///
/// Validates: fun(x) { return x * x; } (square function)
#[test]
fn test_compile_parameter_reuse() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun square(x) { return x * x; }
    let param_names = vec!["x".to_string()];
    let body = AstNode::BinaryOp {
        left: Box::new(AstNode::Identifier("x".to_string())),
        op: BinaryOperator::Multiply,
        right: Box::new(AstNode::Identifier("x".to_string())),
    };

    // Compile function
    let compiled = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile square function");

    // Execute: square(5) should return 25
    let func: fn(i64) -> i64 = compiled;
    assert_eq!(func(5), 25, "square(5) should return 25");
    assert_eq!(func(10), 100, "square(10) should return 100");
    assert_eq!(func(7), 49, "square(7) should return 49");
}

/// Test: Compile multiple functions with different parameter counts
///
/// Validates that JIT can handle multiple functions with different signatures
#[test]
fn test_compile_multiple_functions_different_params() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function 1: fun double(x) { return x * 2; }
    let func1 = jit
        .compile_function_with_params(
            &["x".to_string()],
            &AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("x".to_string())),
                op: BinaryOperator::Multiply,
                right: Box::new(AstNode::IntegerLiteral(2)),
            },
        )
        .expect("Should compile double function");

    // Function 2: fun add(a, b) { return a + b; }
    let func2 = jit
        .compile_function_with_params(
            &["a".to_string(), "b".to_string()],
            &AstNode::BinaryOp {
                left: Box::new(AstNode::Identifier("a".to_string())),
                op: BinaryOperator::Add,
                right: Box::new(AstNode::Identifier("b".to_string())),
            },
        )
        .expect("Should compile add function");

    // Execute both functions
    let double: fn(i64) -> i64 = func1;
    let add: fn(i64, i64) -> i64 = func2;

    assert_eq!(double(21), 42, "double(21) should return 42");
    assert_eq!(add(20, 22), 42, "add(20, 22) should return 42");
}
