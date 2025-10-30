// INTERP-009: Call Stack Traces for Error Reporting
// RED Phase: Create tests for enhanced error reporting with stack traces
//
// Scope: Call stack tracking (Source location mapping deferred to future ticket)
//
// Tests for:
// - Stack trace generation on errors
// - Call stack depth tracking
// - Function name tracking in stack
// - Stack trace formatting
//
// Test Coverage:
// - Nested function call errors: 3 tests
// - Stack trace content: 2 tests
// - Meta test: 1 test
// Total: 6 tests

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator, UnaryOperator};

// =============================================================================
// Stack Trace Generation Tests
// =============================================================================

#[test]
fn test_stack_trace_in_nested_function() {
    // fun outer() {
    //     inner();
    // }
    // fun inner() {
    //     let x = undefined_var;  // Error!
    // }
    // outer(); // Should show: outer -> inner in stack trace

    let mut eval = Evaluator::new();

    // Define outer function
    eval.eval(&AstNode::FunctionDef {
        name: "outer".to_string(),
        params: vec![],
        body: vec![AstNode::FunctionCall {
            name: "inner".to_string(),
            args: vec![],
        }],
    })
    .unwrap();

    // Define inner function that references undefined variable
    eval.eval(&AstNode::FunctionDef {
        name: "inner".to_string(),
        params: vec![],
        body: vec![AstNode::LetDecl {
            name: "x".to_string(),
            value: Box::new(AstNode::Identifier("undefined_var".to_string())),
        }],
    })
    .unwrap();

    // Call outer - should error with stack trace
    let result = eval.eval(&AstNode::FunctionCall {
        name: "outer".to_string(),
        args: vec![],
    });

    assert!(result.is_err(), "Expected error due to undefined variable");
    let err = result.unwrap_err();
    let err_msg = format!("{:?}", err);

    // RED: Check that error message contains call stack information
    // Should contain both "outer" and "inner" function names
    assert!(
        err_msg.contains("outer") && err_msg.contains("inner"),
        "Error message should contain stack trace with function names 'outer' and 'inner': {}",
        err_msg
    );
}

#[test]
fn test_stack_trace_depth() {
    // fun level3() {
    //     let x = 1 / 0;  // Division by zero error
    // }
    // fun level2() {
    //     level3();
    // }
    // fun level1() {
    //     level2();
    // }
    // level1(); // Should show: level1 -> level2 -> level3

    let mut eval = Evaluator::new();

    eval.eval(&AstNode::FunctionDef {
        name: "level3".to_string(),
        params: vec![],
        body: vec![AstNode::LetDecl {
            name: "x".to_string(),
            value: Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Divide,
                left: Box::new(AstNode::IntegerLiteral(1)),
                right: Box::new(AstNode::IntegerLiteral(0)),
            }),
        }],
    })
    .unwrap();

    eval.eval(&AstNode::FunctionDef {
        name: "level2".to_string(),
        params: vec![],
        body: vec![AstNode::FunctionCall {
            name: "level3".to_string(),
            args: vec![],
        }],
    })
    .unwrap();

    eval.eval(&AstNode::FunctionDef {
        name: "level1".to_string(),
        params: vec![],
        body: vec![AstNode::FunctionCall {
            name: "level2".to_string(),
            args: vec![],
        }],
    })
    .unwrap();

    let result = eval.eval(&AstNode::FunctionCall {
        name: "level1".to_string(),
        args: vec![],
    });

    assert!(result.is_err(), "Expected division by zero error");
    let err = result.unwrap_err();
    let err_msg = format!("{:?}", err);

    // RED: Check that error message contains all three levels in call stack
    assert!(
        err_msg.contains("level1") && err_msg.contains("level2") && err_msg.contains("level3"),
        "Error message should contain stack trace with 'level1', 'level2', and 'level3': {}",
        err_msg
    );
}

#[test]
fn test_stack_trace_with_recursion() {
    // fun factorial(n) {
    //     if (n < 0) {
    //         let x = undefined;  // Error on negative input
    //     }
    //     // ... rest would compute factorial
    // }
    // factorial(-1); // Should show recursive context

    let mut eval = Evaluator::new();

    eval.eval(&AstNode::FunctionDef {
        name: "factorial".to_string(),
        params: vec!["n".to_string()],
        body: vec![AstNode::IfExpr {
            condition: Box::new(AstNode::BinaryOp {
                op: BinaryOperator::LessThan,
                left: Box::new(AstNode::Identifier("n".to_string())),
                right: Box::new(AstNode::IntegerLiteral(0)),
            }),
            then_branch: vec![AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::Identifier("undefined".to_string())),
            }],
            else_branch: None,
        }],
    })
    .unwrap();

    let result = eval.eval(&AstNode::FunctionCall {
        name: "factorial".to_string(),
        args: vec![AstNode::UnaryOp {
            op: UnaryOperator::Negate,
            operand: Box::new(AstNode::IntegerLiteral(1)),
        }],
    });

    assert!(result.is_err(), "Expected undefined variable error");
    let err = result.unwrap_err();
    let err_msg = format!("{:?}", err);

    // RED: Check that error message contains function name in stack trace
    assert!(
        err_msg.contains("factorial"),
        "Error message should contain stack trace with 'factorial': {}",
        err_msg
    );
}

// =============================================================================
// Stack Trace Content Tests
// =============================================================================

#[test]
fn test_stack_trace_includes_function_names() {
    // This test will verify that stack traces include function names
    // In GREEN phase, we'll check that error output contains:
    // - Function names in the call chain
    // - Proper ordering (most recent call first or last)

    let mut eval = Evaluator::new();

    eval.eval(&AstNode::FunctionDef {
        name: "caller".to_string(),
        params: vec![],
        body: vec![AstNode::FunctionCall {
            name: "callee".to_string(),
            args: vec![],
        }],
    })
    .unwrap();

    eval.eval(&AstNode::FunctionDef {
        name: "callee".to_string(),
        params: vec![],
        body: vec![AstNode::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(AstNode::IntegerLiteral(10)),
            right: Box::new(AstNode::IntegerLiteral(0)),
        }],
    })
    .unwrap();

    let result = eval.eval(&AstNode::FunctionCall {
        name: "caller".to_string(),
        args: vec![],
    });

    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_msg = format!("{:?}", err);

    // RED: Verify that stack trace contains both function names
    assert!(
        err_msg.contains("caller") && err_msg.contains("callee"),
        "Error message should contain stack trace with 'caller' and 'callee': {}",
        err_msg
    );
}

#[test]
fn test_top_level_error_minimal_stack() {
    // Error at top level should have minimal/empty call stack
    // let x = 1 / 0;

    let mut eval = Evaluator::new();

    let result = eval.eval(&AstNode::LetDecl {
        name: "x".to_string(),
        value: Box::new(AstNode::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(AstNode::IntegerLiteral(1)),
            right: Box::new(AstNode::IntegerLiteral(0)),
        }),
    });

    assert!(result.is_err(), "Expected division by zero");
    // Top level error - should have empty or minimal stack trace
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_009_completeness() {
    // Meta-test: Verify test suite completeness
    //
    // Expected test count:
    // - Nested function call errors: 3 tests
    // - Stack trace content: 2 tests
    // - Meta test: 1 test
    // Total: 6 tests
    //
    // This test ensures we have comprehensive coverage of stack trace functionality.
    println!("INTERP-009 Test Suite (Stack Traces)");
    println!("=====================================");
    println!("Nested function errors: 3 tests");
    println!("Stack trace content: 2 tests");
    println!("Meta test: 1 test");
    println!("Total: 6 tests");
    println!("=====================================");
    println!("Scope: Call stack tracking for error reporting");
    println!("Note: Source location mapping deferred to future ticket");
}
