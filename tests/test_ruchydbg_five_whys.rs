// DEBUGGER-056: ruchydbg five-whys Command Tests (RED PHASE)
//
// EXTREME TDD - RED Phase
//
// Tests for `ruchydbg five-whys` command with JSON input/output and interactive mode
//
// Expected behavior:
// - Read bug report from JSON file
// - Perform Five Whys analysis
// - Output analysis as JSON or human-readable format
// - Support interactive mode with user feedback
// - Return exit code 0 on success
// - Return exit code 1 on invalid input

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_ruchydbg_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("ruchydbg");
    path
}

/// Test 1: Basic five-whys analysis with JSON input
///
/// Validates that ruchydbg can read a bug report from JSON and output analysis
#[test]
fn test_five_whys_json_input() {
    // Create bug report JSON
    let bug_report_json = r#"{
  "category": "InterpreterRuntime",
  "symptom": "Panic: index out of bounds accessing vector",
  "source_code": "fun main() {\n    let vec = [1, 2, 3];\n    let x = vec[5];\n    println(x);\n}",
  "error_message": "index out of bounds: the len is 3 but the index is 5",
  "stack_trace": [
    "at interpreter::evaluate_index_access",
    "at interpreter::evaluate_expression"
  ]
}"#;

    let test_file = "/tmp/test_five_whys_bug_report.json";
    fs::write(test_file, bug_report_json).unwrap();

    // Run five-whys analysis
    let output = Command::new(get_ruchydbg_path())
        .arg("five-whys")
        .arg(test_file)
        .output()
        .expect("Failed to execute ruchydbg");

    // Should succeed
    assert!(
        output.status.success(),
        "Expected success, got exit code: {:?}\nStderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Output should contain "Five Whys Analysis"
    assert!(
        stdout.contains("Five Whys Analysis"),
        "Expected 'Five Whys Analysis' header in output"
    );

    // Should show 5 why questions
    assert!(
        stdout.contains("Why 1:") && stdout.contains("Why 5:"),
        "Expected to see Why 1 through Why 5 in output"
    );

    // Should identify root cause
    assert!(
        stdout.contains("Root Cause:"),
        "Expected 'Root Cause:' in output"
    );

    // Should provide fix recommendation
    assert!(
        stdout.contains("Recommended Fix:") || stdout.contains("Fix:"),
        "Expected fix recommendation in output"
    );
}

/// Test 2: Five-whys with JSON output format
///
/// Validates that --format json outputs valid JSON
#[test]
fn test_five_whys_json_output() {
    let bug_report_json = r#"{
  "category": "Compiler",
  "symptom": "Type error: cannot add i64 and string",
  "source_code": "fun main() { let x: i64 = 42; let y: string = \"hello\"; let z = x + y; }",
  "error_message": "type error: binary operator '+' cannot be applied to types 'i64' and 'string'"
}"#;

    let test_file = "/tmp/test_five_whys_compiler.json";
    fs::write(test_file, bug_report_json).unwrap();

    // Run with JSON output
    let output = Command::new(get_ruchydbg_path())
        .arg("five-whys")
        .arg(test_file)
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(
        output.status.success(),
        "Expected success, got exit code: {:?}",
        output.status.code()
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should be valid JSON
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(
        parsed.is_ok(),
        "Expected valid JSON output, got: {}",
        stdout
    );

    let json = parsed.unwrap();

    // Should have whys array with 5 elements
    assert!(json["whys"].is_array(), "Expected 'whys' array in JSON");
    assert_eq!(
        json["whys"].as_array().unwrap().len(),
        5,
        "Expected 5 why iterations"
    );

    // Should have root_cause
    assert!(
        json["root_cause"].is_string() || json["root_cause"].is_object(),
        "Expected 'root_cause' in JSON"
    );

    // Should have recommended_fix
    assert!(
        json["recommended_fix"].is_string(),
        "Expected 'recommended_fix' string in JSON"
    );
}

/// Test 3: Five-whys with output file
///
/// Validates that --output writes analysis to file
#[test]
fn test_five_whys_output_file() {
    let bug_report_json = r#"{
  "category": "Transpiler",
  "symptom": "Transpiled code produces different result than interpreter",
  "source_code": "fun factorial(n: i64) { if n <= 1 { return 1; } return n * factorial(n - 1); }",
  "error_message": "Interpreter: factorial(5) = 120, Transpiled: factorial(5) = 720"
}"#;

    let test_file = "/tmp/test_five_whys_transpiler.json";
    let output_file = "/tmp/test_five_whys_output.txt";

    fs::write(test_file, bug_report_json).unwrap();

    // Remove output file if exists
    let _ = fs::remove_file(output_file);

    // Run with output file
    let output = Command::new(get_ruchydbg_path())
        .arg("five-whys")
        .arg(test_file)
        .arg("--output")
        .arg(output_file)
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(
        output.status.success(),
        "Expected success, got exit code: {:?}",
        output.status.code()
    );

    // Output file should exist
    assert!(
        PathBuf::from(output_file).exists(),
        "Expected output file to be created"
    );

    // Read output file
    let content = fs::read_to_string(output_file).unwrap();

    // Should contain analysis
    assert!(
        content.contains("Five Whys Analysis"),
        "Expected analysis in output file"
    );
    assert!(
        content.contains("Why 1:") && content.contains("Why 5:"),
        "Expected 5 why iterations in output file"
    );
}

/// Test 4: Five-whys interactive mode (non-interactive test)
///
/// Validates that --interactive flag is recognized (we can't test actual interactivity in CI)
#[test]
fn test_five_whys_interactive_flag() {
    let bug_report_json = r#"{
  "category": "InterpreterRuntime",
  "symptom": "Stack overflow in recursive function",
  "source_code": "fun infinite() { infinite(); }",
  "error_message": "thread has overflowed its stack"
}"#;

    let test_file = "/tmp/test_five_whys_interactive.json";
    fs::write(test_file, bug_report_json).unwrap();

    // Run with --interactive flag (but provide input via stdin to avoid hanging)
    // We'll just verify the flag is recognized
    let output = Command::new(get_ruchydbg_path())
        .arg("five-whys")
        .arg(test_file)
        .arg("--help")
        .output()
        .expect("Failed to execute ruchydbg");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Help text should mention --interactive flag
    assert!(
        stdout.contains("--interactive") || stdout.contains("-i"),
        "Expected --interactive flag in help text"
    );
}

/// Test 5: Five-whys with invalid JSON
///
/// Validates error handling for malformed input
#[test]
fn test_five_whys_invalid_json() {
    let invalid_json = r#"{
  "category": "Invalid",
  "symptom": "Test"
  // Missing closing brace
"#;

    let test_file = "/tmp/test_five_whys_invalid.json";
    fs::write(test_file, invalid_json).unwrap();

    // Run with invalid JSON
    let output = Command::new(get_ruchydbg_path())
        .arg("five-whys")
        .arg(test_file)
        .output()
        .expect("Failed to execute ruchydbg");

    // Should fail with exit code 1
    assert!(
        !output.status.success(),
        "Expected failure for invalid JSON"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show error message
    assert!(
        stderr.contains("Error") || stderr.contains("error") || stderr.contains("Failed"),
        "Expected error message in stderr, got: {}",
        stderr
    );
}

/// Test 6: Five-whys help text
///
/// Validates that help is available
#[test]
fn test_five_whys_help() {
    let output = Command::new(get_ruchydbg_path())
        .arg("five-whys")
        .arg("--help")
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(output.status.success(), "Expected success for --help");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show usage
    assert!(
        stdout.contains("Usage:") || stdout.contains("USAGE:"),
        "Expected usage information"
    );

    // Should mention key features
    assert!(
        stdout.contains("five-whys") || stdout.contains("Five Whys"),
        "Expected 'five-whys' in help text"
    );

    // Should document options
    assert!(
        stdout.contains("--format") || stdout.contains("--output"),
        "Expected command options documented"
    );
}

/// Test 7: Five-whys missing file argument
///
/// Validates error when no file provided
#[test]
fn test_five_whys_missing_file() {
    let output = Command::new(get_ruchydbg_path())
        .arg("five-whys")
        .output()
        .expect("Failed to execute ruchydbg");

    // Should fail
    assert!(
        !output.status.success(),
        "Expected failure when no file provided"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show error about missing file
    assert!(
        stderr.contains("file") || stderr.contains("argument") || stderr.contains("Usage"),
        "Expected error about missing file argument"
    );
}

/// Test 8: Five-whys with knowledge base (multiple bugs)
///
/// Validates pattern detection across multiple bug reports
#[test]
fn test_five_whys_knowledge_base() {
    // Create multiple bug reports
    let bug1_json = r#"{
  "category": "InterpreterRuntime",
  "symptom": "Division by zero panic",
  "source_code": "fun main() { let x = 10 / 0; }",
  "error_message": "attempt to divide by zero"
}"#;

    let bug2_json = r#"{
  "category": "InterpreterRuntime",
  "symptom": "Division by zero in calculation",
  "source_code": "fun divide(a: i64, b: i64) { return a / b; }",
  "error_message": "attempt to divide by zero"
}"#;

    let bug1_file = "/tmp/test_five_whys_kb_bug1.json";
    let bug2_file = "/tmp/test_five_whys_kb_bug2.json";

    fs::write(bug1_file, bug1_json).unwrap();
    fs::write(bug2_file, bug2_json).unwrap();

    // Run with --knowledge-base flag to detect patterns
    let output = Command::new(get_ruchydbg_path())
        .arg("five-whys")
        .arg(bug1_file)
        .arg(bug2_file)
        .arg("--knowledge-base")
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(
        output.status.success(),
        "Expected success, got exit code: {:?}",
        output.status.code()
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show pattern detection
    assert!(
        stdout.contains("Pattern") || stdout.contains("pattern") || stdout.contains("recurring"),
        "Expected pattern detection in output"
    );

    // Should mention division by zero
    assert!(
        stdout.contains("division") || stdout.contains("divide"),
        "Expected division pattern to be detected"
    );
}
