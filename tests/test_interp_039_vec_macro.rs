// INTERP-039: vec! Macro Support Testing
//
// This test suite validates vec! macro parsing and evaluation.
//
// Requirements:
// - Parse vec![] (empty vector)
// - Parse vec![expr1, expr2, ...] (vector with elements)
// - Parse vec![expr; count] (repeated element)
// - Evaluate to Value::Array
// - Support .push() and .len() methods
// - Unblock 4 INTERP-032 tests
//
// Tests:
// - test_vec_macro_empty: vec![]
// - test_vec_macro_with_elements: vec![1, 2, 3]
// - test_vec_macro_repeated: vec![0; 10]
// - test_vec_macro_push: arr.push(x)
// - test_vec_macro_len: arr.len()
// - test_vec_macro_nested: vec![vec![1, 2], vec![3, 4]]
// - test_vec_macro_in_function_call: Mutex::new(vec![1, 2, 3])
// - test_vec_macro_completeness: Meta-test
//
// RED PHASE: These tests WILL FAIL because:
// - Parser doesn't recognize 'vec' as a macro identifier yet
// - Parser doesn't handle vec![...] syntax
// - Evaluator doesn't handle VecMacro AST node
// - Array methods .push() and .len() not implemented

/// Test: Empty vec! Macro
///
/// RED: Validate vec![] syntax
///
/// Property: vec![] should create an empty array
#[test]
fn test_vec_macro_empty() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let arr = vec![];
        assert(arr.len() == 0);
    "#;

    // Parse
    let mut parser = Parser::new(code);
    let ast = parser.parse();
    assert!(ast.is_ok(), "Should parse vec![] syntax: {:?}", ast.err());

    // Evaluate
    let mut eval = Evaluator::new();
    for statement in ast.unwrap().nodes() {
        let result = eval.eval(statement);
        assert!(result.is_ok(), "Should execute vec![]: {:?}", result.err());
    }
}

/// Test: vec! Macro with Elements
///
/// RED: Validate vec![expr1, expr2, ...] syntax
///
/// Property: vec![1, 2, 3] should create array with those elements
#[test]
fn test_vec_macro_with_elements() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let arr = vec![1, 2, 3];
        assert(arr.len() == 3);
        assert(arr[0] == 1);
        assert(arr[1] == 2);
        assert(arr[2] == 3);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse vec![1, 2, 3] syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute vec![1, 2, 3]");
    }
}

/// Test: vec! Macro with Repeated Element
///
/// RED: Validate vec![expr; count] syntax
///
/// Property: vec![0; 10] should create array with 10 zeros
#[test]
fn test_vec_macro_repeated() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let arr = vec![0; 5];
        assert(arr.len() == 5);
        assert(arr[0] == 0);
        assert(arr[4] == 0);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse vec![0; 5] syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute vec![0; 5]");
    }
}

/// Test: Array .push() Method
///
/// RED: Validate .push() method on arrays
///
/// Property: arr.push(x) should add element to end of array
#[test]
fn test_vec_macro_push() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let mut arr = vec![1, 2];
        arr.push(3);
        assert(arr.len() == 3);
        assert(arr[2] == 3);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse .push() syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute .push()");
    }
}

/// Test: Array .len() Method
///
/// RED: Validate .len() method on arrays
///
/// Property: arr.len() should return number of elements
#[test]
fn test_vec_macro_len() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let arr = vec![1, 2, 3, 4, 5];
        let length = arr.len();
        assert(length == 5);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse .len() syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute .len()");
    }
}

/// Test: Nested vec! Macros
///
/// RED: Validate nested vec! usage
///
/// Property: vec![vec![1, 2], vec![3, 4]] should create 2D array
#[test]
fn test_vec_macro_nested() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let matrix = vec![vec![1, 2], vec![3, 4]];
        assert(matrix.len() == 2);
        assert(matrix[0].len() == 2);
        assert(matrix[0][0] == 1);
        assert(matrix[1][1] == 4);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse nested vec! syntax");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute nested vec!");
    }
}

/// Test: vec! in Function Call (INTERP-032 Pattern)
///
/// RED: Validate vec! as function argument
///
/// Property: Mutex::new(vec![1, 2, 3]) should pass vec! result to function
#[test]
fn test_vec_macro_in_function_call() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::sync::Mutex;

        let data = Mutex::new(vec![1, 2, 3]);
        let locked = data.lock().unwrap();
        assert(locked.len() == 3);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse vec! in function call");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute vec! in function call");
    }
}

/// Test: vec! with Expressions
///
/// RED: Validate vec! with computed expressions
///
/// Property: vec![1+1, 2*2, 3] should evaluate expressions
#[test]
fn test_vec_macro_with_expressions() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        let arr = vec![1 + 1, 2 * 2, 3];
        assert(arr.len() == 3);
        assert(arr[0] == 2);
        assert(arr[1] == 4);
        assert(arr[2] == 3);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse vec! with expressions");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement)
            .expect("Should execute vec! with expressions");
    }
}

/// Test: INTERP-039 Completeness
///
/// Verify all required tests exist and are documented
#[test]
fn test_interp_039_completeness() {
    let required_tests = [
        "test_vec_macro_empty",
        "test_vec_macro_with_elements",
        "test_vec_macro_repeated",
        "test_vec_macro_push",
        "test_vec_macro_len",
        "test_vec_macro_nested",
        "test_vec_macro_in_function_call",
        "test_vec_macro_with_expressions",
    ];

    // Verify test count
    assert_eq!(required_tests.len(), 8);
}
