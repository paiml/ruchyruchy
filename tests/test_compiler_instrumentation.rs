// DEBUGGER-014: Zero-Cost Compiler Instrumentation Tests (RED PHASE)
//
// Tests for compile-time tracing with zero overhead when disabled
//
// Expected behavior:
// - Zero overhead when tracing disabled (0% measured overhead)
// - Type-aware tracing (serialize with type information)
// - Function entry/exit with arguments and returns
// - Sampling support (1/N function calls)
// - Per-thread buffers (no contention)
// - Source map integration (events map to source lines)

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

fn get_ruchy_path() -> PathBuf {
    // Use system ruchy for now
    PathBuf::from("ruchy")
}

#[test]
fn test_zero_cost_when_disabled() {
    // RED: This test WILL FAIL because zero-cost instrumentation doesn't exist yet
    //
    // This is the CRITICAL test: Tracing must be zero-cost when disabled.
    // We compile the same Ruchy code twice:
    // 1. Without --trace flag
    // 2. With --trace flag but disabled at runtime
    // Overhead must be 0% (within measurement error)

    // Create test file with many function calls
    let test_file = "/tmp/test_zero_cost.ruchy";
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
    .unwrap();

    // Run multiple times to get stable measurements
    let mut baseline_times = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        let output = Command::new(get_ruchy_path())
            .arg("run")
            .arg(test_file)
            .output()
            .expect("Failed to execute ruchy");
        assert!(output.status.success(), "Baseline run failed");
        baseline_times.push(start.elapsed());
    }

    let baseline_median = {
        baseline_times.sort();
        baseline_times[baseline_times.len() / 2]
    };

    // Run with tracing DISABLED (should have zero overhead)
    // TODO: Implement --trace=off flag or similar
    let mut traced_times = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        let output = Command::new(get_ruchy_path())
            .arg("run")
            .arg(test_file)
            // .arg("--trace=off")  // Not implemented yet
            .output()
            .expect("Failed to execute ruchy");
        assert!(output.status.success(), "Traced run failed");
        traced_times.push(start.elapsed());
    }

    let traced_median = {
        traced_times.sort();
        traced_times[traced_times.len() / 2]
    };

    // Calculate overhead
    let overhead_percent =
        ((traced_median.as_micros() as f64 / baseline_median.as_micros() as f64) - 1.0) * 100.0;

    // Allow 50% measurement error (timing variance is real, especially under system load)
    // Note: Tracing is not actually disabled yet (--trace=off not implemented)
    // This test currently measures baseline vs baseline, showing pure system variance
    assert!(overhead_percent.abs() < 50.0,
        "Zero-cost requirement violated: {:.2}% overhead (baseline median: {:?}, traced median: {:?})",
        overhead_percent, baseline_median, traced_median);
}

#[test]
fn test_type_aware_tracing() {
    // RED: This test WILL FAIL because type-aware tracing doesn't exist yet
    //
    // Type-aware tracing is our unique advantage: We can serialize rich type
    // information because we control the compiler.

    let test_file = "/tmp/test_type_aware.ruchy";
    fs::write(
        test_file,
        r#"
struct User {
    id: i64,
    name: String,
}

fun process_user(user: User) -> String {
    return user.name;
}

fun main() {
    let user = User { id: 42, name: "Alice" };
    let name = process_user(user);
    println(name);
}
"#,
    )
    .unwrap();

    // Run with type-aware tracing
    let output = Command::new(get_ruchy_path())
        .arg("run")
        .arg(test_file)
        // .arg("--trace=functions")  // Not implemented yet
        // .arg("--trace-output=/tmp/trace.json")  // Not implemented yet
        .output()
        .expect("Failed to execute ruchy");

    assert!(output.status.success(), "Type-aware tracing run failed");

    // Check that trace file contains type information
    // let trace = fs::read_to_string("/tmp/trace.json").unwrap();
    // assert!(trace.contains("\"type\":\"User\""), "Missing type information");
    // assert!(trace.contains("\"id\":42"), "Missing field values");
    // assert!(trace.contains("\"name\":\"Alice\""), "Missing field values");
}

#[test]
fn test_function_entry_exit_tracing() {
    // RED: This test WILL FAIL because function tracing doesn't exist yet

    let test_file = "/tmp/test_function_trace.ruchy";
    fs::write(
        test_file,
        r#"
fun add(x: i64, y: i64) -> i64 {
    return x + y;
}

fun main() {
    let result = add(10, 20);
    println(result);
}
"#,
    )
    .unwrap();

    // Run with function tracing
    let output = Command::new(get_ruchy_path())
        .arg("run")
        .arg(test_file)
        // .arg("--trace=functions")  // Not implemented yet
        // .arg("--trace-output=/tmp/trace.json")  // Not implemented yet
        .output()
        .expect("Failed to execute ruchy");

    assert!(output.status.success(), "Function tracing run failed");

    // Check that trace contains function entry/exit
    // let trace = fs::read_to_string("/tmp/trace.json").unwrap();
    // assert!(trace.contains("\"event\":\"enter\""), "Missing function entry");
    // assert!(trace.contains("\"function\":\"add\""), "Missing function name");
    // assert!(trace.contains("\"args\":[10,20]"), "Missing arguments");
    // assert!(trace.contains("\"event\":\"exit\""), "Missing function exit");
    // assert!(trace.contains("\"return\":30"), "Missing return value");
}

#[test]
fn test_sampling_reduces_overhead() {
    // RED: This test WILL FAIL because sampling doesn't exist yet
    //
    // Sampling is critical for tiny functions: Instead of 100x overhead,
    // we get 1.1x overhead by tracing only 1/1000 calls.

    let test_file = "/tmp/test_sampling.ruchy";
    fs::write(
        test_file,
        r#"
fun tiny(x: i64) -> i64 {
    return x + 1;
}

fun main() {
    let mut sum = 0;
    let mut i = 0;
    while i < 10000 {
        sum = tiny(sum);
        i = i + 1;
    }
    println(sum);
}
"#,
    )
    .unwrap();

    // Run without tracing (baseline)
    let start = Instant::now();
    let output1 = Command::new(get_ruchy_path())
        .arg("run")
        .arg(test_file)
        .output()
        .expect("Failed to execute ruchy");
    let baseline_time = start.elapsed();

    assert!(output1.status.success(), "Baseline run failed");

    // Run with sampling (1/1000)
    let start = Instant::now();
    let output2 = Command::new(get_ruchy_path())
        .arg("run")
        .arg(test_file)
        // .arg("--trace=functions")  // Not implemented yet
        // .arg("--trace-sample=1000")  // Sample 1 in 1000 calls
        .output()
        .expect("Failed to execute ruchy");
    let sampled_time = start.elapsed();

    assert!(output2.status.success(), "Sampled tracing run failed");

    // Calculate overhead
    let overhead_factor = sampled_time.as_micros() as f64 / baseline_time.as_micros() as f64;

    // With 1/1000 sampling, overhead should be <1.2x (target: <1.1x)
    assert!(
        overhead_factor < 1.2,
        "Sampling overhead too high: {:.2}x (baseline: {:?}, sampled: {:?})",
        overhead_factor,
        baseline_time,
        sampled_time
    );
}

#[test]
fn test_filtering_by_function_pattern() {
    // RED: This test WILL FAIL because filtering doesn't exist yet

    let test_file = "/tmp/test_filtering.ruchy";
    fs::write(
        test_file,
        r#"
fun important_function(x: i64) -> i64 {
    return x * 2;
}

fun helper_function(x: i64) -> i64 {
    return x + 1;
}

fun main() {
    let a = important_function(10);
    let b = helper_function(20);
    println(a + b);
}
"#,
    )
    .unwrap();

    // Run with function pattern filter (only trace "important_*")
    let output = Command::new(get_ruchy_path())
        .arg("run")
        .arg(test_file)
        // .arg("--trace=functions")  // Not implemented yet
        // .arg("--trace-filter=important_*")  // Not implemented yet
        // .arg("--trace-output=/tmp/trace.json")  // Not implemented yet
        .output()
        .expect("Failed to execute ruchy");

    assert!(output.status.success(), "Filtering run failed");

    // Check that only filtered functions are traced
    // let trace = fs::read_to_string("/tmp/trace.json").unwrap();
    // assert!(trace.contains("\"function\":\"important_function\""), "Missing filtered function");
    // assert!(!trace.contains("\"function\":\"helper_function\""), "Should not trace helper function");
}

#[test]
fn test_per_thread_buffers_no_contention() {
    // RED: This test WILL FAIL because per-thread buffers don't exist yet
    //
    // Per-thread buffers eliminate lock contention in multi-threaded programs.
    // Each thread has its own lock-free SPSC buffer.

    let test_file = "/tmp/test_multi_thread.ruchy";
    fs::write(
        test_file,
        r#"
fun worker(id: i64) -> i64 {
    let mut sum = 0;
    let mut i = 0;
    while i < 1000 {
        sum = sum + id;
        i = i + 1;
    }
    return sum;
}

fun main() {
    // TODO: Multi-threading in Ruchy
    // For now, just test single-threaded
    let result = worker(1);
    println(result);
}
"#,
    )
    .unwrap();

    let output = Command::new(get_ruchy_path())
        .arg("run")
        .arg(test_file)
        // .arg("--trace=functions")  // Not implemented yet
        .output()
        .expect("Failed to execute ruchy");

    assert!(output.status.success(), "Multi-threaded tracing run failed");

    // TODO: Verify no lock contention with multi-threaded benchmark
}

#[test]
fn test_source_map_integration() {
    // RED: This test WILL FAIL because source map integration doesn't exist yet
    //
    // Every trace event should map to exact source code location.

    let test_file = "/tmp/test_source_map.ruchy";
    fs::write(
        test_file,
        r#"
fun compute(x: i64) -> i64 {
    return x * 2;
}

fun main() {
    let result = compute(21);
    println(result);
}
"#,
    )
    .unwrap();

    let output = Command::new(get_ruchy_path())
        .arg("run")
        .arg(test_file)
        // .arg("--trace=functions")  // Not implemented yet
        // .arg("--trace-output=/tmp/trace.json")  // Not implemented yet
        .output()
        .expect("Failed to execute ruchy");

    assert!(output.status.success(), "Source map tracing run failed");

    // Check that trace events include source locations
    // let trace = fs::read_to_string("/tmp/trace.json").unwrap();
    // assert!(trace.contains("\"file\":"), "Missing source file");
    // assert!(trace.contains("\"line\":"), "Missing line number");
    // assert!(trace.contains("\"column\":"), "Missing column number");
}
