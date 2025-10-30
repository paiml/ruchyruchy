// DEBUGGER-015: eBPF Syscall Tracing Tests (RED PHASE)
//
// Tests for low-overhead syscall tracing using eBPF (not ptrace)
//
// Expected behavior:
// - <1% overhead for syscall-heavy workloads
// - Accurate syscall decoding (50+ common syscalls)
// - Correlation with function traces (by PID + timestamp)
// - strace-compatible text output
// - JSON output for machine processing
//
// Technical Approach:
// - Use Aya (pure Rust eBPF library)
// - Attach to raw_syscalls:sys_enter/exit tracepoints
// - Decode syscall arguments using kernel BTF
// - Merge with function traces from DEBUGGER-014

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

fn get_ruchy_path() -> PathBuf {
    PathBuf::from("ruchy")
}

#[test]
#[ignore] // RED: Will fail until eBPF infrastructure exists
fn test_ebpf_syscall_capture() {
    // RED: This test WILL FAIL because eBPF syscall tracing doesn't exist yet
    //
    // This test verifies that we can capture syscalls using eBPF (not ptrace).
    // We trace a simple Ruchy program that makes a few syscalls (open, read, write).

    let test_file = "/tmp/test_syscall_capture.ruchy";
    fs::write(
        test_file,
        r#"
fun main() {
    // This will trigger open, write, close syscalls
    let file = open("/tmp/test.txt", "w");
    write(file, "Hello, eBPF!");
    close(file);

    // This will trigger open, read, close syscalls
    let file2 = open("/tmp/test.txt", "r");
    let content = read(file2);
    close(file2);

    println(content);
}
"#,
    )
    .unwrap();

    // Run with eBPF syscall tracing enabled
    let output = Command::new("ruchydbg")
        .arg("run")
        .arg(test_file)
        .arg("--trace-syscalls") // Not implemented yet
        .arg("--trace-output=/tmp/syscall_trace.json") // Not implemented yet
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(output.status.success(), "Syscall tracing run failed");

    // Check that trace file contains syscall events
    let trace = fs::read_to_string("/tmp/syscall_trace.json").unwrap();

    // Should capture open syscalls
    assert!(
        trace.contains("\"syscall\":\"open\""),
        "Missing open syscall"
    );
    assert!(
        trace.contains("\"/tmp/test.txt\""),
        "Missing filename argument"
    );

    // Should capture write syscall
    assert!(
        trace.contains("\"syscall\":\"write\""),
        "Missing write syscall"
    );
    assert!(trace.contains("\"Hello, eBPF!\""), "Missing write data");

    // Should capture read syscall
    assert!(
        trace.contains("\"syscall\":\"read\""),
        "Missing read syscall"
    );

    // Should capture close syscalls
    assert!(
        trace.contains("\"syscall\":\"close\""),
        "Missing close syscall"
    );
}

#[test]
#[ignore] // RED: Will fail until syscall decoder exists
fn test_syscall_decoding() {
    // RED: This test WILL FAIL because syscall argument decoding doesn't exist yet
    //
    // This test verifies that we correctly decode syscall arguments for common syscalls.
    // We need to support at least 50 common syscalls with proper argument decoding.

    let test_file = "/tmp/test_syscall_decode.ruchy";
    fs::write(
        test_file,
        r#"
fun main() {
    // Test various syscalls
    open("/tmp/file.txt", "r");           // open(pathname, flags)
    read(3, 100);                          // read(fd, count)
    write(1, "hello");                     // write(fd, buf)
    stat("/tmp/file.txt");                 // stat(pathname)
    getpid();                              // getpid() - no args
}
"#,
    )
    .unwrap();

    let output = Command::new("ruchydbg")
        .arg("run")
        .arg(test_file)
        .arg("--trace-syscalls")
        .arg("--trace-output=/tmp/decode_trace.json")
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(output.status.success());

    let trace = fs::read_to_string("/tmp/decode_trace.json").unwrap();

    // Verify open() argument decoding
    assert!(
        trace.contains("\"args\":[{\"/tmp/file.txt\",\"O_RDONLY\"}]"),
        "open() arguments not decoded correctly"
    );

    // Verify read() argument decoding
    assert!(
        trace.contains("\"args\":[{\"fd\":3,\"count\":100}]"),
        "read() arguments not decoded correctly"
    );

    // Verify write() argument decoding
    assert!(
        trace.contains("\"args\":[{\"fd\":1,\"buf\":\"hello\"}]"),
        "write() arguments not decoded correctly"
    );

    // Verify return values
    assert!(
        trace.contains("\"return\":"),
        "Missing syscall return values"
    );
}

#[test]
#[ignore] // RED: Will fail until correlation works
fn test_correlation_with_functions() {
    // RED: This test WILL FAIL because correlation doesn't exist yet
    //
    // This test verifies that we can correlate syscalls with function traces.
    // Each syscall should be linked to the function that called it.

    let test_file = "/tmp/test_correlation.ruchy";
    fs::write(
        test_file,
        r#"
fun write_file(filename: String, content: String) {
    let file = open(filename, "w");
    write(file, content);
    close(file);
}

fun main() {
    write_file("/tmp/test.txt", "Hello");
}
"#,
    )
    .unwrap();

    let output = Command::new("ruchydbg")
        .arg("run")
        .arg(test_file)
        .arg("--trace") // Function tracing (DEBUGGER-014)
        .arg("--trace-syscalls") // Syscall tracing (DEBUGGER-015)
        .arg("--trace-output=/tmp/correlated_trace.json")
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(output.status.success());

    let trace = fs::read_to_string("/tmp/correlated_trace.json").unwrap();

    // Should have function entry for write_file
    assert!(trace.contains("\"function\":\"write_file\""));

    // Should have syscalls INSIDE write_file function
    // Syscalls should have parent_function field linking to write_file
    assert!(
        trace.contains("\"parent_function\":\"write_file\""),
        "Syscalls not correlated with parent function"
    );

    // Syscalls should be timestamped between function enter/exit
    // (This requires parsing JSON and validating timestamp ordering)
}

#[test]
#[ignore] // RED: Will fail until overhead measurement exists
fn test_overhead_under_1_percent() {
    // RED: This test WILL FAIL because eBPF tracing doesn't exist yet
    //
    // CRITICAL: eBPF syscall tracing must have <1% overhead.
    // We test with a syscall-heavy program (many small reads/writes).

    let test_file = "/tmp/test_overhead.ruchy";
    fs::write(
        test_file,
        r#"
fun main() {
    // Make 1000 syscalls (open/read/close in loop)
    let mut i = 0;
    while i < 1000 {
        let file = open("/dev/null", "r");
        let _ = read(file, 1);
        close(file);
        i = i + 1;
    }
}
"#,
    )
    .unwrap();

    // Baseline: Run without eBPF tracing
    let mut baseline_times = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        let output = Command::new(get_ruchy_path())
            .arg("run")
            .arg(test_file)
            .output()
            .expect("Failed to execute ruchy");
        assert!(output.status.success());
        baseline_times.push(start.elapsed());
    }

    let baseline_median = {
        baseline_times.sort();
        baseline_times[baseline_times.len() / 2]
    };

    // With eBPF tracing: Should have <1% overhead
    let mut traced_times = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        let output = Command::new("ruchydbg")
            .arg("run")
            .arg(test_file)
            .arg("--trace-syscalls")
            .output()
            .expect("Failed to execute ruchydbg");
        assert!(output.status.success());
        traced_times.push(start.elapsed());
    }

    let traced_median = {
        traced_times.sort();
        traced_times[traced_times.len() / 2]
    };

    // Calculate overhead
    let overhead_percent =
        ((traced_median.as_micros() as f64 / baseline_median.as_micros() as f64) - 1.0) * 100.0;

    // eBPF overhead requirement: <1%
    assert!(
        overhead_percent < 1.0,
        "eBPF overhead too high: {:.2}% (baseline: {:?}, traced: {:?})",
        overhead_percent,
        baseline_median,
        traced_median
    );
}

#[test]
#[ignore] // RED: Will fail until strace-compatible output exists
fn test_strace_compatible_output() {
    // RED: This test WILL FAIL because strace-style output doesn't exist yet
    //
    // Output should be compatible with strace format for easy migration.

    let test_file = "/tmp/test_strace_format.ruchy";
    fs::write(
        test_file,
        r#"
fun main() {
    open("/tmp/file.txt", "r");
    write(1, "hello");
}
"#,
    )
    .unwrap();

    let output = Command::new("ruchydbg")
        .arg("run")
        .arg(test_file)
        .arg("--trace-syscalls")
        .arg("--trace-format=strace") // Not implemented yet
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(output.status.success());

    let strace_output = String::from_utf8(output.stderr).unwrap();

    // Should match strace format:
    // open("/tmp/file.txt", O_RDONLY) = 3
    // write(1, "hello", 5) = 5

    assert!(
        strace_output.contains("open(\"/tmp/file.txt\", O_RDONLY)"),
        "Not strace-compatible format"
    );
    assert!(
        strace_output.contains("= 3") || strace_output.contains("= 4"),
        "Missing return value"
    );
    assert!(
        strace_output.contains("write(1, \"hello\","),
        "write() not strace-compatible"
    );
}

#[test]
#[ignore] // RED: Will fail until JSON output exists
fn test_json_output_format() {
    // RED: This test WILL FAIL because JSON syscall output doesn't exist yet
    //
    // JSON output should include rich metadata for machine processing.

    let test_file = "/tmp/test_json_syscall.ruchy";
    fs::write(
        test_file,
        r#"
fun main() {
    open("/tmp/file.txt", "r");
}
"#,
    )
    .unwrap();

    let output = Command::new("ruchydbg")
        .arg("run")
        .arg(test_file)
        .arg("--trace-syscalls")
        .arg("--trace-output=/tmp/syscall.json")
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(output.status.success());

    let trace = fs::read_to_string("/tmp/syscall.json").unwrap();

    // Should be valid JSON
    let json: serde_json::Value = serde_json::from_str(&trace).expect("Invalid JSON output");

    // Should have metadata
    assert!(json["metadata"].is_object(), "Missing metadata");
    assert!(
        json["metadata"]["program"].is_string(),
        "Missing program name"
    );

    // Should have syscall events
    assert!(json["events"].is_array(), "Missing events array");
    let events = json["events"].as_array().unwrap();
    assert!(!events.is_empty(), "No syscall events captured");

    // Each event should have required fields
    let first_event = &events[0];
    assert!(
        first_event["type"].as_str() == Some("syscall"),
        "Wrong event type"
    );
    assert!(first_event["name"].is_string(), "Missing syscall name");
    assert!(first_event["args"].is_array(), "Missing syscall args");
    assert!(
        first_event["return_value"].is_number(),
        "Missing return value"
    );
    assert!(first_event["duration_ns"].is_number(), "Missing duration");
    assert!(first_event["timestamp_ns"].is_number(), "Missing timestamp");
    assert!(first_event["pid"].is_number(), "Missing PID");
    assert!(first_event["tid"].is_number(), "Missing TID");
}

#[test]
#[ignore] // RED: Will fail until filtering exists
fn test_filtering_by_syscall_pattern() {
    // RED: This test WILL FAIL because syscall filtering doesn't exist yet
    //
    // Should be able to filter syscalls by pattern (e.g., only file operations).

    let test_file = "/tmp/test_filter.ruchy";
    fs::write(
        test_file,
        r#"
fun main() {
    open("/tmp/file.txt", "r");  // File syscall
    read(3, 100);                 // File syscall
    getpid();                     // Process syscall (should be filtered out)
    write(1, "hello");           // File syscall
}
"#,
    )
    .unwrap();

    let output = Command::new("ruchydbg")
        .arg("run")
        .arg(test_file)
        .arg("--trace-syscalls")
        .arg("--syscall-filter=file") // Only file operations
        .arg("--trace-output=/tmp/filtered.json")
        .output()
        .expect("Failed to execute ruchydbg");

    assert!(output.status.success());

    let trace = fs::read_to_string("/tmp/filtered.json").unwrap();

    // Should include file syscalls
    assert!(
        trace.contains("\"name\":\"open\""),
        "Missing filtered syscall"
    );
    assert!(
        trace.contains("\"name\":\"read\""),
        "Missing filtered syscall"
    );
    assert!(
        trace.contains("\"name\":\"write\""),
        "Missing filtered syscall"
    );

    // Should NOT include process syscalls
    assert!(
        !trace.contains("\"name\":\"getpid\""),
        "Filter didn't exclude non-file syscalls"
    );
}
