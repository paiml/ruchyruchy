// INTERP-018: Chapter 8 Examples - Common Collections (Vec, String, HashMap)
//
// EXTREME TDD - RED Phase
//
// Mission: Implement Rust Book Chapter 8 examples covering:
// - Vec<T>: Growable arrays with vec! macro, push/pop, indexing
// - String: UTF-8 text with concatenation, push_str/push, iteration
// - HashMap<K,V>: Key-value storage with insert/get/entry API
//
// Test Strategy:
// - 10 tests covering major collection operations
// - Simple execution validation (RED phase - features not implemented)
// - All tests WILL FAIL initially due to missing vec!/String/HashMap support

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;

/// Helper function to execute a program
fn execute_program(source: &str) -> Result<(), String> {
    let mut parser = Parser::new(source);
    let ast = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    let mut evaluator = Evaluator::new();

    for node in ast.nodes() {
        evaluator
            .eval(node)
            .map_err(|e| format!("Eval error: {:?}", e))?;
    }

    Ok(())
}

/// Test 1: Creating vectors with vec! macro
#[test]
fn test_vec_creation() {
    let source = r#"
fun main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6, 7, 8];
    println(v1);
    println(v2);
}

main();
"#;

    let result = execute_program(source);
    assert!(result.is_ok(), "vec! macro should work: {:?}", result);
}

/// Test 2: Vector push and pop operations
#[test]
fn test_vec_push_pop() {
    let source = r#"
fun main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    println(v);

    let last = v.pop();
    println(last);
    println(v);
}

main();
"#;

    let result = execute_program(source);
    assert!(result.is_ok(), "vec push/pop should work: {:?}", result);
}

/// Test 3: Vector indexing
#[test]
fn test_vec_indexing() {
    let source = r#"
fun main() {
    let v = vec![10, 20, 30, 40, 50];
    let first = v[0];
    let third = v[2];
    println(first);
    println(third);
}

main();
"#;

    let result = execute_program(source);
    assert!(result.is_ok(), "vec indexing should work: {:?}", result);
}

/// Test 4: Vector iteration
#[test]
fn test_vec_iteration() {
    let source = r#"
fun main() {
    let v = vec![100, 200, 300];
    for element in v {
        println(element);
    }
}

main();
"#;

    let result = execute_program(source);
    assert!(result.is_ok(), "vec iteration should work: {:?}", result);
}

/// Test 5: Vector len and is_empty methods
#[test]
fn test_vec_len_empty() {
    let source = r#"
fun main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![];
    println(v1.len());
    println(v1.is_empty());
    println(v2.len());
    println(v2.is_empty());
}

main();
"#;

    let result = execute_program(source);
    assert!(result.is_ok(), "vec len/is_empty should work: {:?}", result);
}

/// Test 6: String creation and methods
#[test]
fn test_string_creation() {
    let source = r#"
fun main() {
    let s1 = String::new();
    let s2 = "hello".to_string();
    let s3 = String::from("world");
    println(s2);
    println(s3);
}

main();
"#;

    let result = execute_program(source);
    assert!(result.is_ok(), "String creation should work: {:?}", result);
}

/// Test 7: String push_str and push
#[test]
fn test_string_push() {
    let source = r#"
fun main() {
    let mut s = String::from("Hello");
    s.push_str(" world");
    s.push('!');
    println(s);
}

main();
"#;

    let result = execute_program(source);
    assert!(result.is_ok(), "String push should work: {:?}", result);
}

/// Test 8: String concatenation
#[test]
fn test_string_concat() {
    let source = r#"
fun main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + s2;
    println(s3);
}

main();
"#;

    let result = execute_program(source);
    assert!(result.is_ok(), "String concat should work: {:?}", result);
}

/// Test 9: HashMap creation and operations
#[test]
fn test_hashmap_basic() {
    let source = r#"
fun main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 50);

    let blue = scores.get("Blue");
    println(blue);

    scores.insert("Blue", 25);
    let blue2 = scores.get("Blue");
    println(blue2);
}

main();
"#;

    let result = execute_program(source);
    assert!(result.is_ok(), "HashMap should work: {:?}", result);
}

/// Test 10: HashMap iteration
#[test]
fn test_hashmap_iteration() {
    let source = r#"
fun main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    for (key, value) in scores {
        println(key);
        println(value);
    }
}

main();
"#;

    let result = execute_program(source);
    assert!(
        result.is_ok(),
        "HashMap iteration should work: {:?}",
        result
    );
}
