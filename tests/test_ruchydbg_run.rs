// DEBUGGER-013: ruchydbg run Command Tests (RED PHASE)
//
// Tests for `ruchydbg run <file> --timeout <ms>` command
//
// Expected behavior:
// - Execute Ruchy code with timeout
// - Return exit code 0 on success
// - Return exit code 124 on timeout
// - Return non-zero on crash
// - Show execution status

use std::process::Command;
use std::fs;
use std::path::PathBuf;

fn get_ruchydbg_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("ruchydbg");
    path
}

#[test]
fn test_ruchydbg_run_success() {
    // RED: This test WILL FAIL because `ruchydbg run` doesn't exist yet

    // Create test file that succeeds
    let test_file = "/tmp/test_ruchydbg_success.ruchy";
    fs::write(test_file, r#"
fun main() {
    println!("Success!");
}
"#).unwrap();

    // Run with ruchydbg
    let output = Command::new(get_ruchydbg_path())
        .arg("run")
        .arg(test_file)
        .arg("--timeout")
        .arg("1000")
        .output()
        .expect("Failed to execute ruchydbg");

    // Should succeed with exit code 0
    assert!(output.status.success(),
        "Expected success, got exit code: {:?}",
        output.status.code());

    // Should show execution time
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Success!"), "Expected to see 'Success!' in output");
}

#[test]
fn test_ruchydbg_run_timeout() {
    // RED: This test WILL FAIL because `ruchydbg run` doesn't exist yet

    // Create test file that hangs
    let test_file = "/tmp/test_ruchydbg_timeout.ruchy";
    fs::write(test_file, r#"
fun main() {
    loop {}  // Infinite loop
}
"#).unwrap();

    // Run with ruchydbg with 1 second timeout
    let output = Command::new(get_ruchydbg_path())
        .arg("run")
        .arg(test_file)
        .arg("--timeout")
        .arg("1000")
        .output()
        .expect("Failed to execute ruchydbg");

    // Should timeout with exit code 124
    assert_eq!(output.status.code(), Some(124),
        "Expected timeout exit code 124, got: {:?}",
        output.status.code());

    // Should report timeout
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("timeout") || stderr.contains("TIMEOUT"),
        "Expected timeout message in stderr");
}

#[test]
#[ignore] // BLOCKED: Ruchy Issue #81 - panic!() and undefined functions return exit code 0
fn test_ruchydbg_run_crash() {
    // BLOCKED BY: https://github.com/paiml/ruchy/issues/81
    // Ruchy returns exit code 0 for crashes, making crash detection impossible
    // This test will be enabled once Ruchy #81 is fixed

    // Create test file with invalid code (should crash)
    let test_file = "/tmp/test_ruchydbg_crash.ruchy";
    fs::write(test_file, r#"
fun main() {
    // Invalid syntax - undefined function
    undefined_function_that_does_not_exist();
}
"#).unwrap();

    // Run with ruchydbg
    let output = Command::new(get_ruchydbg_path())
        .arg("run")
        .arg(test_file)
        .arg("--timeout")
        .arg("1000")
        .output()
        .expect("Failed to execute ruchydbg");

    // Should fail with non-zero exit code (not 124)
    assert!(!output.status.success(), "Expected crash, got success");
    assert_ne!(output.status.code(), Some(124),
        "Expected crash exit code (not timeout 124), got: {:?}",
        output.status.code());
}

#[test]
fn test_ruchydbg_run_invalid_file() {
    // RED: This test WILL FAIL because `ruchydbg run` doesn't exist yet

    // Run with non-existent file
    let output = Command::new(get_ruchydbg_path())
        .arg("run")
        .arg("/tmp/nonexistent_file.ruchy")
        .arg("--timeout")
        .arg("1000")
        .output()
        .expect("Failed to execute ruchydbg");

    // Should fail with error message
    assert!(!output.status.success(), "Expected error for invalid file");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("No such file"),
        "Expected file not found error");
}

#[test]
fn test_ruchydbg_run_help() {
    // RED: This test WILL FAIL because `ruchydbg run` doesn't exist yet

    // Run with --help
    let output = Command::new(get_ruchydbg_path())
        .arg("run")
        .arg("--help")
        .output()
        .expect("Failed to execute ruchydbg");

    // Should show help text
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("run"), "Expected 'run' in help text");
    assert!(stdout.contains("timeout"), "Expected 'timeout' in help text");
    assert!(stdout.contains("USAGE") || stdout.contains("Usage"),
        "Expected usage information");
}

#[test]
fn test_ruchydbg_run_default_timeout() {
    // RED: This test WILL FAIL because `ruchydbg run` doesn't exist yet

    // Create fast test file
    let test_file = "/tmp/test_ruchydbg_default_timeout.ruchy";
    fs::write(test_file, r#"
fun main() {
    println!("Fast!");
}
"#).unwrap();

    // Run without --timeout argument (should use default)
    let output = Command::new(get_ruchydbg_path())
        .arg("run")
        .arg(test_file)
        .output()
        .expect("Failed to execute ruchydbg");

    // Should succeed with default timeout
    assert!(output.status.success(),
        "Expected success with default timeout, got exit code: {:?}",
        output.status.code());
}

#[test]
fn test_ruchydbg_run_reports_execution_time() {
    // RED: This test WILL FAIL because `ruchydbg run` doesn't exist yet

    // Create test file
    let test_file = "/tmp/test_ruchydbg_timing.ruchy";
    fs::write(test_file, r#"
fun main() {
    println!("Done!");
}
"#).unwrap();

    // Run with ruchydbg
    let output = Command::new(get_ruchydbg_path())
        .arg("run")
        .arg(test_file)
        .arg("--timeout")
        .arg("5000")
        .output()
        .expect("Failed to execute ruchydbg");

    // Should report execution time
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ms") || stdout.contains("milliseconds") || stdout.contains("time"),
        "Expected execution time in output");
}
