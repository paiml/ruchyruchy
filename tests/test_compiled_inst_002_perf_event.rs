// COMPILED-INST-002: perf_event_open Integration (RED PHASE)
//
// Tests for hardware performance counter profiling of compiled Ruchy binaries.
//
// Expected behavior:
// - Profile compiled binaries WITHOUT code instrumentation
// - <1% overhead via perf_event_open (DEBUGGER-016 infrastructure)
// - Capture CPU cycles, cache misses, branch mispredictions
// - Generate flame graphs for hotspot visualization
// - Identify top N functions by hardware counters
//
// Architecture:
// - Reuse DEBUGGER-016 Profiler infrastructure (src/profiling/mod.rs)
// - Extend with multi-counter support (CPU, cache, branches)
// - Integrate with ruchy compiler wrapper (src/bin/ruchy.rs)
//
// All tests are expected to FAIL in RED phase.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

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

/// Test 1: Compile and profile with hardware counters (CPU_CYCLES)
///
/// Requirements:
/// - Compile Ruchy code to binary (NO instrumentation flags)
/// - Profile binary execution with perf_event_open
/// - Capture CPU cycle counts per function
/// - Overhead <1% (measured via timing comparison)
///
/// Acceptance:
/// - Binary compiles successfully
/// - Profiler captures CPU cycles
/// - Profile data includes function-level breakdown
/// - Overhead <1% (profiled time / baseline time < 1.01)
///
/// ❌ STATUS: RED (not implemented)
#[test]
#[ignore] // RED phase - will fail until GREEN implementation
fn test_compile_and_profile_cpu_cycles() {
    // Step 1: Create test Ruchy program
    let test_file = "/tmp/test_cpu_cycles.ruchy";
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

    // Step 2: Compile WITHOUT instrumentation (just regular compile)
    let binary_path = "/tmp/test_cpu_cycles_bin";
    let compile_output = Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .output()
        .expect("Failed to compile");

    assert!(
        compile_output.status.success(),
        "Compilation failed: {}",
        String::from_utf8_lossy(&compile_output.stderr)
    );

    // Step 3: Baseline execution (no profiling)
    let baseline_start = Instant::now();
    let baseline_output = Command::new(binary_path)
        .output()
        .expect("Failed to run baseline");
    let baseline_time = baseline_start.elapsed();

    assert!(baseline_output.status.success(), "Baseline execution failed");

    // Step 4: Profile execution with hardware counters
    let profile_start = Instant::now();
    let profile_output = Command::new(get_ruchy_path())
        .args(&["profile", "--counters=cpu_cycles", "--output=/tmp/cpu_profile.json", binary_path])
        .output()
        .expect("Failed to profile");
    let profile_time = profile_start.elapsed();

    assert!(
        profile_output.status.success(),
        "Profiling failed: {}",
        String::from_utf8_lossy(&profile_output.stderr)
    );

    // Step 5: Verify overhead <1%
    let overhead = profile_time.as_secs_f64() / baseline_time.as_secs_f64();
    assert!(
        overhead < 1.01,
        "Overhead {:.2}% exceeds 1% threshold",
        (overhead - 1.0) * 100.0
    );

    // Step 6: Verify profile data exists
    let profile_data = fs::read_to_string("/tmp/cpu_profile.json")
        .expect("Failed to read profile data");
    let profile: serde_json::Value = serde_json::from_str(&profile_data)
        .expect("Invalid JSON");

    // Verify structure
    assert!(profile["counters"].is_array(), "Missing counters array");
    let counters = profile["counters"].as_array().unwrap();

    // Should have CPU_CYCLES counter with function breakdown
    let cpu_cycles = counters.iter()
        .find(|c| c["name"] == "cpu_cycles")
        .expect("Missing cpu_cycles counter");

    assert!(cpu_cycles["functions"].is_array(), "Missing function breakdown");
    assert!(
        cpu_cycles["functions"].as_array().unwrap().len() > 0,
        "No function-level CPU cycle data"
    );

    println!("✅ CPU cycles profiling: overhead={:.2}%", (overhead - 1.0) * 100.0);
}

/// Test 2: Profile cache misses
///
/// Requirements:
/// - Use CACHE_MISSES hardware counter
/// - Identify cache-unfriendly code patterns
/// - Report L1/L2/L3 miss rates
///
/// Acceptance:
/// - Cache miss data captured per function
/// - Miss rates calculated (misses / accesses)
/// - Overhead <1%
///
/// ❌ STATUS: RED (not implemented)
#[test]
#[ignore] // RED phase
fn test_profile_cache_misses() {
    let test_file = "/tmp/test_cache.ruchy";
    fs::write(test_file, r#"
fun array_sum(size: i64) -> i64 {
    let mut sum = 0;
    for i in 0..size {
        sum = sum + i;
    }
    sum
}

fun main() {
    let result = array_sum(1000000);
    println(result);
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_cache_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    let profile_output = Command::new(get_ruchy_path())
        .args(&["profile", "--counters=cache_misses", "--output=/tmp/cache_profile.json", binary_path])
        .output()
        .expect("Failed to profile");

    assert!(profile_output.status.success(), "Profiling failed");

    let profile_data = fs::read_to_string("/tmp/cache_profile.json")
        .expect("Failed to read profile");
    let profile: serde_json::Value = serde_json::from_str(&profile_data)
        .expect("Invalid JSON");

    let cache_misses = profile["counters"].as_array().unwrap()
        .iter()
        .find(|c| c["name"] == "cache_misses")
        .expect("Missing cache_misses counter");

    assert!(cache_misses["total_misses"].is_number(), "Missing total_misses");
    assert!(cache_misses["functions"].is_array(), "Missing function breakdown");

    println!("✅ Cache miss profiling working");
}

/// Test 3: Profile branch mispredictions
///
/// Requirements:
/// - Use BRANCH_MISSES hardware counter
/// - Identify unpredictable branches
/// - Report misprediction rates
///
/// Acceptance:
/// - Branch misprediction data per function
/// - Misprediction rates calculated
/// - Overhead <1%
///
/// ❌ STATUS: RED (not implemented)
#[test]
#[ignore] // RED phase
fn test_profile_branch_mispredictions() {
    let test_file = "/tmp/test_branches.ruchy";
    fs::write(test_file, r#"
fun unpredictable_sum(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..n {
        if (i * 7) % 13 == 0 {
            sum = sum + i;
        }
    }
    sum
}

fun main() {
    let result = unpredictable_sum(100000);
    println(result);
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_branches_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    let profile_output = Command::new(get_ruchy_path())
        .args(&["profile", "--counters=branch_misses", "--output=/tmp/branch_profile.json", binary_path])
        .output()
        .expect("Failed to profile");

    assert!(profile_output.status.success(), "Profiling failed");

    let profile_data = fs::read_to_string("/tmp/branch_profile.json")
        .expect("Failed to read profile");
    let profile: serde_json::Value = serde_json::from_str(&profile_data)
        .expect("Invalid JSON");

    let branch_misses = profile["counters"].as_array().unwrap()
        .iter()
        .find(|c| c["name"] == "branch_misses")
        .expect("Missing branch_misses counter");

    assert!(branch_misses["total_misses"].is_number(), "Missing total_misses");
    assert!(branch_misses["functions"].is_array(), "Missing function breakdown");

    println!("✅ Branch misprediction profiling working");
}

/// Test 4: Generate flame graph from hardware samples
///
/// Requirements:
/// - Collect stack samples at 1000Hz
/// - Aggregate samples by stack trace
/// - Generate brendangregg-compatible flame graph
/// - Output SVG format
///
/// Acceptance:
/// - Flame graph SVG file generated
/// - Contains function names and sample counts
/// - Visualizes hotspots clearly
///
/// ❌ STATUS: RED (not implemented)
#[test]
#[ignore] // RED phase
fn test_generate_flame_graph() {
    let test_file = "/tmp/test_flamegraph.ruchy";
    fs::write(test_file, r#"
fun compute_a(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..n { sum = sum + i; }
    sum
}

fun compute_b(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..n { sum = sum + (i * i); }
    sum
}

fun main() {
    let a = compute_a(100000);
    let b = compute_b(200000);
    println(a + b);
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_flamegraph_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    let profile_output = Command::new(get_ruchy_path())
        .args(&[
            "profile",
            "--flame-graph=/tmp/flamegraph.svg",
            "--sampling-rate=1000",
            binary_path
        ])
        .output()
        .expect("Failed to profile");

    assert!(profile_output.status.success(), "Flame graph generation failed");

    // Verify SVG file exists
    let svg_content = fs::read_to_string("/tmp/flamegraph.svg")
        .expect("Flame graph SVG not generated");

    // Basic validation
    assert!(svg_content.contains("<svg"), "Invalid SVG format");
    assert!(svg_content.contains("compute_a") || svg_content.contains("compute_b"),
        "Flame graph missing function names");

    println!("✅ Flame graph generated ({} bytes)", svg_content.len());
}

/// Test 5: Identify hotspots (top N functions)
///
/// Requirements:
/// - Aggregate samples by function
/// - Sort by sample count descending
/// - Return top N functions
/// - Include percentage of total time
///
/// Acceptance:
/// - JSON output with top 10 functions
/// - Each entry has: function name, samples, percentage
/// - Percentages sum to ≤100%
///
/// ❌ STATUS: RED (not implemented)
#[test]
#[ignore] // RED phase
fn test_identify_hotspots() {
    let test_file = "/tmp/test_hotspots.ruchy";
    fs::write(test_file, r#"
fun hot_function(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..n { sum = sum + i; }
    sum
}

fun cold_function(n: i64) -> i64 {
    n * 2
}

fun main() {
    let hot = hot_function(1000000);
    let cold = cold_function(10);
    println(hot + cold);
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_hotspots_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    let profile_output = Command::new(get_ruchy_path())
        .args(&[
            "profile",
            "--hotspots=10",
            "--output=/tmp/hotspots.json",
            binary_path
        ])
        .output()
        .expect("Failed to profile");

    assert!(profile_output.status.success(), "Hotspot analysis failed");

    let hotspot_data = fs::read_to_string("/tmp/hotspots.json")
        .expect("Failed to read hotspots");
    let hotspots: serde_json::Value = serde_json::from_str(&hotspot_data)
        .expect("Invalid JSON");

    assert!(hotspots["hotspots"].is_array(), "Missing hotspots array");
    let top_functions = hotspots["hotspots"].as_array().unwrap();

    // Should have at least 1 function
    assert!(top_functions.len() > 0, "No hotspots identified");

    // Verify structure
    let top_fn = &top_functions[0];
    assert!(top_fn["function"].is_string(), "Missing function name");
    assert!(top_fn["samples"].is_number(), "Missing sample count");
    assert!(top_fn["percentage"].is_number(), "Missing percentage");

    // hot_function should be the top hotspot
    assert!(
        top_fn["function"].as_str().unwrap().contains("hot_function"),
        "Expected hot_function to be top hotspot"
    );

    println!("✅ Hotspot identification: {} functions", top_functions.len());
}

/// Test 6: Multi-counter profiling (CPU + cache + branches)
///
/// Requirements:
/// - Profile with multiple hardware counters simultaneously
/// - Correlate counters (e.g., IPC = instructions / cycles)
/// - Report all counters in single JSON output
///
/// Acceptance:
/// - JSON contains cpu_cycles, cache_misses, branch_misses
/// - Derived metrics calculated (IPC, cache miss rate, branch miss rate)
/// - Overhead <1% with 3 counters
///
/// ❌ STATUS: RED (not implemented)
#[test]
#[ignore] // RED phase
fn test_multi_counter_profiling() {
    let test_file = "/tmp/test_multi.ruchy";
    fs::write(test_file, r#"
fun compute(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..n {
        if i % 2 == 0 { sum = sum + i; }
    }
    sum
}

fun main() {
    let result = compute(500000);
    println(result);
}
"#).expect("Failed to write test file");

    let binary_path = "/tmp/test_multi_bin";
    Command::new(get_ruchy_path())
        .args(&["compile", "--output", binary_path, test_file])
        .status()
        .expect("Compilation failed");

    let profile_output = Command::new(get_ruchy_path())
        .args(&[
            "profile",
            "--counters=cpu_cycles,cache_misses,branch_misses",
            "--output=/tmp/multi_profile.json",
            binary_path
        ])
        .output()
        .expect("Failed to profile");

    assert!(profile_output.status.success(), "Multi-counter profiling failed");

    let profile_data = fs::read_to_string("/tmp/multi_profile.json")
        .expect("Failed to read profile");
    let profile: serde_json::Value = serde_json::from_str(&profile_data)
        .expect("Invalid JSON");

    let counters = profile["counters"].as_array().unwrap();
    assert_eq!(counters.len(), 3, "Expected 3 counters");

    // Verify all counters present
    let counter_names: Vec<&str> = counters.iter()
        .map(|c| c["name"].as_str().unwrap())
        .collect();

    assert!(counter_names.contains(&"cpu_cycles"), "Missing cpu_cycles");
    assert!(counter_names.contains(&"cache_misses"), "Missing cache_misses");
    assert!(counter_names.contains(&"branch_misses"), "Missing branch_misses");

    // Verify derived metrics
    assert!(profile["derived_metrics"].is_object(), "Missing derived metrics");
    let metrics = profile["derived_metrics"].as_object().unwrap();

    assert!(metrics.contains_key("ipc"), "Missing IPC metric");
    assert!(metrics.contains_key("cache_miss_rate"), "Missing cache miss rate");
    assert!(metrics.contains_key("branch_miss_rate"), "Missing branch miss rate");

    println!("✅ Multi-counter profiling with derived metrics working");
}
