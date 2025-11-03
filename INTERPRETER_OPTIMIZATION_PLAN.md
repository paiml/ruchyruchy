# RuchyRuchy Interpreter Optimization Plan

**Date**: 2025-11-03
**Context**: Pivoting from PERF-001A (blocked by Ruchy Bug #128) to interpreter optimization
**Current Performance**: 34.71ms (Hello World startup)
**Target**: <10ms (approaching Bytecode VM at 7.88ms)

---

## 📊 Current State

### Performance Ladder (from PERFORMANCE_INSIGHTS_JULIA_JIT.md)

```
Julia JIT:           2.03ms  ← Aspirational (requires JIT)
Ruchy Compiled:      2.64ms
Go (AOT):            2.78ms
C (AOT):             3.02ms
Rust (AOT):          3.04ms
Ruchy Transpiled:    3.21ms
Ruchy Bytecode VM:   7.88ms  ← Target to approach
Python (CPython):   16.69ms  ← Baseline to beat (2.08x faster)
Deno (V8 JIT):      26.77ms
RuchyRuchy AST:     34.71ms  ← Current (17x slower than Julia)
```

**Current Position**: 34.71ms
**Improvement Opportunity**: 4.4x speedup to match Bytecode VM (7.88ms)
**Stretch Goal**: 2.08x speedup to beat Python (16.69ms → 34.71ms)

### Existing Benchmark Results (INTERP-030)

From `cargo test --test test_interp_030_benchmarking --release`:
- **Fibonacci ops**: 2.74 µs/op (8.84x overhead vs native)
- **Vector ops**: 2.71 µs/op (10.68x overhead)
- **HashMap ops**: 2.38 µs/op (9.38x overhead)
- **Throughput**: ~5M ops/sec

**Key Insight**: Micro-operations are fast (~2-3 µs), but full program startup is slow (34.71ms).
**Hypothesis**: Startup overhead dominates (parsing, AST construction, initial evaluation setup).

---

## 🎯 Optimization Strategy

### Phase 1: Measure & Profile (INTERP-OPT-001)

**Objective**: Establish detailed baseline and identify bottlenecks

**Tasks**:
1. **Measure "Hello World" startup breakdown**:
   - Parse time
   - AST construction time
   - Evaluator setup time
   - Execution time
   - Shutdown time

2. **Profile using cargo flamegraph**:
   ```bash
   cargo flamegraph --test test_interp_030_benchmarking
   ```

3. **Measure memory allocations**:
   ```bash
   cargo build --release
   valgrind --tool=massif ./target/release/ruchyruchy
   ```

4. **Identify hot functions**:
   - Use `perf record` + `perf report`
   - Focus on functions >5% of total time

**Acceptance Criteria**:
- [ ] Hello World breakdown documented (ms per phase)
- [ ] Flamegraph generated and analyzed
- [ ] Top 10 hot functions identified
- [ ] Memory allocation hotspots identified

**Expected Findings** (hypothesis):
- Parser: ~40% of time (string operations, allocations)
- Evaluator: ~30% of time (hash map lookups, clones)
- Allocations: ~20% of time (Vec, HashMap, String clones)
- Misc: ~10% of time

---

### Phase 2: Low-Hanging Fruit Optimizations (INTERP-OPT-002)

**Objective**: Quick wins with minimal risk

**Candidate Optimizations**:

1. **String Interning** (if strings are cloned frequently):
   - Use `Arc<str>` for identifiers
   - Reuse common strings ("main", "let", "if", etc.)
   - Expected: 10-15% speedup

2. **Pre-allocate Collections**:
   - Parser: Pre-allocate Vec with capacity
   - Evaluator: Pre-allocate scope HashMap
   - Expected: 5-10% speedup

3. **Avoid Unnecessary Clones**:
   - Use references where possible
   - Use `Rc`/`Arc` for shared data
   - Expected: 10-20% speedup

4. **Inline Hot Functions**:
   - Add `#[inline(always)]` to small, hot functions
   - Focus on getters, simple operations
   - Expected: 5-10% speedup

5. **Optimize Scope Lookup**:
   - Use FxHashMap instead of std HashMap (faster for small keys)
   - Cache last lookup (simple 1-entry cache)
   - Expected: 5-10% speedup

**Total Expected**: 35-65% speedup (34.71ms → 12-23ms)

---

### Phase 3: Structural Optimizations (INTERP-OPT-003)

**Objective**: Deeper changes for significant speedup

**Candidate Optimizations**:

1. **AST Simplification**:
   - Flatten nested structures
   - Use indices instead of Box pointers
   - Expected: 10-15% speedup

2. **Value Representation**:
   - Use NaN-boxing or tagged unions
   - Avoid heap allocation for small values
   - Expected: 15-20% speedup

3. **Stack-Based Evaluator**:
   - Replace recursive evaluation with stack machine
   - Avoid function call overhead
   - Expected: 20-30% speedup

4. **Bytecode Compilation** (stretch goal):
   - Compile AST to bytecode
   - Interpret bytecode instead of AST
   - Expected: 3-4x speedup (match Bytecode VM)

**Total Expected**: 45-65% additional speedup on top of Phase 2

---

### Phase 4: Advanced Optimizations (INTERP-OPT-004)

**Objective**: Approach or beat Python (16.69ms)

**Candidate Optimizations**:

1. **JIT Compilation** (using Cranelift):
   - Compile hot functions to native code
   - Expected: 5-10x speedup for hot code
   - Risk: HIGH (major architectural change)

2. **Profile-Guided Optimization**:
   - Collect runtime profiles
   - Optimize hot paths
   - Expected: 20-30% speedup

3. **Parallel Parsing** (if applicable):
   - Parse multiple files in parallel
   - Expected: 2x speedup for multi-file programs

4. **Constant Folding**:
   - Evaluate constants at parse time
   - Expected: 5-10% speedup

**Total Expected**: 2-10x additional speedup (approach JIT levels)

---

## 📋 Implementation Plan

### Week 1: Profiling & Measurement (INTERP-OPT-001)

**Day 1-2**: Set up profiling infrastructure
- Create `tests/test_interp_opt_001_profiling.rs`
- Measure Hello World breakdown
- Generate flamegraphs

**Day 3-4**: Analyze bottlenecks
- Identify hot functions
- Measure memory allocations
- Document findings

**Day 5**: Create optimization tickets
- Prioritize by impact and effort
- Create roadmap for Phases 2-4

**Deliverable**: `docs/INTERP_OPT_001_PROFILING_REPORT.md`

---

### Week 2-3: Low-Hanging Fruit (INTERP-OPT-002)

**Implement optimizations based on profiling data**:
- String interning (if applicable)
- Pre-allocate collections
- Avoid unnecessary clones
- Inline hot functions
- Optimize scope lookup

**Deliverable**: 35-65% speedup (34.71ms → 12-23ms)

---

### Week 4-6: Structural Optimizations (INTERP-OPT-003)

**Implement deeper changes**:
- AST simplification
- Value representation improvements
- Stack-based evaluator (if beneficial)
- Bytecode compilation (stretch goal)

**Deliverable**: 45-65% additional speedup (aim for <10ms)

---

## 🎯 Success Criteria

### Minimum Viable Improvement (MVI)
- **Target**: <23ms (2x faster than current, beating Deno)
- **Effort**: Low (Phase 2 optimizations)
- **Risk**: Low (no architectural changes)

### Target Performance
- **Target**: <10ms (approaching Bytecode VM)
- **Effort**: Medium (Phases 2+3)
- **Risk**: Medium (some architectural changes)

### Stretch Goal
- **Target**: <16.69ms (beating Python)
- **Effort**: High (all phases)
- **Risk**: High (possible JIT required)

---

## 📊 Metrics to Track

### Performance Metrics
- **Hello World startup time**: Currently 34.71ms
- **Fibonacci(20) execution**: Currently unknown
- **Memory usage**: Currently unknown
- **Throughput (ops/sec)**: Currently 5M ops/sec

### Quality Metrics
- **Test coverage**: Maintain >85%
- **Clippy warnings**: Zero
- **Complexity**: <20 per function
- **PMAT score**: Maintain or improve

---

## 🔄 Comparison with PERF-001A

**PERF-001A** (blocked): Optimize Ruchy compiler's generated code performance
**INTERP-OPT** (current): Optimize RuchyRuchy interpreter performance

**Different Goals**:
- PERF-001A: External compiler (ruchy) optimization
- INTERP-OPT: Internal interpreter optimization

**Same Principles**:
- Measure before optimizing (Genchi Genbutsu)
- Profile to find bottlenecks (Amdahl's Law)
- Data-driven decisions (Kaizen)

---

## 🚀 Getting Started

**Immediate Next Steps**:

1. **Create profiling test**:
   ```bash
   # tests/test_interp_opt_001_profiling.rs
   ```

2. **Run baseline measurements**:
   ```bash
   cargo test --test test_interp_opt_001_profiling --release -- --nocapture
   ```

3. **Generate flamegraph**:
   ```bash
   cargo flamegraph --test test_interp_opt_001_profiling
   ```

4. **Analyze and document**:
   ```bash
   # Create docs/INTERP_OPT_001_PROFILING_REPORT.md
   ```

---

## 📅 Timeline

- **Week 1**: Profiling & analysis (INTERP-OPT-001)
- **Week 2-3**: Low-hanging fruit optimizations (INTERP-OPT-002)
- **Week 4-6**: Structural optimizations (INTERP-OPT-003)
- **Week 7+**: Advanced optimizations (INTERP-OPT-004) - if needed

**Milestone 1**: <23ms (beat Deno) - End of Week 3
**Milestone 2**: <10ms (approach Bytecode VM) - End of Week 6
**Milestone 3**: <16.69ms (beat Python) - Stretch goal

---

## 🔗 References

- **Current Benchmarks**: `tests/test_interp_030_benchmarking.rs`
- **Performance Insights**: `docs/PERFORMANCE_INSIGHTS_JULIA_JIT.md`
- **Profiler Code**: `src/profiler/compiler_profiler.rs`
- **Parser**: `src/interpreter/parser.rs`
- **Evaluator**: `src/interpreter/evaluator.rs`

---

**Status**: 📝 **PLANNING COMPLETE** - Ready to start INTERP-OPT-001
**Next**: Create profiling test and establish detailed baseline
**Date**: 2025-11-03
