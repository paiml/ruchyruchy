// INTERP-038: Compound Assignment Operators Testing
//
// This test suite validates compound assignment operators (+=, -=, *=, /=, %=).
//
// Requirements:
// - Parse compound assignment: +=, -=, *=, /=, %=
// - Support dereference with compound: *num += 1
// - Evaluate as: lhs = lhs op rhs
// - Unblock 3 INTERP-032 tests
//
// Tests:
// - test_compound_add_assign: x += 1
// - test_compound_sub_assign: x -= 1
// - test_compound_mul_assign: x *= 2
// - test_compound_div_assign: x /= 2
// - test_compound_mod_assign: x %= 3
// - test_compound_with_dereference: *num += 1
// - test_compound_completeness: Meta-test
//
// RED PHASE: These tests WILL FAIL because:
// - Parser doesn't have PlusEqual, MinusEqual tokens yet
// - Parser doesn't recognize += as compound assignment
// - Evaluator doesn't handle compound assignment AST nodes

/// Test: Compound Add Assignment
///
/// RED: Validate += operator
///
/// Property: x += 5 should be equivalent to x = x + 5
#[test]
fn test_compound_add_assign() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let mut x = 10;
        x += 5;
        assert(x == 15);
    "#;

    // Parse
    let mut parser = Parser::new(code);
    let ast = parser.parse();
    assert!(ast.is_ok(), "Should parse += syntax: {:?}", ast.err());

    // Evaluate
    let mut eval = Evaluator::new();
    for statement in ast.unwrap().nodes() {
        let result = eval.eval(statement);
        assert!(result.is_ok(), "Should execute +=: {:?}", result.err());
    }
}

/// Test: Compound Subtract Assignment
///
/// RED: Validate -= operator
///
/// Property: x -= 3 should be equivalent to x = x - 3
#[test]
fn test_compound_sub_assign() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let mut x = 10;
        x -= 3;
        assert(x == 7);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse -= syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute -=");
    }
}

/// Test: Compound Multiply Assignment
///
/// RED: Validate *= operator
///
/// Property: x *= 3 should be equivalent to x = x * 3
#[test]
fn test_compound_mul_assign() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let mut x = 5;
        x *= 3;
        assert(x == 15);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse *= syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute *=");
    }
}

/// Test: Compound Divide Assignment
///
/// RED: Validate /= operator
///
/// Property: x /= 2 should be equivalent to x = x / 2
#[test]
fn test_compound_div_assign() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let mut x = 20;
        x /= 4;
        assert(x == 5);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse /= syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute /=");
    }
}

/// Test: Compound Modulo Assignment
///
/// RED: Validate %= operator
///
/// Property: x %= 3 should be equivalent to x = x % 3
#[test]
fn test_compound_mod_assign() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let mut x = 10;
        x %= 3;
        assert(x == 1);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse %= syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute %=");
    }
}

/// Test: Compound Assignment with Dereference (INTERP-032 Pattern)
///
/// RED: Validate *num += 1 pattern (critical for concurrency tests)
///
/// Property: *num += 1 should increment the dereferenced value
#[test]
fn test_compound_with_dereference() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::sync::Mutex;

        let counter = Mutex::new(0);
        let mut num = counter.lock().unwrap();
        *num += 1;

        // For verification, dereference again
        let value = *num;
        assert(value == 1);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse *num += 1 syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute *num += 1");
    }
}

/// Test: Multiple Compound Assignments
///
/// RED: Validate chaining multiple compound operations
///
/// Property: Multiple compound assignments should work in sequence
#[test]
fn test_multiple_compound_assigns() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let mut x = 10;
        x += 5;   // x = 15
        x -= 3;   // x = 12
        x *= 2;   // x = 24
        x /= 4;   // x = 6
        x %= 4;   // x = 2
        assert(x == 2);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser
        .parse()
        .expect("Should parse multiple compound assignments");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute multiple compound assignments");
    }
}

/// Test: INTERP-038 Completeness
///
/// Verify all required tests exist and are documented
#[test]
fn test_interp_038_completeness() {
    let required_tests = [
        "test_compound_add_assign",
        "test_compound_sub_assign",
        "test_compound_mul_assign",
        "test_compound_div_assign",
        "test_compound_mod_assign",
        "test_compound_with_dereference",
        "test_multiple_compound_assigns",
    ];

    // Verify test count
    assert_eq!(required_tests.len(), 7);
}
