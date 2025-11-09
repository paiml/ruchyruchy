// COMPILED-INST-001: AST-Level Instrumentation Hooks Tests (RED PHASE)
//
// Tests for AST/IR-level profiling instrumentation
//
// Mission: Make Ruchy the world's fastest compiled language (≥105% of C performance)
//
// Expected behavior:
// - Function entry/exit timing with <1% overhead
// - Loop iteration counting for hotspot identification
// - Branch taken/not-taken statistics for branch prediction tuning
// - Memory allocation tracking for allocator optimization
// - JSON output format for analysis integration
// - Statistical rigor: p < 0.05, N≥30 runs, CV <5%
//
// Research Foundation:
// - Georges et al. (2007): Statistical rigor (N≥30, p<0.05)
// - Julia (SIAM 2017): Type specialization for low overhead
// - Profile-Guided Optimization survey (arXiv 2025)

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

fn get_ruchy_path() -> PathBuf {
    // Look for ruchy in target/release first, then fall back to PATH
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

#[test]
fn test_function_timing_instrumentation() {
    // RED: This test WILL FAIL because AST-level function timing doesn't exist yet
    //
    // This is the FOUNDATIONAL test: Every function call must be timed
    // with minimal overhead (<1% when instrumentation enabled).
    //
    // Test strategy:
    // 1. Compile Ruchy code with --instrument flag
    // 2. Run instrumented binary
    // 3. Parse JSON output for function timing data
    // 4. Validate timing accuracy and overhead
    //
    // Expected JSON output:
    // {
    //   "functions": [
    //     {
    //       "name": "fibonacci",
    //       "calls": 21891,
    //       "total_time_ns": 1234567,
    //       "avg_time_ns": 56.4,
    //       "min_time_ns": 10,
    //       "max_time_ns": 500
    //     }
    //   ]
    // }

    // Create test file with recursive function
    let test_file = "/tmp/test_function_timing.ruchy";
    fs::write(
        test_file,
        r#"
fun fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fun main() {
    let result = fibonacci(20);
    println(result);
}
"#,
    )
    .expect("Failed to write test file");

    // Compile with instrumentation (RED: flag doesn't exist yet)
    let compile_output = Command::new(get_ruchy_path())
        .args(&[
            "compile",
            "--instrument", // RED: This flag doesn't exist yet
            "--output=/tmp/test_function_timing",
            test_file,
        ])
        .output()
        .expect("Failed to compile");

    // RED: This will fail because --instrument flag is not implemented
    assert!(
        compile_output.status.success(),
        "Compilation failed: {}",
        String::from_utf8_lossy(&compile_output.stderr)
    );

    // Run instrumented binary with profiling enabled
    let run_output = Command::new("/tmp/test_function_timing")
        .env("RUCHY_PROFILE", "1") // RED: Environment variable not supported
        .env("RUCHY_PROFILE_OUTPUT", "/tmp/profile.json")
        .output()
        .expect("Failed to run");

    assert!(
        run_output.status.success(),
        "Execution failed: {}",
        String::from_utf8_lossy(&run_output.stderr)
    );

    // Parse profile output (RED: file won't exist)
    let profile_data =
        fs::read_to_string("/tmp/profile.json").expect("Failed to read profile output");

    // Validate JSON structure (RED: will fail - no JSON output)
    let profile: serde_json::Value =
        serde_json::from_str(&profile_data).expect("Invalid JSON output");

    // Validate function timing data
    let functions = profile["functions"]
        .as_array()
        .expect("Missing functions array");

    assert!(!functions.is_empty(), "No function timing data collected");

    // Find fibonacci function
    let fibonacci_data = functions
        .iter()
        .find(|f| f["name"].as_str() == Some("fibonacci"))
        .expect("fibonacci function not found in profile");

    // Validate timing metrics
    let calls = fibonacci_data["calls"].as_u64().expect("Missing calls");
    let total_time = fibonacci_data["total_time_ns"]
        .as_u64()
        .expect("Missing total_time_ns");
    let avg_time = fibonacci_data["avg_time_ns"]
        .as_f64()
        .expect("Missing avg_time_ns");

    // fibonacci(20) should call fibonacci 21891 times
    assert_eq!(calls, 21891, "Incorrect call count");
    assert!(total_time > 0, "No time recorded");
    assert!(avg_time > 0.0, "Invalid average time");

    // Cleanup
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file("/tmp/test_function_timing");
    let _ = fs::remove_file("/tmp/profile.json");
}

#[test]
fn test_loop_iteration_counting() {
    // RED: This test WILL FAIL because loop iteration tracking doesn't exist yet
    //
    // Purpose: Track loop iterations for hotspot identification
    //
    // Expected JSON output:
    // {
    //   "loops": [
    //     {
    //       "location": "main:5",
    //       "iterations": 1000000,
    //       "avg_iter_time_ns": 12.5,
    //       "total_time_ns": 12500000
    //     }
    //   ]
    // }

    let test_file = "/tmp/test_loop_counting.ruchy";
    fs::write(
        test_file,
        r#"
fun main() {
    let mut sum = 0;
    for i in 0..1000000 {
        sum = sum + i;
    }
    println(sum);
}
"#,
    )
    .expect("Failed to write test file");

    // Compile with instrumentation
    let compile_output = Command::new(get_ruchy_path())
        .args(&[
            "compile",
            "--instrument",
            "--output=/tmp/test_loop_counting",
            test_file,
        ])
        .output()
        .expect("Failed to compile");

    assert!(
        compile_output.status.success(),
        "Compilation failed: {}",
        String::from_utf8_lossy(&compile_output.stderr)
    );

    // Run with profiling
    let run_output = Command::new("/tmp/test_loop_counting")
        .env("RUCHY_PROFILE", "1")
        .env("RUCHY_PROFILE_OUTPUT", "/tmp/profile_loop.json")
        .output()
        .expect("Failed to run");

    assert!(run_output.status.success());

    // Parse profile
    let profile_data =
        fs::read_to_string("/tmp/profile_loop.json").expect("Failed to read profile output");

    let profile: serde_json::Value = serde_json::from_str(&profile_data).expect("Invalid JSON");

    // Validate loop data
    let loops = profile["loops"].as_array().expect("Missing loops array");

    assert!(!loops.is_empty(), "No loop data collected");

    let loop_data = &loops[0];
    let iterations = loop_data["iterations"]
        .as_u64()
        .expect("Missing iterations");

    assert_eq!(iterations, 1000000, "Incorrect iteration count");

    // Cleanup
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file("/tmp/test_loop_counting");
    let _ = fs::remove_file("/tmp/profile_loop.json");
}

#[test]
fn test_branch_statistics() {
    // RED: This test WILL FAIL because branch statistics tracking doesn't exist yet
    //
    // Purpose: Track branch taken/not-taken for branch prediction tuning
    //
    // Expected JSON output:
    // {
    //   "branches": [
    //     {
    //       "location": "is_prime:3",
    //       "taken": 78498,
    //       "not_taken": 21502,
    //       "prediction_rate": 0.78498
    //     }
    //   ]
    // }

    let test_file = "/tmp/test_branch_stats.ruchy";
    fs::write(
        test_file,
        r#"
fun is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i = i + 1;
    }
    return true;
}

fun main() {
    let mut count = 0;
    for i in 0..100000 {
        if is_prime(i) {
            count = count + 1;
        }
    }
    println(count);
}
"#,
    )
    .expect("Failed to write test file");

    // Compile with instrumentation
    let compile_output = Command::new(get_ruchy_path())
        .args(&[
            "compile",
            "--instrument",
            "--output=/tmp/test_branch_stats",
            test_file,
        ])
        .output()
        .expect("Failed to compile");

    assert!(
        compile_output.status.success(),
        "Compilation failed: {}",
        String::from_utf8_lossy(&compile_output.stderr)
    );

    // Run with profiling
    let run_output = Command::new("/tmp/test_branch_stats")
        .env("RUCHY_PROFILE", "1")
        .env("RUCHY_PROFILE_OUTPUT", "/tmp/profile_branch.json")
        .output()
        .expect("Failed to run");

    assert!(run_output.status.success());

    // Parse profile
    let profile_data =
        fs::read_to_string("/tmp/profile_branch.json").expect("Failed to read profile output");

    let profile: serde_json::Value = serde_json::from_str(&profile_data).expect("Invalid JSON");

    // Validate branch data
    let branches = profile["branches"]
        .as_array()
        .expect("Missing branches array");

    assert!(!branches.is_empty(), "No branch data collected");

    // Verify branch statistics
    for branch in branches {
        let taken = branch["taken"].as_u64().expect("Missing taken");
        let not_taken = branch["not_taken"].as_u64().expect("Missing not_taken");
        let total = taken + not_taken;

        assert!(total > 0, "No branch executions recorded");

        let prediction_rate = branch["prediction_rate"]
            .as_f64()
            .expect("Missing prediction_rate");

        assert!(
            prediction_rate >= 0.0 && prediction_rate <= 1.0,
            "Invalid prediction rate: {}",
            prediction_rate
        );
    }

    // Cleanup
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file("/tmp/test_branch_stats");
    let _ = fs::remove_file("/tmp/profile_branch.json");
}

#[test]
fn test_memory_allocation_tracking() {
    // RED: This test WILL FAIL because memory allocation tracking doesn't exist yet
    //
    // Purpose: Track allocations for custom allocator optimization
    //
    // Expected JSON output:
    // {
    //   "allocations": {
    //     "total_allocs": 10000,
    //     "total_bytes": 4000000,
    //     "peak_memory_bytes": 800000,
    //     "by_size": {
    //       "small": {"count": 9000, "bytes": 36000},
    //       "medium": {"count": 900, "bytes": 360000},
    //       "large": {"count": 100, "bytes": 3604000}
    //     }
    //   ]
    // }

    let test_file = "/tmp/test_alloc_tracking.ruchy";
    fs::write(
        test_file,
        r#"
fun main() {
    let mut vectors: Vec<Vec<i64>> = vec![];
    for i in 0..1000 {
        let mut v: Vec<i64> = vec![];
        for j in 0..100 {
            v.push(j);
        }
        vectors.push(v);
    }
    println(vectors.len());
}
"#,
    )
    .expect("Failed to write test file");

    // Compile with instrumentation
    let compile_output = Command::new(get_ruchy_path())
        .args(&[
            "compile",
            "--instrument",
            "--output=/tmp/test_alloc_tracking",
            test_file,
        ])
        .output()
        .expect("Failed to compile");

    assert!(
        compile_output.status.success(),
        "Compilation failed: {}",
        String::from_utf8_lossy(&compile_output.stderr)
    );

    // Run with profiling
    let run_output = Command::new("/tmp/test_alloc_tracking")
        .env("RUCHY_PROFILE", "1")
        .env("RUCHY_PROFILE_OUTPUT", "/tmp/profile_alloc.json")
        .output()
        .expect("Failed to run");

    assert!(run_output.status.success());

    // Parse profile
    let profile_data =
        fs::read_to_string("/tmp/profile_alloc.json").expect("Failed to read profile output");

    let profile: serde_json::Value = serde_json::from_str(&profile_data).expect("Invalid JSON");

    // Validate allocation data
    let allocs = &profile["allocations"];

    let total_allocs = allocs["total_allocs"]
        .as_u64()
        .expect("Missing total_allocs");
    let total_bytes = allocs["total_bytes"].as_u64().expect("Missing total_bytes");
    let peak_memory = allocs["peak_memory_bytes"]
        .as_u64()
        .expect("Missing peak_memory_bytes");

    assert!(total_allocs > 0, "No allocations tracked");
    assert!(total_bytes > 0, "No bytes tracked");
    assert!(peak_memory > 0, "No peak memory tracked");

    // Verify size categorization
    let by_size = allocs["by_size"]
        .as_object()
        .expect("Missing by_size breakdown");

    assert!(
        by_size.contains_key("small"),
        "Missing small allocation category"
    );
    assert!(
        by_size.contains_key("medium"),
        "Missing medium allocation category"
    );
    assert!(
        by_size.contains_key("large"),
        "Missing large allocation category"
    );

    // Cleanup
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file("/tmp/test_alloc_tracking");
    let _ = fs::remove_file("/tmp/profile_alloc.json");
}

#[test]
fn test_instrumentation_overhead() {
    // RED: This test WILL FAIL because overhead measurement infrastructure doesn't exist yet
    //
    // Purpose: Verify <1% overhead requirement (statistical rigor: p < 0.05, N≥30)
    //
    // Research foundation:
    // - Georges et al. (2007): N≥30 runs required for statistical validity
    // - Welch's t-test for significance testing
    // - Coefficient of variation <5% for stability

    let test_file = "/tmp/test_overhead.ruchy";
    fs::write(
        test_file,
        r#"
fun compute_intensive() -> i64 {
    let mut sum = 0;
    for i in 0..1000000 {
        sum = sum + (i * i) % 997;
    }
    return sum;
}

fun main() {
    let result = compute_intensive();
    println(result);
}
"#,
    )
    .expect("Failed to write test file");

    // Compile without instrumentation (baseline)
    let compile_baseline = Command::new(get_ruchy_path())
        .args(&["compile", "--output=/tmp/test_overhead_baseline", test_file])
        .output()
        .expect("Failed to compile baseline");

    assert!(compile_baseline.status.success());

    // Compile with instrumentation
    let compile_instrumented = Command::new(get_ruchy_path())
        .args(&[
            "compile",
            "--instrument",
            "--output=/tmp/test_overhead_instrumented",
            test_file,
        ])
        .output()
        .expect("Failed to compile instrumented");

    assert!(compile_instrumented.status.success());

    // Run baseline N=30 times (Georges et al. 2007)
    const N: usize = 30;
    let mut baseline_times: Vec<Duration> = Vec::new();

    for _ in 0..N {
        let start = Instant::now();
        let output = Command::new("/tmp/test_overhead_baseline")
            .output()
            .expect("Failed to run baseline");
        let elapsed = start.elapsed();

        assert!(output.status.success());
        baseline_times.push(elapsed);
    }

    // Run instrumented N=30 times
    let mut instrumented_times: Vec<Duration> = Vec::new();

    for _ in 0..N {
        let start = Instant::now();
        let output = Command::new("/tmp/test_overhead_instrumented")
            .env("RUCHY_PROFILE", "1")
            .env("RUCHY_PROFILE_OUTPUT", "/tmp/profile_overhead.json")
            .output()
            .expect("Failed to run instrumented");
        let elapsed = start.elapsed();

        assert!(output.status.success());
        instrumented_times.push(elapsed);
    }

    // Calculate statistics
    let baseline_mean = baseline_times
        .iter()
        .map(|d| d.as_nanos() as f64)
        .sum::<f64>()
        / N as f64;

    let instrumented_mean = instrumented_times
        .iter()
        .map(|d| d.as_nanos() as f64)
        .sum::<f64>()
        / N as f64;

    let overhead_percent = ((instrumented_mean - baseline_mean) / baseline_mean) * 100.0;

    // Validate <1% overhead requirement
    assert!(
        overhead_percent < 1.0,
        "Overhead {}% exceeds 1% target (baseline: {:.2}ns, instrumented: {:.2}ns)",
        overhead_percent,
        baseline_mean,
        instrumented_mean
    );

    // Calculate coefficient of variation (CV <5% for stability)
    let baseline_std = calculate_std_dev(&baseline_times);
    let baseline_cv = (baseline_std / baseline_mean) * 100.0;

    assert!(
        baseline_cv < 5.0,
        "Baseline CV {}% exceeds 5% threshold (unstable measurements)",
        baseline_cv
    );

    // Cleanup
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file("/tmp/test_overhead_baseline");
    let _ = fs::remove_file("/tmp/test_overhead_instrumented");
    let _ = fs::remove_file("/tmp/profile_overhead.json");
}

#[test]
fn test_json_output_format() {
    // RED: This test WILL FAIL because JSON schema validation doesn't exist yet
    //
    // Purpose: Validate complete JSON output schema
    //
    // Expected complete schema:
    // {
    //   "version": "1.0",
    //   "timestamp": "2025-11-09T12:34:56Z",
    //   "binary": "/tmp/test_program",
    //   "functions": [...],
    //   "loops": [...],
    //   "branches": [...],
    //   "allocations": {...},
    //   "statistics": {
    //     "total_runtime_ns": 123456789,
    //     "instrumentation_overhead_percent": 0.5
    //   }
    // }

    let test_file = "/tmp/test_json_format.ruchy";
    fs::write(
        test_file,
        r#"
fun main() {
    println("Hello, profiling!");
}
"#,
    )
    .expect("Failed to write test file");

    // Compile with instrumentation
    let compile_output = Command::new(get_ruchy_path())
        .args(&[
            "compile",
            "--instrument",
            "--output=/tmp/test_json_format",
            test_file,
        ])
        .output()
        .expect("Failed to compile");

    assert!(compile_output.status.success());

    // Run with profiling
    let run_output = Command::new("/tmp/test_json_format")
        .env("RUCHY_PROFILE", "1")
        .env("RUCHY_PROFILE_OUTPUT", "/tmp/profile_format.json")
        .output()
        .expect("Failed to run");

    assert!(run_output.status.success());

    // Parse and validate complete schema
    let profile_data =
        fs::read_to_string("/tmp/profile_format.json").expect("Failed to read profile output");

    let profile: serde_json::Value = serde_json::from_str(&profile_data).expect("Invalid JSON");

    // Validate top-level fields
    assert!(profile.get("version").is_some(), "Missing version field");
    assert!(
        profile.get("timestamp").is_some(),
        "Missing timestamp field"
    );
    assert!(profile.get("binary").is_some(), "Missing binary field");
    assert!(
        profile.get("functions").is_some(),
        "Missing functions array"
    );
    assert!(profile.get("loops").is_some(), "Missing loops array");
    assert!(profile.get("branches").is_some(), "Missing branches array");
    assert!(
        profile.get("allocations").is_some(),
        "Missing allocations object"
    );
    assert!(
        profile.get("statistics").is_some(),
        "Missing statistics object"
    );

    // Validate version
    let version = profile["version"].as_str().expect("Invalid version");
    assert_eq!(version, "1.0", "Unexpected version");

    // Validate statistics
    let stats = &profile["statistics"];
    assert!(
        stats.get("total_runtime_ns").is_some(),
        "Missing total_runtime_ns"
    );
    assert!(
        stats.get("instrumentation_overhead_percent").is_some(),
        "Missing overhead metric"
    );

    // Cleanup
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file("/tmp/test_json_format");
    let _ = fs::remove_file("/tmp/profile_format.json");
}

// Helper function for standard deviation calculation
fn calculate_std_dev(times: &[Duration]) -> f64 {
    let n = times.len() as f64;
    let mean = times.iter().map(|d| d.as_nanos() as f64).sum::<f64>() / n;

    let variance = times
        .iter()
        .map(|d| {
            let diff = d.as_nanos() as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / n;

    variance.sqrt()
}
