# Compiler Integration API for Zero-Cost Tracing

## Overview

This document defines the API that the **Ruchy compiler** (at `paiml/ruchy`) would use to integrate zero-cost tracing infrastructure. This is research infrastructure from RuchyRuchy that demonstrates:

1. **Zero overhead when disabled** (conditional compilation)
2. **Type-aware tracing** (leveraging Ruchy's type system)
3. **Per-thread lock-free buffers** (no contention)
4. **Strace-style JSON/text output**

## Purpose

RuchyRuchy is **research infrastructure** for the Ruchy compiler. This API demonstrates how the main compiler could add `--trace` flags without impacting normal execution.

## Integration Points

### 1. Command-Line Flags

The Ruchy compiler would add these flags:

```bash
# Enable function-level tracing
ruchy run program.ruchy --trace

# Enable tracing with sampling (1 in N calls)
ruchy run program.ruchy --trace --trace-sample=1000

# Filter by function pattern
ruchy run program.ruchy --trace --trace-filter="important_*"

# Specify output format
ruchy run program.ruchy --trace --trace-output=trace.json
ruchy run program.ruchy --trace --trace-format=text
```

### 2. Code Generation Hooks

When compiling with `--trace`, the compiler would inject instrumentation:

#### Function Entry

**Original Ruchy Code**:
```ruchy
fun fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

**Generated Code with --trace** (conceptual Rust output):
```rust
fn fibonacci(n: i64) -> i64 {
    #[cfg(feature = "trace")]
    let _trace_entry = {
        use ruchyruchy::tracing::*;
        let entry_time = std::time::Instant::now();
        let event = events::function_enter(
            "fibonacci",
            vec![
                events::TypedValue {
                    type_info: events::TypeInfo {
                        name: "i64".to_string(),
                        fields: None,
                    },
                    value: serde_json::json!(n),
                }
            ],
            events::SourceLocation {
                file: "program.ruchy".to_string(),
                line: 1,
                column: 5,
            },
        );
        buffer::record_event(event);
        entry_time
    };

    // Original user code
    let result = if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    };

    #[cfg(feature = "trace")]
    {
        use ruchyruchy::tracing::*;
        let exit_event = events::function_exit(
            "fibonacci",
            Some(events::TypedValue {
                type_info: events::TypeInfo {
                    name: "i64".to_string(),
                    fields: None,
                },
                value: serde_json::json!(result),
            }),
            _trace_entry.elapsed().as_nanos() as u64,
        );
        buffer::record_event(exit_event);
    }

    result
}
```

### 3. Runtime Initialization

At program startup, the compiler-generated code initializes tracing:

```rust
fn main() {
    #[cfg(feature = "trace")]
    ruchyruchy::tracing::init_tracing(TraceConfig {
        sample_rate: 1,  // Trace every call (or 1000 for sampling)
        filter: None,    // Or Some("important_*")
        output_file: "/tmp/trace.json".to_string(),
    });

    // User's main function
    user_main();

    #[cfg(feature = "trace")]
    ruchyruchy::tracing::finalize_tracing();
}
```

### 4. Trace Finalization

At program exit, collect all events and write output:

```rust
pub fn finalize_tracing() {
    use ruchyruchy::tracing::*;

    // Drain events from current thread
    let events = buffer::drain_thread_events();

    // Create trace file
    let trace = output::TraceFile {
        metadata: output::TraceMetadata {
            program: std::env::args().next().unwrap(),
            start_time: chrono::Utc::now().to_rfc3339(),
            ruchy_version: env!("RUCHY_VERSION"),
            ruchyruchy_version: env!("CARGO_PKG_VERSION"),
        },
        events,
        stats: output::TraceStats {
            total_events: events.len(),
            dropped_events: buffer::thread_buffer_stats().1,
            duration_ns: /* track separately */,
        },
    };

    // Write to file
    let formatter = output::JsonFormatter::new(true);
    let mut file = std::fs::File::create("/tmp/trace.json").unwrap();
    formatter.write(&trace, &mut file).unwrap();
}
```

## API Reference

### Module: `ruchyruchy::tracing::events`

#### `function_enter(name: &str, args: Vec<TypedValue>, location: SourceLocation) -> TraceEvent`

Creates a function entry event.

**Parameters**:
- `name`: Function name
- `args`: Typed function arguments
- `location`: Source code location

**Example**:
```rust
let event = function_enter(
    "add",
    vec![
        TypedValue { type_info: TypeInfo { name: "i64", fields: None }, value: json!(10) },
        TypedValue { type_info: TypeInfo { name: "i64", fields: None }, value: json!(20) },
    ],
    SourceLocation { file: "test.ruchy", line: 5, column: 1 },
);
```

#### `function_exit(name: &str, return_value: Option<TypedValue>, duration_ns: u64) -> TraceEvent`

Creates a function exit event.

**Parameters**:
- `name`: Function name
- `return_value`: Typed return value (None for void functions)
- `duration_ns`: Function execution time in nanoseconds

### Module: `ruchyruchy::tracing::buffer`

#### `record_event(event: TraceEvent)`

Records an event to the current thread's lock-free buffer.

**Thread Safety**: Thread-local storage, no locks required.

#### `drain_thread_events() -> Vec<TraceEvent>`

Drains all events from the current thread's buffer.

**Usage**: Call this at program exit to collect traces.

#### `thread_buffer_stats() -> (usize, usize)`

Returns `(current_event_count, dropped_event_count)`.

### Module: `ruchyruchy::tracing::output`

#### `JsonFormatter::new(pretty: bool) -> Self`

Creates a JSON formatter.

**Parameters**:
- `pretty`: If true, format with indentation. If false, compact JSON.

#### `JsonFormatter::format(&self, trace: &TraceFile) -> Result<String, serde_json::Error>`

Formats a trace file to JSON string.

#### `JsonFormatter::write<W: Write>(&self, trace: &TraceFile, writer: &mut W) -> std::io::Result<()>`

Writes trace file to a writer (file, stdout, etc).

#### `TextFormatter::format_event(event: &TraceEvent) -> String`

Formats a single event as strace-style text.

**Output Example**:
```
[1761761183.405413] -> fibonacci(i64=5) <demo.ruchy:10:5>
[1761761183.405447] <- fibonacci() = 5 [0.040ms]
```

## Type System Integration

### Primitive Types

```rust
TypedValue {
    type_info: TypeInfo {
        name: "i64",      // or "f64", "bool", "String", etc.
        fields: None,
    },
    value: serde_json::json!(42),
}
```

### Struct Types

```rust
// For: struct User { id: i64, name: String }
TypedValue {
    type_info: TypeInfo {
        name: "User",
        fields: Some(vec![
            ("id".to_string(), "i64".to_string()),
            ("name".to_string(), "String".to_string()),
        ]),
    },
    value: serde_json::json!({
        "id": 42,
        "name": "Alice"
    }),
}
```

### Enum Types

```rust
// For: enum Result { Ok(i64), Err(String) }
TypedValue {
    type_info: TypeInfo {
        name: "Result",
        fields: Some(vec![
            ("Ok".to_string(), "i64".to_string()),
            ("Err".to_string(), "String".to_string()),
        ]),
    },
    value: serde_json::json!({
        "variant": "Ok",
        "value": 100
    }),
}
```

## Performance Characteristics

### Zero-Cost When Disabled

When compiled without `--trace`, all instrumentation is removed via `#[cfg(feature = "trace")]`:

```bash
# Without tracing (baseline)
ruchy build program.ruchy
# Generated code has ZERO instrumentation overhead

# With tracing capability
ruchy build program.ruchy --features trace
# Generated code includes instrumentation, but still zero overhead if not enabled
```

**Benchmark**: See `tests/test_compiler_instrumentation.rs::test_zero_cost_when_disabled`

### Overhead When Enabled

| Function Size | Without Sampling | With Sampling (1/1000) |
|---------------|------------------|------------------------|
| Tiny (1-5 LOC) | 100x-1000x overhead | 1.1x-1.2x overhead |
| Medium (10-50 LOC) | 5x-10x overhead | <1.05x overhead |
| Large (100+ LOC) | 1.2x-2x overhead | <1.01x overhead |

**Mitigation**: Use `--trace-sample=1000` for tiny functions.

### Memory Usage

- **Buffer Size**: 10,000 events per thread (configurable)
- **Event Size**: ~200 bytes per event
- **Total**: ~2MB per thread maximum

**Overflow Behavior**: Oldest events dropped when buffer full (FIFO).

## Example Output

### JSON Format

```json
{
  "metadata": {
    "program": "fibonacci.ruchy",
    "start_time": "2025-10-29T18:06:23.405460200+00:00",
    "ruchy_version": "3.147.7",
    "ruchyruchy_version": "1.6.1"
  },
  "events": [
    {
      "type": "function_enter",
      "name": "fibonacci",
      "args": [
        {
          "type_info": { "name": "i64", "fields": null },
          "value": 5
        }
      ],
      "location": { "file": "fibonacci.ruchy", "line": 1, "column": 5 },
      "timestamp_ns": 1761761183405413218,
      "thread_id": 7726391737551464215
    },
    {
      "type": "function_exit",
      "name": "fibonacci",
      "return_value": {
        "type_info": { "name": "i64", "fields": null },
        "value": 5
      },
      "duration_ns": 39801,
      "timestamp_ns": 1761761183405447250,
      "thread_id": 7726391737551464215
    }
  ],
  "stats": {
    "total_events": 30,
    "dropped_events": 0,
    "duration_ns": 40221
  }
}
```

### Text Format (strace-style)

```
[1761761183.405413] -> fibonacci(i64=5) <fibonacci.ruchy:1:5>
[1761761183.405433] -> fibonacci(i64=4) <fibonacci.ruchy:1:5>
[1761761183.405434] -> fibonacci(i64=3) <fibonacci.ruchy:1:5>
[1761761183.405436] -> fibonacci(i64=2) <fibonacci.ruchy:1:5>
[1761761183.405436] -> fibonacci(i64=1) <fibonacci.ruchy:1:5>
[1761761183.405438] <- fibonacci() = 1 [0.002ms]
[1761761183.405438] -> fibonacci(i64=0) <fibonacci.ruchy:1:5>
[1761761183.405439] <- fibonacci() = 0 [0.000ms]
[1761761183.405439] <- fibonacci() = 1 [0.005ms]
[1761761183.405440] -> fibonacci(i64=1) <fibonacci.ruchy:1:5>
[1761761183.405440] <- fibonacci() = 1 [0.000ms]
[1761761183.405441] <- fibonacci() = 2 [0.007ms]
[1761761183.405447] <- fibonacci() = 5 [0.040ms]
```

## Next Steps for Ruchy Compiler Integration

### Phase 1: Proof of Concept (1 week)
1. Add `--trace` flag to Ruchy CLI
2. Inject simple function entry/exit instrumentation
3. Use RuchyRuchy infrastructure for buffering/output
4. Verify zero overhead when disabled

### Phase 2: Type-Aware Tracing (2 weeks)
1. Extract type information during type checking
2. Generate `TypedValue` structures for all traced values
3. Handle structs, enums, generics

### Phase 3: Sampling & Filtering (1 week)
1. Add `--trace-sample=N` flag
2. Add `--trace-filter=pattern` flag
3. Optimize for tiny functions

### Phase 4: Multi-Threading (2 weeks)
1. Per-thread buffer collection
2. Merge events from all threads in finalization
3. Thread-safe output writing

## Testing

See `tests/test_compiler_instrumentation.rs` for comprehensive test suite covering:
- ✅ Zero-cost when disabled
- ✅ Type-aware tracing
- ✅ Function entry/exit
- ✅ Sampling overhead reduction
- ✅ Filtering by function pattern
- ✅ Per-thread buffers
- ✅ Source map integration

## References

- **Main Specification**: `docs/specifications/ruchydbg-run-deep-tracing-strace-style.md`
- **Reality Check Addendum**: `docs/specifications/ruchydbg-run-deep-tracing-ADDENDUM-REALITY-CHECK.md`
- **Demo**: `examples/manual_instrumentation_demo.rs`
- **Infrastructure Tests**: `src/tracing/*/tests.rs`

## Contact

For questions about integrating this into the Ruchy compiler:
- File issue at: https://github.com/paiml/ruchy/issues
- Reference: RuchyRuchy DEBUGGER-014 (Zero-Cost Compiler Instrumentation)
