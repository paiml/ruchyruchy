// INTERP-057 (JIT-005): While Loop Compilation
//
// EXTREME TDD - RED Phase
//
// Mission: Compile basic while loops to machine code
//
// Note: Full while loop implementation requires variable storage (let/assignment)
// which is not yet implemented. These tests start with the simplest possible
// while loops that only test control flow structure.
//
// Cranelift concepts:
// - Loop header block: Evaluates condition
// - Loop body block: Executes loop body
// - Loop exit block: Continues after loop
// - Back edge: Jump from body back to header
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Compile while loop that never executes
///
/// Validates: while (false) { } returns 0
#[test]
fn test_compile_while_false_condition() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Simplest while loop: condition is always false
    // Should skip body entirely and return 0
    let param_names = vec![];
    let body = AstNode::WhileLoop {
        condition: Box::new(AstNode::BooleanLiteral(false)),
        body: vec![],
    };

    let compiled: fn() -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile while loop with false condition");

    let result = compiled();
    assert_eq!(result, 0, "while (false) {{}} should return 0");
}

/// Test: Compile while loop with true condition (infinite loop prevention)
///
/// Note: This would be infinite without a way to modify the condition
/// For now, we test that it compiles but don't execute it
#[test]
fn test_compile_while_true_condition_compiles() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    //  while (true) { } - would be infinite
    // Just test that it compiles, don't execute
    let param_names = vec![];
    let body = AstNode::WhileLoop {
        condition: Box::new(AstNode::BooleanLiteral(true)),
        body: vec![],
    };

    let compiled: Result<fn() -> i64, _> = jit.compile_function_with_params(&param_names, &body);

    assert!(
        compiled.is_ok(),
        "Should compile while (true) even though it would be infinite"
    );
    // Don't call the function - it would hang!
}

/// Test: Compile while loop with parameter condition
///
/// Validates: while (n > 0) { } with n as parameter
/// Without variable modification, this becomes: if (n > 0) { infinite loop } else { return 0 }
/// We can test with n=0 to verify it returns without looping
#[test]
fn test_compile_while_parameter_condition() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // while (n > 0) { }
    // If n <= 0, loop never executes, returns 0
    // If n > 0, would loop forever (no way to modify n yet)
    let param_names = vec!["n".to_string()];
    let body = AstNode::WhileLoop {
        condition: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::Identifier("n".to_string())),
            op: BinaryOperator::GreaterThan,
            right: Box::new(AstNode::IntegerLiteral(0)),
        }),
        body: vec![],
    };

    let compiled: fn(i64) -> i64 = jit
        .compile_function_with_params(&param_names, &body)
        .expect("Should compile while with parameter condition");

    // Test with n=0: condition false, should return 0 immediately
    let result = compiled(0);
    assert_eq!(result, 0, "while (0 > 0) {{}} should return 0");

    // Test with n=-5: condition false, should return 0 immediately
    let result = compiled(-5);
    assert_eq!(result, 0, "while (-5 > 0) {{}} should return 0");

    // Don't test with n>0 - would be infinite loop!
}
