// COMPILED-INST-003: Binary Analysis Tooling (RED PHASE)
//
// Tests for analyzing compiled binary size, sections, symbols, and performance.
//
// Expected behavior:
// - Parse ELF binary format (text, data, rodata, bss sections)
// - Extract symbol table (function names, sizes, addresses)
// - Measure startup time (loader overhead, initialization)
// - Analyze relocation overhead
// - Generate optimization recommendations
// - Target: ≤50% of equivalent C binary size
//
// All tests are expected to FAIL in RED phase.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

/// Helper: Find ruchy binary in target/release or PATH
fn get_ruchy_path() -> PathBuf {
    let target_release = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("release")
        .join("ruchy");

    if target_release.exists() {
        target_release
    } else {
        PathBuf::from("ruchy")
    }
}

/// Test 1: Binary size breakdown by section
///
/// Requirements:
/// - Parse ELF binary format
/// - Extract section sizes: .text (code), .data (initialized data),
///   .rodata (read-only data), .bss (uninitialized data)
/// - Report total binary size
/// - Compare with equivalent C binary (if available)
///
/// Acceptance:
/// - JSON output with section breakdown
/// - Sizes in bytes for each section
/// - Total size matches file size (±headers/metadata)
///
/// ❌ STATUS: RED (not implemented)
#[test]
fn test_binary_size_breakdown() {
    // Create test Ruchy program
    let test_file = "/tmp/test_size.ruchy";
    fs::write(test_file, r#"
fun fibonacci(n: i64) -> i64 {
    if n <= 1 { return n; }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fun main() {
    let result = fibonacci(20);
    println(result);
}
"#).expect("Failed to write test file");

    // Compile to binary
    let binary_path = "/tmp/test_size_bin";
    let compile_output = Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .output()
        .expect("Failed to compile");

    assert!(compile_output.status.success(), "Compilation failed");

    // Analyze binary size
    let analyze_output = Command::new(get_ruchy_path())
        .args(&["analyze", "--size", "--output=/tmp/size_analysis.json", binary_path])
        .output()
        .expect("Failed to analyze");

    assert!(
        analyze_output.status.success(),
        "Binary analysis failed: {}",
        String::from_utf8_lossy(&analyze_output.stderr)
    );

    // Verify JSON structure
    let analysis = fs::read_to_string("/tmp/size_analysis.json")
        .expect("Failed to read analysis");
    let json: serde_json::Value = serde_json::from_str(&analysis)
        .expect("Invalid JSON");

    // Verify sections
    assert!(json["sections"].is_object(), "Missing sections object");
    let sections = json["sections"].as_object().unwrap();

    assert!(sections.contains_key("text"), "Missing .text section");
    assert!(sections.contains_key("data"), "Missing .data section");
    assert!(sections.contains_key("rodata"), "Missing .rodata section");
    assert!(sections.contains_key("bss"), "Missing .bss section");

    // Verify sizes are numbers
    assert!(sections["text"]["size"].is_number(), ".text size not a number");
    assert!(sections["data"]["size"].is_number(), ".data size not a number");

    // Verify total size
    let total_size = json["total_size"].as_u64().expect("Missing total_size");
    let actual_size = fs::metadata(binary_path)
        .expect("Failed to get file metadata")
        .len();

    // Total should match file size (±10% for headers)
    let diff_percent = ((total_size as f64 - actual_size as f64).abs() / actual_size as f64) * 100.0;
    assert!(
        diff_percent < 10.0,
        "Total size {} differs too much from actual size {} ({:.1}%)",
        total_size,
        actual_size,
        diff_percent
    );

    println!("✅ Binary size breakdown working");
}

/// Test 2: Symbol table analysis
///
/// Requirements:
/// - Parse symbol table from ELF binary
/// - Extract function names, addresses, sizes
/// - Identify inlining candidates (small functions called once)
/// - Sort by size to find large functions
///
/// Acceptance:
/// - JSON output with symbol table
/// - Each symbol has: name, address, size, type
/// - Functions sorted by size (descending)
/// - Inlining candidates identified
///
/// ❌ STATUS: RED (not implemented)
#[test]
fn test_symbol_table_analysis() {
    let test_file = "/tmp/test_symbols.ruchy";
    fs::write(test_file, r#"
fun small_helper(x: i64) -> i64 {
    x + 1
}

fun large_function(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..n {
        sum = sum + small_helper(i);
    }
    sum
}

fun main() {
    let result = large_function(1000);
    println(result);
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_symbols_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    let analyze_output = Command::new(get_ruchy_path())
        .args(&["analyze", "--symbols", "--output=/tmp/symbols_analysis.json", binary_path])
        .output()
        .expect("Failed to analyze");

    assert!(analyze_output.status.success(), "Symbol analysis failed");

    let analysis = fs::read_to_string("/tmp/symbols_analysis.json")
        .expect("Failed to read analysis");
    let json: serde_json::Value = serde_json::from_str(&analysis)
        .expect("Invalid JSON");

    // Verify symbol table structure
    assert!(json["symbols"].is_array(), "Missing symbols array");
    let symbols = json["symbols"].as_array().unwrap();

    assert!(symbols.len() > 0, "No symbols found");

    // Verify each symbol has required fields
    for symbol in symbols {
        assert!(symbol["name"].is_string(), "Symbol missing name");
        assert!(symbol["address"].is_string(), "Symbol missing address");
        assert!(symbol["size"].is_number(), "Symbol missing size");
        assert!(symbol["type"].is_string(), "Symbol missing type");
    }

    // Verify inlining candidates identified
    assert!(json["inlining_candidates"].is_array(), "Missing inlining candidates");
    let candidates = json["inlining_candidates"].as_array().unwrap();

    // small_helper should be an inlining candidate (small function, called once)
    let has_small_helper = candidates.iter().any(|c|
        c["name"].as_str().unwrap().contains("small_helper")
    );
    assert!(has_small_helper, "small_helper not identified as inlining candidate");

    println!("✅ Symbol table analysis working");
}

/// Test 3: Startup time profiling
///
/// Requirements:
/// - Measure total startup time (time to first instruction in main)
/// - Break down: loader overhead, dynamic linking, static initialization
/// - Compare with baseline (minimal C program)
/// - Identify slow initializers
///
/// Acceptance:
/// - JSON output with startup breakdown
/// - Loader time, linking time, init time measured
/// - Total startup time <100ms for simple programs
///
/// ❌ STATUS: RED (not implemented)
#[test]
fn test_startup_time_profiling() {
    let test_file = "/tmp/test_startup.ruchy";
    fs::write(test_file, r#"
fun main() {
    println(42);
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_startup_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    // Profile startup time
    let analyze_output = Command::new(get_ruchy_path())
        .args(&["analyze", "--startup", "--output=/tmp/startup_analysis.json", binary_path])
        .output()
        .expect("Failed to analyze");

    assert!(analyze_output.status.success(), "Startup analysis failed");

    let analysis = fs::read_to_string("/tmp/startup_analysis.json")
        .expect("Failed to read analysis");
    let json: serde_json::Value = serde_json::from_str(&analysis)
        .expect("Invalid JSON");

    // Verify startup breakdown
    assert!(json["startup_time_us"].is_number(), "Missing startup time");
    assert!(json["loader_time_us"].is_number(), "Missing loader time");
    assert!(json["linking_time_us"].is_number(), "Missing linking time");
    assert!(json["init_time_us"].is_number(), "Missing init time");

    let startup_us = json["startup_time_us"].as_u64().unwrap();

    // Startup time should be reasonable (<100ms = 100,000µs)
    assert!(
        startup_us < 100_000,
        "Startup time {}µs exceeds 100ms threshold",
        startup_us
    );

    println!("✅ Startup time profiling working: {}µs", startup_us);
}

/// Test 4: Relocation overhead analysis
///
/// Requirements:
/// - Count relocations in binary
/// - Measure relocation processing time
/// - Identify functions with many relocations
/// - Suggest PIC/PIE optimizations
///
/// Acceptance:
/// - JSON output with relocation stats
/// - Total relocations, types (GOT, PLT, etc.)
/// - Per-function relocation counts
/// - Optimization suggestions
///
/// ❌ STATUS: RED (not implemented)
#[test]
fn test_relocation_overhead() {
    let test_file = "/tmp/test_reloc.ruchy";
    fs::write(test_file, r#"
fun call_many_functions() {
    println(1);
    println(2);
    println(3);
    println(4);
    println(5);
}

fun main() {
    call_many_functions();
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_reloc_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    let analyze_output = Command::new(get_ruchy_path())
        .args(&["analyze", "--relocations", "--output=/tmp/reloc_analysis.json", binary_path])
        .output()
        .expect("Failed to analyze");

    assert!(analyze_output.status.success(), "Relocation analysis failed");

    let analysis = fs::read_to_string("/tmp/reloc_analysis.json")
        .expect("Failed to read analysis");
    let json: serde_json::Value = serde_json::from_str(&analysis)
        .expect("Invalid JSON");

    // Verify relocation stats
    assert!(json["total_relocations"].is_number(), "Missing total relocations");
    assert!(json["relocation_types"].is_object(), "Missing relocation types");

    let total_relocs = json["total_relocations"].as_u64().unwrap();
    assert!(total_relocs > 0, "Expected some relocations for println calls");

    // Verify types breakdown (GOT, PLT, etc.)
    let types = json["relocation_types"].as_object().unwrap();
    assert!(types.len() > 0, "No relocation types found");

    println!("✅ Relocation analysis working: {} relocations", total_relocs);
}

/// Test 5: Optimization recommendations
///
/// Requirements:
/// - Analyze binary and generate actionable optimization advice
/// - Dead code elimination candidates
/// - Function outlining opportunities (reduce code bloat)
/// - Compression opportunities
/// - Compare with C equivalent size (target ≤50%)
///
/// Acceptance:
/// - JSON output with optimization recommendations
/// - Each recommendation has: type, impact (bytes saved), priority
/// - At least 3 categories: DCE, outlining, compression
///
/// ❌ STATUS: RED (not implemented)
#[test]
fn test_optimization_recommendations() {
    let test_file = "/tmp/test_optim.ruchy";
    fs::write(test_file, r#"
fun unused_function(x: i64) -> i64 {
    x * 2
}

fun large_repetitive_function(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..n {
        sum = sum + i;
        sum = sum + i * 2;
        sum = sum + i * 3;
    }
    sum
}

fun main() {
    let result = large_repetitive_function(100);
    println(result);
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_optim_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    let analyze_output = Command::new(get_ruchy_path())
        .args(&["analyze", "--optimize", "--output=/tmp/optim_analysis.json", binary_path])
        .output()
        .expect("Failed to analyze");

    assert!(analyze_output.status.success(), "Optimization analysis failed");

    let analysis = fs::read_to_string("/tmp/optim_analysis.json")
        .expect("Failed to read analysis");
    let json: serde_json::Value = serde_json::from_str(&analysis)
        .expect("Invalid JSON");

    // Verify recommendations structure
    assert!(json["recommendations"].is_array(), "Missing recommendations");
    let recommendations = json["recommendations"].as_array().unwrap();

    assert!(recommendations.len() > 0, "No recommendations generated");

    // Verify each recommendation has required fields
    for rec in recommendations {
        assert!(rec["type"].is_string(), "Recommendation missing type");
        assert!(rec["description"].is_string(), "Recommendation missing description");
        assert!(rec["impact_bytes"].is_number(), "Recommendation missing impact");
        assert!(rec["priority"].is_string(), "Recommendation missing priority");
    }

    // Should have dead code elimination recommendation for unused_function
    let has_dce = recommendations.iter().any(|r|
        r["type"].as_str().unwrap() == "dead_code_elimination"
    );
    assert!(has_dce, "No DCE recommendation found");

    println!("✅ Optimization recommendations working: {} suggestions", recommendations.len());
}

/// Test 6: Multi-platform binary format support
///
/// Requirements:
/// - Support ELF format (Linux)
/// - Support Mach-O format (macOS) - optional
/// - Support PE format (Windows) - optional
/// - Auto-detect binary format
/// - Graceful error for unsupported formats
///
/// Acceptance:
/// - At minimum, ELF format fully supported
/// - Auto-detection works correctly
/// - Error messages helpful for unsupported formats
///
/// ❌ STATUS: RED (not implemented)
#[test]
fn test_elf_format_support() {
    let test_file = "/tmp/test_elf.ruchy";
    fs::write(test_file, r#"
fun main() {
    println(42);
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_elf_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    // Analyze should auto-detect ELF format
    let analyze_output = Command::new(get_ruchy_path())
        .args(&["analyze", "--format", "--output=/tmp/format_analysis.json", binary_path])
        .output()
        .expect("Failed to analyze");

    assert!(analyze_output.status.success(), "Format detection failed");

    let analysis = fs::read_to_string("/tmp/format_analysis.json")
        .expect("Failed to read analysis");
    let json: serde_json::Value = serde_json::from_str(&analysis)
        .expect("Invalid JSON");

    // Verify format detection
    assert!(json["format"].is_string(), "Missing format field");
    let format = json["format"].as_str().unwrap();

    // On Linux, should detect ELF
    #[cfg(target_os = "linux")]
    assert_eq!(format, "ELF", "Expected ELF format on Linux");

    // On macOS, should detect Mach-O
    #[cfg(target_os = "macos")]
    assert_eq!(format, "Mach-O", "Expected Mach-O format on macOS");

    println!("✅ Format detection working: {}", format);
}
