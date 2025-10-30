//! Manual Instrumentation Demo
//!
//! This example demonstrates how the tracing infrastructure would be used
//! by a compiler when instrumenting code. It shows:
//! - Recording function entry/exit events
//! - Type-aware tracing (capturing typed values)
//! - Per-thread buffers
//! - JSON output generation
//!
//! This simulates what a compiler would generate when compiling with --trace.

use ruchyruchy::tracing::{
    buffer::{drain_thread_events, record_event},
    events::{function_enter, function_exit, SourceLocation, TypeInfo, TypedValue},
    output::{JsonFormatter, TextFormatter, TraceFile, TraceMetadata, TraceStats},
};
use std::time::Instant;

fn main() {
    println!("🔍 Manual Instrumentation Demo");
    println!("Demonstrating zero-cost compiler instrumentation infrastructure");
    println!();

    // Simulate an instrumented program
    println!("Running instrumented fibonacci calculation...");
    let start = Instant::now();

    // This simulates what a compiler would generate
    instrumented_fibonacci(5);

    let duration = start.elapsed();
    println!("Execution completed in {:?}", duration);
    println!();

    // Collect all trace events from thread-local buffer
    let events = drain_thread_events();

    println!("📊 Trace Statistics:");
    println!("  Events captured: {}", events.len());
    println!();

    // Create trace file with metadata
    let trace = TraceFile {
        metadata: TraceMetadata {
            program: "manual_instrumentation_demo".to_string(),
            start_time: chrono::Utc::now().to_rfc3339(),
            ruchy_version: "3.147.7".to_string(),
            ruchyruchy_version: env!("CARGO_PKG_VERSION").to_string(),
        },
        events: events.clone(),
        stats: TraceStats {
            total_events: events.len(),
            dropped_events: 0,
            duration_ns: duration.as_nanos() as u64,
        },
    };

    // Output as JSON
    println!("📝 JSON Output:");
    let formatter = JsonFormatter::new(true);
    match formatter.format(&trace) {
        Ok(json) => {
            println!("{}", json);

            // Write to file
            if let Err(e) = std::fs::write("/tmp/trace_demo.json", json) {
                eprintln!("Failed to write trace file: {}", e);
            } else {
                println!();
                println!("✅ Trace written to /tmp/trace_demo.json");
            }
        }
        Err(e) => eprintln!("Failed to format trace: {}", e),
    }

    println!();
    println!("📄 Text Output (strace-style):");
    let text = TextFormatter::format(&events);
    println!("{}", text);
}

/// Instrumented version of fibonacci
/// This shows what a compiler would generate when compiling with --trace
fn instrumented_fibonacci(n: i64) -> i64 {
    // === COMPILER-GENERATED: Function entry instrumentation ===
    let _entry_time = Instant::now();
    let entry_event = function_enter(
        "fibonacci",
        vec![TypedValue {
            type_info: TypeInfo {
                name: "i64".to_string(),
                fields: None,
            },
            value: serde_json::json!(n),
        }],
        SourceLocation {
            file: "demo.ruchy".to_string(),
            line: 10,
            column: 5,
        },
    );
    record_event(entry_event);
    // === END COMPILER-GENERATED ===

    // Original user code
    let result = if n <= 1 {
        n
    } else {
        instrumented_fibonacci(n - 1) + instrumented_fibonacci(n - 2)
    };

    // === COMPILER-GENERATED: Function exit instrumentation ===
    let exit_event = function_exit(
        "fibonacci",
        Some(TypedValue {
            type_info: TypeInfo {
                name: "i64".to_string(),
                fields: None,
            },
            value: serde_json::json!(result),
        }),
        _entry_time.elapsed().as_nanos() as u64,
    );
    record_event(exit_event);
    // === END COMPILER-GENERATED ===

    result
}
