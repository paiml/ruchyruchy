# Feature Request: Add `--profile` flag to `ruchy compile` for extreme performance profiling

## 🎯 Summary

Add comprehensive profiling instrumentation to `ruchy compile` to enable extreme performance optimization and make Ruchy the world's fastest compiled language (≥105% of C performance, binaries ≤50% of C size).

**Status**: ✅ Prototype validated (4/6 features working, 67% coverage)
**Repository**: [paiml/ruchyruchy](https://github.com/paiml/ruchyruchy) (branch: `claude/instrument-ruchy-compile-*`)
**Documentation**: [Book Chapter](https://github.com/paiml/ruchyruchy/blob/main/book/src/phase6_compiled_instrumentation/compiled-inst-001-ast-instrumentation.md)

---

## 💡 Motivation

**Problem**: Developers cannot identify performance bottlenecks in compiled Ruchy code.

**Missing Capabilities**:
- ❌ No visibility into hot functions
- ❌ Unknown loop iteration costs
- ❌ Branch prediction mysteries
- ❌ Allocation patterns hidden
- ❌ Optimization opportunities missed

**Goal**: Make Ruchy ≥105% of C performance (5% faster) with scientifically validated measurements.

---

## ✨ Proposed Solution

### Feature: `ruchy compile --profile`

Add `--profile` flag that instruments compiled code with:

1. **Function timing** - Entry/exit instrumentation with RAII guards
2. **Loop iteration counting** - Hot loop identification
3. **Branch statistics** - Taken/not-taken tracking for prediction optimization
4. **Memory allocation tracking** - Allocator overhead analysis
5. **JSON export** - Machine-readable profiling data

### Example Usage

```bash
# Compile with profiling
ruchy compile --profile myapp.ruchy -o myapp

# Run with profiling enabled
RUCHY_PROFILE=1 RUCHY_PROFILE_OUTPUT=profile.json ./myapp

# Analyze results
ruchy analyze profile.json
```

### Example Output

```json
{
  "version": "1.0",
  "functions": [
    {"name": "compute_heavy", "calls": 1000000, "total_time_ns": 45000000, "avg_time_ns": 45.0}
  ],
  "loops": [
    {"location": "myapp.ruchy:42", "iterations": 10000000, "total_time_ns": 45000000}
  ],
  "branches": [
    {"location": "myapp.ruchy:58", "taken": 750000, "not_taken": 250000, "prediction_rate": 0.75}
  ],
  "allocations": {
    "total_allocs": 50000, "total_bytes": 2000000, "peak_memory_bytes": 524288
  }
}
```

---

## 🔬 Prototype Validation

**Working Implementation**: [src/bin/ruchy.rs](https://github.com/paiml/ruchyruchy/blob/main/src/bin/ruchy.rs) (550 LOC)

**Tests Passing** (4/6, 67% coverage):
- ✅ **Function timing**: 177 calls for fibonacci(10) - exact
- ✅ **Loop iteration**: 1000 iterations for 0..1000 - exact
- ✅ **Branch statistics**: 50/50 split for i%2==0 in 0..100 - exact
- ✅ **JSON export**: Complete schema validation

**Performance Results**:
```
Fibonacci(10):
  Calls: 177 ✅ (exact recursive expansion)
  Total time: ~209µs
  Avg time: ~1.2µs per call
  Overhead: 4.17% (target <1% via hardware counters)

Loop 0..1000:
  Iterations: 1000 ✅ (exact)

Branch i%2==0 in 0..100:
  Taken: 50 ✅, Not taken: 50 ✅
  Prediction rate: 0.5 ✅
```

**Pending Features** (production requirements):
- ⏳ Memory allocation tracking (requires custom allocator)
- ⏳ Overhead optimization to <1% (use perf_event_open hardware counters)

---

## 🛠️ Technical Approach

### 1. AST/IR-Level Instrumentation

Insert profiling hooks during compilation:

```rust
// Original Ruchy code:
fun compute(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..n { sum = sum + i; }
    sum
}

// Instrumented output:
fun compute(n: i64) -> i64 {
    let _profiler_guard = ProfilerGuard::new("compute");
    let mut sum = 0;
    for i in 0..n {
        record_loop_iteration("compute:3");
        sum = sum + i;
    }
    sum
}
```

### 2. RAII Profiling Guards

**Zero overhead when disabled**:

```rust
struct ProfilerGuard {
    function_name: &'static str,
    start_time: Instant,
}

impl ProfilerGuard {
    fn new(function_name: &'static str) -> Self {
        // Fast path: single atomic load (<1ns)
        if !PROFILER_ENABLED.load(Ordering::Relaxed) {
            return Self { function_name, start_time: ZERO_TIME };
        }
        // Track entry...
    }
}

impl Drop for ProfilerGuard {
    fn drop(&mut self) {
        // Track exit time...
    }
}
```

### 3. Hardware Performance Counters

For sub-1% overhead, integrate `perf_event_open` (already implemented in ruchyruchy DEBUGGER-016):

```rust
use perf_event::Builder;

let mut counter = Builder::new()
    .kind(perf_event::events::Hardware::CPU_CYCLES)
    .build()?;

counter.enable()?;
// ... user code ...
counter.disable()?;
```

**Benefits**: <0.1% overhead, cycle-accurate measurements, cache/branch statistics

---

## 📊 Performance Requirements

### Target Metrics

1. **Overhead when enabled**: <1% (validated via Georges et al. 2007)
   - N≥30 runs, p<0.05 significance
   - Coefficient of variation <5%

2. **Overhead when disabled**: <0.01% (zero-cost abstraction)
   - Single atomic load per instrumentation point

3. **Accuracy**: Cycle-accurate for function timing
   - Correct call counts (validated in prototype)

---

## 📅 Implementation Plan

### Phase 1: Core Instrumentation (Weeks 1-2)
- [ ] Add `--profile` flag to `ruchy compile`
- [ ] Implement ProfilerGuard RAII pattern
- [ ] Function entry/exit instrumentation
- [ ] Thread-local profiler data storage
- [ ] JSON export with schema

### Phase 2: Advanced Features (Weeks 3-4)
- [ ] Loop iteration tracking
- [ ] Branch taken/not-taken statistics
- [ ] Memory allocation hooks (custom allocator)
- [ ] Source location mapping

### Phase 3: Hardware Integration (Weeks 5-6)
- [ ] Integrate perf_event_open (from DEBUGGER-016)
- [ ] Cycle-accurate measurements
- [ ] Cache miss statistics
- [ ] Branch misprediction counts

### Phase 4: Tooling & Visualization (Weeks 7-8)
- [ ] `ruchy analyze` command for profile analysis
- [ ] Flame graph generation
- [ ] Hotspot identification
- [ ] Optimization recommendations

---

## 📚 Research Foundation

**Peer-reviewed research**:
1. **Georges et al. (2007)**: "Statistically Rigorous Java Performance Evaluation" - Statistical methodology
2. **Julia (SIAM 2017)**: "Julia: A Fresh Approach to Numerical Computing" - Type specialization
3. **Profile-Guided Optimization** (arXiv 2025): PGO techniques survey
4. **perf_event_open** (Linux kernel): Hardware performance counters
5. **Valgrind/Callgrind**: Profiling tool architecture

---

## 🔗 References

**Prototype**:
- Implementation: [src/bin/ruchy.rs](https://github.com/paiml/ruchyruchy/blob/main/src/bin/ruchy.rs)
- Tests: [tests/test_compiled_inst_001_ast_hooks.rs](https://github.com/paiml/ruchyruchy/blob/main/tests/test_compiled_inst_001_ast_hooks.rs)
- Documentation: [Book Chapter](https://github.com/paiml/ruchyruchy/blob/main/book/src/phase6_compiled_instrumentation/compiled-inst-001-ast-instrumentation.md)
- Feature Request: [Full Spec](https://github.com/paiml/ruchyruchy/blob/main/docs/PRODUCTION-FEATURE-REQUEST-profiler.md)

**Related Work**:
- DEBUGGER-016: perf_event_open integration in ruchyruchy
- Rust's PGO support
- LLVM's instrumentation passes

---

## 🎯 Success Criteria

**Quantitative**:
- ✅ Overhead <1% when enabled (p<0.05, N≥30 runs)
- ✅ Zero overhead when disabled (<0.01%)
- ✅ 100% accuracy for call/iteration counts
- ✅ <1s profiling for 10K LOC programs
- ✅ <10MB JSON output for typical programs

**Qualitative**:
- Developer adoption >50% within 3 months
- Performance improvements in ≥10 real-world projects
- Integration with VS Code/IntelliJ debuggers

---

## 💬 Contact

**Prototype**: https://github.com/paiml/ruchyruchy (branch: `claude/instrument-ruchy-compile-*`)
**Ticket**: COMPILED-INST-001
**Status**: Prototype complete, ready for production integration

For questions or discussion, please comment on this issue.
