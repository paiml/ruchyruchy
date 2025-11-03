# Performance Optimization Roadmap: Beat Julia JIT & C

**Goal**: Close gap to Julia JIT (2.03ms) and beat C performance (3.02ms) through transpile/compile optimization

**Focus**: ONLY transpile/compile optimization (NOT interpreter)

**Current Status**: Ruchy Transpiled: 3.21ms | Ruchy Compiled: 2.64ms | Julia JIT: 2.03ms | C: 3.02ms

---

## 🎯 Performance Targets

### Current State (Baseline)
```
Julia JIT:           2.03ms  ← TARGET TO BEAT
Ruchy Compiled:      2.64ms  (30% slower than Julia, beating C by 13%)
Go (AOT):            2.78ms
C (AOT):             3.02ms
Rust (AOT):          3.04ms
Ruchy Transpiled:    3.21ms  ← CURRENT FOCUS (58% slower than Julia)
```

### Optimization Goals
1. **Phase 1**: Beat C (3.02ms) → Target: <3.00ms ✅ **ALREADY ACHIEVED** (Ruchy Compiled: 2.64ms)
2. **Phase 2**: Beat Go (2.78ms) → Target: <2.75ms ✅ **ALREADY ACHIEVED** (Ruchy Compiled: 2.64ms)
3. **Phase 3**: Close gap to Julia JIT (2.03ms) → Target: <2.10ms
4. **Phase 4**: Beat Julia JIT → Target: <2.00ms (stretch goal)

---

## 📊 Performance Gap Analysis

### Current Performance Ladder
```
Component                  Time (ms)   Gap from Julia   Status
─────────────────────────────────────────────────────────────────
Julia JIT (LLVM)           2.03        0.00 (baseline)  TARGET
Ruchy Compiled             2.64        +0.61 (+30%)     ✅ BEATS C
Go AOT                     2.78        +0.75 (+37%)
C AOT                      3.02        +0.99 (+49%)     ✅ BEATEN
Rust AOT                   3.04        +1.01 (+50%)
Ruchy Transpiled (rustc)   3.21        +1.18 (+58%)     🎯 OPTIMIZE THIS
```

### Gap to Close
- **Ruchy Compiled → Julia JIT**: 0.61ms (30% improvement needed)
- **Ruchy Transpiled → Julia JIT**: 1.18ms (58% improvement needed)

---

## 🔍 Bottleneck Identification (Phase 0)

### PERF-001: Profile Current Transpile/Compile Path

**Objective**: Measure where the 3.21ms is spent in Ruchy transpiled mode

**Tasks**:
1. ✅ Instrument CompilerProfiler with phase tracking
2. 📊 Measure each compilation phase:
   - Parsing: ? ms
   - Type checking: ? ms
   - Optimization passes: ? ms
   - Code generation (transpile to Rust): ? ms
   - rustc compilation: ? ms
   - Linking: ? ms
   - Startup overhead: ? ms

3. 📊 Identify bottlenecks (>20% of total time)
4. 📊 Compare with Julia's LLVM JIT pipeline

**Acceptance Criteria**:
- [ ] Complete phase-by-phase timing breakdown
- [ ] Bottleneck identification (top 3 slowest phases)
- [ ] Baseline measurements documented
- [ ] Comparison with Julia JIT phases

**Deliverables**:
- `tests/test_perf_001_baseline_profiling.rs`
- `docs/PERF_001_BASELINE_ANALYSIS.md`

---

## 🚀 Phase 1: Compiler Optimizations (Target: <2.75ms)

### PERF-002: Optimize Parser Performance

**Baseline**: ? ms (to be measured in PERF-001)
**Target**: 50% reduction in parse time

**Optimizations**:
1. **Zero-copy parsing**: Avoid string allocations
2. **Incremental parsing**: Cache parsed AST nodes
3. **Parallel parsing**: Parse independent modules concurrently
4. **Lazy parsing**: Delay parsing of unused functions
5. **Optimized tokenizer**: Use SIMD for lexical analysis

**Benchmarks**:
- Hello World: ? ms → target: <0.5ms
- Fibonacci (recursive): ? ms → target: <1.0ms
- Large file (1000 LOC): ? ms → target: <10ms

**Acceptance Criteria**:
- [ ] 50% reduction in parse time
- [ ] Zero allocations for simple programs
- [ ] Benchmarks pass in release mode

**Deliverables**:
- `src/parser_optimized.rs`
- `tests/test_perf_002_parser_opt.rs`
- `benchmarks/bench_parser.rs`

### PERF-003: Optimize Type Checker Performance

**Baseline**: ? ms (to be measured in PERF-001)
**Target**: 50% reduction in type checking time

**Optimizations**:
1. **Type inference caching**: Memoize type inference results
2. **Parallel type checking**: Check independent functions concurrently
3. **Incremental type checking**: Only recheck changed code
4. **Constraint solving optimization**: Use union-find for unification
5. **Type propagation**: Forward/backward dataflow analysis

**Benchmarks**:
- Hello World: ? ms → target: <0.2ms
- Complex types (generics): ? ms → target: <2ms

**Acceptance Criteria**:
- [ ] 50% reduction in type checking time
- [ ] Cached type inference working
- [ ] Benchmarks pass in release mode

**Deliverables**:
- `src/typechecker_optimized.rs`
- `tests/test_perf_003_typecheck_opt.rs`
- `benchmarks/bench_typecheck.rs`

### PERF-004: Optimize Code Generation (Transpile)

**Baseline**: ? ms (to be measured in PERF-001)
**Target**: 50% reduction in codegen time

**Optimizations**:
1. **String building optimization**: Use `String::with_capacity`
2. **Template-based codegen**: Pre-compiled templates
3. **Parallel codegen**: Generate functions concurrently
4. **Incremental codegen**: Only regenerate changed code
5. **Optimized Rust emission**: Generate minimal, efficient Rust code

**Benchmarks**:
- Hello World: ? ms → target: <0.3ms
- Large program (1000 functions): ? ms → target: <50ms

**Acceptance Criteria**:
- [ ] 50% reduction in codegen time
- [ ] Generated Rust is optimized (no unnecessary allocations)
- [ ] Benchmarks pass in release mode

**Deliverables**:
- `src/codegen_optimized.rs`
- `tests/test_perf_004_codegen_opt.rs`
- `benchmarks/bench_codegen.rs`

---

## 🔥 Phase 2: rustc Compilation Optimizations (Target: <2.50ms)

### PERF-005: Optimize rustc Compilation Pipeline

**Baseline**: ? ms (to be measured in PERF-001)
**Target**: 50% reduction in rustc compile time

**Optimizations**:
1. **Minimal dependencies**: Remove unused stdlib dependencies
2. **LTO optimization**: Enable Link-Time Optimization
3. **Codegen units**: Optimize for single-threaded compilation
4. **Target-specific codegen**: Use native CPU features
5. **Incremental compilation**: Cache rustc artifacts

**Cargo.toml optimizations**:
```toml
[profile.release]
opt-level = 3              # Maximum optimization
lto = "fat"                # Full LTO
codegen-units = 1          # Single codegen unit for max optimization
panic = "abort"            # Smaller binary
strip = true               # Strip symbols
```

**Acceptance Criteria**:
- [ ] 50% reduction in rustc compile time
- [ ] Generated binary is optimized
- [ ] Startup time <2.50ms

**Deliverables**:
- `Cargo.toml` optimizations
- `tests/test_perf_005_rustc_opt.rs`
- `benchmarks/bench_rustc.rs`

### PERF-006: Optimize Linking Phase

**Baseline**: ? ms (to be measured in PERF-001)
**Target**: 50% reduction in link time

**Optimizations**:
1. **Static linking**: Eliminate dynamic library overhead
2. **LLD linker**: Use LLVM's lld (faster than ld)
3. **Minimal symbols**: Strip unnecessary symbols
4. **Link-time optimization**: Cross-module inlining

**`.cargo/config.toml`**:
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

**Acceptance Criteria**:
- [ ] 50% reduction in link time
- [ ] lld linker working
- [ ] Binary size optimized

**Deliverables**:
- `.cargo/config.toml`
- `tests/test_perf_006_link_opt.rs`
- `benchmarks/bench_link.rs`

---

## ⚡ Phase 3: Runtime Optimizations (Target: <2.10ms)

### PERF-007: Optimize Startup Overhead

**Baseline**: ? ms (to be measured in PERF-001)
**Target**: <0.5ms startup overhead

**Optimizations**:
1. **Lazy initialization**: Defer stdlib init
2. **Minimal runtime**: No unnecessary global state
3. **Fast allocator**: Use jemalloc or mimalloc
4. **Pre-computed constants**: Compile-time evaluation
5. **Optimized main()**: Minimal entry point overhead

**Benchmarks**:
- Empty program: ? ms → target: <0.5ms
- Hello World: ? ms → target: <2.0ms

**Acceptance Criteria**:
- [ ] <0.5ms startup overhead
- [ ] Lazy initialization working
- [ ] jemalloc integrated

**Deliverables**:
- `src/runtime_optimized.rs`
- `tests/test_perf_007_startup_opt.rs`
- `benchmarks/bench_startup.rs`

### PERF-008: Optimize Standard Library

**Baseline**: ? ms for stdlib calls
**Target**: 50% reduction in stdlib overhead

**Optimizations**:
1. **Inline critical functions**: `#[inline(always)]`
2. **SIMD optimizations**: Use packed SIMD for arrays
3. **Zero-cost abstractions**: Eliminate runtime overhead
4. **Const evaluation**: Compile-time computation
5. **Specialized implementations**: Type-specific fast paths

**Benchmarks**:
- String operations: ? ms → target: <0.1ms
- Array operations: ? ms → target: <0.1ms
- I/O operations: ? ms → target: <0.5ms

**Acceptance Criteria**:
- [ ] 50% reduction in stdlib overhead
- [ ] Inlining effective (check assembly)
- [ ] Benchmarks pass

**Deliverables**:
- `src/stdlib_optimized.rs`
- `tests/test_perf_008_stdlib_opt.rs`
- `benchmarks/bench_stdlib.rs`

---

## 🎯 Phase 4: Advanced Optimizations (Target: <2.00ms, Beat Julia!)

### PERF-009: LLVM IR Direct Emission

**Objective**: Bypass rustc, emit LLVM IR directly (like Julia)

**Current**: Ruchy → Rust → rustc → LLVM → Binary
**Optimized**: Ruchy → LLVM IR → LLVM → Binary

**Advantages**:
1. **Skip rustc overhead**: Direct LLVM compilation
2. **Julia-like performance**: Same backend as Julia
3. **JIT capability**: Runtime compilation possible
4. **Fine-grained control**: Custom LLVM passes

**Implementation**:
1. Use `inkwell` crate for LLVM IR generation
2. Emit LLVM IR from Ruchy AST directly
3. Use LLVM optimization passes
4. AOT compilation via LLVM
5. (Future) JIT compilation via LLVM ORC

**Benchmarks**:
- Hello World: Target: <2.0ms (beat Julia's 2.03ms)
- Complex programs: Target: Parity with Julia

**Acceptance Criteria**:
- [ ] LLVM IR emission working
- [ ] Compilation <2.0ms
- [ ] Correctness preserved

**Deliverables**:
- `src/llvm_backend.rs`
- `tests/test_perf_009_llvm.rs`
- `benchmarks/bench_llvm.rs`

### PERF-010: Profile-Guided Optimization (PGO)

**Objective**: Use runtime profiles to optimize hot paths

**Implementation**:
1. **Instrumentation**: Add profiling instrumentation
2. **Profile collection**: Run benchmarks, collect profiles
3. **PGO compilation**: Recompile with profiles
4. **Optimization**: LLVM uses profiles for better codegen

**rustc PGO**:
```bash
# Step 1: Build instrumented binary
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

# Step 2: Run benchmarks to collect profile
./target/release/ruchy benchmarks/*.ruchy

# Step 3: Merge profiles
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

# Step 4: Build optimized binary with PGO
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
```

**Acceptance Criteria**:
- [ ] PGO pipeline working
- [ ] 10-20% speedup from PGO
- [ ] Benchmarks faster

**Deliverables**:
- `scripts/pgo_build.sh`
- `tests/test_perf_010_pgo.rs`
- `docs/PGO_GUIDE.md`

---

## 📊 Continuous Benchmarking

### PERF-BENCH: Benchmark Suite

**Benchmarks** (all from ruchy-book):
1. **Hello World** (BENCH-012)
2. **Fibonacci (recursive)**
3. **Factorial (recursive)**
4. **Array operations**
5. **String operations**
6. **I/O operations**
7. **Complex programs**

**Targets** (Hello World baseline):
```
Phase 0 (Baseline):      3.21ms  (Ruchy Transpiled current)
Phase 1 (Compiler opt):  <2.75ms (Beat Go)
Phase 2 (rustc opt):     <2.50ms (Close to Ruchy Compiled)
Phase 3 (Runtime opt):   <2.10ms (Close to Julia)
Phase 4 (LLVM IR):       <2.00ms (Beat Julia!)
```

**CI Integration**:
```yaml
# .github/workflows/performance.yml
name: Performance Benchmarks
on: [push, pull_request]
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run benchmarks
        run: cargo bench --bench perf_suite
      - name: Check performance regression
        run: ./scripts/check_perf_regression.sh
```

**Acceptance Criteria**:
- [ ] All benchmarks automated
- [ ] CI integration working
- [ ] Performance regression detection
- [ ] Results tracked over time

**Deliverables**:
- `benches/perf_suite.rs`
- `.github/workflows/performance.yml`
- `scripts/check_perf_regression.sh`
- `docs/BENCHMARK_RESULTS.md`

---

## 🏆 Success Criteria

### Phase 1: Beat C ✅
- [x] Ruchy Compiled: 2.64ms < C: 3.02ms **ALREADY ACHIEVED**

### Phase 2: Beat Go ✅
- [x] Ruchy Compiled: 2.64ms < Go: 2.78ms **ALREADY ACHIEVED**

### Phase 3: Close Gap to Julia
- [ ] Ruchy Optimized: <2.10ms (within 3% of Julia: 2.03ms)

### Phase 4: Beat Julia (Stretch Goal)
- [ ] Ruchy LLVM: <2.00ms (faster than Julia: 2.03ms)

---

## 📅 Timeline & Priorities

### Week 1-2: Baseline & Phase 1
- **PERF-001**: Baseline profiling ✅
- **PERF-002**: Parser optimization
- **PERF-003**: Type checker optimization
- **PERF-004**: Codegen optimization
- **Target**: <2.75ms (beat Go)

### Week 3-4: Phase 2
- **PERF-005**: rustc optimization
- **PERF-006**: Linking optimization
- **Target**: <2.50ms

### Week 5-6: Phase 3
- **PERF-007**: Startup optimization
- **PERF-008**: Stdlib optimization
- **Target**: <2.10ms

### Week 7-8: Phase 4 (Stretch)
- **PERF-009**: LLVM IR direct emission
- **PERF-010**: Profile-Guided Optimization
- **Target**: <2.00ms (beat Julia!)

---

## 🔧 Tools & Infrastructure

### Required Tools
- [ ] `perf` (Linux performance profiler)
- [ ] `flamegraph` (visualization)
- [ ] `cargo-bench` (microbenchmarks)
- [ ] `criterion` (statistical benchmarking)
- [ ] `hyperfine` (command-line benchmarking)
- [ ] `llvm-profdata` (PGO)
- [ ] `inkwell` (LLVM IR generation)

### Setup
```bash
# Install tools
cargo install cargo-flamegraph
cargo install hyperfine
sudo apt-get install linux-tools-generic  # perf

# Run benchmarks
cargo bench --bench perf_suite
hyperfine './target/release/ruchy hello.ruchy'

# Profile with perf
perf record -g ./target/release/ruchy hello.ruchy
perf report

# Generate flamegraph
cargo flamegraph --bench perf_suite
```

---

## 📚 References

- **Julia JIT Performance**: `docs/PERFORMANCE_INSIGHTS_JULIA_JIT.md`
- **ruchy-book BENCH-012**: Chapter 23 Benchmark Analysis
- **LLVM Documentation**: https://llvm.org/docs/
- **Rust Performance Book**: https://nnethercote.github.io/perf-book/
- **PGO Guide**: https://doc.rust-lang.org/rustc/profile-guided-optimization.html

---

## 🎯 Next Actions

### Immediate (Week 1)
1. **PERF-001**: Create baseline profiling test
2. **Measure**: Profile current transpile/compile path
3. **Analyze**: Identify top 3 bottlenecks
4. **Document**: Write PERF_001_BASELINE_ANALYSIS.md

### Command to Start
```bash
# Create baseline profiling test
cargo test --test test_perf_001_baseline_profiling --release -- --nocapture

# Profile with hyperfine
hyperfine --warmup 3 './target/release/ruchy benchmarks/hello.ruchy'

# Profile with perf
perf stat ./target/release/ruchy benchmarks/hello.ruchy
```

---

**Status**: 🎯 **READY TO START**
**Focus**: ONLY transpile/compile optimization (NOT interpreter)
**Goal**: Beat Julia JIT (2.03ms) and C (3.02ms)
**Current**: Ruchy Compiled: 2.64ms ✅ (already beating C!), Ruchy Transpiled: 3.21ms
**Target**: <2.00ms (beat Julia by ~1.5%)
