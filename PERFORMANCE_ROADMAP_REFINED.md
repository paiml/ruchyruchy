# Performance Optimization Roadmap (REFINED)

**Refined**: 2025-11-03 (Critical Review Applied)
**Principles**: Toyota Way (Genchi Genbutsu, Kaizen, Jidoka) + CS Research

---

## 🎯 Critical Insight: Two Distinct Goals

**Problem Identified**: Original roadmap conflated two different, sometimes opposing metrics.

### Goal 1: Generated Code Performance (Runtime)
**What**: How fast the compiled binary runs
**Metric**: Execution time of `./fib` (e.g., 2.64ms)
**Target**: <2.10ms (approach Julia's 2.03ms)
**Comparison**: Julia JIT, C, Go, Rust

### Goal 2: Compiler Throughput (Build Time)
**What**: How fast the compiler turns source → binary
**Metric**: Time for `ruchy build fib.ruchy` (currently unknown!)
**Target**: <500ms for 1000 LOC project (example)
**Comparison**: rustc, go build, clang

**Key Principle**: Optimizing one does NOT optimize the other!
- PGO makes builds slower but executables faster
- Parser optimization helps builds but NOT runtime
- Startup optimization helps runtime but NOT builds

---

## 📊 PERF-001: Rigorous Baseline Analysis (BLOCKING ALL OTHER WORK)

**Status**: 🚨 **MANDATORY FIRST STEP** - Nothing else proceeds until this is complete

**Problem**: Current baseline (2.64ms, 3.21ms) lacks critical context:
- Which benchmark? (Hello World vs Fibonacci vs compute-bound)
- What does it include? (Compiler time? Runtime? Both?)
- Warm JIT vs cold start?
- Which phase dominates?

### PERF-001A: Decompose Generated Code Performance

**Objective**: Break down the 2.64ms runtime into measurable components

**Benchmark**: Fibonacci(35) recursive (compute-bound, non-trivial)

**Measurements Required**:
```bash
# Total runtime (what we compare to Julia)
time ./target/release/fib  # Target: 2.64ms total

# Breakdown (use perf, dtrace, custom instrumentation):
# 1. OS Process Startup: ? ms
#    - fork/exec overhead
#    - ELF loading
#    - Dynamic linking (if any)
#
# 2. Ruchy Runtime Init: ? ms
#    - Static constructors
#    - Stdlib initialization
#    - Global state setup
#
# 3. Actual Computation: ? ms
#    - User code execution (fib(35))
#    - Function call overhead
#    - Allocation overhead
#
# 4. Shutdown: ? ms
#    - Destructors
#    - Exit handlers
```

**Amdahl's Law Analysis**:
If runtime breakdown is:
- Process startup: 0.5ms (19%)
- Runtime init: 0.3ms (11%)
- Computation: 1.8ms (68%)
- Shutdown: 0.04ms (2%)

Then: **Optimizing computation is 3x more valuable than optimizing startup**

**Acceptance Criteria**:
- [ ] Measured breakdown for 5 benchmarks (Hello World, Fib, Factorial, Array ops, I/O)
- [ ] Identified dominant phase (>50% of time)
- [ ] Comparison table with Julia's equivalent breakdown
- [ ] Clear statement: "To beat Julia, we must optimize [X] by [Y]%"

**Deliverables**:
- `tests/test_perf_001a_runtime_baseline.rs`
- `docs/PERF_001A_RUNTIME_ANALYSIS.md` with profiling data
- Flamegraphs for each benchmark

### PERF-001B: Decompose Compiler Throughput

**Objective**: Break down `ruchy build` time into phases

**Benchmark**: 1000 LOC project (realistic size)

**Measurements Required**:
```bash
# Total build time
time ruchy build myproject.ruchy  # Currently unknown!

# Breakdown (instrument CompilerProfiler):
# Ruchy Frontend:
# 1. Parsing: ? ms
# 2. Type Checking: ? ms
# 3. Optimization Passes: ? ms
# 4. Codegen (emit Rust): ? ms
#
# Rust Backend:
# 5. rustc compile: ? ms
#    - Frontend (parse Rust): ? ms
#    - MIR passes: ? ms
#    - LLVM codegen: ? ms
# 6. Linking: ? ms
#
# Total: ? ms
```

**Amdahl's Law Analysis**:
If build breakdown is:
- Ruchy frontend: 50ms (10%)
- rustc compile: 400ms (80%)
- Linking: 50ms (10%)

Then: **Optimizing rustc dominates, frontend optimization is low-impact**

**Acceptance Criteria**:
- [ ] Measured phase-by-phase breakdown
- [ ] Identified bottleneck (>40% of time)
- [ ] Comparison with `cargo build` time for equivalent Rust code
- [ ] Clear statement: "To achieve <500ms builds, we must optimize [X] by [Y]%"

**Deliverables**:
- `tests/test_perf_001b_compiler_baseline.rs`
- `docs/PERF_001B_COMPILER_ANALYSIS.md`
- CompilerProfiler phase reports

---

## 🚀 Goal 1: Generated Code Performance (<2.10ms Runtime)

**Current**: 2.64ms (Ruchy Compiled)
**Target**: <2.10ms (within 3.4% of Julia's 2.03ms)
**Stretch**: <2.00ms (beat Julia!)

**Strategy**: Focus on runtime, not compile time

### Priority 0: Data-Driven Planning (PERF-001A) 🚨 BLOCKING

**Status**: Must complete before any optimization work

**Output**: Ranked list of optimization opportunities with expected impact

**Example Output** (hypothetical):
```
Priority Ranking (based on Amdahl's Law):
1. Computation overhead (68% of time) - 50% reduction → 0.90ms saved
2. Process startup (19% of time) - 50% reduction → 0.25ms saved
3. Runtime init (11% of time) - 50% reduction → 0.15ms saved
4. Shutdown (2% of time) - 50% reduction → 0.02ms saved

Conclusion: Focus 80% of effort on #1 (computation), 20% on #2 (startup)
```

### Priority 1: Backend Optimization (Quick Wins)

**PERF-002: Optimize Cargo.toml (rustc codegen)**

**Rationale**: Zero code changes, maximum impact

**Changes**:
```toml
[profile.release]
opt-level = 3              # Max optimization
lto = "fat"                # Cross-crate inlining
codegen-units = 1          # Single unit (slower build, faster code)
panic = "abort"            # Smaller binary, faster unwinding
strip = true               # Remove symbols
overflow-checks = false    # Remove integer overflow checks (if safe)
```

**Expected Impact**: 5-10% runtime improvement (based on Rust benchmarks)

**Acceptance Criteria**:
- [ ] Fibonacci benchmark <2.50ms
- [ ] No correctness regressions
- [ ] Documented in PERF_002.md

**Deliverables**:
- Updated `Cargo.toml`
- `tests/test_perf_002_cargo_opt.rs`

**PERF-003: Optimize Linker (lld)**

**Rationale**: Faster linking, better optimization

**Changes**:
```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = [
  "-C", "link-arg=-fuse-ld=lld",
  "-C", "target-cpu=native"  # Use CPU-specific instructions
]
```

**Expected Impact**: 2-5% runtime improvement + faster linking

**Acceptance Criteria**:
- [ ] Fibonacci benchmark <2.45ms
- [ ] Build time reduced

**Deliverables**:
- `.cargo/config.toml`
- `tests/test_perf_003_lld.rs`

### Priority 2: Startup Optimization (Fixed Cost Reduction)

**PERF-004: Minimize Process Startup Overhead**

**Objective**: Reduce OS-level startup cost

**Optimizations** (only if PERF-001A shows startup >0.5ms):
1. **Static linking**: Eliminate dynamic library overhead
   ```toml
   [profile.release]
   target-feature = "+crt-static"
   ```

2. **Minimal entry point**: Optimize `main()`
   ```rust
   #[no_mangle]
   pub extern "C" fn main() -> i32 {
       // Minimal initialization
       unsafe { ruchy_main() }
   }
   ```

3. **Lazy initialization**: Defer stdlib init
   ```rust
   static RUNTIME: OnceLock<Runtime> = OnceLock::new();
   ```

**Expected Impact**: 20-30% reduction in startup (if it's a bottleneck)

**Acceptance Criteria**:
- [ ] Hello World <1.0ms
- [ ] Startup overhead <0.3ms
- [ ] Lazy init working

**Deliverables**:
- `src/runtime/startup_optimized.rs`
- `tests/test_perf_004_startup.rs`

### Priority 3: Computation Optimization (Hot Path)

**PERF-005: Profile-Guided Optimization (PGO) - STRATEGIC WEAPON**

**Rationale**: This is the AOT answer to JIT's runtime specialization

**Context**: Julia's advantage is runtime type specialization. PGO gives us "offline specialization" using profile data.

**Implementation**:
```bash
#!/bin/bash
# scripts/pgo_build.sh

# Step 1: Instrumented build
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
  cargo build --release

# Step 2: Run representative workload
./target/release/ruchy benchmarks/fib.ruchy
./target/release/ruchy benchmarks/factorial.ruchy
./target/release/ruchy benchmarks/array_ops.ruchy

# Step 3: Merge profiles
llvm-profdata merge -o /tmp/pgo-data/merged.profdata \
  /tmp/pgo-data/*.profraw

# Step 4: Optimized build with profiles
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata \
  -Cllvm-args=-pgo-warn-missing-function" \
  cargo build --release
```

**Expected Impact**: 10-20% improvement (Rust documentation)

**Research Foundation**:
- Profile-guided optimization is proven effective (LLVM, GCC)
- Specialization is key to JIT performance
- PGO enables similar specialization at AOT time

**Acceptance Criteria**:
- [ ] PGO pipeline automated
- [ ] 10%+ improvement on benchmarks
- [ ] Fibonacci <2.10ms (GOAL ACHIEVED)

**Deliverables**:
- `scripts/pgo_build.sh`
- `tests/test_perf_005_pgo.rs`
- `docs/PGO_GUIDE.md`
- CI integration (`.github/workflows/pgo.yml`)

**PERF-006: Stdlib Zero-Cost Abstractions**

**Objective**: Eliminate abstraction overhead in hot paths

**Optimizations** (only if PERF-001A shows stdlib is bottleneck):
1. **Aggressive inlining**: `#[inline(always)]` on critical functions
2. **Monomorphization**: Type-specific fast paths
3. **SIMD operations**: Use `packed_simd` for arrays
4. **Const evaluation**: Compile-time computation where possible

**Expected Impact**: 5-10% if stdlib is on hot path

**Acceptance Criteria**:
- [ ] Inlining verified in assembly
- [ ] No unnecessary allocations
- [ ] Benchmarks faster

**Deliverables**:
- `src/stdlib/optimized.rs`
- `tests/test_perf_006_stdlib.rs`

---

## 🏗️ Goal 2: Compiler Throughput (<500ms for 1000 LOC)

**Current**: Unknown (measure in PERF-001B)
**Target**: <500ms for 1000 LOC project
**Stretch**: <200ms (incremental compilation)

**Strategy**: Focus on build time, not runtime

### Priority 0: Data-Driven Planning (PERF-001B) 🚨 BLOCKING

**Status**: Must complete before optimization

**Output**: Phase-by-phase breakdown showing bottleneck

### Priority 1: Low-Hanging Fruit

**PERF-007: Optimize Code Generation (String Building)**

**Objective**: Reduce memory allocations during codegen

**Optimizations** (only if PERF-001B shows codegen >20% of time):
1. **Pre-allocation**: `String::with_capacity(estimated_size)`
2. **Reusable buffers**: Buffer pool to avoid allocations
3. **Direct write**: Write to file without intermediate String

**Expected Impact**: 30-50% reduction in codegen time

**Acceptance Criteria**:
- [ ] Zero allocations for simple programs
- [ ] Codegen time reduced
- [ ] Correctness preserved

**Deliverables**:
- `src/codegen/optimized.rs`
- `tests/test_perf_007_codegen.rs`

### Priority 2: Algorithmic Improvements

**PERF-008: Parser Optimization**

**Optimizations** (only if PERF-001B shows parsing >20% of time):
1. **Zero-copy parsing**: Use string slices instead of allocations
2. **Caching**: Memoize repeated patterns
3. **Parallel parsing**: Parse independent modules concurrently

**Expected Impact**: 50% reduction in parse time

**Note**: This optimizes **compiler throughput** (build time), NOT runtime!

**Acceptance Criteria**:
- [ ] Parse time reduced
- [ ] Build time improved
- [ ] Correctness preserved

**Deliverables**:
- `src/parser/optimized.rs`
- `tests/test_perf_008_parser.rs`

**PERF-009: Type Checker Optimization**

**Optimizations** (only if PERF-001B shows type checking >20% of time):
1. **Caching**: Memoize type inference results
2. **Better algorithms**: Use union-find for unification
3. **Parallel checking**: Check independent functions concurrently

**Expected Impact**: 50% reduction in type check time

**Acceptance Criteria**:
- [ ] Type check time reduced
- [ ] Build time improved
- [ ] Correctness preserved

**Deliverables**:
- `src/typechecker/optimized.rs`
- `tests/test_perf_009_typecheck.rs`

### Priority 3: Architectural Shift (FUTURE - HIGH RISK)

**PERF-010: Incremental Compilation (Major Epic)**

**Warning**: This is NOT a simple optimization. This is an architectural rewrite.

**Scope**:
1. Dependency tracking system
2. Cache invalidation logic
3. Persistent compilation cache
4. Change detection
5. Partial recompilation

**Complexity**: This is why `rustc` feels fast. It's 100,000+ lines of code.

**Expected Impact**: 10x improvement on incremental builds (first build still slow)

**Recommendation**:
- **Do NOT** attempt until compiler is stable and correct
- Consider as "Phase 5" (post-v1.0)
- Requires dedicated team and 6+ months

**Deliverables** (if pursued):
- Full design document
- Separate epic with 20+ tickets
- Extensive testing infrastructure

---

## 🚨 High-Risk Ticket: LLVM Backend (Jidoka Analysis)

### PERF-011: Direct LLVM IR Emission (FUTURE - VERY HIGH RISK)

**Original Claim**: "Bypass rustc, emit LLVM IR directly (like Julia)"

**Jidoka Analysis** (Stop and Fix Problems):

**Problem**: This is not an optimization. This is a **complete backend rewrite**.

**What You're Abandoning**:
- `rustc`'s proven code generation
- Type safety guarantees
- Memory safety guarantees
- Cross-platform support
- Debugger integration
- All optimizations `rustc` provides for free

**What You Must Implement**:
- LLVM IR generation for all language features
- Type lowering (Ruchy types → LLVM types)
- ABI compliance (calling conventions)
- Stack management
- Exception handling
- Garbage collection (if needed)
- Debug info generation
- Multi-target support (x86, ARM, etc.)
- All of LLVM's complexity

**Estimated Effort**: 2-3 engineer-years (based on similar projects)

**Risk Assessment**: **CRITICAL - High probability of failure**

**Alternative Approaches** (Lower Risk):

1. **Option A: Optimize what we have** (Recommended)
   - PGO can achieve 10-20% improvement
   - Cargo.toml optimizations: 5-10%
   - Combined: Potentially beat Julia without rewrite

2. **Option B: Cranelift Backend** (Medium Risk)
   - Rust-native code generator
   - Simpler than LLVM
   - Used by Wasmtime (proven)
   - Still a major effort (6-12 months)

3. **Option C: LLVM via Inkwell** (High Risk)
   - Use `inkwell` crate (safe LLVM bindings)
   - Start with simple subset
   - Prove feasibility on toy examples
   - Only proceed if clearly superior to rustc

**Decision Point**:
- After PERF-005 (PGO) is complete, measure results
- If PGO gets us <2.10ms: **Do NOT attempt LLVM backend**
- If PGO plateaus >2.20ms: **Consider** Cranelift or LLVM (with off-ramp)

**Off-Ramp**: After 3 months of LLVM work, reassess:
- Is it faster than PGO-optimized rustc?
- Is it correct?
- Is it maintainable?
- If NO to any: Abandon and focus on PGO

**Deliverables** (only if pursued):
- Full design document with risk analysis
- Proof-of-concept (Hello World only)
- Decision checkpoint at 3 months
- Clear success criteria

---

## 📊 Success Criteria (Revised with Rigor)

### Goal 1: Generated Code Performance

**Phase 1: Quick Wins** (Weeks 1-2)
- [x] PERF-001A: Rigorous runtime baseline ✅ BLOCKING
- [ ] PERF-002: Cargo.toml optimization
- [ ] PERF-003: LLD linker
- [ ] **Target**: <2.50ms (5% improvement)

**Phase 2: Startup** (Weeks 3-4)
- [ ] PERF-004: Startup optimization
- [ ] **Target**: <2.30ms (13% improvement)

**Phase 3: PGO** (Weeks 5-6) - **STRATEGIC WEAPON**
- [ ] PERF-005: Profile-Guided Optimization
- [ ] **Target**: <2.10ms (20% improvement) - **GOAL ACHIEVED**

**Stretch: Beat Julia** (Weeks 7-8)
- [ ] PERF-006: Stdlib optimizations
- [ ] **Target**: <2.00ms (24% improvement) - **BEAT JULIA**

### Goal 2: Compiler Throughput

**Phase 1: Measurement** (Weeks 1-2)
- [x] PERF-001B: Rigorous compiler baseline ✅ BLOCKING
- [ ] Identify bottleneck (>40% of time)

**Phase 2: Quick Wins** (Weeks 3-4)
- [ ] PERF-007: Codegen string optimization
- [ ] **Target**: 20% improvement

**Phase 3: Algorithms** (Weeks 5-8)
- [ ] PERF-008: Parser optimization
- [ ] PERF-009: Type checker optimization
- [ ] **Target**: <500ms for 1000 LOC project

---

## 🎯 Immediate Next Actions

### Week 1: Establish Ground Truth

**Step 1: Create PERF-001A (Runtime Baseline)**
```bash
# Create test infrastructure
cargo new --lib perf_baseline
cd perf_baseline

# Write test (with perf instrumentation)
cat > tests/test_perf_001a.rs <<'EOF'
// PERF-001A: Runtime Baseline Analysis
// Measures: Process startup, runtime init, computation, shutdown

use std::time::Instant;

#[test]
fn test_fibonacci_35_baseline() {
    let total_start = Instant::now();

    // Measure computation
    let compute_start = Instant::now();
    let result = fib(35);
    let compute_time = compute_start.elapsed();

    let total_time = total_start.elapsed();

    println!("Result: {}", result);
    println!("Computation: {:?}", compute_time);
    println!("Total: {:?}", total_time);
    println!("Overhead: {:?}", total_time - compute_time);

    assert_eq!(result, 9227465); // Verify correctness
}

fn fib(n: u64) -> u64 {
    if n <= 1 { n } else { fib(n-1) + fib(n-2) }
}
EOF

# Run with perf
cargo test --release -- --nocapture
perf stat cargo test --release
```

**Step 2: Create PERF-001B (Compiler Baseline)**
```bash
# Instrument CompilerProfiler
# Add phase timing to ruchy build

# Measure
time ruchy build benchmarks/fib.ruchy
```

**Step 3: Analyze and Prioritize**
```bash
# Generate report
./scripts/analyze_perf_001.sh > docs/PERF_001_BASELINE_REPORT.md

# Read report and create prioritized backlog
# DO NOT start optimizations until this is complete!
```

---

## 📚 Research References

### Toyota Way
- **Genchi Genbutsu**: Go and See - Measure, don't guess
- **Kaizen**: Continuous improvement based on data
- **Jidoka**: Stop and fix problems - Risk analysis for LLVM

### Computer Science
- **Amdahl's Law**: Focus on dominant bottleneck
- **AOT vs JIT**: PGO as strategic answer to runtime specialization
- **Zero-Cost Abstractions**: Rust principle for stdlib design

### Performance Engineering
- **Rust Performance Book**: https://nnethercote.github.io/perf-book/
- **PGO Research**: Profile-Guided Optimization (LLVM, GCC papers)
- **Julia Performance Tips**: https://docs.julialang.org/en/v1/manual/performance-tips/

---

**Status**: 🎯 **READY - Rigorously Structured**
**Focus**: Measurement FIRST, optimization SECOND
**Next Step**: Create PERF-001A baseline test (runtime)
**Blocking**: ALL optimizations wait for PERF-001 data
