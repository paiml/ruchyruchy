// INTERP-066 (JIT-013): String Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement basic string support in JIT compiler
//
// What we need to support:
// 1. String literals: "hello world"
// 2. String in variables: let s = "test";
// 3. String concatenation: "hello" + " world"
// 4. String comparison: "abc" == "abc"
// 5. String as return value
//
// Why this is critical:
// - Strings are fundamental data type
// - Required for real-world programs
// - Foundation for I/O and user interaction
// - Enables print statements and debugging
//
// Implementation strategy:
// - Store strings as pointers to heap-allocated data
// - Use length-prefixed format (i64 length + data)
// - String concatenation creates new heap allocation
// - Defer advanced features (substring, formatting, escapes)
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator};
use ruchyruchy::jit::JitCompiler;

/// Test: Simple string literal
///
/// Validates: return "hello";
/// For MVP: String pointers are opaque - we just verify non-null
#[test]
fn test_compile_string_literal() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return "hello"; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::StringLiteral("hello".to_string()))),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile string literal");

    let result = main();
    // For MVP: Just verify we got a non-null pointer
    assert_ne!(result, 0, "String pointer should be non-null");
}

/// Test: String in variable
///
/// Validates: let s = "world"; return s;
#[test]
fn test_compile_string_in_variable() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { let s = "world"; return s; }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "s".to_string(),
                value: Box::new(AstNode::StringLiteral("world".to_string())),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::Identifier("s".to_string()))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile string in variable");

    let result = main();
    assert_ne!(result, 0, "String pointer should be non-null");
}

/// Test: String concatenation
///
/// Validates: return "hello" + " world";
#[test]
fn test_compile_string_concatenation() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return "hello" + " world"; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::StringLiteral("hello".to_string())),
            op: BinaryOperator::Add,
            right: Box::new(AstNode::StringLiteral(" world".to_string())),
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile string concatenation");

    let result = main();
    assert_ne!(result, 0, "Concatenated string pointer should be non-null");
}

/// Test: String comparison (equality)
///
/// Validates: if ("abc" == "abc") { return 1; } else { return 0; }
#[test]
fn test_compile_string_equality() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     if ("abc" == "abc") { return 1; } else { return 0; }
    // }
    let body = AstNode::IfExpr {
        condition: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::StringLiteral("abc".to_string())),
            op: BinaryOperator::Equal,
            right: Box::new(AstNode::StringLiteral("abc".to_string())),
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
        .expect("Should compile string equality");

    assert_eq!(main(), 1, "\"abc\" == \"abc\" should be true");
}

/// Test: String inequality
///
/// Validates: if ("abc" == "def") { return 1; } else { return 0; }
#[test]
fn test_compile_string_inequality() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     if ("abc" == "def") { return 1; } else { return 0; }
    // }
    let body = AstNode::IfExpr {
        condition: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::StringLiteral("abc".to_string())),
            op: BinaryOperator::Equal,
            right: Box::new(AstNode::StringLiteral("def".to_string())),
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
        .expect("Should compile string inequality");

    assert_eq!(main(), 0, "\"abc\" == \"def\" should be false");
}

/// Test: String concatenation with variables
///
/// Validates: let a = "hello"; let b = " world"; return a + b;
#[test]
fn test_compile_string_concatenation_variables() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let a = "hello";
    //     let b = " world";
    //     return a + b;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "a".to_string(),
                value: Box::new(AstNode::StringLiteral("hello".to_string())),
            },
            AstNode::LetDecl {
                name: "b".to_string(),
                value: Box::new(AstNode::StringLiteral(" world".to_string())),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::BinaryOp {
                    left: Box::new(AstNode::Identifier("a".to_string())),
                    op: BinaryOperator::Add,
                    right: Box::new(AstNode::Identifier("b".to_string())),
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile string concatenation with variables");

    let result = main();
    assert_ne!(result, 0, "Concatenated string pointer should be non-null");
}

/// Test: Empty string
///
/// Validates: return "";
#[test]
fn test_compile_empty_string() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() { return ""; }
    let body = AstNode::Return {
        value: Some(Box::new(AstNode::StringLiteral("".to_string()))),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile empty string");

    let result = main();
    // Empty string should still have a valid pointer (to length=0)
    assert_ne!(result, 0, "Empty string pointer should be non-null");
}

/// Test: String as function parameter
///
/// Validates: fun greet(name) { return name; } fun main() { return greet("Alice"); }
#[test]
fn test_compile_string_as_parameter() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Helper: fun greet(name) { return name; }
    let greet_body = AstNode::Return {
        value: Some(Box::new(AstNode::Identifier("name".to_string()))),
    };

    let greet: fn(i64) -> i64 = jit
        .compile_function_with_params(&["name".to_string()], &greet_body)
        .expect("Should compile greet function");

    jit.register_function("greet".to_string(), greet as *const u8);

    // Main: fun main() { return greet("Alice"); }
    let main_body = AstNode::Return {
        value: Some(Box::new(AstNode::FunctionCall {
            name: "greet".to_string(),
            args: vec![AstNode::StringLiteral("Alice".to_string())],
        })),
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &main_body)
        .expect("Should compile main with string parameter");

    let result = main();
    assert_ne!(result, 0, "String parameter should be passed correctly");
}
