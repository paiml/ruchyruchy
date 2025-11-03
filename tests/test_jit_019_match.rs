// INTERP-071 (JIT-018): Match Expression Support in JIT
//
// EXTREME TDD - RED Phase
//
// Mission: Implement match expression support in JIT compiler
//
// What we need to support:
// 1. Match expressions: match expr { pattern => body, ... }
// 2. Literal patterns: 1, 2, "hello", true, false
// 3. Wildcard pattern: _
// 4. Variable binding pattern: x
// 5. Multiple match arms
// 6. Match as expression (returns value)
//
// Why this is critical:
// - Match is fundamental control flow in Rust/Ruchy
// - Pattern matching enables exhaustive case analysis
// - Foundation for algebraic data types
// - More powerful than if/else chains
//
// Implementation strategy:
// - Compile to if-else chain checking each pattern in order
// - Literal patterns: compare with constant
// - Wildcard pattern: always matches (final else)
// - Variable binding: assign value to variable
// - Use blocks to ensure proper SSA form
//
// Method: Incremental TDD starting with simplest cases

use ruchyruchy::interpreter::parser::{AstNode, MatchArm, Pattern};
use ruchyruchy::jit::JitCompiler;

/// Test: Simple match with integer literal patterns
///
/// Validates: match x { 1 => 10, 2 => 20, _ => 0 }
#[test]
fn test_compile_match_integer_literals() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 2;
    //     return match x {
    //         1 => 10,
    //         2 => 20,
    //         _ => 0
    //     };
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(2)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MatchExpr {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    arms: vec![
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                            body: vec![AstNode::IntegerLiteral(10)],
                        },
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(2)),
                            body: vec![AstNode::IntegerLiteral(20)],
                        },
                        MatchArm {
                            pattern: Pattern::Wildcard,
                            body: vec![AstNode::IntegerLiteral(0)],
                        },
                    ],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile match with integer literals");

    assert_eq!(main(), 20, "match should return 20 for x=2");
}

/// Test: Match with wildcard pattern only
///
/// Validates: match x { _ => 42 }
#[test]
fn test_compile_match_wildcard_only() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 100;
    //     return match x {
    //         _ => 42
    //     };
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(100)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MatchExpr {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    arms: vec![MatchArm {
                        pattern: Pattern::Wildcard,
                        body: vec![AstNode::IntegerLiteral(42)],
                    }],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile match with wildcard");

    assert_eq!(main(), 42, "match with wildcard should return 42");
}

/// Test: Match with variable binding
///
/// Validates: match x { n => n + 10 }
#[test]
fn test_compile_match_variable_binding() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 5;
    //     return match x {
    //         n => n + 10
    //     };
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(5)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MatchExpr {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    arms: vec![MatchArm {
                        pattern: Pattern::Identifier("n".to_string()),
                        body: vec![AstNode::BinaryOp {
                            left: Box::new(AstNode::Identifier("n".to_string())),
                            op: ruchyruchy::interpreter::parser::BinaryOperator::Add,
                            right: Box::new(AstNode::IntegerLiteral(10)),
                        }],
                    }],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile match with variable binding");

    assert_eq!(main(), 15, "match should bind n=5 and return 5+10=15");
}

/// Test: Match with multiple arms, testing first match
///
/// Validates: match x { 1 => 100, 2 => 200, 3 => 300, _ => 0 } with x=1
#[test]
fn test_compile_match_first_arm() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 1;
    //     return match x {
    //         1 => 100,
    //         2 => 200,
    //         3 => 300,
    //         _ => 0
    //     };
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(1)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MatchExpr {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    arms: vec![
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                            body: vec![AstNode::IntegerLiteral(100)],
                        },
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(2)),
                            body: vec![AstNode::IntegerLiteral(200)],
                        },
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(3)),
                            body: vec![AstNode::IntegerLiteral(300)],
                        },
                        MatchArm {
                            pattern: Pattern::Wildcard,
                            body: vec![AstNode::IntegerLiteral(0)],
                        },
                    ],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile match with multiple arms");

    assert_eq!(main(), 100, "match should return 100 for x=1");
}

/// Test: Match with multiple arms, testing last literal match
///
/// Validates: match x { 1 => 100, 2 => 200, 3 => 300, _ => 0 } with x=3
#[test]
fn test_compile_match_last_literal_arm() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 3;
    //     return match x {
    //         1 => 100,
    //         2 => 200,
    //         3 => 300,
    //         _ => 0
    //     };
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(3)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MatchExpr {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    arms: vec![
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                            body: vec![AstNode::IntegerLiteral(100)],
                        },
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(2)),
                            body: vec![AstNode::IntegerLiteral(200)],
                        },
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(3)),
                            body: vec![AstNode::IntegerLiteral(300)],
                        },
                        MatchArm {
                            pattern: Pattern::Wildcard,
                            body: vec![AstNode::IntegerLiteral(0)],
                        },
                    ],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile match with multiple arms");

    assert_eq!(main(), 300, "match should return 300 for x=3");
}

/// Test: Match falling through to wildcard
///
/// Validates: match x { 1 => 100, 2 => 200, _ => 999 } with x=99
#[test]
fn test_compile_match_wildcard_fallthrough() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 99;
    //     return match x {
    //         1 => 100,
    //         2 => 200,
    //         _ => 999
    //     };
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(99)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MatchExpr {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    arms: vec![
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                            body: vec![AstNode::IntegerLiteral(100)],
                        },
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(2)),
                            body: vec![AstNode::IntegerLiteral(200)],
                        },
                        MatchArm {
                            pattern: Pattern::Wildcard,
                            body: vec![AstNode::IntegerLiteral(999)],
                        },
                    ],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile match with wildcard fallthrough");

    assert_eq!(main(), 999, "match should return 999 for x=99 (wildcard)");
}

/// Test: Match with boolean literals
///
/// Validates: match flag { true => 1, false => 0 }
#[test]
fn test_compile_match_boolean_literals() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let flag = true;
    //     return match flag {
    //         true => 1,
    //         false => 0
    //     };
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "flag".to_string(),
                value: Box::new(AstNode::BooleanLiteral(true)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MatchExpr {
                    expr: Box::new(AstNode::Identifier("flag".to_string())),
                    arms: vec![
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::BooleanLiteral(true)),
                            body: vec![AstNode::IntegerLiteral(1)],
                        },
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::BooleanLiteral(false)),
                            body: vec![AstNode::IntegerLiteral(0)],
                        },
                    ],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile match with boolean literals");

    assert_eq!(main(), 1, "match should return 1 for flag=true");
}

/// Test: Match with complex body (multiple statements)
///
/// Validates: match x { 1 => { let y = 10; y + 5 }, _ => 0 }
#[test]
fn test_compile_match_complex_body() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 1;
    //     return match x {
    //         1 => {
    //             let y = 10;
    //             y + 5
    //         },
    //         _ => 0
    //     };
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(1)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MatchExpr {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    arms: vec![
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                            body: vec![
                                AstNode::LetDecl {
                                    name: "y".to_string(),
                                    value: Box::new(AstNode::IntegerLiteral(10)),
                                },
                                AstNode::BinaryOp {
                                    left: Box::new(AstNode::Identifier("y".to_string())),
                                    op: ruchyruchy::interpreter::parser::BinaryOperator::Add,
                                    right: Box::new(AstNode::IntegerLiteral(5)),
                                },
                            ],
                        },
                        MatchArm {
                            pattern: Pattern::Wildcard,
                            body: vec![AstNode::IntegerLiteral(0)],
                        },
                    ],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile match with complex body");

    assert_eq!(main(), 15, "match should execute block and return 10+5=15");
}

/// Test: Nested match expressions
///
/// Validates: match x { 1 => match y { 2 => 12, _ => 10 }, _ => 0 }
#[test]
#[ignore = "Nested match requires proper scoping - future work"]
fn test_compile_nested_match() {
    let mut jit = JitCompiler::new().expect("Failed to create JIT compiler");

    // Function: fun main() {
    //     let x = 1;
    //     let y = 2;
    //     return match x {
    //         1 => match y {
    //             2 => 12,
    //             _ => 10
    //         },
    //         _ => 0
    //     };
    // }
    let body = AstNode::Block {
        statements: vec![
            AstNode::LetDecl {
                name: "x".to_string(),
                value: Box::new(AstNode::IntegerLiteral(1)),
            },
            AstNode::LetDecl {
                name: "y".to_string(),
                value: Box::new(AstNode::IntegerLiteral(2)),
            },
            AstNode::Return {
                value: Some(Box::new(AstNode::MatchExpr {
                    expr: Box::new(AstNode::Identifier("x".to_string())),
                    arms: vec![
                        MatchArm {
                            pattern: Pattern::Literal(AstNode::IntegerLiteral(1)),
                            body: vec![AstNode::MatchExpr {
                                expr: Box::new(AstNode::Identifier("y".to_string())),
                                arms: vec![
                                    MatchArm {
                                        pattern: Pattern::Literal(AstNode::IntegerLiteral(2)),
                                        body: vec![AstNode::IntegerLiteral(12)],
                                    },
                                    MatchArm {
                                        pattern: Pattern::Wildcard,
                                        body: vec![AstNode::IntegerLiteral(10)],
                                    },
                                ],
                            }],
                        },
                        MatchArm {
                            pattern: Pattern::Wildcard,
                            body: vec![AstNode::IntegerLiteral(0)],
                        },
                    ],
                })),
            },
        ],
    };

    let main: fn() -> i64 = jit
        .compile_function_with_params(&[], &body)
        .expect("Should compile nested match");

    assert_eq!(main(), 12, "nested match should return 12");
}
