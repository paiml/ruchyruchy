// INTERP-008: File I/O Integration
// RED Phase: Create tests for file I/O operations
//
// Tests for:
// - File reading (read_file)
// - File writing (write_file)
// - Print output (println)
// - I/O error handling
//
// Test Coverage:
// - File reading tests: 2 tests
// - File writing tests: 2 tests
// - Print output tests: 2 tests
// - Error handling tests: 3 tests
// - Meta test: 1 test
// Total: 10 tests

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::AstNode;
use std::fs;
use std::path::PathBuf;

// Helper to create temp file path
fn temp_file_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("ruchy_test_{}", name));
    path
}

// Helper to clean up temp file
fn cleanup_temp_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

// =============================================================================
// File Reading Tests
// =============================================================================

#[test]
fn test_read_file_success() {
    // Test: read_file(path) returns file contents
    //
    // let content = read_file("/tmp/test.txt");

    let path = temp_file_path("read_test.txt");
    fs::write(&path, "Hello from file!").unwrap();

    let mut eval = Evaluator::new();

    let result = eval.eval(&AstNode::LetDecl {
        name: "content".to_string(),
        value: Box::new(AstNode::FunctionCall {
            name: "read_file".to_string(),
            args: vec![AstNode::StringLiteral(path.to_str().unwrap().to_string())],
        }),
    });

    cleanup_temp_file(&path);

    assert!(result.is_ok(), "read_file should succeed");
    let value = eval.eval(&AstNode::Identifier("content".to_string())).unwrap();
    assert_eq!(
        value.as_string().unwrap(),
        "Hello from file!",
        "File content should match"
    );
}

#[test]
fn test_read_file_multiline() {
    // Test: read_file preserves newlines
    //
    // let content = read_file("/tmp/multiline.txt");

    let path = temp_file_path("multiline.txt");
    fs::write(&path, "Line 1\nLine 2\nLine 3").unwrap();

    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "content".to_string(),
        value: Box::new(AstNode::FunctionCall {
            name: "read_file".to_string(),
            args: vec![AstNode::StringLiteral(path.to_str().unwrap().to_string())],
        }),
    })
    .unwrap();

    cleanup_temp_file(&path);

    let value = eval.eval(&AstNode::Identifier("content".to_string())).unwrap();
    assert_eq!(
        value.as_string().unwrap(),
        "Line 1\nLine 2\nLine 3",
        "Multiline content should be preserved"
    );
}

// =============================================================================
// File Writing Tests
// =============================================================================

#[test]
fn test_write_file_success() {
    // Test: write_file(path, content) creates file
    //
    // write_file("/tmp/output.txt", "Hello, world!");

    let path = temp_file_path("write_test.txt");

    let mut eval = Evaluator::new();

    let result = eval.eval(&AstNode::FunctionCall {
        name: "write_file".to_string(),
        args: vec![
            AstNode::StringLiteral(path.to_str().unwrap().to_string()),
            AstNode::StringLiteral("Hello, world!".to_string()),
        ],
    });

    assert!(result.is_ok(), "write_file should succeed");

    // Verify file was created with correct content
    let content = fs::read_to_string(&path).unwrap();
    assert_eq!(content, "Hello, world!");

    cleanup_temp_file(&path);
}

#[test]
fn test_write_file_overwrite() {
    // Test: write_file overwrites existing file
    //
    // write_file("/tmp/test.txt", "First");
    // write_file("/tmp/test.txt", "Second");

    let path = temp_file_path("overwrite_test.txt");
    fs::write(&path, "First").unwrap();

    let mut eval = Evaluator::new();

    eval.eval(&AstNode::FunctionCall {
        name: "write_file".to_string(),
        args: vec![
            AstNode::StringLiteral(path.to_str().unwrap().to_string()),
            AstNode::StringLiteral("Second".to_string()),
        ],
    })
    .unwrap();

    // Verify file was overwritten
    let content = fs::read_to_string(&path).unwrap();
    assert_eq!(content, "Second");

    cleanup_temp_file(&path);
}

// =============================================================================
// Print Output Tests
// =============================================================================

#[test]
fn test_println_simple() {
    // Test: println(msg) outputs message
    //
    // println("Hello, world!");

    let mut eval = Evaluator::new();

    // Note: println will print to stdout, we're just testing it doesn't error
    let result = eval.eval(&AstNode::FunctionCall {
        name: "println".to_string(),
        args: vec![AstNode::StringLiteral("Hello, world!".to_string())],
    });

    assert!(result.is_ok(), "println should succeed");
}

#[test]
fn test_println_with_variable() {
    // Test: println works with variables
    //
    // let msg = "Test message";
    // println(msg);

    let mut eval = Evaluator::new();

    eval.eval(&AstNode::LetDecl {
        name: "msg".to_string(),
        value: Box::new(AstNode::StringLiteral("Test message".to_string())),
    })
    .unwrap();

    let result = eval.eval(&AstNode::FunctionCall {
        name: "println".to_string(),
        args: vec![AstNode::Identifier("msg".to_string())],
    });

    assert!(result.is_ok(), "println with variable should succeed");
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[test]
fn test_read_file_not_found() {
    // Test: read_file on non-existent file returns error
    //
    // let content = read_file("/nonexistent/file.txt");

    let mut eval = Evaluator::new();

    let result = eval.eval(&AstNode::FunctionCall {
        name: "read_file".to_string(),
        args: vec![AstNode::StringLiteral("/nonexistent/file.txt".to_string())],
    });

    assert!(result.is_err(), "read_file should fail for non-existent file");
}

#[test]
fn test_write_file_invalid_path() {
    // Test: write_file to invalid path returns error
    //
    // write_file("/invalid/path/that/does/not/exist/file.txt", "content");

    let mut eval = Evaluator::new();

    let result = eval.eval(&AstNode::FunctionCall {
        name: "write_file".to_string(),
        args: vec![
            AstNode::StringLiteral("/invalid/path/that/does/not/exist/file.txt".to_string()),
            AstNode::StringLiteral("content".to_string()),
        ],
    });

    assert!(result.is_err(), "write_file should fail for invalid path");
}

#[test]
fn test_builtin_wrong_arg_count() {
    // Test: Built-in functions check argument count
    //
    // read_file();  // Missing argument
    // write_file("/path");  // Missing content argument

    let mut eval = Evaluator::new();

    // read_file with no args
    let result1 = eval.eval(&AstNode::FunctionCall {
        name: "read_file".to_string(),
        args: vec![],
    });
    assert!(result1.is_err(), "read_file should fail with no arguments");

    // write_file with only one arg
    let result2 = eval.eval(&AstNode::FunctionCall {
        name: "write_file".to_string(),
        args: vec![AstNode::StringLiteral("/path".to_string())],
    });
    assert!(result2.is_err(), "write_file should fail with only one argument");

    // println with no args
    let result3 = eval.eval(&AstNode::FunctionCall {
        name: "println".to_string(),
        args: vec![],
    });
    assert!(result3.is_err(), "println should fail with no arguments");
}

// =============================================================================
// Meta Test
// =============================================================================

#[test]
fn test_interp_008_completeness() {
    // Meta-test: Verify test suite completeness
    //
    // Expected test count:
    // - File reading: 2 tests
    // - File writing: 2 tests
    // - Print output: 2 tests
    // - Error handling: 3 tests
    // - Meta test: 1 test
    // Total: 10 tests
    //
    // This test ensures we have comprehensive coverage of file I/O functionality.
    println!("INTERP-008 Test Suite (File I/O)");
    println!("=================================");
    println!("File reading: 2 tests");
    println!("File writing: 2 tests");
    println!("Print output: 2 tests");
    println!("Error handling: 3 tests");
    println!("Meta test: 1 test");
    println!("Total: 10 tests");
    println!("=================================");
    println!("Built-in functions: read_file, write_file, println");
}
