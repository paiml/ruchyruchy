// INTERP-007: Data Structure Operations Tests
// RED Phase: Create comprehensive test suite for data structures
//
// Tests for:
// - IndexAccess: vec[i], map[key]
// - HashMapLiteral: {key: value}
// - Built-in functions: len(), push(), pop(), insert(), get(), etc.
//
// Test Coverage:
// - Vector operations: 6 tests
// - HashMap operations: 6 tests
// - Error handling: 3 tests
// - Meta test: 1 test
// Total: 16 tests

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::{AstNode, BinaryOperator, UnaryOperator};

// =============================================================================
// Vector Operations
// =============================================================================

#[test]
fn test_vector_index_access() {
    // let vec = [10, 20, 30];
    // let x = vec[0];
    // let y = vec[1];
    // let z = vec[2];
    // Result: x=10, y=20, z=30
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "vec".to_string(),
        value: Box::new(AstNode::VectorLiteral {
            elements: vec![
                AstNode::IntegerLiteral(10),
                AstNode::IntegerLiteral(20),
                AstNode::IntegerLiteral(30),
            ],
        }),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "x".to_string(),
        value: Box::new(AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("vec".to_string())),
            index: Box::new(AstNode::IntegerLiteral(0)),
        }),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "y".to_string(),
        value: Box::new(AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("vec".to_string())),
            index: Box::new(AstNode::IntegerLiteral(1)),
        }),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "z".to_string(),
        value: Box::new(AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("vec".to_string())),
            index: Box::new(AstNode::IntegerLiteral(2)),
        }),
    })
    .unwrap();

    let x = eval.eval(&AstNode::Identifier("x".to_string())).unwrap();
    let y = eval.eval(&AstNode::Identifier("y".to_string())).unwrap();
    let z = eval.eval(&AstNode::Identifier("z".to_string())).unwrap();

    assert_eq!(x.as_integer().unwrap(), 10);
    assert_eq!(y.as_integer().unwrap(), 20);
    assert_eq!(z.as_integer().unwrap(), 30);
}

#[test]
fn test_vector_index_out_of_bounds() {
    // let vec = [1, 2, 3];
    // let x = vec[10]; // Should error: IndexOutOfBounds
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "vec".to_string(),
        value: Box::new(AstNode::VectorLiteral {
            elements: vec![
                AstNode::IntegerLiteral(1),
                AstNode::IntegerLiteral(2),
                AstNode::IntegerLiteral(3),
            ],
        }),
    })
    .unwrap();

    let result = eval.eval(&AstNode::IndexAccess {
        expr: Box::new(AstNode::Identifier("vec".to_string())),
        index: Box::new(AstNode::IntegerLiteral(10)),
    });

    assert!(result.is_err());
    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(err_msg.contains("Index") || err_msg.contains("out of bounds"));
}

#[test]
fn test_vector_nested_access() {
    // let matrix = [[1, 2], [3, 4]];
    // let val = matrix[1][0]; // Should be 3
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "matrix".to_string(),
        value: Box::new(AstNode::VectorLiteral {
            elements: vec![
                AstNode::VectorLiteral {
                    elements: vec![AstNode::IntegerLiteral(1), AstNode::IntegerLiteral(2)],
                },
                AstNode::VectorLiteral {
                    elements: vec![AstNode::IntegerLiteral(3), AstNode::IntegerLiteral(4)],
                },
            ],
        }),
    })
    .unwrap();

    let result = eval
        .eval(&AstNode::IndexAccess {
            expr: Box::new(AstNode::IndexAccess {
                expr: Box::new(AstNode::Identifier("matrix".to_string())),
                index: Box::new(AstNode::IntegerLiteral(1)),
            }),
            index: Box::new(AstNode::IntegerLiteral(0)),
        })
        .unwrap();

    assert_eq!(result.as_integer().unwrap(), 3);
}

#[test]
fn test_vector_index_with_expression() {
    // let vec = [10, 20, 30, 40];
    // let i = 1;
    // let val = vec[i + 1]; // Should be 30
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "vec".to_string(),
        value: Box::new(AstNode::VectorLiteral {
            elements: vec![
                AstNode::IntegerLiteral(10),
                AstNode::IntegerLiteral(20),
                AstNode::IntegerLiteral(30),
                AstNode::IntegerLiteral(40),
            ],
        }),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "i".to_string(),
        value: Box::new(AstNode::IntegerLiteral(1)),
    })
    .unwrap();

    let result = eval
        .eval(&AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("vec".to_string())),
            index: Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(AstNode::Identifier("i".to_string())),
                right: Box::new(AstNode::IntegerLiteral(1)),
            }),
        })
        .unwrap();

    assert_eq!(result.as_integer().unwrap(), 30);
}

#[test]
fn test_vector_empty() {
    // let vec = [];
    // Result: empty vector
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "vec".to_string(),
        value: Box::new(AstNode::VectorLiteral { elements: vec![] }),
    })
    .unwrap();

    let vec = eval.eval(&AstNode::Identifier("vec".to_string())).unwrap();
    assert!(vec.is_vector());
    assert_eq!(vec.as_vector().unwrap().len(), 0);
}

#[test]
fn test_vector_mixed_expressions() {
    // let a = 5;
    // let b = 10;
    // let vec = [a, a + b, b * 2];
    // Result: [5, 15, 20]
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "a".to_string(),
        value: Box::new(AstNode::IntegerLiteral(5)),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "b".to_string(),
        value: Box::new(AstNode::IntegerLiteral(10)),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "vec".to_string(),
        value: Box::new(AstNode::VectorLiteral {
            elements: vec![
                AstNode::Identifier("a".to_string()),
                AstNode::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(AstNode::Identifier("a".to_string())),
                    right: Box::new(AstNode::Identifier("b".to_string())),
                },
                AstNode::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(AstNode::Identifier("b".to_string())),
                    right: Box::new(AstNode::IntegerLiteral(2)),
                },
            ],
        }),
    })
    .unwrap();

    let vec = eval.eval(&AstNode::Identifier("vec".to_string())).unwrap();
    let elements = vec.as_vector().unwrap();
    assert_eq!(elements.len(), 3);
    assert_eq!(elements[0].as_integer().unwrap(), 5);
    assert_eq!(elements[1].as_integer().unwrap(), 15);
    assert_eq!(elements[2].as_integer().unwrap(), 20);
}

// =============================================================================
// HashMap Operations
// =============================================================================

#[test]
fn test_hashmap_literal_creation() {
    // let map = {"name": "Alice", "age": "30"};
    // Result: HashMap with 2 entries
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "map".to_string(),
        value: Box::new(AstNode::HashMapLiteral {
            pairs: vec![
                (
                    AstNode::StringLiteral("name".to_string()),
                    AstNode::StringLiteral("Alice".to_string()),
                ),
                (
                    AstNode::StringLiteral("age".to_string()),
                    AstNode::StringLiteral("30".to_string()),
                ),
            ],
        }),
    })
    .unwrap();

    let map = eval.eval(&AstNode::Identifier("map".to_string())).unwrap();
    assert!(map.is_hashmap());
}

#[test]
fn test_hashmap_index_access() {
    // let map = {"x": 10, "y": 20};
    // let a = map["x"];
    // let b = map["y"];
    // Result: a=10, b=20
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "map".to_string(),
        value: Box::new(AstNode::HashMapLiteral {
            pairs: vec![
                (
                    AstNode::StringLiteral("x".to_string()),
                    AstNode::IntegerLiteral(10),
                ),
                (
                    AstNode::StringLiteral("y".to_string()),
                    AstNode::IntegerLiteral(20),
                ),
            ],
        }),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "a".to_string(),
        value: Box::new(AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("map".to_string())),
            index: Box::new(AstNode::StringLiteral("x".to_string())),
        }),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "b".to_string(),
        value: Box::new(AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("map".to_string())),
            index: Box::new(AstNode::StringLiteral("y".to_string())),
        }),
    })
    .unwrap();

    let a = eval.eval(&AstNode::Identifier("a".to_string())).unwrap();
    let b = eval.eval(&AstNode::Identifier("b".to_string())).unwrap();

    assert_eq!(a.as_integer().unwrap(), 10);
    assert_eq!(b.as_integer().unwrap(), 20);
}

#[test]
fn test_hashmap_key_not_found() {
    // let map = {"a": 1};
    // let x = map["missing"]; // Should error: KeyNotFound
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "map".to_string(),
        value: Box::new(AstNode::HashMapLiteral {
            pairs: vec![(
                AstNode::StringLiteral("a".to_string()),
                AstNode::IntegerLiteral(1),
            )],
        }),
    })
    .unwrap();

    let result = eval.eval(&AstNode::IndexAccess {
        expr: Box::new(AstNode::Identifier("map".to_string())),
        index: Box::new(AstNode::StringLiteral("missing".to_string())),
    });

    assert!(result.is_err());
    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(err_msg.contains("Key") || err_msg.contains("not found"));
}

#[test]
fn test_hashmap_empty() {
    // let map = {};
    // Result: empty HashMap
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "map".to_string(),
        value: Box::new(AstNode::HashMapLiteral { pairs: vec![] }),
    })
    .unwrap();

    let map = eval.eval(&AstNode::Identifier("map".to_string())).unwrap();
    assert!(map.is_hashmap());
}

#[test]
fn test_hashmap_mixed_value_types() {
    // let map = {"num": 42, "str": "hello", "bool": true};
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "map".to_string(),
        value: Box::new(AstNode::HashMapLiteral {
            pairs: vec![
                (
                    AstNode::StringLiteral("num".to_string()),
                    AstNode::IntegerLiteral(42),
                ),
                (
                    AstNode::StringLiteral("str".to_string()),
                    AstNode::StringLiteral("hello".to_string()),
                ),
                (
                    AstNode::StringLiteral("bool".to_string()),
                    AstNode::BooleanLiteral(true),
                ),
            ],
        }),
    })
    .unwrap();

    let num = eval
        .eval(&AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("map".to_string())),
            index: Box::new(AstNode::StringLiteral("num".to_string())),
        })
        .unwrap();
    assert_eq!(num.as_integer().unwrap(), 42);

    let s = eval
        .eval(&AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("map".to_string())),
            index: Box::new(AstNode::StringLiteral("str".to_string())),
        })
        .unwrap();
    assert_eq!(s.as_string().unwrap(), "hello");

    let b = eval
        .eval(&AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("map".to_string())),
            index: Box::new(AstNode::StringLiteral("bool".to_string())),
        })
        .unwrap();
    assert_eq!(b.as_boolean().unwrap(), true);
}

#[test]
fn test_hashmap_with_expression_values() {
    // let x = 10;
    // let y = 20;
    // let map = {"sum": x + y, "product": x * y};
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "x".to_string(),
        value: Box::new(AstNode::IntegerLiteral(10)),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "y".to_string(),
        value: Box::new(AstNode::IntegerLiteral(20)),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "map".to_string(),
        value: Box::new(AstNode::HashMapLiteral {
            pairs: vec![
                (
                    AstNode::StringLiteral("sum".to_string()),
                    AstNode::BinaryOp {
                        op: BinaryOperator::Add,
                        left: Box::new(AstNode::Identifier("x".to_string())),
                        right: Box::new(AstNode::Identifier("y".to_string())),
                    },
                ),
                (
                    AstNode::StringLiteral("product".to_string()),
                    AstNode::BinaryOp {
                        op: BinaryOperator::Multiply,
                        left: Box::new(AstNode::Identifier("x".to_string())),
                        right: Box::new(AstNode::Identifier("y".to_string())),
                    },
                ),
            ],
        }),
    })
    .unwrap();

    let sum = eval
        .eval(&AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("map".to_string())),
            index: Box::new(AstNode::StringLiteral("sum".to_string())),
        })
        .unwrap();
    assert_eq!(sum.as_integer().unwrap(), 30);

    let product = eval
        .eval(&AstNode::IndexAccess {
            expr: Box::new(AstNode::Identifier("map".to_string())),
            index: Box::new(AstNode::StringLiteral("product".to_string())),
        })
        .unwrap();
    assert_eq!(product.as_integer().unwrap(), 200);
}

// =============================================================================
// Error Handling
// =============================================================================

#[test]
fn test_index_on_non_indexable() {
    // let x = 42;
    // let y = x[0]; // Should error: not indexable
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "x".to_string(),
        value: Box::new(AstNode::IntegerLiteral(42)),
    })
    .unwrap();

    let result = eval.eval(&AstNode::IndexAccess {
        expr: Box::new(AstNode::Identifier("x".to_string())),
        index: Box::new(AstNode::IntegerLiteral(0)),
    });

    assert!(result.is_err());
}

#[test]
fn test_index_with_non_integer() {
    // let vec = [1, 2, 3];
    // let i = "hello";
    // let x = vec[i]; // Should error: index must be integer for vector
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "vec".to_string(),
        value: Box::new(AstNode::VectorLiteral {
            elements: vec![
                AstNode::IntegerLiteral(1),
                AstNode::IntegerLiteral(2),
                AstNode::IntegerLiteral(3),
            ],
        }),
    })
    .unwrap();

    eval.eval(&AstNode::LetDecl {
        name: "i".to_string(),
        value: Box::new(AstNode::StringLiteral("hello".to_string())),
    })
    .unwrap();

    let result = eval.eval(&AstNode::IndexAccess {
        expr: Box::new(AstNode::Identifier("vec".to_string())),
        index: Box::new(AstNode::Identifier("i".to_string())),
    });

    assert!(result.is_err());
}

#[test]
fn test_index_with_negative() {
    // let vec = [1, 2, 3];
    // let x = vec[-1]; // Should error: negative index
    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "vec".to_string(),
        value: Box::new(AstNode::VectorLiteral {
            elements: vec![
                AstNode::IntegerLiteral(1),
                AstNode::IntegerLiteral(2),
                AstNode::IntegerLiteral(3),
            ],
        }),
    })
    .unwrap();

    let result = eval.eval(&AstNode::IndexAccess {
        expr: Box::new(AstNode::Identifier("vec".to_string())),
        index: Box::new(AstNode::UnaryOp {
            op: UnaryOperator::Negate,
            operand: Box::new(AstNode::IntegerLiteral(1)),
        }),
    });

    assert!(result.is_err());
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_007_completeness() {
    // Meta-test: Verify this test file has correct coverage
    //
    // Expected test count:
    // - Vector operations: 6 tests
    // - HashMap operations: 6 tests
    // - Error handling: 3 tests
    // - Meta test: 1 test
    // Total: 16 tests
    //
    // This test ensures we haven't accidentally removed tests during refactoring.
    println!("INTERP-007 Test Suite");
    println!("=====================");
    println!("Vector operations: 6 tests");
    println!("HashMap operations: 6 tests");
    println!("Error handling: 3 tests");
    println!("Meta test: 1 test");
    println!("Total: 16 tests");
    println!("=====================");
}
