// INTERP-053 (JIT-002): Basic JIT Compiler Infrastructure
//
// EXTREME TDD - RED Phase
//
// Mission: Build JitCompiler that translates AST → Cranelift IR → Machine code
//
// JIT Compiler Architecture:
// 1. Input: RuchyRuchy AST (from parser)
// 2. Translation: AST → Cranelift IR (function builder)
// 3. Compilation: Cranelift IR → Machine code (JIT module)
// 4. Execution: Call compiled code via function pointer
//
// Starting Simple:
// - Arithmetic expressions: 2 + 3, 10 - 5, 4 * 6, 20 / 4
// - Integer constants only (no variables yet)
// - Single expression evaluation (no statements yet)
//
// Why Cranelift IR:
// - SSA form (Static Single Assignment) - each value assigned once
// - Explicit control flow (blocks, branches)
// - Type-safe intermediate representation
// - Fast compilation to machine code
//
// Method: Test-driven development with incremental complexity

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Create JIT compiler instance
///
/// Validates that JitCompiler can be instantiated
#[test]
fn test_create_jit_compiler() {
    let jit = JitCompiler::new();

    // Should be able to create a JIT compiler without errors
    assert!(jit.is_ok(), "JitCompiler::new() should succeed");
}

/// Test: Compile integer constant
///
/// Validates that JIT can compile simple constant: 42
#[test]
fn test_compile_integer_constant() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // AST: IntegerLiteral(42)
    let ast = AstNode::IntegerLiteral(42);

    // Compile to machine code
    let compiled = jit.compile_expression(&ast);

    assert!(
        compiled.is_ok(),
        "Should compile integer constant successfully"
    );

    // Execute compiled code
    let func = compiled.unwrap();
    let result = func();

    assert_eq!(result, 42, "Compiled constant should return 42");
}

/// Test: Compile addition expression
///
/// Validates that JIT can compile: 2 + 3 = 5
#[test]
fn test_compile_addition() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // AST: 2 + 3
    let ast = AstNode::BinaryOp {
        left: Box::new(AstNode::IntegerLiteral(2)),
        op: BinaryOperator::Add,
        right: Box::new(AstNode::IntegerLiteral(3)),
    };

    // Compile to machine code
    let compiled = jit
        .compile_expression(&ast)
        .expect("Compilation should succeed");

    // Execute compiled code
    let result = compiled();

    assert_eq!(result, 5, "Compiled 2 + 3 should return 5");
}

/// Test: Compile subtraction expression
///
/// Validates that JIT can compile: 10 - 5 = 5
#[test]
fn test_compile_subtraction() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // AST: 10 - 5
    let ast = AstNode::BinaryOp {
        left: Box::new(AstNode::IntegerLiteral(10)),
        op: BinaryOperator::Subtract,
        right: Box::new(AstNode::IntegerLiteral(5)),
    };

    // Compile to machine code
    let compiled = jit
        .compile_expression(&ast)
        .expect("Compilation should succeed");

    // Execute compiled code
    let result = compiled();

    assert_eq!(result, 5, "Compiled 10 - 5 should return 5");
}

/// Test: Compile multiplication expression
///
/// Validates that JIT can compile: 4 * 6 = 24
#[test]
fn test_compile_multiplication() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // AST: 4 * 6
    let ast = AstNode::BinaryOp {
        left: Box::new(AstNode::IntegerLiteral(4)),
        op: BinaryOperator::Multiply,
        right: Box::new(AstNode::IntegerLiteral(6)),
    };

    // Compile to machine code
    let compiled = jit
        .compile_expression(&ast)
        .expect("Compilation should succeed");

    // Execute compiled code
    let result = compiled();

    assert_eq!(result, 24, "Compiled 4 * 6 should return 24");
}

/// Test: Compile division expression
///
/// Validates that JIT can compile: 20 / 4 = 5
#[test]
fn test_compile_division() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // AST: 20 / 4
    let ast = AstNode::BinaryOp {
        left: Box::new(AstNode::IntegerLiteral(20)),
        op: BinaryOperator::Divide,
        right: Box::new(AstNode::IntegerLiteral(4)),
    };

    // Compile to machine code
    let compiled = jit
        .compile_expression(&ast)
        .expect("Compilation should succeed");

    // Execute compiled code
    let result = compiled();

    assert_eq!(result, 5, "Compiled 20 / 4 should return 5");
}

/// Test: Compile nested expression
///
/// Validates that JIT can compile: (2 + 3) * 4 = 20
#[test]
fn test_compile_nested_expression() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // AST: (2 + 3) * 4
    let ast = AstNode::BinaryOp {
        left: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::IntegerLiteral(2)),
            op: BinaryOperator::Add,
            right: Box::new(AstNode::IntegerLiteral(3)),
        }),
        op: BinaryOperator::Multiply,
        right: Box::new(AstNode::IntegerLiteral(4)),
    };

    // Compile to machine code
    let compiled = jit
        .compile_expression(&ast)
        .expect("Compilation should succeed");

    // Execute compiled code
    let result = compiled();

    assert_eq!(result, 20, "Compiled (2 + 3) * 4 should return 20");
}

/// Test: Compile complex nested expression
///
/// Validates that JIT can compile: (10 - 2) / (3 + 1) = 2
#[test]
fn test_compile_complex_expression() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // AST: (10 - 2) / (3 + 1)
    let ast = AstNode::BinaryOp {
        left: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::IntegerLiteral(10)),
            op: BinaryOperator::Subtract,
            right: Box::new(AstNode::IntegerLiteral(2)),
        }),
        op: BinaryOperator::Divide,
        right: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::IntegerLiteral(3)),
            op: BinaryOperator::Add,
            right: Box::new(AstNode::IntegerLiteral(1)),
        }),
    };

    // Compile to machine code
    let compiled = jit
        .compile_expression(&ast)
        .expect("Compilation should succeed");

    // Execute compiled code
    let result = compiled();

    assert_eq!(result, 2, "Compiled (10 - 2) / (3 + 1) should return 2");
}

/// Test: Multiple compilations in same JIT compiler
///
/// Validates that JitCompiler can compile multiple expressions
#[test]
fn test_multiple_compilations() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Compile first expression: 5 + 5
    let ast1 = AstNode::BinaryOp {
        left: Box::new(AstNode::IntegerLiteral(5)),
        op: BinaryOperator::Add,
        right: Box::new(AstNode::IntegerLiteral(5)),
    };
    let func1 = jit
        .compile_expression(&ast1)
        .expect("First compilation should succeed");
    assert_eq!(func1(), 10, "First compiled function should return 10");

    // Compile second expression: 3 * 7
    let ast2 = AstNode::BinaryOp {
        left: Box::new(AstNode::IntegerLiteral(3)),
        op: BinaryOperator::Multiply,
        right: Box::new(AstNode::IntegerLiteral(7)),
    };
    let func2 = jit
        .compile_expression(&ast2)
        .expect("Second compilation should succeed");
    assert_eq!(func2(), 21, "Second compiled function should return 21");

    // Both functions should still work
    assert_eq!(func1(), 10, "First function should still return 10");
    assert_eq!(func2(), 21, "Second function should still return 21");
}
