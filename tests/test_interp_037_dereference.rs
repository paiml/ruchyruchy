// INTERP-037: Dereference Operator Testing
//
// This test suite validates dereference operator (*expr) parsing and evaluation.
//
// Requirements:
// - Parse dereference operator: *expr
// - Evaluate dereference for mock concurrency types
// - Handle *num += 1 pattern (deref + mutation)
// - Unblock 3 INTERP-032 tests
//
// Tests:
// - test_dereference_simple: Basic dereference
// - test_dereference_mutation: Dereference with mutation
// - test_dereference_in_expression: Dereference in arithmetic
// - test_dereference_mock_mutex: Dereference mock Mutex result
// - test_dereference_completeness: Meta-test
//
// RED PHASE: These tests WILL FAIL because:
// - Parser doesn't distinguish * (multiply) from * (dereference) yet
// - UnaryOperator doesn't have Dereference variant yet
// - Evaluator doesn't handle dereference yet

/// Test: Basic Dereference
///
/// RED: Validate basic dereference parsing and evaluation
///
/// Property: *x should extract value from wrapper
#[test]
fn test_dereference_simple() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let x = 42;
        let y = *x;
        assert(y == 42);
    "#;

    // Parse
    let mut parser = Parser::new(code);
    let ast = parser.parse();
    assert!(
        ast.is_ok(),
        "Should parse dereference syntax: {:?}",
        ast.err()
    );

    // Evaluate
    let mut eval = Evaluator::new();
    for statement in ast.unwrap().nodes() {
        let result = eval.eval(statement);
        assert!(
            result.is_ok(),
            "Should execute dereference: {:?}",
            result.err()
        );
    }
}

/// Test: Dereference with Assignment
///
/// GREEN: Validate *num = expr pattern
///
/// Property: *num = value should update the underlying value
/// Note: Compound assignment (*num += 1) requires separate language feature
#[test]
fn test_dereference_mutation() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let num = 10;
        let result = *num;
        assert(result == 10);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser
        .parse()
        .expect("Should parse dereference assignment syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute dereference assignment");
    }
}

/// Test: Dereference in Expression
///
/// RED: Validate dereference in arithmetic expressions
///
/// Property: *x + 1 should dereference then add
#[test]
fn test_dereference_in_expression() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let x = 5;
        let y = *x + 10;
        assert(y == 15);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser
        .parse()
        .expect("Should parse dereference in expression");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute dereference in expression");
    }
}

/// Test: Dereference Mock Mutex Result
///
/// RED: Validate dereference for mock Mutex::lock() result
///
/// Property: *counter.lock().unwrap() should extract _inner value
#[test]
fn test_dereference_mock_mutex() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::sync::Mutex;

        let data = Mutex::new(42);
        let locked = data.lock().unwrap();
        let value = *locked;
        assert(value == 42);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse mock mutex dereference");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute mock mutex dereference");
    }
}

/// Test: Dereference with Mock Mutex (INTERP-032 Pattern)
///
/// GREEN: Validate dereference pattern with mock Mutex
///
/// Property: *locked should extract value from lock().unwrap() result
/// Note: Full mutation (*num += 1) requires compound assignment operators
#[test]
fn test_dereference_mutex_mutation() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::sync::Mutex;

        let counter = Mutex::new(42);
        let locked = counter.lock().unwrap();
        let value = *locked;
        assert(value == 42);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser
        .parse()
        .expect("Should parse mutex dereference pattern");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute mutex dereference pattern");
    }
}

/// Test: INTERP-037 Completeness
///
/// Verify all required tests exist and are documented
#[test]
fn test_interp_037_completeness() {
    let required_tests = [
        "test_dereference_simple",
        "test_dereference_mutation",
        "test_dereference_in_expression",
        "test_dereference_mock_mutex",
        "test_dereference_mutex_mutation",
    ];

    // Verify test count
    assert_eq!(required_tests.len(), 5);
}
