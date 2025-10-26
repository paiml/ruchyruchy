# Deep Bug & Performance Discovery Debugger Plan

**Project**: RuchyRuchy - Self-Hosting Compiler Bug Discovery & Performance Optimization
**Version**: 2.0.0 (Toyota Way Enhanced)
**Date**: October 26, 2025
**Status**: Specification v2.0 (Toyota Way Peer Review Applied)
**Reviewer**: Toyota Way Critical Analysis

---

## 🎯 Mission Statement

**Build a comprehensive bug discovery and performance optimization system** that systematically finds bugs in the Ruchy self-hosting compiler and language implementation, identifies performance bottlenecks, and produces actionable feedback for the upstream Ruchy team through:

**Outcome A**: Enhanced `ruchydbg` debugger with automated bug detection capabilities
**Outcome B**: Structured YAML reports (`docs/reports/*.yaml`) with actionable upstream feedback

### Long-Term Vision (10-Year Philosophy)

This system is not merely a one-time quality improvement effort but the foundation for **continuous evolution** of compiler quality assurance:

**Phase 1** (Current - 8 weeks): Automated Bug & Performance Discovery
- 8 discovery techniques finding bugs and bottlenecks
- Enhanced debugger with root cause analysis
- Actionable YAML reports for upstream team

**Phase 2** (6-12 months): Statistical Fault Localization
- Spectrum-based fault localization (Tarantula, Ochiai)
- Suspiciousness ranking for code elements
- Increased confidence in suggested fixes (>90%)

**Phase 3** (1-2 years): Automated Program Repair (APR)
- Template-based repair (GenProg, Prophet)
- Semantics-preserving transformations
- Automated patch generation for common bug classes
- Human-in-the-loop validation

**Phase 4** (3-5 years): Self-Healing Compiler
- Continuous integration of APR into development workflow
- Automatic bug fixing in CI/CD pipeline
- Feedback loop: fixes improve future discovery
- Zero-defect aspiration (Toyota Production System ideal)

**Philosophy**: Following Toyota's principle of "long-term philosophy, even at the expense of short-term financial goals," this system is designed to evolve over years, ultimately achieving a self-healing compiler that automatically discovers and repairs its own bugs.

---

## 📊 Research Foundation

### State-of-the-Art Compiler Testing (2024-2025)

Based on comprehensive research into modern compiler testing methodologies:

#### 1. **Creal** (PLDI 2024)
- **Approach**: Real-world code injection via function-level fusion
- **Key Innovation**: Leverages large volumes of real-world code to exercise rich language features
- **Effectiveness**: Triggers long-latent bugs in mature compilers (GCC)
- **Application to Ruchy**: Inject real Ruchy programs from ecosystem into bootstrap compiler tests

#### 2. **Rustlantis** (OOPSLA 2024)
- **Approach**: Randomized differential testing of Rust compiler
- **Key Innovation**: Generate MIR programs, compare across backends/optimization levels/Miri
- **Effectiveness**: Found 22 new bugs in rustc, 12 in heavily-fuzzed LLVM backend
- **Application to Ruchy**: Differential testing across Ruchy compiler stages and optimization levels

#### 3. **Rust-twins** (ASE 2024)
- **Approach**: Program mutation + dual macros generation with LLM assistance
- **Key Innovation**: Generate semantically equivalent programs via macros, compare HIR/MIR
- **Effectiveness**: 2x line coverage vs RustSmith, more crashes detected
- **Application to Ruchy**: Mutate Ruchy programs, verify AST/type inference equivalence

#### 4. **Metamorphic Testing**
- **Approach**: Solves oracle problem via metamorphic relations
- **Key Innovation**: Define properties that should hold across transformations
- **Effectiveness**: 60+ bugs in shader compilers, graphics pipelines
- **Application to Ruchy**: Define metamorphic properties for bootstrap chain

#### 5. **DIFFER** (2024)
- **Approach**: Differential + regression + fuzz testing combination
- **Key Innovation**: Test transformed programs for equivalence
- **Effectiveness**: Found failures in 71% of debloated programs
- **Application to Ruchy**: Test bootstrap stages preserve semantics

### Established Ruchy Bug Discovery Track Record

We've already discovered and documented **8+ critical bugs** through dogfooding:

1. **Boolean Negation Hang** (Issue #54) - OPEN ⭐ **HIGH SEVERITY**
2. **Variable Collision** (Issue #38) - FIXED in v3.98.0
3. **Enum Tuple Pattern Matching** - FIXED in v3.93.0
4. **String .nth() Method** - FIXED in v3.94.0
5. **Loop+Mut+Tuple Return** - FIXED in v3.95.0
6. **Box<T>/Vec<T> in Enums** - FIXED in v3.96.0
7. **Nested Match with Box** (Issue #39) - FIXED in v3.99.1
8. **Early Return Bug** (Issue #40) - FIXED in v3.100.0

**Success Rate**: 7/8 bugs fixed (87.5% resolution rate)

---

## 🏗️ System Architecture

### Two-Outcome Design

```
┌─────────────────────────────────────────────────────────────┐
│              Bug & Performance Discovery System              │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         Input: Ruchy Self-Hosting Compiler           │  │
│  │  - bootstrap/ (4 stages: lexer, parser, types, gen)  │  │
│  │  - validation/ (property, fuzz, boundary tests)      │  │
│  │  - Real-world Ruchy programs from ecosystem          │  │
│  └──────────────────────────────────────────────────────┘  │
│                            │                                 │
│                            ▼                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │        Discovery Techniques (8 strategies)            │  │
│  │  1. Differential Testing (stages, opt levels)         │  │
│  │  2. Metamorphic Testing (bootstrap chain)             │  │
│  │  3. Real-World Code Injection (Creal-style)           │  │
│  │  4. Mutation Testing (AST/HIR/MIR equivalence)        │  │
│  │  5. Fuzzing (grammar-based + mutation)                │  │
│  │  6. Property-Based Testing (invariants)               │  │
│  │  7. Performance Profiling (bottleneck detection)      │  │
│  │  8. Coverage-Guided Exploration (maximize paths)      │  │
│  └──────────────────────────────────────────────────────┘  │
│                            │                                 │
│                            ▼                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │             Bug & Performance Database                │  │
│  │  - Crash logs, hang detection, assertion failures     │  │
│  │  - Performance bottlenecks, hot paths                 │  │
│  │  - Minimized reproduction cases (delta debugging)     │  │
│  │  - Root cause analysis (bisection, tracing)           │  │
│  │  - **Deduplication Engine** (Muda elimination)        │  │
│  │    - Hash-based bug fingerprinting                    │  │
│  │    - AST structural similarity (>80% = duplicate)     │  │
│  │    - Root cause merge (same file/function/line)       │  │
│  └──────────────────────────────────────────────────────┘  │
│                            │                                 │
│              ┌─────────────┴───────────────┐                │
│              ▼                              ▼                │
│  ┌────────────────────────┐   ┌────────────────────────┐   │
│  │   Outcome A: ruchydbg  │   │  Outcome B: YAML       │   │
│  │   Enhanced Debugger    │   │  Reports (upstream)    │   │
│  │                        │   │                        │   │
│  │  - Auto bug detection  │   │  - docs/reports/       │   │
│  │  - Performance viz     │   │  - Actionable items    │   │
│  │  - Root cause hints    │   │  - Reproduction steps  │   │
│  │  - Bisection support   │   │  - Performance data    │   │
│  └────────────────────────┘   └────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔍 Discovery Techniques (8 Strategies)

### 1. Differential Testing

**Goal**: Find bugs by comparing output of semantically equivalent executions

**Approach**:
- **Cross-Stage Differential**: Compare Stage 0→1→2→3 bootstrap chain
- **Cross-Optimization**: Compare `-O0` vs `-O1` vs `-O2` vs `-O3`
- **Cross-Target**: Compare TypeScript output vs Rust output
- **Cross-Version**: Compare Ruchy v3.111.0 vs v3.110.0 vs v3.100.0

**Metamorphic Relations**:
1. `compile(P, opt=0)` and `compile(P, opt=3)` should have same semantics
2. `parse(emit(AST))` should equal `AST` (roundtrip property)
3. `typecheck(stage2(P))` should equal `typecheck(stage2(stage2(P)))` (idempotence)
4. Bootstrap chain: `stage3(stage2(stage1(stage0(P)))) ≡ production_compile(P)`

**Implementation**:
- Execute each Ruchy program with all combinations
- Compare outputs (stdout, return codes, memory usage)
- Flag any divergence as potential bug
- Minimize divergent case (delta debugging)

**Expected Bugs**:
- Optimizer introduces miscompilation
- Stage N incorrectly implements Stage N-1
- Target backend divergence (TypeScript vs Rust)

---

### 2. Metamorphic Testing

**Goal**: Solve oracle problem by defining properties that should hold across transformations

**Approach**:
- **Bootstrap Chain Property**: Self-compilation should be idempotent
- **Type Safety Property**: Well-typed programs should not crash
- **Determinism Property**: Same input → same output (every execution)
- **Commutativity Property**: Order of independent declarations shouldn't matter

**Metamorphic Relations**:

```yaml
# Example: Bootstrap Chain Idempotence
property: bootstrap_idempotence
description: "Compiling compiler N times should converge to fixed point"
relation: |
  Let C0 = production Ruchy compiler
  Let C1 = C0(bootstrap_compiler)
  Let C2 = C1(bootstrap_compiler)
  Let C3 = C2(bootstrap_compiler)

  Assert: C1 == C2 == C3 (bit-identical)

# Example: Type Safety
property: type_safety
description: "Well-typed programs do not crash"
relation: |
  For all P where typecheck(P) = success:
    execute(P) terminates with output or error (never crash/hang)

# Example: Optimization Commutativity
property: optimization_commutativity
description: "Optimization order shouldn't affect semantics"
relation: |
  optimize_dce(optimize_cf(P)) ≡ optimize_cf(optimize_dce(P))
```

**Implementation**:
- Define 20+ metamorphic properties
- Generate test cases exercising each property
- Detect violations (property failure = bug)
- Bisect to minimal failing case

**Expected Bugs**:
- Bootstrap divergence (compiler mutation over iterations)
- Type checker unsoundness (accepts invalid programs)
- Optimizer non-determinism (randomized algorithms)

---

### 3. Real-World Code Injection (Creal-Style)

**Goal**: Leverage real Ruchy programs to exercise rich language features

**Approach**:
- **Corpus Collection**: Gather all Ruchy code from:
  - `../ruchy` (production compiler)
  - `../ruchy-book` (examples)
  - `../ruchy-repl-demos` (REPL demonstrations)
  - `../paiml-mcp-agent-toolkit` (MCP servers)
  - RuchyRuchy bootstrap stages (dogfooding)
- **Function-Level Fusion**: Extract functions, inject into seed programs
- **Dynamic Analysis**: Track coverage, crashes, hangs

**Seed Program Template**:
```ruchy
// Seed: Basic program structure
fun main() {
    // INJECTION POINT: Real-world function calls
    let result = INJECTED_FUNCTION(INJECTED_ARGS);
    println("Result: {}", result);
}

// INJECTION POINT: Real-world function definition
INJECTED_FUNCTION_DEF
```

**Injection Strategy**:
1. Parse real-world Ruchy files
2. Extract all function definitions
3. Build type-compatible call graph
4. Inject chains of 2-5 function calls
5. Execute, monitor for crashes/hangs/assertions

**Implementation**:
- `real_world_corpus/` directory with 100+ Ruchy files
- `injector.ruchy` that performs AST-level injection
- `executor.ruchy` that runs injected programs with timeout
- Coverage tracking via `ruchy runtime --coverage`

**Expected Bugs**:
- Complex type inference failures
- Nested borrow checker edge cases
- Interaction bugs between language features

---

### 4. Mutation Testing (AST/HIR Equivalence)

**Goal**: Generate semantically equivalent programs via mutation, verify equivalence

**Approach** (inspired by Rust-twins):
- **Ruchy-Specific Mutators**:
  1. **Borrow Mutation**: `&x` ↔ `&mut x` (verify type checker catches)
  2. **Lifetime Mutation**: Add/remove lifetime annotations
  3. **Pattern Mutation**: `match` → `if let` → `while let`
  4. **Loop Mutation**: `for` ↔ `while` ↔ `loop`

- **General Mutators** (14 adapted for Ruchy):
  1. Expression reordering (commutative ops)
  2. Constant folding/unfolding
  3. Dead code insertion/removal
  4. Variable renaming (alpha-conversion)
  5. Function inlining/outlining
  6. Control flow restructuring
  7. Operator precedence changes
  8. Type annotation addition/removal
  9. Closure conversion (λ-lifting)
  10. Pattern match reordering
  11. Let-binding hoisting
  12. Tail call transformation
  13. Iterator fusion/defusion
  14. Macro expansion/contraction

**Dual Programs via Macros**:
```ruchy
// Original program
fun sum(xs: Vec<i32>) -> i32 {
    let mut total = 0;
    for x in xs {
        total = total + x;
    }
    total
}

// Dual macro: semantically equivalent
macro_rules! sum_dual {
    ($xs:expr) => {
        $xs.iter().fold(0, |acc, x| acc + x)
    }
}
```

**Verification**:
- Compare AST structure (isomorphic modulo renaming)
- Compare type inference results (identical types)
- Compare execution (identical outputs)
- **Divergence = Bug**

**Implementation**:
- 18 mutators (4 Ruchy-specific + 14 general)
- AST diff tool (structural comparison)
- Type inference diff (verify determinism)
- Execution diff (verify semantics)

**Expected Bugs**:
- Type inference non-determinism
- Optimizer over-optimization (changes semantics)
- Borrow checker unsoundness

---

### 5. Fuzzing (Grammar-Based + Mutation)

**Goal**: Generate valid/invalid Ruchy programs to find crashes/hangs

**Approach**:
- **Grammar-Based Fuzzing**:
  - Generate programs from Ruchy grammar
  - Ensure syntactic validity
  - Bias towards complex features (generics, lifetimes, traits)

- **Mutation-Based Fuzzing**:
  - Start with valid seed programs
  - Mutate at token/AST level
  - Mix valid and invalid mutations
  - Target boundaries (UB, edge cases)

**Fuzzing Strategies**:

```yaml
strategy: coverage_guided_fuzzing
approach: |
  1. Maintain corpus of interesting programs
  2. Mutate programs from corpus
  3. Execute with instrumentation
  4. Track coverage (line, branch, path)
  5. Add to corpus if new coverage
  6. Minimize corpus periodically

targets:
  - lexer: "Tokenization edge cases (unicode, escapes, comments)"
  - parser: "Syntax errors, recovery, ambiguity"
  - types: "Type inference failure modes, infinite types"
  - codegen: "Code generation crashes, incorrect output"
  - optimizer: "Miscompilation, performance regression"
```

**Implementation**:
- `fuzzer.ruchy` with 6 mutation strategies
- Coverage instrumentation via `ruchy runtime --coverage`
- Crash detection (exit codes, signals, timeouts)
- Corpus management (minimize, deduplicate)
- 24-hour fuzzing campaigns (1M+ test cases)

**Expected Bugs**:
- Parser crashes on malformed input
- Type checker infinite loops
- Code generator assertion failures
- Optimizer miscompilations

---

### 6. Property-Based Testing (Invariants)

**Goal**: Verify compiler invariants hold across all inputs

**Approach**:
- **Define 50+ Compiler Invariants**:
  - Type system: `Γ ⊢ e : τ → eval(e) : τ` (preservation)
  - Parser: `parse(emit(AST)) = AST` (roundtrip)
  - Optimizer: `semantics(optimize(P)) = semantics(P)` (equivalence)
  - Codegen: `execute(codegen(P)) = interpret(P)` (correctness)

**Property Examples**:

```ruchy
// Property: Type Preservation
property type_preservation() {
    forall program: Program {
        if let Some(typed_ast) = typecheck(program) {
            let result = execute(program);
            assert!(result.has_type(typed_ast.return_type));
        }
    }
}

// Property: Optimization Soundness
property optimization_soundness() {
    forall program: Program, opt_level: OptLevel {
        let original_output = execute(compile(program, opt_level=0));
        let optimized_output = execute(compile(program, opt_level));
        assert_eq!(original_output, optimized_output);
    }
}

// Property: Bootstrap Convergence
property bootstrap_convergence() {
    let C0 = production_compiler();
    let C1 = C0.compile(bootstrap_source);
    let C2 = C1.compile(bootstrap_source);
    let C3 = C2.compile(bootstrap_source);
    assert_eq!(C1, C2);  // Should converge by iteration 2
    assert_eq!(C2, C3);  // Fixed point
}
```

**Implementation**:
- 50+ properties defined in `validation/properties/`
- Property test framework in pure Ruchy
- Generate 10,000+ test cases per property
- Shrink failing cases to minimal counterexample
- Use `ruchy prove` for formal verification where possible

**Expected Bugs**:
- Type soundness violations
- Optimization introduces UB
- Bootstrap divergence (mutation over iterations)

---

### 7. Performance Profiling (Bottleneck Detection)

**Goal**: Identify performance bottlenecks in bootstrap compiler

**Approach**:
- **Compiler Profiling**:
  - Measure time spent in each compilation phase
  - Track memory allocations (heap, stack)
  - Identify hot functions (>10% time)
  - Detect quadratic algorithms (O(n²) scaling)

- **Runtime Profiling**:
  - Measure execution time of compiled code
  - Compare against production compiler output
  - Identify slow vs fast optimization levels
  - Track instruction counts (cache misses, branches)

**Profiling Tools**:

```yaml
tools:
  - ruchy_runtime:
      command: "ruchy runtime --profile"
      measures: ["time", "memory", "allocations", "hot_functions"]

  - perf_stat:
      command: "perf stat"
      measures: ["instructions", "cache_misses", "branch_mispredicts"]

  - flamegraph:
      command: "perf record && flamegraph.pl"
      output: "flamegraph.svg"

  - memory_profiler:
      command: "valgrind --tool=massif"
      output: "massif.out"
```

**Bottleneck Categories**:

1. **Algorithmic Complexity**:
   - O(n²) or worse algorithms
   - Unnecessary recomputation
   - Inefficient data structures

2. **Memory Issues**:
   - Excessive allocations
   - Memory fragmentation
   - Cache unfriendly access patterns

3. **I/O Bottlenecks**:
   - Repeated file reads
   - Inefficient parsing
   - Excessive string allocations

**Implementation**:
- Automated profiling in CI pipeline
- Profile 100+ real-world programs
- Generate performance reports
- Compare bootstrap vs production compiler
- Track performance regression over time

**Expected Findings**:
- Parser: Excessive string allocations
- Type checker: Quadratic constraint solving
- Code generator: Inefficient register allocation
- Optimizer: Redundant passes over IR

---

### 8. Coverage-Guided Exploration (Maximize Paths)

**Goal**: Maximize code coverage to find untested paths

**Approach**:
- **Coverage Metrics**:
  - Line coverage (executed lines)
  - Branch coverage (taken branches)
  - Path coverage (unique execution paths)
  - MC/DC coverage (decision coverage)

- **Guided Generation**:
  - Start with seed programs (high coverage)
  - Mutate to explore uncovered branches
  - Prioritize inputs that increase coverage
  - Track coverage over time

**Implementation**:

```yaml
coverage_strategy:
  instrumentation:
    tool: "ruchy runtime --coverage"
    format: "lcov"

  exploration:
    seed_corpus: "validation/**/*.ruchy"
    mutations: ["random", "grammar", "targeted"]
    target: ">95% line coverage, >85% branch coverage"

  reporting:
    format: "html"
    output: "coverage_report/"
    update_frequency: "hourly"
```

**Coverage Goals**:

| Component | Line Coverage | Branch Coverage | Path Coverage |
|-----------|---------------|-----------------|---------------|
| Lexer | >98% | >95% | >90% |
| Parser | >95% | >90% | >85% |
| Type Checker | >92% | >88% | >80% |
| Code Generator | >90% | >85% | >75% |
| Optimizer | >88% | >82% | >70% |

**Implementation**:
- Coverage tracking via `ruchy runtime --coverage`
- Visualize uncovered code (HTML reports)
- Generate tests targeting uncovered lines
- Continuous coverage monitoring (CI)

**Expected Findings**:
- Dead code (unreachable paths)
- Untested error handling
- Edge cases in type inference
- Missing optimization opportunities

---

## 📈 Outcome A: Enhanced `ruchydbg` Debugger

### New Features

#### 1. **Automatic Bug Detection Mode**

**Command**: `ruchydbg auto-detect <file.ruchy>`

**Capabilities**:
- Run all 8 discovery techniques automatically
- Detect crashes, hangs, assertion failures
- Minimize reproduction case (delta debugging)
- Generate bug report with root cause analysis

**Output**:
```yaml
bug_report:
  id: BUG-2025-001
  severity: high
  category: type_checker
  description: "Infinite loop in occurs check for recursive types"
  reproduction:
    file: "minimal_case.ruchy"
    command: "ruchy check minimal_case.ruchy"
    expected: "Type error"
    actual: "Hang (infinite loop)"
  root_cause: "Occurs check missing visited set for cycle detection"
  suggested_fix: "Add HashSet<TypeVar> to track visited type variables"
```

#### 2. **Performance Visualization**

**Command**: `ruchydbg perf-viz <file.ruchy>`

**Capabilities**:
- Display flamegraph of compilation phases
- Show memory allocation timeline
- Highlight hot functions (>10% time)
- Compare against production compiler

**Output**:
```
Compilation Profile for program.ruchy:
┌────────────────────────────────────────────────────┐
│ Phase           Time     Memory    Percentage       │
├────────────────────────────────────────────────────┤
│ Lexer           45ms     2.1MB     8.5%            │
│ Parser          120ms    8.3MB     22.6%           │
│ Type Checker    280ms    15.2MB    52.8% ⚠️ HOT   │
│ Code Generator  75ms     4.8MB     14.2%           │
│ Optimizer       10ms     1.2MB     1.9%            │
├────────────────────────────────────────────────────┤
│ TOTAL           530ms    31.6MB    100%            │
└────────────────────────────────────────────────────┘

Bottleneck: Type Checker (52.8%)
Recommendation: Profile occurs_check() and unify_types()
Comparison: Production compiler: 180ms (2.9x faster)
```

#### 3. **Differential Testing Mode**

**Command**: `ruchydbg diff <file.ruchy> --stages --opt-levels`

**Capabilities**:
- Run program through all bootstrap stages
- Compare output at each stage
- Test all optimization levels (-O0 to -O3)
- Detect divergence (potential bugs)

**Output**:
```
Differential Testing Results:
Program: fibonacci.ruchy

Stage Comparison:
✅ Stage 0 (Lexer) → Stage 1 (Parser): PASS
✅ Stage 1 (Parser) → Stage 2 (Type Checker): PASS
❌ Stage 2 (Type Checker) → Stage 3 (Code Gen): FAIL

   Divergence detected:
   - Stage 2 output: Type(i32)
   - Stage 3 output: Type(f64)  ⚠️ TYPE MISMATCH

   Root cause: Code generator incorrectly infers numeric literal type

Optimization Comparison:
✅ -O0 vs -O1: PASS (output identical)
❌ -O1 vs -O2: FAIL

   Divergence detected:
   - -O1 output: 55
   - -O2 output: 89  ⚠️ WRONG RESULT

   Root cause: Loop unrolling optimizer miscalculates bounds
```

#### 4. **Root Cause Analysis**

**Command**: `ruchydbg root-cause <bug_id>`

**Capabilities**:
- Bisect to find first bad commit (git bisect)
- Trace execution to pinpoint failure
- Suggest likely root cause
- Provide fix recommendations

**Output**:
```
Root Cause Analysis for BUG-2025-001:

Bisection Results:
- Last good commit: a3f55c0 (2025-10-25)
- First bad commit: 426e036 (2025-10-26)
- Commits in range: 8
- Bisection steps: 3

Implicated Code:
File: bootstrap/stage2/algorithm_w.ruchy
Function: occurs_check(var: TypeVar, type: Type) -> bool
Lines: 145-178

Root Cause Hypothesis:
The occurs_check function is missing cycle detection for recursive types.
When checking if type variable 'a appears in type 'μa.List<a>', it recurses
infinitely because it doesn't track visited type variables.

Suggested Fix:
```diff
-fun occurs_check(var: TypeVar, type: Type) -> bool {
+fun occurs_check(var: TypeVar, type: Type) -> bool {
+    occurs_check_impl(var, type, HashSet::new())
+}
+
+fun occurs_check_impl(var: TypeVar, type: Type, visited: HashSet<TypeVar>) -> bool {
+    if visited.contains(&var) { return false; }
     match type {
         Type::Var(v) => v == var,
-        Type::Fun(arg, ret) => occurs_check(var, *arg) || occurs_check(var, *ret),
+        Type::Fun(arg, ret) => {
+            let mut visited_next = visited.clone();
+            visited_next.insert(var);
+            occurs_check_impl(var, *arg, visited_next.clone()) ||
+            occurs_check_impl(var, *ret, visited_next)
+        },
         _ => false
     }
 }
```

Confidence: HIGH (95%)
Testing: Add test case for recursive types
```

#### 5. **Metamorphic Property Checker**

**Command**: `ruchydbg check-property <property_name> <file.ruchy>`

**Capabilities**:
- Verify specific metamorphic properties
- Test bootstrap chain idempotence
- Check type safety invariants
- Validate optimization soundness

**Properties Available**:
```
Available Properties:
1. bootstrap_idempotence - Compiler converges to fixed point
2. type_preservation - Well-typed programs preserve types
3. optimization_soundness - Optimizations preserve semantics
4. roundtrip_property - parse(emit(AST)) = AST
5. determinism - Same input always produces same output
6. commutativity - Independent declarations order-independent
... (50 total)

Usage:
  ruchydbg check-property bootstrap_idempotence bootstrap/
  ruchydbg check-property type_preservation examples/fibonacci.ruchy
  ruchydbg check-property all validation/  # Check all properties
```

---

## 📄 Outcome B: YAML Reports for Upstream Feedback

### Report Structure

All reports stored in `docs/reports/<category>/<report_id>.yaml`

#### Report Categories:

1. **Bugs** (`docs/reports/bugs/BUG-*.yaml`)
2. **Performance** (`docs/reports/performance/PERF-*.yaml`)
3. **Suggestions** (`docs/reports/suggestions/SUGG-*.yaml`)
4. **Regressions** (`docs/reports/regressions/REGR-*.yaml`)

### YAML Report Format

#### Bug Report Template

```yaml
# docs/reports/bugs/BUG-2025-001.yaml

report:
  id: BUG-2025-001
  type: bug
  category: type_checker
  severity: high
  status: open
  created: 2025-10-26T14:30:00Z
  updated: 2025-10-26T14:30:00Z

discovery:
  technique: metamorphic_testing
  property_violated: type_preservation
  discoverer: ruchydbg auto-detect
  ruchy_version: v3.111.0
  ruchyruchy_version: v1.0.0

description:
  summary: "Infinite loop in occurs check for recursive types"
  details: |
    When type checking recursive type definitions like `type List<T> = Cons(T, Box<List<T>>) | Nil`,
    the occurs check in the type inference algorithm enters an infinite loop.

    The occurs_check function recurses without tracking visited type variables,
    causing infinite recursion when checking if a type variable appears in a
    recursive type that references itself.

reproduction:
  minimal_case: |
    // File: minimal_case.ruchy
    type List<T> = Cons(T, Box<List<T>>) | Nil;

    fun create_list() -> List<i32> {
        Cons(1, Box::new(Nil))
    }

  steps:
    - "Save code above to minimal_case.ruchy"
    - "Run: ruchy check minimal_case.ruchy"
    - "Observe: Program hangs indefinitely"

  expected_behavior: |
    ruchy check minimal_case.ruchy
    ✅ Type checking succeeded
    Type: List<i32>

  actual_behavior: |
    ruchy check minimal_case.ruchy
    [hangs - no output, 100% CPU usage]

environment:
  os: Linux 6.8.0-85-generic
  ruchy_version: v3.111.0
  rust_version: 1.83.0
  install_method: cargo install ruchy

root_cause:
  analysis: |
    The occurs_check function in bootstrap/stage2/algorithm_w.ruchy (lines 145-178)
    is missing cycle detection for recursive types.

    Current implementation:
    ```ruchy
    fun occurs_check(var: TypeVar, type: Type) -> bool {
        match type {
            Type::Var(v) => v == var,
            Type::Fun(arg, ret) =>
                occurs_check(var, *arg) || occurs_check(var, *ret),
            _ => false
        }
    }
    ```

    When checking recursive type 'μa.List<a>', it recurses infinitely because
    it doesn't track visited type variables.

  implicated_code:
    file: "bootstrap/stage2/algorithm_w.ruchy"
    function: "occurs_check"
    lines: [145, 178]

  confidence: high  # Options: low, medium, high

suggested_fix:
  approach: "Add visited set to track type variables during occurs check"

  code: |
    // Add helper function with visited tracking
    fun occurs_check(var: TypeVar, type: Type) -> bool {
        occurs_check_impl(var, type, HashSet::new())
    }

    fun occurs_check_impl(var: TypeVar, type: Type, visited: HashSet<TypeVar>) -> bool {
        // If we've already visited this type variable, no cycle
        if visited.contains(&var) {
            return false;
        }

        match type {
            Type::Var(v) => v == var,
            Type::Fun(arg, ret) => {
                let mut visited_next = visited.clone();
                visited_next.insert(var);
                occurs_check_impl(var, *arg, visited_next.clone()) ||
                occurs_check_impl(var, *ret, visited_next)
            },
            Type::App(constructor, args) => {
                let mut visited_next = visited.clone();
                visited_next.insert(var);
                args.iter().any(|arg| occurs_check_impl(var, arg, visited_next.clone()))
            },
            _ => false
        }
    }

  test_case: |
    // Add to validation/stage2/test_occurs_check.ruchy
    #[test]
    fun test_occurs_check_recursive_types() {
        // Type: μa.List<a> = Cons(i32, Box<List<a>>) | Nil
        let type_var = TypeVar::fresh("a");
        let recursive_type = Type::App(
            "List",
            vec![Type::Var(type_var)]
        );

        // Should not hang, should return false (no occurrence)
        let result = occurs_check(type_var, recursive_type);
        assert!(!result, "Recursive type should not contain itself");
    }

  validation:
    - "Run: ruchy test validation/stage2/test_occurs_check.ruchy"
    - "Verify test passes"
    - "Run: ruchy check examples/recursive_types.ruchy"
    - "Verify no hang"
    - "Run full test suite: ruchy test validation/**/*.ruchy"

impact:
  severity_justification: |
    HIGH severity because:
    - Affects core type checking functionality
    - Causes infinite hang (100% CPU, no error message)
    - Blocks usage of recursive data types (common pattern)
    - No workaround available

  affected_users: "All users defining recursive data types (List, Tree, Graph, etc.)"

  workaround: "None available - recursive types are not usable"

metadata:
  tags: [type_checker, recursive_types, infinite_loop, occurs_check]
  priority: p0  # p0 (critical), p1 (high), p2 (medium), p3 (low)
  assignee: null
  milestone: v3.112.0

links:
  related_issues:
    - https://github.com/paiml/ruchy/issues/54  # Boolean negation hang

  related_commits: []

  references:
    - "Algorithm W paper: Damas & Milner (1982)"
    - "Occurs check with cycle detection: TAPL Chapter 22"

attachments:
  - name: "minimal_case.ruchy"
    path: "docs/reports/bugs/BUG-2025-001/minimal_case.ruchy"

  - name: "full_trace.log"
    path: "docs/reports/bugs/BUG-2025-001/full_trace.log"

  - name: "flamegraph.svg"
    path: "docs/reports/bugs/BUG-2025-001/flamegraph.svg"
```

#### Performance Report Template

```yaml
# docs/reports/performance/PERF-2025-001.yaml

report:
  id: PERF-2025-001
  type: performance
  category: type_checker
  severity: medium
  status: open
  created: 2025-10-26T15:00:00Z
  updated: 2025-10-26T15:00:00Z

discovery:
  technique: performance_profiling
  tool: "ruchy runtime --profile + perf stat"
  discoverer: ruchydbg perf-viz
  ruchy_version: v3.111.0
  ruchyruchy_version: v1.0.0

description:
  summary: "Type checker spends 52.8% of time in unify_types(), quadratic complexity"
  details: |
    Profiling the bootstrap compiler on 100 real-world Ruchy programs revealed
    that type checking is the primary bottleneck, consuming 52.8% of total
    compilation time.

    Within type checking, unify_types() accounts for 80% of the time, and
    exhibits O(n²) scaling behavior due to repeated linear scans through
    substitution maps.

benchmarks:
  programs_tested: 100
  total_compilation_time: "53.0s"
  type_checking_time: "28.0s"
  type_checking_percentage: 52.8%

  worst_cases:
    - program: "complex_generics.ruchy"
      lines: 500
      type_checking_time: "2.8s"
      compilation_time: "3.5s"
      percentage: 80.0%

    - program: "trait_heavy.ruchy"
      lines: 300
      type_checking_time: "1.5s"
      compilation_time: "2.0s"
      percentage: 75.0%

profiling_data:
  flamegraph: "docs/reports/performance/PERF-2025-001/flamegraph.svg"

  hot_functions:
    - function: "unify_types"
      time: "22.4s"
      percentage: 42.3%
      calls: 1_250_000
      time_per_call: "17.9µs"

    - function: "occurs_check"
      time: "3.2s"
      percentage: 6.0%
      calls: 850_000
      time_per_call: "3.8µs"

    - function: "apply_substitution"
      time: "2.4s"
      percentage: 4.5%
      calls: 2_100_000
      time_per_call: "1.1µs"

  complexity_analysis:
    unify_types:
      current_complexity: "O(n²)"
      reason: "Linear scan through substitution map on each unification"
      evidence: |
        Time scaling analysis:
        - 100 type variables:   10ms
        - 200 type variables:   42ms  (4.2x, not 2x)
        - 400 type variables:  168ms  (4.0x, not 2x)
        - 800 type variables:  672ms  (4.0x, not 2x)

        Expected for O(n): 2x per doubling
        Observed: 4x per doubling → O(n²)

root_cause:
  analysis: |
    The substitution map in unify_types() is implemented as a Vec<(TypeVar, Type)>,
    which requires O(n) time for lookup/insert operations.

    Current implementation (pseudocode):
    ```ruchy
    struct Subst {
        bindings: Vec<(TypeVar, Type)>  // ❌ O(n) lookup
    }

    fun apply_subst(subst: Subst, var: TypeVar) -> Option<Type> {
        for (v, t) in subst.bindings {  // ❌ Linear scan
            if v == var { return Some(t); }
        }
        None
    }
    ```

    Since unify_types() calls apply_subst() for each of n type variables,
    total complexity is O(n) × O(n) = O(n²).

  implicated_code:
    file: "bootstrap/stage2/unification.ruchy"
    struct: "Subst"
    lines: [45, 89]

  confidence: high

suggested_fix:
  approach: "Replace Vec with HashMap for O(1) lookup/insert"

  code: |
    // Change Subst to use HashMap
    struct Subst {
        bindings: HashMap<TypeVar, Type>  // ✅ O(1) lookup
    }

    fun apply_subst(subst: Subst, var: TypeVar) -> Option<Type> {
        subst.bindings.get(&var).cloned()  // ✅ O(1)
    }

    fun compose_subst(s1: Subst, s2: Subst) -> Subst {
        let mut result = s1.bindings.clone();
        for (var, type) in s2.bindings {
            result.insert(var, apply_subst_to_type(s1, type));  // ✅ O(1) insert
        }
        Subst { bindings: result }
    }

  expected_improvement:
    complexity: "O(n log n)"  # HashMap operations are O(log n) in Ruchy
    speedup_estimate: "10-50x for programs with 1000+ type variables"

  validation:
    - "Benchmark on 100 programs before/after"
    - "Verify all tests pass"
    - "Profile to confirm O(n log n) scaling"

comparison:
  production_compiler:
    type_checking_time: "18.0s"
    speedup_vs_bootstrap: "1.6x faster"
    reason: "Production compiler uses HashMap for substitutions"

  rustc:
    type_checking_approach: "Chalk (trait solver) with efficient unification"
    data_structure: "InferenceTable with O(1) lookup"

impact:
  severity_justification: |
    MEDIUM severity because:
    - Affects compilation time (UX issue, not correctness)
    - Noticeable on programs with many type variables (>100)
    - Has straightforward fix with large impact
    - Workaround: Use smaller files, fewer generics

  affected_users: "Users with large codebases or heavy use of generics"

metadata:
  tags: [type_checker, performance, quadratic_complexity, hash_map]
  priority: p1
  estimated_effort: "1-2 days"
  assignee: null
  milestone: v3.112.0

attachments:
  - name: "flamegraph.svg"
    path: "docs/reports/performance/PERF-2025-001/flamegraph.svg"

  - name: "benchmark_results.csv"
    path: "docs/reports/performance/PERF-2025-001/benchmark_results.csv"

  - name: "perf_stat_output.txt"
    path: "docs/reports/performance/PERF-2025-001/perf_stat_output.txt"
```

#### Suggestion Report Template

```yaml
# docs/reports/suggestions/SUGG-2025-001.yaml

report:
  id: SUGG-2025-001
  type: suggestion
  category: api_improvement
  severity: low
  status: open
  created: 2025-10-26T16:00:00Z
  updated: 2025-10-26T16:00:00Z

discovery:
  technique: real_world_code_injection
  observation: "Repeated pattern across 50+ real-world programs"
  discoverer: Creal-style corpus analysis
  ruchy_version: v3.111.0
  ruchyruchy_version: v1.0.0

description:
  summary: "Add Result::transpose() method for Option<Result<T, E>> conversion"
  details: |
    Analysis of 100+ real-world Ruchy programs from the ecosystem reveals a
    common pattern: converting Option<Result<T, E>> to Result<Option<T>, E>.

    This pattern appears 50+ times across projects, with developers writing
    manual match expressions each time. A standard library method would
    improve ergonomics and reduce boilerplate.

usage_pattern:
  frequency: "50+ occurrences in 100 programs analyzed"

  current_approach: |
    // Current: Manual match expression (boilerplate)
    let option_result: Option<Result<i32, String>> = Some(Ok(42));
    let result_option: Result<Option<i32>, String> = match option_result {
        Some(Ok(value)) => Ok(Some(value)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    };

  proposed_approach: |
    // Proposed: Standard library method (ergonomic)
    let option_result: Option<Result<i32, String>> = Some(Ok(42));
    let result_option: Result<Option<i32>, String> = option_result.transpose();

real_world_examples:
  - project: "../ruchy (production compiler)"
    file: "src/parser/mod.ruchy"
    lines: [245, 289, 312]
    use_case: "Parsing optional configuration values that may fail"

  - project: "../paiml-mcp-agent-toolkit"
    file: "src/tools/filesystem.ruchy"
    lines: [128, 156]
    use_case: "Optional file operations that may return errors"

  - project: "RuchyRuchy bootstrap"
    file: "bootstrap/stage1/parser.ruchy"
    lines: [167, 203, 245]
    use_case: "Optional AST nodes with validation"

proposed_implementation:
  signature: |
    impl<T, E> Option<Result<T, E>> {
        fun transpose(self) -> Result<Option<T>, E> {
            match self {
                Some(Ok(value)) => Ok(Some(value)),
                Some(Err(e)) => Err(e),
                None => Ok(None),
            }
        }
    }

  tests: |
    #[test]
    fun test_transpose_some_ok() {
        let input: Option<Result<i32, String>> = Some(Ok(42));
        let output: Result<Option<i32>, String> = input.transpose();
        assert_eq!(output, Ok(Some(42)));
    }

    #[test]
    fun test_transpose_some_err() {
        let input: Option<Result<i32, String>> = Some(Err("error".to_string()));
        let output: Result<Option<i32>, String> = input.transpose();
        assert_eq!(output, Err("error".to_string()));
    }

    #[test]
    fun test_transpose_none() {
        let input: Option<Result<i32, String>> = None;
        let output: Result<Option<i32>, String> = input.transpose();
        assert_eq!(output, Ok(None));
    }

  documentation: |
    /// Transposes an `Option` of a `Result` into a `Result` of an `Option`.
    ///
    /// `None` will be mapped to `Ok(None)`.
    /// `Some(Ok(_))` and `Some(Err(_))` will be mapped to
    /// `Ok(Some(_))` and `Err(_)`.
    ///
    /// # Examples
    ///
    /// ```ruchy
    /// let x: Result<Option<i32>, String> = Ok(Some(5));
    /// let y: Option<Result<i32, String>> = Some(Ok(5));
    /// assert_eq!(x, y.transpose());
    /// ```

comparison:
  rust: |
    // Rust has Option::transpose() since 1.33.0
    impl<T, E> Option<Result<T, E>> {
        pub fn transpose(self) -> Result<Option<T>, E>
    }

  haskell: |
    -- Haskell has sequence :: Monad m => t (m a) -> m (t a)
    sequence :: Option (Result T E) -> Result (Option T) E

  other_languages: "F#, Scala, OCaml all provide similar combinators"

benefits:
  ergonomics: "Reduces 7-line match to 1-line method call"
  readability: "Intent is clear from method name"
  consistency: "Matches Rust standard library API"
  correctness: "Standard library implementation tested once, reused everywhere"

metadata:
  tags: [api_improvement, ergonomics, standard_library, option, result]
  priority: p2
  estimated_effort: "4-8 hours (implementation + tests + docs)"
  assignee: null
  milestone: v3.113.0

  related_methods:
    - "Result::ok() -> Option<T>"
    - "Result::err() -> Option<E>"
    - "Option::ok_or(E) -> Result<T, E>"
    - "Option::ok_or_else(FnOnce() -> E) -> Result<T, E>"

links:
  rust_reference: "https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose"
  haskell_reference: "https://hackage.haskell.org/package/base/docs/Data-Traversable.html#v:sequence"

attachments:
  - name: "usage_analysis.csv"
    path: "docs/reports/suggestions/SUGG-2025-001/usage_analysis.csv"
    description: "50+ occurrences across 100 programs"
```

---

## 🗺️ Implementation Roadmap

### Roadmap Integration (roadmap.yaml)

Add new sprint to `roadmap.yaml`:

```yaml
# SPRINT 10: Deep Bug & Performance Discovery (Week 21-28)
- id: sprint-10
  name: "Bug & Performance Discovery System"
  goal: "Build comprehensive bug discovery system for self-hosting compiler"
  duration: 8_weeks
  tickets:
    - id: DISCOVERY-001
      title: "Discovery Infrastructure & Framework"
      priority: critical
      status: pending
      requirements:
        - "8 discovery techniques framework"
        - "Report generation (YAML)"
        - "Database for bugs/performance"
        - "Integration with ruchydbg"
      tests:
        - "test_framework_architecture.ruchy"
        - "test_report_generation.ruchy"
        - "test_database_storage.ruchy"
      acceptance:
        - "Framework can run all 8 techniques"
        - "YAML reports generated correctly"
        - "Database stores all findings"

    - id: DISCOVERY-002
      title: "Differential Testing Implementation"
      priority: critical
      status: pending
      requirements:
        - "Cross-stage differential (Stage 0-3)"
        - "Cross-optimization (-O0 to -O3)"
        - "Cross-target (TS vs Rust)"
        - "Divergence detection & minimization"
      tests:
        - "test_differential_stages.ruchy (15 tests)"
        - "test_differential_opt_levels.ruchy (12 tests)"
        - "test_differential_targets.ruchy (10 tests)"
        - "property_test_differential.ruchy (10k cases)"
      acceptance:
        - "Detects known divergence (validation)"
        - "Minimizes to <20 LOC reproduction"
        - "Generates YAML report with root cause"

    - id: DISCOVERY-003
      title: "Metamorphic Testing Implementation"
      priority: critical
      status: pending
      requirements:
        - "20+ metamorphic properties defined"
        - "Bootstrap chain idempotence"
        - "Type safety property"
        - "Optimization soundness property"
      tests:
        - "test_metamorphic_properties.ruchy (20 properties)"
        - "test_bootstrap_convergence.ruchy (5 tests)"
        - "property_test_metamorphic.ruchy (50k cases)"
      acceptance:
        - "All 20 properties can be checked"
        - "Bootstrap converges in ≤3 iterations"
        - "Finds at least 1 new bug"

    - id: DISCOVERY-004
      title: "Real-World Code Injection (Creal-style)"
      priority: high
      status: pending
      requirements:
        - "Corpus collection (100+ programs)"
        - "Function-level extraction"
        - "Type-compatible injection"
        - "Coverage tracking"
      tests:
        - "test_corpus_collection.ruchy (5 tests)"
        - "test_function_extraction.ruchy (8 tests)"
        - "test_injection_engine.ruchy (12 tests)"
        - "fuzz_test_injection.ruchy (100k cases)"
      acceptance:
        - "Corpus has 100+ programs"
        - "Can inject 1000+ function combinations"
        - "Achieves >90% line coverage"

    - id: DISCOVERY-005
      title: "Mutation Testing (AST/HIR Equivalence)"
      priority: high
      status: pending
      requirements:
        - "18 mutators (4 Ruchy + 14 general)"
        - "AST diff tool"
        - "Type inference diff"
        - "Execution diff (semantic)"
      tests:
        - "test_mutators.ruchy (18 tests, 1 per mutator)"
        - "test_ast_diff.ruchy (10 tests)"
        - "test_type_diff.ruchy (8 tests)"
        - "property_test_mutations.ruchy (25k cases)"
      acceptance:
        - "All 18 mutators work correctly"
        - "AST diff detects structural changes"
        - "Finds at least 2 new bugs"

    - id: DISCOVERY-006
      title: "Fuzzing (Grammar + Mutation)"
      priority: high
      status: pending
      requirements:
        - "Grammar-based generator"
        - "Mutation-based fuzzer"
        - "Coverage tracking"
        - "Crash/hang detection"
      tests:
        - "test_grammar_fuzzer.ruchy (10 tests)"
        - "test_mutation_fuzzer.ruchy (8 tests)"
        - "test_coverage_tracking.ruchy (6 tests)"
        - "fuzz_test_compiler.ruchy (1M cases, 24-hour run)"
      acceptance:
        - "Generates 10k valid programs/hour"
        - "Achieves >85% branch coverage"
        - "Detects all known crash bugs"

    - id: DISCOVERY-007
      title: "Property-Based Testing (50+ Invariants)"
      priority: high
      status: pending
      requirements:
        - "50+ compiler invariants defined"
        - "Property test framework"
        - "Shrinking for minimal counterexamples"
        - "Integration with ruchy prove"
      tests:
        - "test_properties.ruchy (50 properties)"
        - "test_shrinking.ruchy (8 tests)"
        - "property_test_all.ruchy (500k cases, 10k per property)"
      acceptance:
        - "All 50 properties can be tested"
        - "Shrinking reduces cases to <50 LOC"
        - "Finds at least 3 new bugs"

    - id: DISCOVERY-008
      title: "Performance Profiling (Bottleneck Detection)"
      priority: high
      status: pending
      requirements:
        - "Compiler profiling (time, memory)"
        - "Hot function detection (>10% time)"
        - "Complexity analysis (O(n²) detection)"
        - "Flamegraph generation"
      tests:
        - "test_profiling.ruchy (8 tests)"
        - "test_hot_function_detection.ruchy (6 tests)"
        - "test_complexity_analysis.ruchy (10 tests)"
        - "benchmark_100_programs.ruchy"
      acceptance:
        - "Profiles 100 programs successfully"
        - "Detects known bottlenecks (validation)"
        - "Generates flamegraph and perf report"

    - id: DISCOVERY-009
      title: "Coverage-Guided Exploration"
      priority: medium
      status: pending
      requirements:
        - "Coverage instrumentation"
        - "Guided mutation (target uncovered)"
        - "Coverage visualization (HTML)"
        - "Continuous monitoring (CI)"
      tests:
        - "test_coverage_tracking.ruchy (8 tests)"
        - "test_guided_mutation.ruchy (10 tests)"
        - "test_coverage_reporting.ruchy (6 tests)"
      acceptance:
        - "Achieves >95% line coverage on lexer"
        - "Achieves >90% branch coverage on parser"
        - "Generates HTML coverage report"

    - id: DISCOVERY-010
      title: "ruchydbg Enhancement: Auto-Detect Mode"
      priority: critical
      status: pending
      requirements:
        - "Auto-detect command"
        - "Run all 8 techniques automatically"
        - "Delta debugging (minimize cases)"
        - "Root cause analysis"
      tests:
        - "test_auto_detect.ruchy (10 tests)"
        - "test_delta_debugging.ruchy (8 tests)"
        - "test_root_cause.ruchy (12 tests)"
      acceptance:
        - "Detects all known bugs (validation)"
        - "Minimizes to <20 LOC"
        - "Provides actionable root cause"

    - id: DISCOVERY-011
      title: "ruchydbg Enhancement: Performance Visualization"
      priority: high
      status: pending
      requirements:
        - "perf-viz command"
        - "Flamegraph integration"
        - "Memory timeline"
        - "Comparison with production compiler"
      tests:
        - "test_perf_viz.ruchy (8 tests)"
        - "test_flamegraph_generation.ruchy (6 tests)"
        - "test_comparison.ruchy (10 tests)"
      acceptance:
        - "Generates accurate flamegraph"
        - "Identifies top 5 hot functions"
        - "Compares correctly vs production"

    - id: DISCOVERY-012
      title: "ruchydbg Enhancement: Differential Mode"
      priority: high
      status: pending
      requirements:
        - "diff command"
        - "Stage comparison"
        - "Opt level comparison"
        - "Divergence reporting"
      tests:
        - "test_diff_mode.ruchy (10 tests)"
        - "test_stage_comparison.ruchy (8 tests)"
        - "test_divergence_detection.ruchy (12 tests)"
      acceptance:
        - "Detects all known divergences"
        - "Reports clear diff output"
        - "Generates YAML report"

    - id: DISCOVERY-013
      title: "ruchydbg Enhancement: Root Cause Analysis"
      priority: high
      status: pending
      requirements:
        - "root-cause command"
        - "Git bisection integration"
        - "Execution tracing"
        - "Fix suggestions (code diffs)"
      tests:
        - "test_root_cause.ruchy (12 tests)"
        - "test_bisection.ruchy (8 tests)"
        - "test_fix_suggestions.ruchy (10 tests)"
      acceptance:
        - "Bisects to correct commit"
        - "Provides accurate root cause (>80%)"
        - "Suggests compilable fix"

    - id: DISCOVERY-014
      title: "ruchydbg Enhancement: Property Checker"
      priority: medium
      status: pending
      requirements:
        - "check-property command"
        - "50+ properties available"
        - "Property violation reporting"
        - "Integration with prove"
      tests:
        - "test_property_checker.ruchy (10 tests)"
        - "test_all_properties.ruchy (50 tests)"
      acceptance:
        - "All 50 properties can be checked"
        - "Detects property violations"
        - "Generates YAML report"

    - id: DISCOVERY-015
      title: "YAML Report Generation System"
      priority: critical
      status: pending
      requirements:
        - "Bug report template (YAML)"
        - "Performance report template (YAML)"
        - "Suggestion report template (YAML)"
        - "Automated report generation"
      tests:
        - "test_yaml_generation.ruchy (12 tests)"
        - "test_report_validation.ruchy (8 tests)"
        - "test_schema_compliance.ruchy (10 tests)"
      acceptance:
        - "Generates valid YAML (schema validated)"
        - "All fields populated correctly"
        - "Reports are actionable (manual review)"

    - id: DISCOVERY-016
      title: "Integration Testing & CI Pipeline"
      priority: high
      status: pending
      requirements:
        - "Run discovery on every commit (CI)"
        - "Track bug discovery over time"
        - "Performance regression detection"
        - "Automated upstream reporting"
      tests:
        - "test_ci_integration.ruchy (8 tests)"
        - "test_regression_detection.ruchy (10 tests)"
      acceptance:
        - "CI runs all 8 techniques on every PR"
        - "Detects regressions within 1 commit"
        - "Generates daily summary report"

    - id: DISCOVERY-017
      title: "Documentation & User Guide"
      priority: medium
      status: pending
      requirements:
        - "Complete specification documentation"
        - "ruchydbg user guide"
        - "YAML report guide for upstream"
        - "Case studies (bugs found)"
      tests:
        - "test_documentation_completeness.ruchy"
        - "test_examples_executable.ruchy"
      acceptance:
        - "All commands documented"
        - "All examples work"
        - "Upstream team can use reports"
```

---

## 📏 Success Metrics

### Bug Discovery

| Metric | Target | Measurement |
|--------|--------|-------------|
| **New bugs found** | ≥10 new bugs | Count of unique bugs discovered |
| **Bug severity** | ≥3 high/critical | Severity classification |
| **Reproduction rate** | 100% reproducible | All bugs have minimal reproduction |
| **Upstream acceptance** | ≥80% fixed | % of bugs fixed by Ruchy team |

### Performance Discovery

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Bottlenecks identified** | ≥5 major | Functions consuming >10% time |
| **Speedup potential** | ≥2x aggregate | Estimated speedup from all fixes |
| **Coverage achieved** | >95% line, >85% branch | Coverage metrics on bootstrap |
| **Profiling overhead** | <10% | Slowdown when profiling enabled |

### Quality Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Test coverage** | >90% | Coverage of discovery code itself |
| **SATD** | 0 | Zero TODO/FIXME/HACK |
| **Lint grade** | A+ | ruchy lint --grade |
| **TDG score** | >85 | pmat TDG score |
| **Mutation score** | >80% | % of mutations killed by tests |

### Reporting Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Reports generated** | 50+ YAML files | Count of bug/perf/suggestion reports |
| **Report quality** | 100% schema-valid | All YAML validates against schema |
| **Actionability** | >90% actionable | Manual review of report quality |
| **Upstream usage** | ≥20 reports used | Ruchy team acts on reports |

---

## 🛠️ Technology Stack

### Pure Ruchy Implementation (Dogfooding)

All discovery code written in **pure Ruchy** to dogfood the language:

```yaml
implementation:
  language: Ruchy v3.111.0+
  tools:
    - "ruchy check" # Syntax/type checking
    - "ruchy test"  # Test execution
    - "ruchy lint"  # Code quality (A+)
    - "ruchy fmt"   # Code formatting
    - "ruchy prove" # Formal verification
    - "ruchy score" # Quality metrics
    - "ruchy runtime" # Performance profiling

  quality_enforcement:
    - PMAT TDG score >85
    - Zero SATD tolerance
    - Extreme TDD (RED-GREEN-REFACTOR-TOOL)
    - Pre-commit hooks (blocking)
```

### External Tools (Integration Only)

For performance profiling and comparison:

```yaml
external_tools:
  profiling:
    - perf: "Linux perf_events for CPU profiling"
    - flamegraph: "Flamegraph generation from perf data"
    - valgrind: "Memory profiling (massif)"

  analysis:
    - llvm-cov: "Coverage data collection"
    - lcov: "Coverage report generation"

  validation:
    - schema: "YAML schema validation"
    - git: "Bisection, history analysis"
```

---

## 📅 Timeline: Iterative 3-Cycle Approach (24 Weeks Total)

**Toyota Way Principle**: Level out workload (Heijunka) to avoid overburden (Muri) and unevenness (Mura)

### Rationale for Iterative Approach

The original 8-week timeline risked **Muri** (overburden) by attempting to implement 8 state-of-the-art techniques simultaneously. Following the Toyota Way principle of **Heijunka** (leveling), we adopt a **3-cycle iterative approach**:

- **Cycle 1 (8 weeks)**: Core high-impact techniques + foundational tooling
- **Cycle 2 (8 weeks)**: Advanced techniques + enhanced automation
- **Cycle 3 (8 weeks)**: Comprehensive integration + long-term evolution

Each cycle delivers **production value** and provides **feedback** for subsequent cycles, embodying **Kaizen** (continuous improvement).

---

### Cycle 1: Core Discovery & Foundational Tools (8 Weeks)

**Goal**: Deliver high-impact bug discovery with minimal viable tooling

**Success Criteria**:
- ≥5 new bugs discovered
- 20+ YAML reports generated
- ruchydbg with auto-detect and differential modes
- CI/CD integration operational

#### Week 1-2: Foundation (DISCOVERY-001, DISCOVERY-015)
- Build framework for discovery orchestration
- Implement YAML report generation system
- Database for bugs/performance (with deduplication engine)
- Integration with ruchydbg skeleton

#### Week 3-4: Core Discovery - Differential & Metamorphic (DISCOVERY-002, DISCOVERY-003)
- **DISCOVERY-002**: Differential Testing
  - Cross-stage (Stage 0→1→2→3)
  - Cross-optimization (-O0 vs -O3)
  - Cross-target (TypeScript vs Rust)
  - Expected: 2-3 new bugs (optimizer miscompilations)

- **DISCOVERY-003**: Metamorphic Testing
  - Bootstrap chain idempotence
  - Type safety property
  - Optimization soundness
  - Expected: 2-3 new bugs (type checker, bootstrap divergence)

#### Week 5-6: Fuzzing (DISCOVERY-006)
- **DISCOVERY-006**: Grammar-based + mutation-based fuzzing
  - Coverage-guided fuzzing (1M+ test cases)
  - Crash/hang detection
  - Expected: 3-5 new bugs (parser crashes, type checker hangs)

#### Week 7: ruchydbg Enhancements - Core Modes (DISCOVERY-010, DISCOVERY-012)
- **DISCOVERY-010**: Auto-detect mode
  - Run differential + metamorphic + fuzzing automatically
  - Delta debugging (minimize cases)
  - Root cause hints

- **DISCOVERY-012**: Differential mode
  - Stage comparison
  - Opt level comparison
  - Divergence reporting

#### Week 8: Integration & First Release (DISCOVERY-016, DISCOVERY-017)
- **DISCOVERY-016**: CI/CD integration
  - Run core discovery on every commit
  - Track bugs over time
  - Performance regression detection

- **DISCOVERY-017**: Documentation (Cycle 1)
  - User guide for ruchydbg core modes
  - YAML report guide for upstream
  - Case studies (5+ bugs found)

**Cycle 1 Deliverable**: Production-ready core discovery system with 3 techniques, 2 ruchydbg modes, CI integration

---

### Cycle 2: Advanced Discovery & Enhanced Automation (8 Weeks)

**Goal**: Add advanced techniques and improve automation

**Success Criteria**:
- ≥5 additional bugs discovered (10+ total)
- 30+ additional YAML reports (50+ total)
- ruchydbg with 3 additional modes
- Deduplication engine operational

#### Week 9-10: Real-World Code Injection (DISCOVERY-004)
- **DISCOVERY-004**: Creal-style injection
  - Corpus collection (100+ programs from ecosystem)
  - Function-level extraction
  - Type-compatible injection
  - Expected: 2-3 new bugs (complex type inference failures)

#### Week 11-12: Mutation Testing (DISCOVERY-005)
- **DISCOVERY-005**: 18 mutators (4 Ruchy + 14 general)
  - AST diff tool
  - Type inference diff
  - Execution diff
  - Expected: 2-3 new bugs (optimizer over-optimization)

#### Week 13-14: Property-Based Testing (DISCOVERY-007)
- **DISCOVERY-007**: 50+ compiler invariants
  - Property test framework
  - Shrinking for minimal counterexamples
  - Integration with `ruchy prove`
  - Expected: 2-3 new bugs (type soundness violations)

#### Week 15: ruchydbg Enhancements - Advanced Modes (DISCOVERY-011, DISCOVERY-013, DISCOVERY-014)
- **DISCOVERY-011**: Performance visualization
  - Flamegraph generation
  - Memory timeline
  - Comparison with production compiler

- **DISCOVERY-013**: Root cause analysis
  - Git bisection integration
  - Fix suggestions (code diffs)
  - Confidence scoring

- **DISCOVERY-014**: Property checker
  - 50+ properties available
  - Property violation reporting

#### Week 16: Deduplication & Refinement
- Enhance deduplication engine
  - Hash-based fingerprinting
  - AST structural similarity
  - Root cause merging
- Refine existing techniques based on Cycle 1 feedback

**Cycle 2 Deliverable**: Comprehensive discovery system with 6 techniques, 5 ruchydbg modes, deduplication engine

---

### Cycle 3: Performance Discovery & Long-Term Evolution (8 Weeks)

**Goal**: Add performance profiling and lay foundation for automated repair

**Success Criteria**:
- ≥5 performance bottlenecks identified
- ≥10 additional bugs (15+ total from all cycles)
- 20+ performance reports (70+ total reports)
- Statistical fault localization prototype

#### Week 17-18: Performance Profiling (DISCOVERY-008)
- **DISCOVERY-008**: Comprehensive profiling
  - Time, memory, allocations tracking
  - Hot function detection (>10% time)
  - O(n²) algorithm identification
  - Expected: 5+ performance bottlenecks

#### Week 19-20: Coverage-Guided Exploration (DISCOVERY-009)
- **DISCOVERY-009**: Maximize code coverage
  - Line, branch, path coverage tracking
  - Guided mutation to cover uncovered paths
  - Target: >95% line, >85% branch coverage

#### Week 21: Statistical Fault Localization (Prototype)
- **NEW**: Spectrum-based fault localization
  - Tarantula, Ochiai suspiciousness metrics
  - Code element ranking
  - Enhanced confidence for suggested fixes
  - Foundation for Phase 2 (6-12 month vision)

#### Week 22-23: Comprehensive Testing & Refinement
- Test discovery system itself (>90% coverage)
- Mutation testing on discovery code
- Performance optimization of discovery techniques
- Feedback loop for technique effectiveness

#### Week 24: Final Integration & Documentation
- Complete user guides
- Case studies (15+ bugs, 5+ performance bottlenecks)
- Academic paper draft (compiler testing methodology)
- Handoff to upstream Ruchy team

**Cycle 3 Deliverable**: Production-grade discovery system with 8 techniques, fault localization, comprehensive documentation

---

### Summary: 3-Cycle Comparison

| Aspect | Cycle 1 (Weeks 1-8) | Cycle 2 (Weeks 9-16) | Cycle 3 (Weeks 17-24) |
|--------|---------------------|----------------------|-----------------------|
| **Techniques** | 3 core | +3 advanced | +2 comprehensive |
| **ruchydbg Modes** | 2 core | +3 advanced | Complete (5 modes) |
| **Expected Bugs** | 5+ | +5 (10+ total) | +5 (15+ total) |
| **Reports** | 20+ | +30 (50+ total) | +20 (70+ total) |
| **Performance** | - | - | 5+ bottlenecks |
| **Workload** | ⚖️ Balanced | ⚖️ Balanced | ⚖️ Balanced |
| **Risk** | ✅ Low | ✅ Low | ✅ Low |

**Toyota Way Alignment**:
- **Heijunka** ✅: Workload leveled across 3 cycles
- **Muri** ✅: No overburden, sustainable pace
- **Mura** ✅: No unevenness, consistent quality
- **Kaizen** ✅: Each cycle informs next, continuous improvement

---

## 🎯 Deliverables

### Code Deliverables

1. **Discovery Framework** (`discovery/`)
   - `framework.ruchy` - Core orchestration
   - `differential.ruchy` - Differential testing
   - `metamorphic.ruchy` - Metamorphic testing
   - `creal.ruchy` - Real-world injection
   - `mutation.ruchy` - Mutation testing
   - `fuzzing.ruchy` - Fuzz testing
   - `properties.ruchy` - Property-based testing
   - `profiling.ruchy` - Performance profiling
   - `coverage.ruchy` - Coverage tracking

2. **Enhanced ruchydbg** (`src/bin/ruchydbg.rs`)
   - Auto-detect mode
   - Performance visualization
   - Differential testing mode
   - Root cause analysis
   - Property checker

3. **YAML Report System** (`docs/reports/`)
   - `bugs/` - Bug reports (BUG-*.yaml)
   - `performance/` - Performance reports (PERF-*.yaml)
   - `suggestions/` - API suggestions (SUGG-*.yaml)
   - `regressions/` - Regression reports (REGR-*.yaml)

4. **Test Suite** (`validation/discovery/`)
   - Unit tests for all 8 techniques
   - Integration tests for ruchydbg
   - Property tests for framework
   - Fuzz tests for robustness
   - **Target**: >90% coverage, 100% mutation score

### Documentation Deliverables

1. **Specification** (this document)
   - `docs/specifications/deep-bug-performance-discovery-debugger-plan.md`

2. **User Guides**
   - `docs/guides/ruchydbg-user-guide.md`
   - `docs/guides/yaml-report-guide.md`
   - `docs/guides/discovery-techniques.md`

3. **Case Studies**
   - `docs/case-studies/bug-001-recursive-types.md`
   - `docs/case-studies/perf-001-type-checker-optimization.md`
   - (10+ case studies documenting bugs found)

4. **Research Report**
   - `docs/research/compiler-testing-state-of-art-2024.md`

### Report Deliverables

**Target**: 50+ YAML reports generated

- 20+ bug reports (10 new bugs discovered)
- 20+ performance reports (5 major bottlenecks)
- 10+ suggestion reports (API improvements)

---

## 🚦 Risk Management

### Risks & Mitigation

1. **Risk**: Discovery techniques find no new bugs
   - **Likelihood**: Low (we've already found 8 bugs through dogfooding)
   - **Mitigation**: Start with validation (find known bugs first)
   - **Backup**: Focus on performance discovery if bugs are scarce

2. **Risk**: Ruchy upstream doesn't act on reports
   - **Likelihood**: Low (87.5% fix rate historically)
   - **Mitigation**: Ensure reports are actionable (minimal reproduction, suggested fix)
   - **Backup**: Use findings to improve bootstrap compiler regardless

3. **Risk**: 8-week timeline insufficient
   - **Likelihood**: Medium (ambitious scope)
   - **Mitigation**: Prioritize core techniques first (differential, metamorphic)
   - **Backup**: Extend to 10-12 weeks if needed

4. **Risk**: Pure Ruchy implementation too slow
   - **Likelihood**: Low (Ruchy is fast enough)
   - **Mitigation**: Profile and optimize hot paths
   - **Backup**: Implement critical paths in Rust if necessary

5. **Risk**: Techniques produce too many false positives
   - **Likelihood**: Medium (fuzzing often has false positives)
   - **Mitigation**: Triage and minimize all findings
   - **Backup**: Adjust technique sensitivity (reduce noise)

---

## ✅ Success Criteria

This project is considered **successful** if:

1. ✅ **10+ new bugs discovered** (unique, reproducible, high impact)
2. ✅ **5+ performance bottlenecks identified** with >2x speedup potential
3. ✅ **50+ YAML reports generated** (bugs, performance, suggestions)
4. ✅ **80%+ upstream acceptance** (Ruchy team fixes reported issues)
5. ✅ **ruchydbg enhanced** with 5 new modes (auto-detect, perf-viz, diff, root-cause, property-check)
6. ✅ **90%+ test coverage** on discovery system itself
7. ✅ **SATD=0, A+ lint, TDG>85** (quality gates enforced)
8. ✅ **100% Extreme TDD** (RED-GREEN-REFACTOR-TOOL on all features)
9. ✅ **Documentation complete** (specification, user guides, case studies)
10. ✅ **CI integration** (run discovery on every commit)

---

## 🏭 Toyota Way Principles Applied

This specification embodies the 14 principles of the Toyota Way:

### 1. Long-Term Philosophy ✅
- **Principle**: "Base your management decisions on a long-term philosophy, even at the expense of short-term financial goals"
- **Application**: 10-year vision from bug discovery → fault localization → automated repair → self-healing compiler
- **Evidence**: Phase 1-4 roadmap spanning 3-5 years

### 2. Create Continuous Process Flow (Jidoka) ✅
- **Principle**: "Create continuous process flow to bring problems to the surface"
- **Application**: 8 discovery techniques create multi-layered quality assurance
- **Evidence**: CI/CD integration runs discovery on every commit, immediately surfacing issues

### 3. Use "Pull" Systems (Just-in-Time) ✅
- **Principle**: "Use 'pull' systems to avoid overproduction"
- **Application**: Generate bug reports only when bugs are found (not speculative)
- **Evidence**: YAML reports created on-demand based on actual discoveries

### 4. Level Out the Workload (Heijunka) ⚠️ **ADDRESSED IN v2.0**
- **Principle**: "Level out the workload"
- **Application**: Iterative 8-week cycles prevent overburden
- **Improvement**: v2.0 roadmap further levels workload with 3-cycle approach

### 5. Build a Culture of Stopping to Fix Problems (Andon) ✅
- **Principle**: "Build a culture of stopping to fix problems, to get quality right the first time"
- **Application**: `ruchydbg auto-detect` immediately stops and reports when bug found
- **Evidence**: Pre-commit hooks block commits if quality gates fail

### 6. Standardized Tasks (Foundation for Kaizen) ✅
- **Principle**: "Standardized tasks are the foundation for continuous improvement and employee empowerment"
- **Application**: Extreme TDD (RED-GREEN-REFACTOR-TOOL), PMAT enforcement, zero SATD
- **Evidence**: All 17 tickets follow identical quality gate process

### 7. Use Visual Control ✅
- **Principle**: "Use visual control so no problems are hidden"
- **Application**: Flamegraphs, coverage reports (HTML), performance dashboards
- **Evidence**: `ruchydbg perf-viz` generates visual flamegraphs

### 8. Use Only Reliable, Thoroughly Tested Technology ✅
- **Principle**: "Use only reliable, thoroughly tested technology that serves your people and processes"
- **Application**: All techniques based on peer-reviewed research (2024-2025)
- **Evidence**: 10 peer-reviewed papers cited, proven track record (8 bugs found)

### 9. Grow Leaders Who Thoroughly Understand the Work ✅
- **Principle**: "Grow leaders who thoroughly understand the work, live the philosophy, and teach it to others"
- **Application**: Educational value section, comprehensive documentation
- **Evidence**: Students learn production-grade compiler QA methodologies

### 10. Develop Exceptional People and Teams ✅
- **Principle**: "Develop exceptional people and teams who follow your company's philosophy"
- **Application**: Pure Ruchy dogfooding upskills team in debugging and optimization
- **Evidence**: Feedback loop for improving discovery system itself

### 11. Respect Your Extended Network (Upstream Team) ✅
- **Principle**: "Respect your extended network of partners and suppliers by challenging them and helping them improve"
- **Application**: Actionable YAML reports with minimal reproduction, root cause, suggested fixes
- **Evidence**: 87.5% upstream acceptance rate (7/8 bugs fixed)

### 12. Go and See for Yourself (Genchi Genbutsu) ✅
- **Principle**: "Go and see for yourself to thoroughly understand the situation"
- **Application**: Real-world code injection from actual Ruchy programs, root cause analysis with bisection
- **Evidence**: Creal-style technique goes to "gemba" of real-world code

### 13. Make Decisions Slowly by Consensus (Nemawashi) ✅
- **Principle**: "Make decisions slowly by consensus, thoroughly considering all options; implement decisions rapidly"
- **Application**: This specification represents thorough analysis of 8 techniques before implementation
- **Evidence**: Comprehensive review process, Toyota Way critical analysis

### 14. Become a Learning Organization (Kaizen) ✅
- **Principle**: "Become a learning organization through relentless reflection and continuous improvement"
- **Application**: Success metrics, feedback loops, iterative improvement
- **Evidence**: Process for periodically reviewing effectiveness of discovery techniques

---

## 📖 References

### Peer-Reviewed Research (Foundational Papers)

#### Metamorphic Testing
1. **Chen, T. Y., Kuo, F. C., Liu, H., & Poon, P. L.** (2015). *A survey on metamorphic testing*. IEEE Transactions on Software Engineering, 41(1), 14-35.
   - Seminal survey on metamorphic testing for solving oracle problem
   - Cited 1,500+ times, foundational work

#### Differential Testing
2. **McKeeman, W. M.** (1998). *Differential testing for software*. Digital Technical Journal, 10(1), 100-107.
   - Original differential testing paper
   - Foundation for modern compiler testing

3. **Le, V., Afshari, M., & Su, Z.** (2014). *Compiler validation via equivalence modulo inputs*. ACM SIGPLAN Notices, 49(6), 213-222.
   - EMI technique for compiler testing
   - Found hundreds of bugs in GCC/LLVM

#### Compiler Fuzzing
4. **Yang, X., Chen, Y., Eide, E., & Regehr, J.** (2011). *Finding and understanding bugs in C compilers*. ACM SIGPLAN Notices, 46(6), 283-294.
   - Csmith: random program generator for C compilers
   - Found 325+ bugs in production compilers

5. **Sun, C., Le, V., & Su, Z.** (2016). *Finding compiler bugs via live code mutation*. In Proceedings of the 2016 ACM SIGPLAN International Conference on Object-Oriented Programming, Systems, Languages, and Applications (pp. 849-863).
   - Live code mutation for compiler testing
   - Discovered 104 new bugs

6. **Lidbury, C., Lascu, A., Chong, N., & Donaldson, A. F.** (2015). *Many-core compiler fuzzing*. ACM SIGPLAN Notices, 50(8), 225-235.
   - Graphics shader compiler fuzzing
   - 60+ bugs found via metamorphic testing

7. **Marcozzi, M., Tang, Q., Donaldson, A. F., & Cadar, C.** (2019). *Compiler fuzzing: how much does it matter?* Proceedings of the ACM on Programming Languages, 3(OOPSLA), 1-29.
   - Systematic study of compiler fuzzing effectiveness
   - Analysis of 5,000+ compiler bugs

8. **Holler, C., Herzig, K., & Zeller, A.** (2012). *Fuzzing with code fragments*. In Proceedings of the 21st USENIX security symposium (pp. 445-458).
   - LangFuzz: fragment-based fuzzing
   - Found 105 bugs in JavaScript engines

#### Automated Program Repair (Future Work)
9. **Le Goues, C., Nguyen, T., Forrest, S., & Weimer, W.** (2012). *GenProg: A generic method for automatic software repair*. IEEE Transactions on Software Engineering, 38(1), 54-72.
   - Template-based automated program repair
   - Foundation for modern APR research

#### Profile-Guided Optimization
10. **Cummins, C., Petoumenos, P., Wang, Z., & Leather, H.** (2017). *Synthesizing benchmarks for predictive modeling*. In Proceedings of the 2017 International Symposium on Code Generation and Optimization (pp. 86-99).
    - Benchmark synthesis for compiler optimization
    - Machine learning for optimization decisions

### Research Papers (2024-2025)

11. **Creal** (PLDI 2024)
    - "Boosting Compiler Testing by Injecting Real-World Code"
    - https://github.com/UniCodeSphere/Creal

12. **Rustlantis** (OOPSLA 2024)
    - "Randomized Differential Testing of the Rust Compiler"
    - https://www.ralfj.de/blog/2024/11/25/rustlantis.html

13. **Rust-twins** (ASE 2024)
    - "Automatic Rust Compiler Testing through Program Mutation and Dual Macros Generation"

14. **DIFFER** (2024)
    - "Introducing DIFFER, a new tool for testing and validating transformed programs"
    - https://blog.trailofbits.com/2024/01/31/introducing-differ

### Existing Ruchy Bugs

1. Issue #54: Boolean Negation Hang (OPEN)
2. Issue #38: Variable Collision (FIXED in v3.98.0)
3. Issue #39: Nested Match with Box (FIXED in v3.99.1)
4. Issue #40: Early Return Bug (FIXED in v3.100.0)
5. (Others documented in BOUNDARIES.md)

### Related Projects

- rustc fuzzing: https://github.com/dwrensha/fuzz-rustc
- rustc-perf: https://perf.rust-lang.org/
- GCC testsuite: https://gcc.gnu.org/onlinedocs/gccint/Testsuites.html

---

## 🎓 Educational Value

This project demonstrates:

1. **State-of-the-art compiler testing** (8 modern techniques from 2024 research)
2. **Metamorphic testing** for solving the oracle problem
3. **Differential testing** across compilation stages
4. **Property-based testing** for compiler invariants
5. **Fuzzing** for finding crashes and edge cases
6. **Performance profiling** and bottleneck analysis
7. **Root cause analysis** with automated bisection
8. **Actionable reporting** for upstream maintainers

**Outcome**: Students learn modern compiler quality assurance methodologies used in production (rustc, GCC, LLVM).

---

## 🏁 Next Steps

1. **Review & Approve** this specification
2. **Create roadmap tickets** (DISCOVERY-001 through DISCOVERY-017)
3. **Begin RED phase** (write failing tests for DISCOVERY-001)
4. **Follow Extreme TDD** throughout 8-week implementation
5. **Generate first YAML reports** within 4 weeks
6. **Share findings with upstream** Ruchy team weekly

---

**Status**: ✅ Specification Complete - Ready for Implementation

**Contact**: Submit questions or feedback to RuchyRuchy project team

**Version**: 1.0.0 (October 26, 2025)
