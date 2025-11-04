// INTERP-076 (JIT-023): F-String Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement f-string interpolation in JIT compiler
//
// What we need to support:
// 1. Simple f-string with one integer interpolation: f"value: {x}"
// 2. F-string with multiple interpolations: f"{a} + {b} = {c}"
// 3. F-string with integer expressions: f"result: {10 + 5}"
// 4. F-string with variable references: f"x = {x}"
// 5. Plain text f-strings (no interpolation): f"plain text"
// 6. Empty f-strings: f""
//
// Why this is critical:
// - F-strings are idiomatic for string formatting
// - Essential for debugging and logging
// - Common pattern in modern code
//
// Implementation strategy (MVP):
// - Parse f-string content to extract literal parts and {expr} parts
// - Compile each expression to get i64 value
// - For MVP: Return a simple hash/representation of the string
// - Full implementation would need:
//   - Runtime helper for i64-to-string conversion
//   - String concatenation runtime function
//   - Heap-allocated result strings
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::AstNode;
use ruchyruchy::jit::JitCompiler;

/// Test: Simple f-string with one integer interpolation
///
/// Validates: let x = 42; let s = f"value: {x}"; return 1;
/// MVP: Just ensure it compiles and runs without error
#[test]
fn test_compile_fstring_single_interpolation() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 42;
    //     let s = f"value: {x}";
    //     return 1;  // Just verify it compiles
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(42)),
            },
            AstNode::LetDecl {
                name: "s".to_string(),
                value: Box::new(AstNode::FString {
                    content: "value: {x}".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile f-string with single interpolation");

    assert_eq!(main(), 1, "f-string should compile successfully");
}

/// Test: F-string with integer expression interpolation
///
/// Validates: let s = f"result: {10 + 5}"; return 1;
#[test]
fn test_compile_fstring_expression_interpolation() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let s = f"result: {10 + 5}";
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "s".to_string(),
                value: Box::new(AstNode::FString {
                    content: "result: {10 + 5}".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile f-string with expression interpolation");

    assert_eq!(main(), 1, "f-string with expression should compile");
}

/// Test: F-string with multiple interpolations
///
/// Validates: let a = 10; let b = 20; let s = f"{a} + {b} = {a + b}"; return 1;
#[test]
fn test_compile_fstring_multiple_interpolations() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let a = 10;
    //     let b = 20;
    //     let s = f"{a} + {b} = {a + b}";
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "a".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::LetDecl {
                name: "b".to_string(),
                value: Box::new(AstNode::IntegerLiteral(20)),
            },
            AstNode::LetDecl {
                name: "s".to_string(),
                value: Box::new(AstNode::FString {
                    content: "{a} + {b} = {a + b}".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile f-string with multiple interpolations");

    assert_eq!(
        main(),
        1,
        "f-string with multiple interpolations should compile"
    );
}

/// Test: Plain text f-string (no interpolation)
///
/// Validates: let s = f"plain text"; return 1;
#[test]
fn test_compile_fstring_plain_text() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let s = f"plain text";
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "s".to_string(),
                value: Box::new(AstNode::FString {
                    content: "plain text".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile plain text f-string");

    assert_eq!(main(), 1, "plain text f-string should compile");
}

/// Test: Empty f-string
///
/// Validates: let s = f""; return 1;
#[test]
fn test_compile_fstring_empty() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let s = f"";
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "s".to_string(),
                value: Box::new(AstNode::FString {
                    content: "".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile empty f-string");

    assert_eq!(main(), 1, "empty f-string should compile");
}

/// Test: F-string with nested expression
///
/// Validates: let x = 5; let s = f"result: {x * 2 + 1}"; return 1;
#[test]
fn test_compile_fstring_nested_expression() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 5;
    //     let s = f"result: {x * 2 + 1}";
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(5)),
            },
            AstNode::LetDecl {
                name: "s".to_string(),
                value: Box::new(AstNode::FString {
                    content: "result: {x * 2 + 1}".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile f-string with nested expression");

    assert_eq!(main(), 1, "f-string with nested expression should compile");
}

/// Test: F-string with only interpolations (no literal text)
///
/// Validates: let a = 10; let b = 20; let s = f"{a}{b}"; return 1;
#[test]
fn test_compile_fstring_only_interpolations() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let a = 10;
    //     let b = 20;
    //     let s = f"{a}{b}";
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "a".to_string(),
                value: Box::new(AstNode::IntegerLiteral(10)),
            },
            AstNode::LetDecl {
                name: "b".to_string(),
                value: Box::new(AstNode::IntegerLiteral(20)),
            },
            AstNode::LetDecl {
                name: "s".to_string(),
                value: Box::new(AstNode::FString {
                    content: "{a}{b}".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile f-string with only interpolations");

    assert_eq!(
        main(),
        1,
        "f-string with only interpolations should compile"
    );
}

/// Test: F-string with literal braces (escaped)
/// Note: In MVP, we'll skip this test - escaping is complex
#[test]
#[ignore]
fn test_compile_fstring_escaped_braces() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let s = f"{{escaped}}";
    //     return 1;
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "s".to_string(),
                value: Box::new(AstNode::FString {
                    content: "{{escaped}}".to_string(),
                }),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::IntegerLiteral(1))),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile f-string with escaped braces");

    assert_eq!(main(), 1, "f-string with escaped braces should compile");
}
