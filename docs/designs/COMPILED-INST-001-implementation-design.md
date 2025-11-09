# COMPILED-INST-001: AST-Level Instrumentation Implementation Design

**Status**: Design Phase (Awaiting Production Ruchy Implementation)
**Date**: 2025-11-09
**Target**: Production Ruchy Compiler (https://github.com/paiml/ruchy)

## Overview

This document specifies the implementation design for AST-level instrumentation hooks in the **production Ruchy compiler**. This feature will enable profiling of compiled Ruchy binaries with <1% overhead, supporting the mission to make Ruchy the world's fastest compiled language (≥105% of C performance).

## Scope

**What This Document Covers**:
- Design for `ruchy compile --instrument` flag
- AST/IR-level instrumentation insertion points
- Runtime profiling data collection
- JSON output format specification
- Performance overhead requirements (<1% target)

**What This Document Does NOT Cover**:
- RuchyRuchy educational compiler (this is for production Ruchy)
- Hardware profiling (COMPILED-INST-002 - perf_event_open)
- Binary analysis (COMPILED-INST-003)

## Implementation Location

**Production Ruchy Compiler**: https://github.com/paiml/ruchy

This requires changes to the production Ruchy compiler, NOT RuchyRuchy. RuchyRuchy's role is:
- Define requirements via RED tests
- Track implementation status
- Validate production Ruchy implementation
- Document via book chapters

## Compiler Flag Design

### New Compilation Flag

```bash
ruchy compile --instrument <file.ruchy> --output <binary>
```

**Flag Behavior**:
- Inserts profiling instrumentation at AST/IR level
- Generates binary with embedded profiling hooks
- Profiling enabled/disabled via environment variables at runtime
- Zero overhead when `RUCHY_PROFILE=0` or unset

### Runtime Environment Variables

```bash
# Enable profiling
RUCHY_PROFILE=1 ./binary

# Specify output file
RUCHY_PROFILE=1 RUCHY_PROFILE_OUTPUT=/tmp/profile.json ./binary

# Disable profiling (default)
RUCHY_PROFILE=0 ./binary  # or just ./binary
```

## Instrumentation Points

### 1. Function Entry/Exit Timing

**Where**: Insert hooks at every function entry and exit point

**Pseudocode**:
```rust
// Original Ruchy code
fun fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// Transpiled Rust with instrumentation
fn fibonacci(n: i64) -> i64 {
    let _profiler_guard = if PROFILER_ENABLED.load(Ordering::Relaxed) {
        Some(ProfilerGuard::enter("fibonacci"))
    } else {
        None
    };

    // Original function body
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);

    // ProfilerGuard::drop() records exit time
}
```

**Data Collected**:
- Function name
- Call count
- Total time (nanoseconds)
- Min/max/avg time per call

**Implementation Strategy**:
- Use RAII guard pattern for automatic entry/exit tracking
- Thread-local storage for per-thread profiling data
- Atomic flag for zero-cost disable check

### 2. Loop Iteration Counting

**Where**: Insert counter at loop entry point

**Pseudocode**:
```rust
// Original Ruchy code
for i in 0..1000000 {
    sum = sum + i;
}

// Transpiled Rust with instrumentation
{
    let mut _loop_counter = if PROFILER_ENABLED.load(Ordering::Relaxed) {
        Some(LoopCounter::new("main:5"))
    } else {
        None
    };

    for i in 0..1000000 {
        if let Some(ref mut counter) = _loop_counter {
            counter.increment();
        }
        sum = sum + i;
    }

    // LoopCounter::drop() records total iterations
}
```

**Data Collected**:
- Loop location (file:line)
- Total iterations
- Average iteration time
- Total loop time

**Implementation Strategy**:
- Track loop location via source map
- Increment counter on each iteration
- Minimal overhead check (single atomic load per iteration)

### 3. Branch Statistics

**Where**: Insert tracking at each conditional branch

**Pseudocode**:
```rust
// Original Ruchy code
if n % i == 0 {
    return false;
}

// Transpiled Rust with instrumentation
{
    let condition = n % i == 0;
    if PROFILER_ENABLED.load(Ordering::Relaxed) {
        BRANCH_STATS.record("is_prime:7", condition);
    }
    if condition {
        return false;
    }
}
```

**Data Collected**:
- Branch location (file:line)
- Taken count
- Not-taken count
- Prediction rate (taken / total)

**Implementation Strategy**:
- Evaluate condition once, store in variable
- Record branch outcome
- Calculate prediction rate during JSON export

### 4. Memory Allocation Tracking

**Where**: Wrap allocation calls

**Pseudocode**:
```rust
// Original Ruchy code
let mut v: Vec<i64> = vec![];

// Transpiled Rust with instrumentation
let mut v: Vec<i64> = if PROFILER_ENABLED.load(Ordering::Relaxed) {
    let v = vec![];
    ALLOC_TRACKER.record_allocation(std::mem::size_of::<Vec<i64>>());
    v
} else {
    vec![]
};
```

**Data Collected**:
- Total allocations
- Total bytes allocated
- Peak memory usage
- Allocation size distribution (small/medium/large)

**Size Categories**:
- Small: <256 bytes
- Medium: 256 bytes - 4KB
- Large: >4KB

**Implementation Strategy**:
- Hook into allocator (custom allocator wrapper)
- Track allocation sizes
- Calculate peak memory via running total

## Runtime Profiling Infrastructure

### Global Profiler State

```rust
// Profiler state (thread-local)
thread_local! {
    static PROFILER_DATA: RefCell<ProfilerData> = RefCell::new(ProfilerData::new());
}

// Global enable flag (atomic for zero-cost check)
static PROFILER_ENABLED: AtomicBool = AtomicBool::new(false);

struct ProfilerData {
    functions: HashMap<String, FunctionStats>,
    loops: HashMap<String, LoopStats>,
    branches: HashMap<String, BranchStats>,
    allocations: AllocationStats,
}

struct FunctionStats {
    calls: u64,
    total_time_ns: u64,
    min_time_ns: u64,
    max_time_ns: u64,
}

struct LoopStats {
    iterations: u64,
    total_time_ns: u64,
}

struct BranchStats {
    taken: u64,
    not_taken: u64,
}

struct AllocationStats {
    total_allocs: u64,
    total_bytes: u64,
    peak_memory_bytes: u64,
    small_count: u64,
    small_bytes: u64,
    medium_count: u64,
    medium_bytes: u64,
    large_count: u64,
    large_bytes: u64,
}
```

### Initialization

```rust
// Called at program startup (before main)
#[ctor::ctor]
fn init_profiler() {
    // Check environment variable
    if std::env::var("RUCHY_PROFILE").unwrap_or_default() == "1" {
        PROFILER_ENABLED.store(true, Ordering::Relaxed);
    }
}

// Called at program exit (after main)
#[ctor::dtor]
fn finalize_profiler() {
    if PROFILER_ENABLED.load(Ordering::Relaxed) {
        export_profile_data();
    }
}
```

### JSON Export

```rust
fn export_profile_data() {
    let output_path = std::env::var("RUCHY_PROFILE_OUTPUT")
        .unwrap_or_else(|_| "profile.json".to_string());

    // Collect data from all threads
    let data = PROFILER_DATA.with(|data| data.borrow().clone());

    // Serialize to JSON
    let json = serde_json::json!({
        "version": "1.0",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "binary": std::env::current_exe().ok(),
        "functions": serialize_functions(&data.functions),
        "loops": serialize_loops(&data.loops),
        "branches": serialize_branches(&data.branches),
        "allocations": serialize_allocations(&data.allocations),
        "statistics": {
            "total_runtime_ns": calculate_total_runtime(&data),
            "instrumentation_overhead_percent": estimate_overhead(&data),
        }
    });

    // Write to file
    std::fs::write(output_path, serde_json::to_string_pretty(&json).unwrap())
        .expect("Failed to write profile data");
}
```

## JSON Output Schema

### Complete Schema

```json
{
  "version": "1.0",
  "timestamp": "2025-11-09T12:34:56Z",
  "binary": "/tmp/test_program",
  "functions": [
    {
      "name": "fibonacci",
      "calls": 21891,
      "total_time_ns": 1234567,
      "avg_time_ns": 56.4,
      "min_time_ns": 10,
      "max_time_ns": 500
    }
  ],
  "loops": [
    {
      "location": "main:5",
      "iterations": 1000000,
      "avg_iter_time_ns": 12.5,
      "total_time_ns": 12500000
    }
  ],
  "branches": [
    {
      "location": "is_prime:7",
      "taken": 78498,
      "not_taken": 21502,
      "prediction_rate": 0.78498
    }
  ],
  "allocations": {
    "total_allocs": 10000,
    "total_bytes": 4000000,
    "peak_memory_bytes": 800000,
    "by_size": {
      "small": {
        "count": 9000,
        "bytes": 36000
      },
      "medium": {
        "count": 900,
        "bytes": 360000
      },
      "large": {
        "count": 100,
        "bytes": 3604000
      }
    }
  },
  "statistics": {
    "total_runtime_ns": 123456789,
    "instrumentation_overhead_percent": 0.5
  }
}
```

## Performance Requirements

### Overhead Target: <1%

**Measured By**:
- N≥30 runs (Georges et al. 2007)
- Welch's t-test, p < 0.05
- Coefficient of variation <5%

**Measurement Strategy**:
```bash
# Baseline (no instrumentation)
ruchy compile test.ruchy --output baseline
time ./baseline  # Run 30 times

# Instrumented (profiling disabled)
ruchy compile --instrument test.ruchy --output instrumented
RUCHY_PROFILE=0 time ./instrumented  # Run 30 times

# Compare: (instrumented_mean - baseline_mean) / baseline_mean < 0.01
```

**Optimization Strategies**:
1. **Atomic flag check**: Single `load(Ordering::Relaxed)` per hook
2. **Inlining**: Mark hook functions `#[inline(always)]`
3. **Branch prediction**: Compiler hints for disabled path
4. **Thread-local storage**: No lock contention
5. **Lazy initialization**: Only allocate when enabled

### Expected Overhead Breakdown

| Hook Type | Per-Call Overhead | Frequency | Total Impact |
|-----------|------------------|-----------|--------------|
| Function entry/exit | 2ns | High | 0.4% |
| Loop iteration | 1ns | Very High | 0.3% |
| Branch tracking | 1ns | High | 0.2% |
| Allocation tracking | 5ns | Low | 0.1% |
| **Total** | - | - | **1.0%** |

## Implementation Roadmap

### Phase 1: Core Infrastructure (Week 1)

**Tasks**:
1. Add `--instrument` flag to `ruchy compile`
2. Implement `ProfilerGuard` RAII type
3. Implement thread-local `ProfilerData`
4. Implement atomic enable/disable flag
5. Basic JSON export

**Deliverable**: Function entry/exit timing working

### Phase 2: Loop and Branch Tracking (Week 2)

**Tasks**:
1. Implement `LoopCounter` type
2. Add loop instrumentation to codegen
3. Implement branch tracking
4. Add branch stats to JSON export

**Deliverable**: Loop and branch data in JSON

### Phase 3: Allocation Tracking (Week 3)

**Tasks**:
1. Implement custom allocator wrapper
2. Track allocation sizes
3. Calculate peak memory
4. Add allocation stats to JSON export

**Deliverable**: Complete allocation tracking

### Phase 4: Optimization and Validation (Week 4)

**Tasks**:
1. Optimize overhead to <1%
2. Run N≥30 statistical validation
3. Add CI/CD tests
4. Documentation

**Deliverable**: <1% overhead validated (p < 0.05)

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_profiler_guard_basic() {
    PROFILER_ENABLED.store(true, Ordering::Relaxed);

    {
        let _guard = ProfilerGuard::enter("test_function");
        // Simulate work
        std::thread::sleep(Duration::from_millis(10));
    }

    // Verify data recorded
    PROFILER_DATA.with(|data| {
        let stats = data.borrow().functions.get("test_function").unwrap();
        assert_eq!(stats.calls, 1);
        assert!(stats.total_time_ns >= 10_000_000); // ≥10ms
    });
}

#[test]
fn test_zero_overhead_when_disabled() {
    PROFILER_ENABLED.store(false, Ordering::Relaxed);

    // This should compile to a no-op
    let _guard = ProfilerGuard::enter("test_function");

    // Verify no data recorded
    PROFILER_DATA.with(|data| {
        assert!(data.borrow().functions.is_empty());
    });
}
```

### Integration Tests

Use the 6 tests from `tests/test_compiled_inst_001_ast_hooks.rs`:
1. test_function_timing_instrumentation
2. test_loop_iteration_counting
3. test_branch_statistics
4. test_memory_allocation_tracking
5. test_instrumentation_overhead
6. test_json_output_format

## Dependencies

**Rust Crates**:
- `serde` + `serde_json`: JSON serialization
- `chrono`: Timestamps
- `ctor`: Constructor/destructor attributes
- `parking_lot`: Fast thread-local storage (optional)

**Ruchy Compiler**:
- Modify codegen to insert instrumentation
- Add source map integration for locations
- Add CLI flag parsing

## Success Criteria

- ✅ `ruchy compile --instrument` flag implemented
- ✅ Function timing: calls, total/min/max/avg time
- ✅ Loop iteration counting with location
- ✅ Branch statistics (taken/not-taken)
- ✅ Memory allocation tracking by size
- ✅ JSON output with complete schema
- ✅ <1% overhead when profiling disabled (p < 0.05, N≥30)
- ✅ All 6 integration tests passing
- ✅ Statistical validation (Welch's t-test, CV <5%)

## Open Questions

1. **Multi-threading**: How to aggregate data across threads?
   - **Answer**: Collect per-thread, aggregate on exit

2. **Async/await**: How to handle async function profiling?
   - **Answer**: Track logical function time (include await time)

3. **Inline functions**: Should we profile inlined functions?
   - **Answer**: Yes, but mark as inlined in JSON

4. **Dynamic dispatch**: How to track trait method calls?
   - **Answer**: Use mangled name including trait

## References

- Georges et al. (2007): Statistical rigor (N≥30, p<0.05)
- Julia (SIAM 2017): Type specialization for low overhead
- DEBUGGER-016: Hardware profiling integration (Phase 2)
- ruchy-docker: Benchmarking patterns for validation

---

**Status**: Ready for implementation in production Ruchy compiler
**Next Step**: File feature request at https://github.com/paiml/ruchy/issues
**Blocking**: Awaiting production Ruchy compiler changes
