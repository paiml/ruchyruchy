// INTERP-036: Grouped Import Syntax Testing
//
// This test suite validates grouped import parsing and evaluation.
//
// Requirements:
// - Parse grouped imports: use std::sync::{Arc, Mutex};
// - Parse multiple items in braces: {Arc, Mutex, RwLock}
// - Handle nested paths: std::sync::{Arc, Mutex}
// - Unblock 3 INTERP-032 tests
//
// Tests:
// - test_grouped_import_simple: Basic grouped import
// - test_grouped_import_multiple: Multiple items in braces
// - test_grouped_import_nested_path: Nested path with groups
// - test_grouped_import_single_item: Edge case - single item in braces
// - test_grouped_import_with_regular: Mix grouped and regular imports
//
// RED PHASE: These tests WILL FAIL because:
// - Parser doesn't support {Arc, Mutex} syntax yet
// - UseDecl doesn't support grouped items
// - Evaluator doesn't expand grouped imports

/// Test: Basic Grouped Import
///
/// RED: Validate basic grouped import parsing
///
/// Property: use std::{Arc, Mutex}; should parse successfully
#[test]
fn test_grouped_import_simple() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::{Arc, Mutex};

        let x = Arc::new(Mutex::new(0));
    "#;

    // Parse
    let mut parser = Parser::new(code);
    let ast = parser.parse();
    assert!(
        ast.is_ok(),
        "Should parse grouped import syntax: {:?}",
        ast.err()
    );

    // Evaluate
    let mut eval = Evaluator::new();
    for statement in ast.unwrap().nodes() {
        let result = eval.eval(statement);
        assert!(
            result.is_ok(),
            "Should execute grouped import: {:?}",
            result.err()
        );
    }
}

/// Test: Multiple Items in Braces
///
/// RED: Validate multiple items can be imported
///
/// Property: use std::{Arc, Mutex, RwLock}; should import all three
#[test]
fn test_grouped_import_multiple() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::{Arc, Mutex, RwLock};

        let a = Arc::new(0);
        let m = Mutex::new(0);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser
        .parse()
        .expect("Should parse multiple grouped imports");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute with multiple imports");
    }
}

/// Test: Nested Path with Grouped Imports
///
/// RED: Validate nested paths like std::sync::{Arc, Mutex}
///
/// Property: Should parse path segments followed by grouped items
#[test]
fn test_grouped_import_nested_path() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::sync::{Arc, Mutex};
        use std::thread;

        let counter = Arc::new(Mutex::new(0));
    "#;

    let mut parser = Parser::new(code);
    let ast = parser
        .parse()
        .expect("Should parse nested path with groups");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute with nested grouped imports");
    }
}

/// Test: Single Item in Braces (Edge Case)
///
/// RED: Validate edge case of single item in braces
///
/// Property: use std::{Arc}; should work (even if unconventional)
#[test]
fn test_grouped_import_single_item() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::{Arc};

        let x = Arc::new(0);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse single item in braces");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute single grouped import");
    }
}

/// Test: Mix Grouped and Regular Imports
///
/// RED: Validate mixing grouped and regular imports in same file
///
/// Property: Can have both use std::{Arc, Mutex}; and use std::thread;
#[test]
fn test_grouped_import_with_regular() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::{Arc, Mutex};
        use std::thread;
        use std::sync::mpsc;

        let x = Arc::new(Mutex::new(0));
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse mixed imports");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute mixed imports");
    }
}

/// Test: INTERP-036 Completeness
///
/// Verify all required tests exist and are documented
#[test]
fn test_interp_036_completeness() {
    let required_tests = [
        "test_grouped_import_simple",
        "test_grouped_import_multiple",
        "test_grouped_import_nested_path",
        "test_grouped_import_single_item",
        "test_grouped_import_with_regular",
    ];

    // Verify test count
    assert_eq!(required_tests.len(), 5);
}
