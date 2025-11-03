// INTERP-050: CLI Tests for Type Stability Tracking
//
// EXTREME TDD - RED Phase (CLI)
//
// Mission: Add `ruchydbg profile --types <file>` command
//
// Command: ruchydbg profile --types <file>
// Output:
// - Per-function type signatures observed
// - Type stability classification (monomorphic/polymorphic/megamorphic)
// - Excellent JIT candidates (hot + type-stable)
//
// Method: Test-driven development

use std::process::Command;

/// Helper: Run ruchydbg command
fn run_ruchydbg(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("ruchydbg")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to run ruchydbg");

    // Combine stdout and stderr for testing
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!("{}{}", stdout, stderr)
}

/// Test: ruchydbg profile --types shows type stability
///
/// Validates that command shows monomorphic/polymorphic/megamorphic classification
#[test]
fn test_profile_types_basic() {
    // Create test file with monomorphic function
    let test_file = "/tmp/test_profile_types_basic.ruchy";
    std::fs::write(
        test_file,
        r#"
fun add(a, b) {
    return a + b;
}

let r1 = add(1, 2);
let r2 = add(3, 4);
let r3 = add(5, 6);
    "#,
    )
    .unwrap();

    let output = run_ruchydbg(&["profile", "--types", test_file]);

    // Should show function name
    assert!(
        output.contains("add") || output.contains("Function"),
        "Should show function name"
    );

    // Should show type stability classification
    assert!(
        output.contains("monomorphic")
            || output.contains("Monomorphic")
            || output.contains("type-stable")
            || output.contains("Type Stability"),
        "Should show type stability classification"
    );

    // Should show type signature
    assert!(
        output.contains("Integer") || output.contains("Int") || output.contains("Type"),
        "Should show type signatures"
    );

    // Clean up
    let _ = std::fs::remove_file(test_file);
}

/// Test: ruchydbg profile --types identifies polymorphic functions
///
/// Validates that command shows functions with multiple type signatures
#[test]
fn test_profile_types_polymorphic() {
    let test_file = "/tmp/test_profile_types_poly.ruchy";
    std::fs::write(
        test_file,
        r#"
fun identity(x) {
    return x;
}

let r1 = identity(42);
let r2 = identity("hello");
    "#,
    )
    .unwrap();

    let output = run_ruchydbg(&["profile", "--types", test_file]);

    // Should show polymorphic classification
    assert!(
        output.contains("polymorphic") || output.contains("Polymorphic") || output.contains("2"),
        "Should indicate polymorphic (2 type signatures)"
    );

    // Should show both type signatures
    assert!(
        output.contains("Integer") || output.contains("Int"),
        "Should show Integer type"
    );
    assert!(output.contains("String"), "Should show String type");

    // Clean up
    let _ = std::fs::remove_file(test_file);
}

/// Test: ruchydbg profile --types identifies megamorphic functions
///
/// Validates that command shows functions with many type signatures
#[test]
fn test_profile_types_megamorphic() {
    let test_file = "/tmp/test_profile_types_mega.ruchy";
    std::fs::write(
        test_file,
        r#"
fun process(x) {
    return x;
}

let r1 = process(42);
let r2 = process("hello");
let r3 = process(true);
let r4 = process(3.14);
    "#,
    )
    .unwrap();

    let output = run_ruchydbg(&["profile", "--types", test_file]);

    // Should show megamorphic classification
    assert!(
        output.contains("megamorphic") || output.contains("Megamorphic") || output.contains("4"),
        "Should indicate megamorphic (4+ type signatures)"
    );

    // Should warn about poor JIT candidate
    assert!(
        output.contains("poor")
            || output.contains("unstable")
            || output.contains("not recommended")
            || output.contains("generic"),
        "Should indicate poor JIT candidate"
    );

    // Clean up
    let _ = std::fs::remove_file(test_file);
}

/// Test: ruchydbg profile --types highlights excellent JIT candidates
///
/// Validates that command identifies hot + type-stable functions
#[test]
fn test_profile_types_jit_candidates() {
    let test_file = "/tmp/test_profile_types_jit.ruchy";
    std::fs::write(
        test_file,
        r#"
fun hot_stable(n) {
    let i = 0;
    while (i < 100) { i = i + 1; }
    return n;
}

let r1 = hot_stable(1);
let r2 = hot_stable(2);
let r3 = hot_stable(3);
    "#,
    )
    .unwrap();

    let output = run_ruchydbg(&["profile", "--types", test_file]);

    // Should highlight as excellent JIT candidate
    assert!(
        output.contains("excellent")
            || output.contains("JIT candidate")
            || output.contains("recommended")
            || output.contains("hot")
            || output.contains("stable"),
        "Should highlight hot + type-stable function as excellent JIT candidate"
    );

    // Clean up
    let _ = std::fs::remove_file(test_file);
}

/// Test: ruchydbg profile --types (missing file)
///
/// Validates error handling
#[test]
fn test_profile_types_missing_file() {
    let output = run_ruchydbg(&["profile", "--types"]);

    assert!(
        output.contains("Error") || output.contains("Missing") || output.contains("Usage"),
        "Should show error for missing file"
    );
}
