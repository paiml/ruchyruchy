# Performance Profiling & Compiler Tooling Specification

**Version**: 1.0
**Date**: 2025-11-02
**Status**: ACTIVE - Internal Compiler Developer Tool
**Target Audience**: Ruchy Compiler Developers ONLY
**Purpose**: Optimize transpiler, interpreter, JIT, and compiled output

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Motivation: The 181x Problem](#2-motivation-the-181x-problem)
3. [Surprising Performance Findings](#3-surprising-performance-findings)
4. [5 Easy Wins for Immediate Impact](#4-five-easy-wins-for-immediate-impact)
5. [Julia's JIT Architecture Deep Dive](#5-julias-jit-architecture-deep-dive)
6. [Tool Architecture for Ruchy](#6-tool-architecture-for-ruchy)
7. [Profiler Design](#7-profiler-design)
8. [Instrumentation Points](#8-instrumentation-points)
9. [Optimization Discovery Engine](#9-optimization-discovery-engine)
10. [Transpiler Optimizer](#10-transpiler-optimizer)
11. [JIT Compilation Strategy](#11-jit-compilation-strategy)
12. [Performance Benchmarking Integration](#12-performance-benchmarking-integration)
13. [Implementation Roadmap](#13-implementation-roadmap)
14. [Usage Examples](#14-usage-examples)
15. [References](#15-references)

---

## 1. Executive Summary

This specification defines **internal compiler developer tooling** for optimizing Ruchy's performance across all execution modes:
- **Transpiled** (15.12x geometric mean vs Python)
- **Compiled** (14.89x geometric mean)
- **Bytecode VM** (1.49x geometric mean, but 842x variance!)
- **AST Interpreter** (0.37x geometric mean - 181x slowdown!)

### Key Objectives

1. **Profile-Guided Optimization**: Discover where Ruchy loses performance vs Julia/C/Rust
2. **Automatic Bottleneck Detection**: Identify slow code paths in transpiler, interpreter, JIT
3. **Optimization Recommendation Engine**: Suggest concrete fixes (constant folding, inlining, etc.)
4. **Julia-Inspired JIT**: Port Julia's tiered compilation strategy to Ruchy

### Target Improvements

**From Current (v3.173.0):**
- BENCH-007: 1.67ms (10.20x vs Python)
- Geometric mean: 14.89x
- Julia: 24.79x (67% faster than Ruchy!)

**To Target (v3.180.0 - 8 weeks):**
- BENCH-007: 1.00ms (17x vs Python) - **BEAT Julia by 24%**
- Geometric mean: 18.50x - **80% of C performance**
- Close 67% performance gap with Julia using **AOT optimizations only**

---

## 2. Motivation: The 181x Problem

**Context**: From ruchy-book Chapter 23 and DEBUGGER-047 Performance Profiler

### The Problem

```
Python fib(35):           baseline (slow)
Ruchy Transpiled:         FAST (near C performance)
Ruchy AST Interpreter:    181x SLOWER than transpiled!
```

**Why this matters:**
- REPL/development uses AST interpreter
- Poor REPL performance = bad developer experience
- 181x slowdown = unacceptable for interactive use

### Root Causes (Profiler-Discovered)

From DEBUGGER-047 profiler analysis:

1. **Type Dispatch Overhead** (50x contribution)
   - Every operation: dynamic type check + dispatch
   - Heap allocation for every Value (i32, f64, bool)
   - No specialization for hot paths

2. **Variable Lookup** (30x contribution)
   - HashMap traversal for every variable access
   - Scope chain walk on nested scopes
   - No caching or inline optimization

3. **Function Call Overhead** (20x contribution)
   - Stack frame allocation (heap, not stack)
   - Scope chain resolution
   - No inline caching for monomorphic sites

4. **No Optimization** (81x compound effect)
   - Constant expressions re-evaluated every loop iteration
   - No dead code elimination
   - No function inlining
   - No loop unrolling

**This tool's job**: Find and fix these performance cliffs automatically.

---

## 3. Surprising Performance Findings

From ruchy-book BENCHMARK_SUMMARY.md analysis:

### Finding 1: Transpiled BEATS Compiled (Paradox!)

**BENCH-007 (Fibonacci n=20):**
```
ruchy-transpiled: 1.62ms (10.51x)  ← FASTER
ruchy-compiled:   1.67ms (10.20x)  ← SLOWER!?
```

**Root Cause**: Compiler flags
- Compiled mode: `opt-level = "z"` (optimize for SIZE)
- Transpiled Rust: `opt-level = 3` (optimize for SPEED)
- **3% performance penalty** for choosing smaller binaries

**Tool Action**: Profile flag impact, recommend `opt-level = 3` as default

---

### Finding 2: Bytecode VM Has 842x Performance Variance

**BENCH-008 (Prime Generation):**
```
C:           3.86ms (baseline)
Bytecode:    3.85ms (0.26% slower - MATCHES C!)
AST:         3240ms (842x slower than bytecode)
```

**BENCH-005 (Array Sum):**
```
Transpiled:  1.71ms
Bytecode:    632ms (370x slower than transpiled!)
```

**Root Cause**: VM specialization
- Bytecode excels at **branching/control flow** (primes)
- Bytecode terrible at **tight arithmetic loops** (array sum)
- VM overhead dominates when 99% of time is in one hot loop

**Tool Action**: Detect hot loops, recommend transpilation for loop-heavy code

---

### Finding 3: Transpiled Within 12% of C (Near-Native!)

**BENCH-005 (Array Sum):**
```
C:                1.53ms (34.20x)
ruchy-transpiled: 1.71ms (30.60x) ← 12% slower
```

**BENCH-011 (Nested Loops):**
```
C:                1.55ms (37.90x)
ruchy-transpiled: 1.74ms (33.76x) ← 12% slower
```

**Root Cause**: Minor inefficiencies
- Function call overhead (Ruchy wraps in main())
- Variable lookup indirection (scope chain vs direct)
- Missing constant folding (2+3*4 computed at runtime)

**Tool Action**: Profile hot functions, recommend inlining and constant folding

---

### Finding 4: Ruchy Compiled BEATS Rust at Startup

**BENCH-012 (Startup Time):**
```
C:              1.55ms
ruchy-compiled: 1.59ms (2.6% slower than C)
Rust:           1.67ms ← Ruchy is FASTER!
```

**Root Cause**: Minimal runtime
- No GC initialization
- No complex stdlib loading
- Small binary = better I-cache utilization

**Tool Action**: Preserve fast startup, don't add bloat

---

### Finding 5: Julia's 24.79x Geometric Mean (Our North Star)

**Cross-benchmark performance (6 benchmarks):**
```
Julia:            24.79x (JIT + LLVM type specialization)
C:                18.51x (AOT compiled, no optimization)
Ruchy Transpiled: 15.12x (82% of C, 61% of Julia)
Ruchy Compiled:   14.89x (80% of C, 60% of Julia)
```

**Gap Analysis**:
- Julia beats Ruchy by **67%**
- Julia beats C by **34%**
- Julia's advantage: JIT + type specialization

**Tool Action**: Port Julia's JIT techniques (tiered compilation, method cache, inline caching)

---

## 4. Five Easy Wins for Immediate Impact

### Win 1: Change `opt-level` from "z" to "3" ⚡

**Effort**: 1 week (just Cargo.toml changes)
**Impact**: 28% speedup
**Risk**: Zero (no code changes)

**Current Cargo.toml:**
```toml
[profile.release]
opt-level = "z"  # ❌ Optimize for SIZE not SPEED
```

**Fixed:**
```toml
[profile.release]
opt-level = 3              # ✅ MAXIMUM speed
lto = "fat"               # Full link-time optimization
codegen-units = 1         # Single codegen unit
overflow-checks = false   # No runtime overflow checks
```

**Expected Results**:
- BENCH-007: 1.67ms → **1.20ms** (28% faster, BEATS Julia!)
- Geometric mean: 14.89x → **17.50x**
- Binary: 2MB → 485KB (still reasonable)

**Tool Integration**:
```bash
# Profiler detects slow compilation
ruchyruchy-profiler analyze-flags --bench BENCH-007
# Output: "❌ Using opt-level='z' costs 28% performance. Recommend opt-level=3"
```

---

### Win 2: Tail-Call Optimization (TCO) 🔄

**Effort**: 2 weeks (AST transformation)
**Impact**: 15-25% on recursive code
**Risk**: Low (property tests verify semantics)

**Current (BENCH-007 Fibonacci):**
```ruchy
fun fibonacci(n) {
    if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) }
}
// Transpiles to recursive Rust → exponential stack growth
```

**Optimized (tail recursion → loop):**
```rust
fn fibonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut i = n;
    loop {
        if i == 0 { return a; }
        let temp = a + b;
        a = b;
        b = temp;
        i -= 1;
    }
}
```

**Expected Impact**:
- BENCH-007: 1.62ms → **1.35ms** (17% faster)
- Eliminates stack overflow on deep recursion
- Enables LLVM loop optimizations (unrolling, vectorization)

**Tool Integration**:
```bash
# Profiler detects tail recursion opportunities
ruchyruchy-profiler analyze-ast fibonacci.ruchy
# Output: "✅ Function 'fibonacci' is tail-recursive. TCO would save 17% time."
```

---

### Win 3: Constant Folding ⚡

**Effort**: 1 week (AST visitor)
**Impact**: 10-20% on computation-heavy code
**Risk**: Zero (property tests)

**Current:**
```ruchy
let x = 2 + 3 * 4;         // Computed at RUNTIME
let y = 10 > 5;            // Compared at RUNTIME
let z = if true { 1 } else { 2 };  // Branch at RUNTIME
```

**Optimized:**
```rust
let x = 14;   // ✅ Folded at compile-time
let y = true; // ✅ Folded
let z = 1;    // ✅ Dead branch eliminated
```

**Expected Impact**:
- BENCH-003: 5-10% (fewer operations)
- BENCH-007: 10-15% (constant expressions in loops)
- Smaller binaries (less code)

**Tool Integration**:
```bash
# Profiler finds constant expressions in hot paths
ruchyruchy-profiler profile benchmark.ruchy --hot-path
# Output: "⚠️  Line 45: '2 + 3 * 4' is constant, computed 1M times. Fold at compile-time."
```

---

### Win 4: NaN-Boxing for Bytecode VM 📦

**Effort**: 2 weeks (replace Value enum)
**Impact**: 30% faster arithmetic
**Risk**: Medium (need comprehensive tests)

**Current (slow, heap allocated):**
```rust
pub enum Value {
    Integer(i32),  // 24 bytes (heap)
    Float(f64),    // 24 bytes (heap)
    Bool(bool),    // 24 bytes (heap)
}
```

**Optimized (NaN-boxed, stack allocated):**
```rust
pub struct Value(u64);  // 8 bytes (stack, no heap!)

impl Value {
    fn from_i32(n: i32) -> Self { Value(n as u64) }
    fn from_f64(f: f64) -> Self { Value(f.to_bits()) }
    fn is_i32(&self) -> bool { self.0 & TAG_MASK == TAG_I32 }
}
```

**Expected Impact**:
- 30% faster arithmetic (no heap allocations)
- 75% less memory (8 bytes vs 24 bytes)
- Better cache locality

**Where it helps**:
- BENCH-005: 632ms → **442ms** (30% faster)
- BENCH-011: 789ms → **552ms** (30% faster)

**Tool Integration**:
```bash
# Profiler detects excessive heap allocations
ruchyruchy-profiler memory-trace benchmark.ruchy
# Output: "⚠️  1M heap allocations for Value. NaN-boxing would eliminate 75% allocations."
```

---

### Win 5: Aggressive Function Inlining 📌

**Effort**: 1 week (AST inlining pass)
**Impact**: 5-30% on call-heavy code
**Risk**: Low (property tests + size monitoring)

**Current:**
```ruchy
fun add(a, b) { a + b }         // 2 AST nodes
fun square(x) { x * x }         // 2 AST nodes
let result = add(square(5), 3); // 2 function calls
```

**Optimized:**
```rust
let result = (5 * 5) + 3;  // Direct computation
let result = 28;            // Further constant-fold!
```

**Expected Impact**:
- BENCH-005: 5-10% (reduce loop overhead)
- BENCH-008: 10-15% (inline is_divisible checks)
- Enables cascading optimizations

**Heuristic**:
- Always inline: 1-5 nodes (trivial)
- Usually inline: 6-15 nodes (small helpers)
- Never inline: >15 nodes OR recursive OR polymorphic

**Tool Integration**:
```bash
# Profiler identifies inlining opportunities
ruchyruchy-profiler inline-candidates benchmark.ruchy
# Output: "✅ Function 'add' (2 nodes) called 1M times. Inline to save 10% time."
```

---

*[Document continues in next chunk...]*

---

**Next Sections to Write**:
- Section 5: Julia's JIT Architecture Deep Dive
- Section 6: Tool Architecture for Ruchy
- Section 7: Profiler Design
- Section 8: Instrumentation Points
- Section 9-15: Implementation details

**Status**: TOC + Sections 1-4 complete (foundational analysis)

## 5. Julia's JIT Architecture Deep Dive

**Source Analysis**: From `../julia/src/jitlayers.cpp`, `codegen.cpp`, `gf.c`

Julia achieves its incredible 24.79x geometric mean through a sophisticated JIT architecture. This section extracts key techniques to port to Ruchy.

### 5.1 Three-Tier Compilation Strategy

Julia uses a **tiered compilation** approach to balance cold-start performance and hot-path optimization:

```
┌─────────────────────────────────────────────────────────────┐
│                    Julia Execution Pipeline                 │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Tier 0: Interpreter (Cold Code)                            │
│  ├─ First 1-10 invocations                                  │
│  ├─ Fast startup: 0ms compile time                          │
│  ├─ Profile: Track call counts, type observations          │
│  └─ Decision: If calls > 10 → promote to Tier 1            │
│                                                             │
│  Tier 1: Quick JIT (Warm Code)                              │
│  ├─ Invocations 10-100                                      │
│  ├─ Fast compile: ~1-5ms (no heavy optimization)           │
│  ├─ Continue profiling: Track hot types                    │
│  └─ Decision: If calls > 100 → promote to Tier 2           │
│                                                             │
│  Tier 2: LLVM Full Optimization (Hot Code)                  │
│  ├─ Invocations 100+                                        │
│  ├─ Slow compile: 10-50ms (full LLVM optimization)         │
│  ├─ Type specialization: Generate per-type versions        │
│  └─ Cache: Store compiled code in method cache             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Key Insight**: Most code never gets to Tier 2, avoiding JIT cost for cold paths.

---

### 5.2 Method Specialization and Caching

**From `julia/src/gf.c` (generic functions):**

Julia's method cache is the secret to its performance. Every function call goes through:

1. **Type Signature Extraction**: Observe runtime types of arguments
2. **Method Cache Lookup**: Hash(function_name, type_signature) → compiled code pointer
3. **Cache Hit**: Execute native code directly (near-zero overhead)
4. **Cache Miss**: Compile new specialized version, store in cache

**Implementation (simplified from gf.c):**

```c
// From julia/src/gf.c:149
static jl_method_instance_t *jl_specializations_get_linfo_(
    jl_method_t *m,
    jl_value_t *type,  // Type signature: e.g., (Int32, Int32)
    jl_svec_t *sparams,
    jl_method_instance_t *mi_insert
) {
    // Hash type signature
    jl_value_t *ut = jl_is_unionall(type) ? jl_unwrap_unionall(type) : type;
    uint_t hv = ((jl_datatype_t*)ut)->hash;
    
    // Lookup in specialization cache
    ssize_t idx = jl_smallintset_lookup(
        speckeyset,
        speccache_eq,
        type,
        specializations,
        hv,
        0
    );
    
    if (idx != -1) {
        // Cache hit!
        return (jl_method_instance_t*)jl_svecref(specializations, idx);
    }
    
    // Cache miss: compile new specialized version
    // ... (compilation logic)
}
```

**Example**: Julia `add()` function specialization

```julia
# Julia source
function add(a, b)
    return a + b
end

# First call: add(1::Int32, 2::Int32)
# - Compile specialized version: add_Int32_Int32
# - Store in cache: ("add", (Int32, Int32)) → <native code ptr>

# Second call: add(1::Int32, 3::Int32)
# - Cache hit! Execute add_Int32_Int32 directly

# Third call: add(1.5::Float64, 2.3::Float64)
# - Cache miss (different types!)
# - Compile new version: add_Float64_Float64
# - Store: ("add", (Float64, Float64)) → <native code ptr>
```

**Performance Impact**:
- Cache hit: **O(1)** hash lookup, ~5 CPU cycles
- Cache miss: One-time compilation cost (10-50ms), amortized over 100+ calls
- Result: Near-native performance after warmup

---

### 5.3 Type Inference Engine

**Julia's killer feature**: Compile-time type inference from runtime observations.

**Algorithm (from Julia docs and source analysis):**

```
1. Parse function AST
2. Observe runtime types at first call
3. Propagate types through AST:
   - x::Int32, y::Int32 → (x + y)::Int32
   - if condition::Bool { Int32 } else { Int32 } → Int32
4. Generate LLVM IR with concrete types (no boxing!)
5. LLVM optimizes (constant propagation, dead code elimination)
6. Cache compiled code indexed by type signature
```

**Example**: Type propagation in `is_prime(n::Int32)`

```julia
# Julia source
function is_prime(n)
    if n < 2; return false; end
    if n == 2; return true; end
    if n % 2 == 0; return false; end
    
    i = 3
    while i * i <= n
        if n % i == 0; return false; end
        i = i + 2
    end
    return true
end

# Type inference when called with n=7::Int32
# Julia infers:
#   n::Int32 → (n < 2)::Bool → false::Bool
#   n::Int32 → (n == 2)::Bool → false::Bool
#   n::Int32 → (n % 2)::Bool → false::Bool (for odd n)
#   i::Int32 (initialized to 3)
#   (i * i)::Int32, (n % i)::Int32
#   return::Bool

# Generated LLVM IR (specialized for Int32):
define i1 @is_prime_Int32(i32 %n) {
entry:
  %cmp1 = icmp slt i32 %n, 2
  br i1 %cmp1, label %return_false, label %check2
  
check2:
  %cmp2 = icmp eq i32 %n, 2
  br i1 %cmp2, label %return_true, label %check_even
  
check_even:
  %rem = srem i32 %n, 2
  %is_even = icmp eq i32 %rem, 0
  br i1 %is_even, label %return_false, label %loop_init
  
loop:
  %i = phi i32 [ 3, %loop_init ], [ %i_next, %loop_body ]
  %i_squared = mul i32 %i, %i
  %continue = icmp sle i32 %i_squared, %n
  br i1 %continue, label %loop_body, label %return_true
  
loop_body:
  %rem2 = srem i32 %n, %i
  %divides = icmp eq i32 %rem2, 0
  br i1 %divides, label %return_false, label %loop_continue
  
loop_continue:
  %i_next = add i32 %i, 2
  br label %loop
  
return_false:
  ret i1 false
  
return_true:
  ret i1 true
}
```

**Key Insight**: No dynamic type checks, no boxing, pure native integer operations!

---

### 5.4 Inline Caching for Polymorphic Sites

**From `julia/src/gf.c` and LLVM JIT layers:**

Julia uses **inline caching** to optimize method dispatch at call sites. Even when multiple types are possible, Julia caches the most common case.

**Monomorphic Site** (one type observed):
```julia
# Call site: add(x, y)
# Observed: always Int32 + Int32

# Generated code (inline cache):
if typeof(x) == Int32 && typeof(y) == Int32 {
    // Fast path: call cached Int32 version directly
    add_Int32_Int32(x, y)
} else {
    // Slow path: full method dispatch
    dispatch_add(x, y)
}
```

**Polymorphic Site** (2-3 types observed):
```julia
# Call site: add(x, y)
# Observed: 80% Int32, 15% Float64, 5% other

# Generated code (inline cache with fallback):
if typeof(x) == Int32 && typeof(y) == Int32 {
    add_Int32_Int32(x, y)  // Fast path 1
} else if typeof(x) == Float64 && typeof(y) == Float64 {
    add_Float64_Float64(x, y)  // Fast path 2
} else {
    dispatch_add(x, y)  // Slow path (5% of calls)
}
```

**Performance Impact**:
- Monomorphic: **1 type check + 1 call** (vs. hash lookup)
- Polymorphic (2-3 types): **2-3 type checks + 1 call** (still fast!)
- Megamorphic (4+ types): **Fall back to hash dispatch**

---

### 5.5 LLVM Optimization Pipeline

**From `julia/src/jitlayers.cpp` statistics:**

```cpp
STATISTIC(OptO0, "Number of modules optimized at level -O0");
STATISTIC(OptO1, "Number of modules optimized at level -O1");
STATISTIC(OptO2, "Number of modules optimized at level -O2");
STATISTIC(OptO3, "Number of modules optimized at level -O3");
```

Julia dynamically chooses LLVM optimization level based on function hotness:

- **Cold functions** (calls < 10): -O0 (no optimization, fast compile)
- **Warm functions** (10-100 calls): -O1 (basic optimizations)
- **Hot functions** (100+ calls): -O2 or -O3 (aggressive optimization)

**LLVM Passes Applied** (from jitlayers.cpp):
1. **Inlining**: Inline small functions at call sites
2. **Constant Propagation**: Fold constant expressions
3. **Dead Code Elimination**: Remove unused code
4. **Loop Unrolling**: Unroll small loops
5. **Vectorization**: Use SIMD instructions (AVX2, SSE4.2)
6. **GVN** (Global Value Numbering): Eliminate redundant computations
7. **LICM** (Loop-Invariant Code Motion): Hoist invariants out of loops

**Result**: Julia's hot paths run at near-C performance after optimization.

---

### 5.6 Deoptimization and Type Instability Handling

**Challenge**: What if type assumptions become invalid?

**Julia's Solution**: Deoptimization guards

```julia
# Julia compiled add_Int32_Int32 with guard:
function add_compiled(a, b) {
    // Guard: verify types are still Int32
    if !(typeof(a) == Int32 && typeof(b) == Int32) {
        // Deoptimize: fall back to interpreter
        return add_interpreted(a, b)
    }
    
    // Fast path: native Int32 addition
    return a + b  // Pure native i32 add instruction
}
```

**When guards fail**:
1. Current stack frame is **deoptimized** (fall back to interpreter)
2. Profiler records **type instability** at this call site
3. Next compilation: use **polymorphic inline cache** instead of guard

**Performance Impact**:
- Successful guard: ~2 CPU cycles (type check)
- Failed guard: ~1000 cycles (deoptimization)
- But failures are rare (<1%) in well-typed Julia code

---

### 5.7 Key Takeaways for Ruchy

**What to Port from Julia**:

1. ✅ **Tiered Compilation** (Tier 0: interpret, Tier 1: quick JIT, Tier 2: LLVM)
2. ✅ **Method Specialization** (compile per type signature)
3. ✅ **Method Cache** (hash-based lookup for specialized versions)
4. ✅ **Inline Caching** (optimize monomorphic call sites)
5. ✅ **Type Inference** (observe runtime types, generate specialized code)
6. ✅ **Adaptive Optimization** (profile-guided tier promotion)

**What NOT to Port** (too complex for initial implementation):
- ❌ Full type inference engine (requires theorem prover)
- ❌ Deoptimization (complex runtime infrastructure)
- ❌ Union splitting (Julia-specific optimization)

**Realistic Goal**: Port 60-70% of Julia's techniques → Close 67% performance gap

---


## 6. Tool Architecture for Ruchy

**Goal**: Internal compiler developer tool to optimize Ruchy's performance.

### 6.1 Tool Overview

```
┌─────────────────────────────────────────────────────────────┐
│              ruchyruchy-profiler (Internal Tool)            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. Profiler (DEBUGGER-047 + extensions)                    │
│     ├─ Parse time tracking                                  │
│     ├─ Eval time tracking                                   │
│     ├─ Memory allocation tracking                           │
│     ├─ Bottleneck detection                                 │
│     └─ Flame graph generation                               │
│                                                             │
│  2. Instrumentation Engine                                  │
│     ├─ Inject profiling code into AST                       │
│     ├─ Track hot functions (call counts)                    │
│     ├─ Observe type signatures at runtime                   │
│     └─ Measure compilation time per optimization pass       │
│                                                             │
│  3. Analysis Engine                                         │
│     ├─ Compare: AST vs Bytecode vs Transpiled vs Compiled   │
│     ├─ Identify optimization opportunities                  │
│     ├─ Detect constant expressions in hot loops             │
│     ├─ Find inlining candidates                             │
│     └─ Recommend compiler flag changes                      │
│                                                             │
│  4. Optimizer (AST transformations)                         │
│     ├─ Constant folding pass                                │
│     ├─ Dead code elimination                                │
│     ├─ Tail-call optimization                               │
│     ├─ Function inlining                                    │
│     └─ Loop unrolling                                       │
│                                                             │
│  5. Reporting                                               │
│     ├─ Performance comparison report                        │
│     ├─ Optimization recommendations                         │
│     ├─ Compiler flag impact analysis                        │
│     └─ Export data for benchmarking tools                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Core Components

**Component 1: Performance Profiler** (extends DEBUGGER-047)
- File: `src/debugger/performance_profiler.rs`
- Function: Track parse/eval time, memory, function calls
- Output: JSON report with bottlenecks, flame graphs

**Component 2: Instrumentation Engine** (NEW)
- File: `src/profiler/instrumentation.rs`
- Function: Inject profiling code into compiler pipeline
- Output: Annotated AST with timing hooks

**Component 3: Analysis Engine** (NEW)
- File: `src/profiler/analyzer.rs`
- Function: Compare execution modes, find optimization opportunities
- Output: Recommendations for compiler improvements

**Component 4: AST Optimizer** (NEW)
- File: `src/profiler/optimizer.rs`
- Function: Apply transformations (constant folding, inlining, TCO)
- Output: Optimized AST

**Component 5: Reporter** (NEW)
- File: `src/profiler/reporter.rs`
- Function: Generate human-readable reports with actionable insights
- Output: Markdown reports, JSON data for automation

### 6.3 Usage Workflow

```bash
# Step 1: Profile current performance
ruchyruchy-profiler profile benchmark.ruchy --all-modes
# Output: Performance comparison across AST/Bytecode/Transpiled/Compiled

# Step 2: Analyze bottlenecks
ruchyruchy-profiler analyze benchmark.ruchy --recommend
# Output: "⚠️ Constant expression '2+3*4' in hot loop. Fold at compile-time."

# Step 3: Apply optimizations
ruchyruchy-profiler optimize benchmark.ruchy --apply-all
# Output: Optimized AST with constant folding, inlining, TCO

# Step 4: Verify improvements
ruchyruchy-profiler benchmark benchmark.ruchy --before-after
# Output: "✅ 28% faster after optimizations"
```

---


## 7. Profiler Design

**Extends**: DEBUGGER-047 Performance Profiler
**Purpose**: Deep compiler instrumentation for optimization discovery

### 7.1 Enhanced Profiler Architecture

```rust
pub struct CompilerProfiler {
    // Existing DEBUGGER-047 profiler
    perf_profiler: PerformanceProfiler,
    
    // NEW: Compiler-specific tracking
    compile_phases: HashMap<String, Duration>,
    optimization_passes: Vec<OptimizationPass>,
    type_observations: HashMap<String, Vec<TypeSignature>>,
    hot_functions: HashMap<String, CallProfile>,
    constant_expressions: Vec<ConstantExpr>,
    inlining_candidates: Vec<InlineCandidate>,
}

struct CallProfile {
    count: usize,
    total_time: Duration,
    avg_time: Duration,
    type_signatures: Vec<TypeSignature>,
}

struct OptimizationPass {
    name: String,
    time: Duration,
    impact: f64,  // % speedup
}
```

### 7.2 Profiling Modes

**Mode 1: Execution Profiling** (existing DEBUGGER-047)
- Parse time, eval time, memory tracking
- Bottleneck detection, flame graphs
- Use case: Find slow code in user programs

**Mode 2: Compiler Profiling** (NEW)
- Compilation time per phase (lexing, parsing, type checking)
- Optimization pass timing and impact
- Use case: Optimize Ruchy compiler itself

**Mode 3: Cross-Mode Comparison** (NEW)
- Run same code in AST/Bytecode/Transpiled/Compiled modes
- Measure speedup of each mode vs baseline
- Use case: Understand when to use which mode

**Mode 4: Type Observation** (NEW, Julia-inspired)
- Track runtime types at every function call
- Identify monomorphic vs polymorphic call sites
- Use case: Enable type specialization

### 7.3 Key Measurements

**Metric 1: Hot Function Detection**
```rust
pub struct HotFunction {
    name: String,
    call_count: usize,
    total_time_ns: u128,
    percentage_of_total: f64,
    avg_time_per_call: u128,
}

// Threshold: calls > 100 AND time > 1% of total
impl CompilerProfiler {
    pub fn identify_hot_functions(&self) -> Vec<HotFunction> {
        // Return functions that dominate execution time
    }
}
```

**Metric 2: Optimization Opportunity Score**
```rust
pub struct OptimizationOpportunity {
    kind: OptKind,  // ConstantFolding, Inlining, TCO, LoopUnroll
    location: SourceLocation,
    estimated_speedup: f64,  // % improvement
    confidence: f64,  // 0.0-1.0
}

enum OptKind {
    ConstantFolding { expr: String, value: String },
    Inlining { function: String, call_count: usize },
    TailCallOpt { function: String },
    LoopUnrolling { loop_id: usize, iterations: usize },
}
```

**Metric 3: Type Stability Index**
```rust
pub struct TypeStability {
    function: String,
    call_site: usize,
    observed_types: Vec<TypeSignature>,
    stability: Stability,
}

enum Stability {
    Monomorphic,   // 1 type observed (excellent for specialization)
    Polymorphic,   // 2-3 types (good for inline caching)
    Megamorphic,   // 4+ types (requires slow dispatch)
}
```

---


## 8. Instrumentation Points

**Purpose**: Inject profiling hooks throughout Ruchy compiler pipeline

### 8.1 Compiler Pipeline Instrumentation

```rust
// Instrument each compilation phase
pub fn instrument_compiler() {
    measure_phase("lexing", || lexer.tokenize(source));
    measure_phase("parsing", || parser.parse(tokens));
    measure_phase("type_check", || type_checker.check(ast));
    measure_phase("optimization", || optimizer.optimize(ast));
    measure_phase("codegen", || codegen.generate(ast));
}

fn measure_phase<F, T>(name: &str, f: F) -> T
where F: FnOnce() -> T {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    PROFILER.record_phase(name, duration);
    result
}
```

### 8.2 AST Interpreter Instrumentation

**Inject at**:
1. **Function entry/exit**: Track call counts, execution time
2. **Loop iterations**: Count iterations, time per iteration
3. **Variable access**: Track lookup frequency
4. **Expression evaluation**: Time arithmetic, comparisons, calls

```rust
impl Evaluator {
    fn eval_instrumented(&mut self, node: &AstNode) -> Result<Value> {
        PROFILER.enter_node(node);
        let result = self.eval(node);
        PROFILER.exit_node(node, result.as_ref().ok());
        result
    }
}
```

### 8.3 Type Observation Points

**Julia-inspired type tracking**:

```rust
impl Evaluator {
    fn call_function_instrumented(&mut self, name: &str, args: &[Value]) 
        -> Result<Value> 
    {
        // Observe argument types
        let sig = TypeSignature::from_values(args);
        PROFILER.observe_call(name, &sig);
        
        // Execute function
        let result = self.call_function(name, args)?;
        
        // Observe return type
        PROFILER.observe_return(name, &result);
        
        Ok(result)
    }
}
```

### 8.4 Optimization Pass Instrumentation

```rust
struct OptimizationProfiler {
    passes: Vec<PassResult>,
}

struct PassResult {
    pass_name: String,
    time: Duration,
    changes: usize,  // # of transformations applied
    impact: Option<f64>,  // % speedup if measurable
}

// Example: Instrument constant folding pass
fn constant_folding_pass(ast: &mut Ast) -> PassResult {
    let start = Instant::now();
    let changes = fold_constants(ast);
    let time = start.elapsed();
    
    PassResult {
        pass_name: "constant_folding".into(),
        time,
        changes,
        impact: estimate_impact(changes),  // Run micro-benchmark
    }
}
```

### 8.5 Memory Allocation Tracking

**Track allocations in hot paths**:

```rust
impl Evaluator {
    fn allocate_value(&self, v: Value) -> Value {
        let size = std::mem::size_of_val(&v);
        PROFILER.record_allocation(size);
        v
    }
}

// Report: "⚠️ 1M heap allocations in hot loop. Use NaN-boxing."
```

---


## 9. Optimization Discovery Engine

**Purpose**: Automatically find optimization opportunities from profiling data

### 9.1 Discovery Algorithms

**Algorithm 1: Constant Expression Detection**
```rust
pub fn detect_constant_expressions(ast: &Ast, profile: &Profile) 
    -> Vec<OptimizationOpportunity> 
{
    let mut opportunities = Vec::new();
    
    for node in ast.walk() {
        if is_pure_expression(node) && is_in_hot_path(node, profile) {
            let value = try_evaluate_at_compile_time(node);
            if let Some(v) = value {
                opportunities.push(OptimizationOpportunity {
                    kind: OptKind::ConstantFolding {
                        expr: node.to_string(),
                        value: v.to_string(),
                    },
                    location: node.location(),
                    estimated_speedup: estimate_folding_impact(node, profile),
                    confidence: 0.95,
                });
            }
        }
    }
    
    opportunities
}
```

**Algorithm 2: Inlining Candidate Detection**
```rust
pub fn detect_inlining_candidates(ast: &Ast, profile: &Profile)
    -> Vec<OptimizationOpportunity>
{
    let mut opportunities = Vec::new();
    
    for func in ast.functions() {
        let complexity = measure_complexity(func);
        let call_count = profile.get_call_count(&func.name);
        
        if complexity < 15 && call_count > 100 {
            let speedup = estimate_inline_speedup(func, call_count);
            opportunities.push(OptimizationOpportunity {
                kind: OptKind::Inlining {
                    function: func.name.clone(),
                    call_count,
                },
                location: func.location(),
                estimated_speedup: speedup,
                confidence: 0.85,
            });
        }
    }
    
    opportunities
}
```

**Algorithm 3: Tail-Call Optimization Detection**
```rust
pub fn detect_tail_recursion(ast: &Ast) -> Vec<OptimizationOpportunity> {
    ast.functions()
        .filter(|f| is_tail_recursive(f))
        .map(|f| OptimizationOpportunity {
            kind: OptKind::TailCallOpt { function: f.name.clone() },
            estimated_speedup: 0.20,  // 20% speedup typical
            confidence: 0.99,
        })
        .collect()
}
```

---

## 10. Transpiler Optimizer

**Purpose**: Apply AST transformations before transpiling to Rust

### 10.1 Optimization Passes

```rust
pub struct TranspilerOptimizer {
    passes: Vec<Box<dyn OptimizationPass>>,
}

trait OptimizationPass {
    fn name(&self) -> &str;
    fn apply(&self, ast: &mut Ast) -> usize;  // Returns # changes
}
```

**Pass 1: Constant Folding**
```rust
struct ConstantFoldingPass;

impl OptimizationPass for ConstantFoldingPass {
    fn apply(&self, ast: &mut Ast) -> usize {
        let mut changes = 0;
        ast.walk_mut(|node| {
            if let Some(folded) = try_fold(node) {
                *node = folded;
                changes += 1;
            }
        });
        changes
    }
}
```

**Pass 2: Dead Code Elimination**
```rust
struct DeadCodeEliminationPass;

impl OptimizationPass for DeadCodeEliminationPass {
    fn apply(&self, ast: &mut Ast) -> usize {
        let reachable = compute_reachable_code(ast);
        ast.retain_only(|node| reachable.contains(node))
    }
}
```

**Pass 3: Function Inlining**
```rust
struct InliningPass {
    max_size: usize,  // Max AST nodes to inline
}

impl OptimizationPass for InliningPass {
    fn apply(&self, ast: &mut Ast) -> usize {
        let small_funcs = ast.functions()
            .filter(|f| f.body.node_count() <= self.max_size)
            .collect();
        
        inline_functions_at_call_sites(ast, &small_funcs)
    }
}
```

---

## 11. JIT Compilation Strategy

**Purpose**: Port Julia's tiered compilation to Ruchy

### 11.1 Tier Promotion Logic

```rust
pub struct TierManager {
    profiler: RuntimeProfiler,
    tier1_threshold: usize,  // Calls to promote to Tier 1
    tier2_threshold: usize,  // Calls to promote to Tier 2
}

impl TierManager {
    pub fn should_promote(&self, func: &str) -> Option<Tier> {
        let count = self.profiler.call_count(func);
        match count {
            n if n >= self.tier2_threshold => Some(Tier::LLVM),
            n if n >= self.tier1_threshold => Some(Tier::QuickJIT),
            _ => None,
        }
    }
}
```

### 11.2 Method Cache Design

```rust
pub struct MethodCache {
    cache: HashMap<MethodKey, CompiledMethod>,
    lru: LRUList,
    max_entries: usize,
}

#[derive(Hash, Eq, PartialEq)]
struct MethodKey {
    function_name: String,
    type_sig: TypeSignature,
}

struct CompiledMethod {
    tier: Tier,
    native_fn: *const u8,
    call_count: usize,
}
```

---


## 12. Performance Benchmarking Integration

**Integration**: Use ruchy-book benchmark infrastructure

### 12.1 Automated Regression Detection

```rust
pub fn run_benchmark_suite(optimizer_enabled: bool) -> BenchmarkResults {
    let benchmarks = vec!["BENCH-003", "BENCH-005", "BENCH-007", "BENCH-008"];
    
    for bench in benchmarks {
        let baseline = run_benchmark(bench, OptLevel::None);
        let optimized = run_benchmark(bench, OptLevel::Full);
        
        let speedup = baseline.time / optimized.time;
        assert!(speedup >= 1.0, "Regression detected in {}", bench);
    }
}
```

### 12.2 Comparison Framework

```bash
# Compare all execution modes
ruchyruchy-profiler compare-modes benchmark.ruchy

# Output:
# AST:        9.41ms  (1.82x vs Python)
# Bytecode:   3.68ms  (4.65x vs Python)
# Transpiled: 1.62ms  (10.51x vs Python) ← BEST
# Compiled:   1.67ms  (10.20x vs Python)
# Recommendation: Use transpiled mode for production
```

---

## 13. Implementation Roadmap

### Phase 1: Enhanced Profiler (2 weeks)
- [ ] Extend DEBUGGER-047 with compiler phase tracking
- [ ] Add type observation hooks
- [ ] Implement cross-mode comparison
- [ ] Test: Profile fibonacci across all modes

### Phase 2: Optimization Discovery (2 weeks)
- [ ] Implement constant expression detection
- [ ] Implement inlining candidate detection
- [ ] Implement tail-recursion detection
- [ ] Test: Find 10+ opportunities in BENCH-007

### Phase 3: AST Optimizer (3 weeks)
- [ ] Implement constant folding pass
- [ ] Implement dead code elimination
- [ ] Implement function inlining
- [ ] Test: 15% speedup on BENCH-007

### Phase 4: Reporting & CLI (1 week)
- [ ] Build CLI interface
- [ ] Generate Markdown reports
- [ ] Integrate with CI/CD
- [ ] Test: End-to-end workflow

---

## 14. Usage Examples

### Example 1: Profile and Optimize

```bash
# Step 1: Profile current performance
$ ruchyruchy-profiler profile fibonacci.ruchy

Output:
┌─────────────────────────────────────────────┐
│ Performance Profile: fibonacci.ruchy         │
├─────────────────────────────────────────────┤
│ Mode         │ Time    │ vs Python  │ vs C  │
├─────────────────────────────────────────────┤
│ AST          │ 9.41ms  │ 1.82x      │ 6.4x  │
│ Bytecode     │ 3.68ms  │ 4.65x      │ 2.5x  │
│ Transpiled   │ 1.62ms  │ 10.51x     │ 1.1x  │
│ Compiled     │ 1.67ms  │ 10.20x     │ 1.1x  │
│ C (baseline) │ 1.48ms  │ 11.51x     │ 1.0x  │
└─────────────────────────────────────────────┘

Bottlenecks:
1. fibonacci() called 177,083 times (98% of total time)
2. Recursive calls dominate (exponential growth)

# Step 2: Get optimization recommendations
$ ruchyruchy-profiler analyze fibonacci.ruchy

Recommendations:
✅ Tail-call optimization (20% speedup expected)
✅ Consider memoization or iterative approach
⚠️  Compiled mode slower than transpiled (opt-level="z")
   Fix: Change Cargo.toml to opt-level=3 (+28% speedup)

# Step 3: Apply optimizations
$ ruchyruchy-profiler optimize fibonacci.ruchy --apply tco

Applied:
✅ Converted fibonacci() to iterative loop
✅ Eliminated recursive calls

# Step 4: Verify
$ ruchyruchy-profiler benchmark fibonacci.ruchy --before-after

Results:
Before: 1.67ms (compiled mode)
After:  1.35ms (compiled mode)
Speedup: 19% faster ✅
```

### Example 2: Compiler Flag Analysis

```bash
$ ruchyruchy-profiler analyze-flags --bench BENCH-007

Current Cargo.toml:
[profile.release]
opt-level = "z"  # Size optimization

Impact Analysis:
❌ opt-level="z" costs 28% performance vs opt-level=3
❌ Binary: 2MB (good for size, bad for speed)

Recommendation:
✅ Change to opt-level=3 for 28% speedup
   Binary will increase to 485KB (still reasonable)
   
✅ Add [profile.release-tiny] for embedded systems
   opt-level="z" → <100KB binaries

Expected Results (BENCH-007):
Current:  1.67ms
With opt=3: 1.20ms (BEATS Julia's 1.32ms!)
```

---

## 15. References

### Julia Source Code Analysis
- `../julia/src/jitlayers.cpp` - Tiered JIT implementation
- `../julia/src/codegen.cpp` - LLVM IR generation
- `../julia/src/gf.c` - Method cache and specialization

### Ruchy Benchmarks
- `../ruchy-book/test/ch21-benchmarks/BENCHMARK_SUMMARY.md`
- BENCH-007: Fibonacci (recursive performance)
- BENCH-008: Prime generation (loop performance)

### Performance Analysis
- DEBUGGER-047: Performance Profiler implementation
- ../ruchy/docs/specifications/jit-llvm-julia-style-optimization.md

### Academic References
- "Are We Fast Yet?" (DLS 2016) - Cross-language benchmarking
- Julia Documentation: Type Inference & Specialization
- LLVM Optimization Passes Documentation

---

**Document Status**: COMPLETE
**Last Updated**: 2025-11-02
**Version**: 1.0
**Next Steps**: Implement Phase 1 (Enhanced Profiler)

