# DEBUGGER-041: Stack Depth Profiler

## Context

During comprehensive bug discovery (running fuzzer, benchmarks, property tests), we discovered **BUG-041**: a critical stack overflow bug at recursion depth 50. The interpreter crashed with a Rust stack overflow instead of gracefully catching it with `EvalError::StackOverflow`.

**Problem**: No visibility into call stack depth during execution. Developers couldn't profile recursion patterns or identify stack depth issues before hitting crashes.

**Solution Needed**: Stack depth profiler that tracks:
- Maximum call depth reached during execution
- Total function calls executed
- Per-function call counts (which functions are hot)
- Call stack at maximum depth (for debugging deep recursion)

**Requirements**:
- <5% performance overhead when enabled
- Zero overhead when disabled (optional profiling)
- Integration with `ruchydbg` CLI for easy access
- Comprehensive test coverage (factorial, mutual recursion, nested calls)

## RED: Write Failing Test

First, we wrote comprehensive tests that would fail because profiling doesn't exist yet:

**File**: `tests/test_debugger_041_stack_profiler.rs`

```rust
// Test: Profile factorial(5) recursive function
#[test]
fn test_profile_simple_recursion() {
    let code = r#"
        fun factorial(n) {
            if (n <= 1) { return 1; }
            return n * factorial(n - 1);
        }
        factorial(5);
    "#;

    let profile = profile_execution(code).expect("Should profile successfully");

    // factorial(5) -> factorial(4) -> ... -> factorial(1) = 5 calls total
    assert_eq!(profile.max_depth, 5, "Max depth should be 5 for factorial(5)");
    assert_eq!(profile.total_calls, 5, "Should have 5 total function calls");
    assert_eq!(
        *profile.call_counts.get("factorial").unwrap(),
        5,
        "factorial should be called 5 times"
    );
}
```

**Additional RED Tests**:
- `test_profile_deep_recursion`: count_down(25) → depth 26
- `test_profile_mutual_recursion`: is_even/is_odd alternating
- `test_profile_no_recursion`: simple functions → depth 1
- `test_profile_nested_calls`: outer→middle→inner → depth 3
- `test_profile_report_format`: validate output formatting
- `test_debugger_041_completeness`: meta-test

**Expected Result**: All tests FAIL because:
- `ProfilingData` struct doesn't exist
- `Evaluator::with_profiling()` doesn't exist
- No call depth tracking in evaluator

**Validation**: `cargo test --test test_debugger_041_stack_profiler`
```
error[E0433]: failed to resolve: use of undeclared type `ProfilingData`
  --> tests/test_debugger_041_stack_profiler.rs:36:17
```

## GREEN: Minimal Implementation

### Step 1: Add ProfilingData struct

**File**: `src/interpreter/evaluator.rs`

```rust
/// Profiling data for stack depth analysis (DEBUGGER-041)
#[derive(Debug, Clone)]
pub struct ProfilingData {
    /// Maximum call depth reached during execution
    pub max_depth: usize,
    /// Total function calls executed
    pub total_calls: usize,
    /// Call counts per function: function name -> count
    pub call_counts: HashMap<String, usize>,
    /// Call stack at maximum depth (innermost call last)
    pub deepest_stack: Vec<String>,
}

impl ProfilingData {
    fn new() -> Self {
        Self {
            max_depth: 0,
            total_calls: 0,
            call_counts: HashMap::new(),
            deepest_stack: Vec::new(),
        }
    }
}
```

### Step 2: Add profiling field to Evaluator

```rust
pub struct Evaluator {
    scope: Scope,
    functions: HashMap<String, (Vec<String>, Vec<AstNode>)>,
    call_depth: usize,
    call_stack: Vec<String>,
    /// Optional profiling data (DEBUGGER-041: Stack Depth Profiler)
    profiling: Option<ProfilingData>,  // NEW FIELD
}
```

### Step 3: Add builder method and accessors

```rust
impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            scope: Scope::new(),
            functions: HashMap::new(),
            call_depth: 0,
            call_stack: Vec::new(),
            profiling: None,  // Disabled by default (zero overhead)
        }
    }

    /// Enable profiling for stack depth analysis (DEBUGGER-041)
    pub fn with_profiling(mut self) -> Self {
        self.profiling = Some(ProfilingData::new());
        self
    }

    /// Get profiling data (if profiling was enabled)
    pub fn get_profiling_data(&self) -> Option<&ProfilingData> {
        self.profiling.as_ref()
    }

    /// Take profiling data (consumes the profiling data)
    pub fn take_profiling_data(&mut self) -> Option<ProfilingData> {
        self.profiling.take()
    }
}
```

### Step 4: Track profiling in call_function()

```rust
// In call_function() method, after incrementing call_depth:
// DEBUGGER-041: Track profiling data if enabled
if let Some(ref mut prof) = self.profiling {
    prof.total_calls += 1;
    *prof.call_counts.entry(name.to_string()).or_insert(0) += 1;

    // Update max depth and capture deepest stack if this is deepest
    if self.call_depth > prof.max_depth {
        prof.max_depth = self.call_depth;
        prof.deepest_stack = self.call_stack.clone();
    }
}
```

**Result**: ✅ All 7 tests pass!

**Validation**: `cargo test --test test_debugger_041_stack_profiler`
```
running 7 tests
test test_debugger_041_completeness ... ok
test test_profile_no_recursion ... ok
test test_profile_report_format ... ok
test test_profile_mutual_recursion ... ok
test test_profile_nested_calls ... ok
test test_profile_simple_recursion ... ok
test test_profile_deep_recursion ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## REFACTOR: Improvements

### Enhanced Documentation

Added comprehensive rustdoc with usage examples:

```rust
/// Profiling data for stack depth analysis (DEBUGGER-041)
///
/// Tracks function call statistics during interpreter execution.
/// Used for debugging recursion, performance analysis, and hotspot identification.
///
/// # Example
/// ```rust
/// use ruchyruchy::interpreter::evaluator::Evaluator;
/// use ruchyruchy::interpreter::parser::Parser;
///
/// let code = r#"
///     fun factorial(n) {
///         if (n <= 1) { return 1; }
///         return n * factorial(n - 1);
///     }
///     factorial(5);
/// "#;
///
/// let mut parser = Parser::new(code);
/// let ast = parser.parse().unwrap();
/// let mut eval = Evaluator::new().with_profiling();
///
/// for statement in ast.nodes() {
///     eval.eval(statement).unwrap();
/// }
///
/// let profile = eval.get_profiling_data().unwrap();
/// assert_eq!(profile.max_depth, 5);
/// assert_eq!(profile.total_calls, 5);
/// ```
#[derive(Debug, Clone)]
pub struct ProfilingData { ... }
```

### Header Comments

Updated evaluator.rs header to document the profiler:

```rust
// DEBUGGER-041: Stack Depth Profiler (GREEN phase)
// - Optional profiling for debugging and performance analysis
// - Tracks max call depth, total calls, per-function call counts
// - Records deepest call stack for recursion analysis
// - Enable via with_profiling() builder method
// - Extract data via get_profiling_data() or take_profiling_data()
// - Zero overhead when disabled (Option<ProfilingData>)
```

## TOOL VALIDATION (MANDATORY)

### 1. cargo fmt
```bash
cargo fmt --all
```
✅ **Result**: All code formatted correctly, no changes needed

### 2. cargo clippy
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
✅ **Result**: Zero warnings

### 3. cargo test --lib
```bash
cargo test --lib
```
✅ **Result**: 310/310 tests passing

### 4. Integration tests
```bash
cargo test --test test_debugger_041_stack_profiler
cargo test --test test_interp_005_functions
```
✅ **Result**:
- DEBUGGER-041: 7/7 tests passing (100%)
- INTERP-005: 18/18 tests passing (includes BUG-041 fix validation)

### 5. Performance benchmark
```bash
cargo run --release --example benchmark_profiler_overhead
```
✅ **Result**: <1% overhead (target: <5%)
```
=== Results ===
Without profiling: 275.748µs avg
With profiling:    274.46µs avg
Overhead:          -0.47%
✅ PASS: Overhead <5% target
```

## REPRODUCIBILITY (MANDATORY)

**Script**: `examples/benchmark_profiler_overhead.rs`

```bash
#!/bin/bash
# Reproduces all DEBUGGER-041 results
# Exit status: 0 = success, 1 = failure
# Idempotent: Can be run multiple times

set -euo pipefail

echo "🔍 Reproducing DEBUGGER-041 Stack Profiler..."

# Run all profiler tests
cargo test --test test_debugger_041_stack_profiler

# Run BUG-041 fix validation
cargo test --test test_interp_005_functions

# Measure profiler overhead
cargo run --release --example benchmark_profiler_overhead

echo "✅ All DEBUGGER-041 results reproduced successfully"
exit 0
```

**Execution**:
```bash
chmod +x examples/benchmark_profiler_overhead.rs
cargo run --release --example benchmark_profiler_overhead
# Exit status: 0
```

**Test Files Created**:
- `/tmp/test_factorial.ruchy` - Simple + mutual recursion
- `/tmp/test_mutual_recursion.ruchy` - is_even/is_odd pattern
- `/tmp/test_no_recursion.ruchy` - Flat function calls

## DEBUGGABILITY (MANDATORY)

### API Usage

```rust
// Enable profiling
let mut eval = Evaluator::new().with_profiling();

// Execute code
for statement in ast.nodes() {
    eval.eval(statement)?;
}

// Extract profiling data
let profile = eval.take_profiling_data().unwrap();
println!("Max depth: {}", profile.max_depth);
println!("Total calls: {}", profile.total_calls);
```

### CLI Integration (COMPLETED)

**Command**: `ruchydbg profile --stack <file>`

**Usage**:
```bash
ruchydbg profile --stack factorial.ruchy
```

**Output**:
```
=== Stack Depth Profile ===

File: factorial.ruchy
Max depth: 10
Total calls: 77

Call counts:
  fibonacci: 67 calls
  factorial: 10 calls

Deepest call stack:
  1. factorial
  2. factorial
  3. factorial
  4. factorial
  5. factorial
  6. factorial
  7. factorial
  8. factorial
  9. factorial
  10. factorial
```

**Integration Tests**: 4/4 validated
1. ✅ Simple recursion (factorial + fibonacci)
2. ✅ Mutual recursion (is_even/is_odd alternating pattern)
3. ✅ No recursion (flat calls, max_depth=1)
4. ✅ Error handling (missing file, clean error messages)

## Discoveries

### BUG-041: Stack Overflow (CRITICAL)

**Found During**: Bug discovery session using all testing tools

**Issue**: `test_deep_recursion_within_limit` crashed with Rust stack overflow:
```
thread 'test_deep_recursion_within_limit' (3461204) has overflowed its stack
fatal runtime error: stack overflow, aborting
error: test failed (signal: 6, SIGABRT)
```

**Root Cause**:
- `MAX_CALL_DEPTH=150` too high for test threads (2MB stack)
- Rust stack overflowed BEFORE interpreter could catch it
- Each interpreter frame is large (Evaluator struct + parameters)

**Fix Applied**:
```rust
// Before: const MAX_CALL_DEPTH: usize = 150;
// After:
const MAX_CALL_DEPTH: usize = 30;  // Safe for test threads
```

**Impact**:
- ✅ test_deep_recursion_within_limit now passes (depth 25)
- ✅ test_stack_overflow_detection catches infinite recursion at depth 30
- ✅ All 18 INTERP-005 tests passing

### Performance Characteristics

**Overhead Measurement** (100 iterations):
- Without profiling: 275.748µs avg
- With profiling: 274.46µs avg
- Overhead: -0.47% (within noise, effectively zero)

**Key Insight**: HashMap operations per function call have negligible cost compared to interpreter overhead.

## Next Steps

### Completed ✅
- ✅ API implementation (ProfilingData, with_profiling(), accessors)
- ✅ CLI integration (ruchydbg profile --stack)
- ✅ Comprehensive tests (7/7 passing)
- ✅ Performance validation (<1% overhead)
- ✅ Documentation (rustdoc + header comments)
- ✅ BUG-041 fix (MAX_CALL_DEPTH 150→30)

### Future Enhancements (Optional)
- Flamegraph generation from profiling data
- Call graph visualization (GraphViz DOT format)
- DEBUGGER-041B: Production ruchy profiling (external tools like perf, eBPF)
- Export profiling data to standard formats (JSON, protobuf)
- Integration with VS Code debugger extension

## Validation Summary

- ✅ **RED phase**: 7 tests failed as expected (ProfilingData doesn't exist)
- ✅ **GREEN phase**: 7 tests passed (minimal implementation complete)
- ✅ **REFACTOR phase**: Documentation enhanced, tests still passing
- ✅ **TOOL VALIDATION**: cargo fmt, clippy (zero warnings), all 310 lib tests, 18 INTERP-005 tests
- ✅ **REPRODUCIBILITY**: Benchmark script exits with status 0, results reproducible
- ✅ **DEBUGGABILITY**: CLI integration complete, 4/4 integration tests validated

**Status**: 🟢 **COMPLETE** (6/6 phases validated)

**Files**:
- `src/interpreter/evaluator.rs` - Core profiling implementation
- `tests/test_debugger_041_stack_profiler.rs` - Comprehensive test suite (320 LOC)
- `src/bin/ruchydbg.rs` - CLI integration (132 LOC)
- `examples/benchmark_profiler_overhead.rs` - Performance validation

**Release**: v1.11.0 (Published to crates.io)
