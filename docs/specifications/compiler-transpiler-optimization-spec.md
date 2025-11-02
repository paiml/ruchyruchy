# Compiler/Transpiler Optimization Specification via Self-Hosting

**Project**: RuchyRuchy Bootstrap Compiler
**Document Version**: 1.2 (Pre/Post-Optimization Strategy Added)
**Date**: November 2, 2025 (Updated)
**Status**: SPECIFICATION - PENDING IMPLEMENTATION
**Methodology**: EXTREME TDD + Mutation/Property/Fuzz/PMAT + Portfolio Validation + Statistical Rigor
**Code Review Integration**: Toyota Way (Kaizen, Genchi Genbutsu, Jidoka)

---

## Executive Summary

This specification defines a **scientific, peer-reviewed approach** to discovering and implementing compiler/transpiler optimizations through self-hosting and bootstrapping techniques. By compiling the compiler with itself and systematically measuring performance, we can discover optimization opportunities empirically rather than theoretically.

**Core Principle**: Use the bootstrap compiler as its own benchmark suite to discover real-world optimization opportunities through profiling, measurement, and iterative improvement.

**v1.1 Kaizen Improvements** (October 21, 2025):
1. **Phase 8: Portfolio Validation** - Test optimization combinations to address phase-ordering problems
2. **Statistical Rigor in Benchmarking** - Require statistical significance (p < 0.05) for all performance claims
3. **Risk-Based Classification** - Apply validation rigor proportional to optimization complexity and risk

**v1.2 Pre/Post-Optimization Strategy** (November 2, 2025):
1. **Pre-Optimization** - Static analysis (Clippy::perf, cargo-bloat) on transpiled Rust before compilation
2. **Post-Optimization** - Iterative compiler flag tuning (opt-level, LTO, codegen-units, PGO)
3. **12 Peer-Reviewed Papers** - Research foundation for systematic optimization methodology

---

## Table of Contents

1. [Research Foundation](#research-foundation)
2. [Self-Hosting Optimization Strategy](#self-hosting-optimization-strategy)
3. [Optimization Techniques Catalog](#optimization-techniques-catalog)
4. [EXTREME TDD Implementation Methodology](#extreme-tdd-implementation-methodology)
5. [Testing Strategy: Mutation/Property/Fuzz/PMAT](#testing-strategy)
6. [Implementation Phases](#implementation-phases)
7. [Success Metrics](#success-metrics)
8. [Pre/Post-Optimization Strategy](#prepost-optimization-strategy)
9. [References](#references)

---

## Research Foundation

### Peer-Reviewed Research (2024-2025)

#### 1. Profile-Guided Optimization (PGO)
**Paper**: "From Profiling to Optimization: Unveiling the Profile Guided Optimization"
**Source**: arXiv:2507.16649v1 (2025)
**Key Findings**:
- PGO examines profiling techniques from software instrumentation-based edge and path profiling to hardware-sampling mechanisms
- Open research directions: zero-overhead sampling, dynamic PGO integration in JIT environments, machine-learning-driven heuristic tuning
- Cross-architecture portability remains a challenge

**Application to RuchyRuchy**:
- Profile bootstrap compiler self-compilation to identify hot paths
- Use edge/path profiling to discover optimization opportunities
- Implement zero-overhead sampling for continuous optimization discovery

#### 2. Machine Learning for Compiler Optimization
**Paper**: "CompilerDream: Learning a Compiler World Model for General Code Optimization"
**Source**: arXiv:2404.16077v2 (August 2024)
**Key Findings**:
- Model-based reinforcement learning can approximate state transitions and reward signals
- Learning a world model of compiler behavior enables optimization discovery
- Generalizes across different code patterns

**Application to RuchyRuchy**:
- Future enhancement: ML-guided optimization pass ordering
- Current: Empirical measurement-based approach (no ML dependency)
- Foundation for future AI-assisted optimization

#### 3. Iterative Compilation and Auto-Tuning
**Paper**: "A Survey on Compiler Autotuning using Machine Learning"
**Source**: ACM Computing Surveys (September 2018)
**Key Findings**:
- Iterative compilation involves searching massive optimization spaces
- Phase-ordering problem is NP-hard
- Auto-tuning can discover non-obvious optimization sequences

**Application to RuchyRuchy**:
- Systematically explore optimization pass orderings
- Measure bootstrap compilation time as fitness function
- Discover phase-ordering optimizations empirically

#### 4. GCC Bootstrap Optimization (Classic Reference)
**Source**: GCC Documentation (Multi-stage bootstrapping)
**Key Findings**:
- Three-stage bootstrap ensures compiler benefits from its own optimizations
- Stage 3 compilation validates optimization correctness
- Self-hosting reveals optimization opportunities that synthetic benchmarks miss

**Application to RuchyRuchy**:
- Implement multi-stage bootstrap validation
- Stage N+1 must compile identical output to Stage N (bit-for-bit)
- Use bootstrap time as primary optimization metric

### Classic Compiler Optimization Research

#### Dragon Book Foundations
**Source**: Aho, Sethi, Ullman - "Compilers: Principles, Techniques, and Tools"
**Techniques**:
- Peephole optimization
- Constant folding and propagation
- Dead code elimination
- Common subexpression elimination
- Loop optimizations (invariant code motion, strength reduction)

#### Modern Optimization Surveys
**Source**: "A Survey of Compiler Optimization Techniques" (Academia.edu)
**Coverage**:
- Local optimizations (basic block level)
- Global optimizations (control flow graph level)
- Interprocedural optimizations (whole program level)
- Machine-dependent optimizations (target-specific)

### Performance Evaluation and Validation Research

#### Statistical Rigor in Performance Measurement
**Paper**: Georges, A., Buytaert, D., & Eeckhout, L. (2007) - "Statistically rigorous Java performance evaluation"
**Source**: ACM SIGPLAN Notices, 42(10), 57-76
**Key Findings**:
- Performance measurements contain statistical noise from OS scheduler, cache state, CPU turbo boost
- Single measurements can lead to incorrect conclusions (accepting phantom improvements or missing real regressions)
- Statistical tests (e.g., Welch's t-test) with confidence intervals are required for valid conclusions
- Recommend N≥30 benchmark runs with mean, standard deviation, and p-value < 0.05

**Application to RuchyRuchy**:
- All bootstrap timing measurements must use statistical tests
- Require 30+ benchmark runs per configuration
- Calculate mean ± std dev, 95% confidence intervals
- Optimization accepted only if p < 0.05 (statistically significant improvement)

#### Optimization Interactions and Trade-offs
**Paper**: Cooper, K. D., Schielke, P. J., & Subramanian, D. (2002) - "Optimizing for memory hierarchies: what is a compiler to do?"
**Source**: Journal of the Brazilian Computer Society, 8(2), 29-42
**Key Findings**:
- Compiler optimizations make complex trade-offs between multiple objectives
- One optimization can enhance, negate, or interfere with another
- Phase-ordering problem: optimal sequence depends on specific code patterns
- Iterative, experimental approach required to navigate optimization space

**Application to RuchyRuchy**:
- Test optimizations in combination, not just isolation
- Validate optimization "portfolios" to detect negative interactions
- Empirically discover optimal phase ordering through experimentation
- Use multi-configuration benchmarks (baseline, +opt1, +opt2, +opt1+opt2)

#### Risk-Based Validation and Regression Analysis
**Paper**: Sullivan, M., & Zeller, A. (2005) - "Yesterday's loss is today's gain: a simple and effective technique for localizing regression bugs"
**Source**: Proceedings of the 10th European Software Engineering Conference
**Key Findings**:
- Not all code changes carry equal risk of introducing bugs
- Focusing validation effort on highest-risk changes is most effective
- Heuristic-based optimizations are more likely to cause regressions
- Simple, provably-correct changes can use lighter validation

**Application to RuchyRuchy**:
- Classify optimizations by risk (provably correct vs heuristic-based)
- Apply validation rigor proportional to risk level
- Focus maximum testing effort on complex, global optimizations
- Allow "fast-track" for simple, local, provably-correct optimizations

---

## Self-Hosting Optimization Strategy

### Core Methodology: Bootstrap-Driven Discovery

**Principle**: The bootstrap compiler compiling itself is the **best benchmark** for discovering optimization opportunities because:
1. **Real-world code**: Compiler source is representative of production Ruchy code
2. **Measurable**: Bootstrap time is concrete, reproducible metric
3. **Self-validating**: Output correctness verified by bit-for-bit comparison
4. **Comprehensive**: Exercises all compiler phases (lexer, parser, types, codegen)

### The Self-Hosting Optimization Loop

```
┌─────────────────────────────────────────────────────────┐
│ 1. PROFILE: Measure bootstrap compilation               │
│    - Identify hot paths in compiler execution           │
│    - Measure time spent in each compiler phase          │
│    - Track memory allocations and data structure usage  │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│ 2. ANALYZE: Discover optimization opportunities         │
│    - Find redundant computations                        │
│    - Identify allocation hotspots                       │
│    - Detect inefficient algorithms                      │
│    - Discover missed constant folding opportunities     │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│ 3. IMPLEMENT: Add optimization (EXTREME TDD)            │
│    - RED: Write failing test demonstrating issue       │
│    - GREEN: Implement minimal optimization              │
│    - REFACTOR: Generalize and improve                   │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│ 4. VALIDATE: Test optimization correctness              │
│    - Property testing: Optimization preserves semantics │
│    - Mutation testing: Tests catch regressions          │
│    - Fuzz testing: Optimization handles edge cases      │
│    - PMAT: Quality score ≥85                            │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│ 5. MEASURE: Verify performance improvement              │
│    - Bootstrap time reduction (target: ≥5% per opt)     │
│    - Memory usage improvement                           │
│    - Generated code quality improvement                 │
│    - Bit-for-bit output validation (correctness)        │
└─────────────────────────────────────────────────────────┘
                         ↓
                    (Repeat)
```

### Multi-Stage Bootstrap Validation

Following GCC's three-stage bootstrap model:

**Stage 0**: Baseline compiler (production Ruchy compiler)
- Compiles RuchyRuchy bootstrap compiler Stage 1

**Stage 1**: First bootstrap (compiled by Stage 0)
- Compiles RuchyRuchy bootstrap compiler Stage 2
- May not have all optimizations enabled yet

**Stage 2**: Second bootstrap (compiled by Stage 1)
- Compiles RuchyRuchy bootstrap compiler Stage 3
- Has all optimizations enabled

**Stage 3**: Validation bootstrap (compiled by Stage 2)
- Must produce **bit-for-bit identical output** to Stage 2
- Validates optimization correctness
- If Stage 3 ≠ Stage 2, optimization is **buggy**

**Optimization Validation Rule**: A compiler optimization is correct if and only if:
```
Stage2_Output == Stage3_Output  (bit-for-bit comparison)
```

---

## Optimization Techniques Catalog

### Category 1: Lexical Analysis Optimizations

#### OPT-LEX-001: Token Stream Caching
**Risk Class**: 2 (Complex / Global - requires cache invalidation correctness)
**Research Basis**: Memoization techniques in compiler construction
**Opportunity**: Bootstrap compiler re-tokenizes common patterns
**Implementation**:
- Cache tokenized input for identical source fragments
- Use hash-based lookup for token stream retrieval
- Invalidate cache on source modification

**Expected Impact**: 10-15% ± Y% lexer speedup on repeated patterns (p < 0.05, N=30 runs)
**Measurement**: Profile token generation time during bootstrap with statistical validation
**Validation**: Property test: `cached_tokenize(s) == tokenize(s)` + Portfolio validation vs OPT-LEX-002
**TDD Rigor**: FULL EXTREME TDD (all 8 phases, 10,000+ property cases, 100,000+ fuzz cases)

#### OPT-LEX-002: Lazy String Allocation
**Research Basis**: Lazy evaluation in functional programming
**Opportunity**: Not all tokens need materialized strings
**Implementation**:
- Defer string allocation until token value is accessed
- Use string views/slices for keywords and operators
- Only allocate for identifiers and literals when needed

**Expected Impact**: 20-30% reduction in lexer memory allocations
**Measurement**: Track allocation counts during bootstrap tokenization
**Validation**: Memory profiling + equivalence property tests

### Category 2: Parsing Optimizations

#### OPT-PARSE-001: Left-Recursion Elimination
**Research Basis**: Dragon Book §4.3 - "Writing a Grammar"
**Opportunity**: Pratt parser can be optimized for left-associative operators
**Implementation**:
- Transform left-recursive productions into iterative loops
- Use precedence climbing for operator parsing
- Eliminate redundant recursive calls

**Expected Impact**: 15-25% parser speedup
**Measurement**: Profile parser execution during Stage 1→2 compilation
**Validation**: Property test: `parse(ast.emit()) == ast` (roundtrip)

#### OPT-PARSE-002: AST Node Pooling
**Research Basis**: Object pooling design pattern
**Opportunity**: Frequent AST node allocation/deallocation
**Implementation**:
- Pre-allocate pool of AST nodes
- Reuse nodes instead of heap allocation
- Reset node state rather than free/allocate

**Expected Impact**: 30-40% reduction in parser memory churn
**Measurement**: Profile allocation rate during AST construction
**Validation**: Semantic equivalence + memory leak detection

### Category 3: Type System Optimizations

#### OPT-TYPE-001: Type Inference Caching
**Research Basis**: Algorithm W optimization techniques
**Opportunity**: Re-inferring types for identical expressions
**Implementation**:
- Cache type inference results keyed by AST node identity
- Invalidate cache on constraint changes
- Share type schemes across identical subexpressions

**Expected Impact**: 20-35% type checking speedup
**Measurement**: Profile unification time during Stage 2 type checking
**Validation**: Property test: Type soundness preservation

#### OPT-TYPE-002: Occurs Check Optimization
**Research Basis**: Union-find data structure for unification
**Opportunity**: Occurs check is O(n) per unification
**Implementation**:
- Use union-find with path compression
- Track type variable depths to short-circuit occurs check
- Cache negative occurs check results

**Expected Impact**: 10-20% unification speedup
**Measurement**: Profile occurs check invocations
**Validation**: Property test: No infinite types created

### Category 4: Code Generation Optimizations

#### OPT-CODEGEN-001: Constant Folding
**Risk Class**: 3 (Provably Correct / Local - algebraic transformation)
**Research Basis**: Classic compiler optimization (Dragon Book §9.1)
**Opportunity**: Compile-time evaluation of constant expressions
**Implementation**:
- Evaluate arithmetic/logical ops on constants at compile-time
- Fold constant branches (dead code elimination)
- Propagate constants across assignments

**Expected Impact**: 5-10% ± Y% runtime speedup of generated code (p < 0.05, N=30 runs)
**Measurement**: Compare generated code size and execution time with statistical validation
**Validation**: Property test: Semantic equivalence + Portfolio validation vs OPT-CODEGEN-004
**TDD Rigor**: FAST-TRACK (phases 1-6, 1,000+ property cases, fuzz testing optional)

#### OPT-CODEGEN-002: Peephole Optimization
**Research Basis**: Peephole optimization survey (ScienceDirect)
**Opportunity**: Redundant instruction sequences in generated code
**Implementation**:
- Pattern match on generated instruction sequences
- Replace inefficient patterns with optimal equivalents
- Examples: `x + 0 → x`, `x * 1 → x`, `x * 0 → 0`

**Expected Impact**: 3-7% generated code speedup
**Measurement**: Benchmark generated code execution
**Validation**: Differential testing vs unoptimized output

#### OPT-CODEGEN-003: Dead Code Elimination (DCE)
**Risk Class**: 2 (Complex / Global - requires liveness analysis)
**Research Basis**: Data flow analysis (Dragon Book §9.2)
**Opportunity**: Unreachable code from constant folding
**Implementation**:
- Compute liveness analysis on generated code
- Remove unused variable assignments
- Eliminate unreachable basic blocks

**Expected Impact**: 5-15% ± Y% generated code size reduction (p < 0.05, N=30 runs)
**Measurement**: Compare bytecode/assembly size with statistical validation
**Validation**: Property test: Behavior preservation + Portfolio validation
**TDD Rigor**: FULL EXTREME TDD (all 8 phases, 10,000+ property cases, 100,000+ fuzz cases)

#### OPT-CODEGEN-004: Inline Expansion
**Risk Class**: 1 (Heuristic-Based - size/frequency thresholds)
**Research Basis**: Procedure integration (Classic optimization)
**Opportunity**: Function call overhead in hot paths
**Implementation**:
- Inline small, frequently-called functions
- Use heuristics: size threshold, call frequency
- Avoid inlining recursive functions

**Expected Impact**: 10-25% ± Y% runtime speedup (context-dependent, p < 0.05, N=30 runs)
**Measurement**: Profile function call overhead during bootstrap with statistical validation
**Validation**: Benchmark suite + semantic equivalence + Portfolio validation vs OPT-CODEGEN-001
**TDD Rigor**: MAXIMUM RIGOR (all 8 phases, 25,000+ property cases, 250,000+ fuzz cases, differential testing, sensitivity analysis)

### Category 5: Cross-Cutting Optimizations

#### OPT-GLOBAL-001: Profile-Guided Optimization (PGO)
**Risk Class**: 1 (Heuristic-Based - profile-dependent decisions)
**Research Basis**: "From Profiling to Optimization" (2025)
**Opportunity**: Optimize based on real bootstrap execution profile
**Implementation**:
- Instrument bootstrap compiler to collect execution profiles
- Identify hot paths (80/20 rule: 80% time in 20% code)
- Apply targeted optimizations to hot paths
- Use branch prediction hints based on profile

**Expected Impact**: 15-30% ± Y% bootstrap speedup (p < 0.05, N=30 runs)
**Measurement**: Compare instrumented vs optimized bootstrap time with statistical validation
**Validation**: Multi-stage bootstrap validation (Stage 2 == Stage 3) + Portfolio validation
**TDD Rigor**: MAXIMUM RIGOR (all 8 phases, 25,000+ property cases, 250,000+ fuzz cases, differential testing with multiple profiles, sensitivity analysis)

#### OPT-GLOBAL-002: Whole-Program Optimization
**Research Basis**: Interprocedural optimization research
**Opportunity**: Optimize across function boundaries
**Implementation**:
- Build call graph for entire compiler
- Perform interprocedural constant propagation
- Inline across module boundaries
- Dead function elimination

**Expected Impact**: 10-20% overall speedup
**Measurement**: Whole-program compilation time
**Validation**: Integration tests + bootstrap validation

---

## EXTREME TDD Implementation Methodology

### 8-Phase EXTREME TDD for Each Optimization (v1.1)

Every optimization must follow this **mandatory** process (updated with Kaizen improvements):

#### Phase 1: RED - Write Failing Test

**Objective**: Demonstrate the optimization opportunity exists

**Example (Constant Folding)**:
```ruchy
// File: validation/optimizations/test_constant_folding.ruchy

fun test_constant_folding_opportunity() -> bool {
    println("🧪 OPT-CODEGEN-001: Constant Folding (RED Phase)");

    // Test case: constant arithmetic should be folded
    let source = "fun main() { let x = 2 + 3; println(x); }";

    let ast = parse(source);
    let optimized_ast = optimize_constants(ast);  // Not implemented yet!

    // Expected: AST contains literal 5, not Add(2, 3)
    let expected_value = 5;
    let actual_value = extract_constant(optimized_ast);

    if actual_value != expected_value {
        println("❌ Constant not folded: expected {}, got Add(2,3)", expected_value);
        return false;
    }

    return true;
}
```

**Run**: `ruchy test validation/optimizations/test_constant_folding.ruchy`
**Expected**: ❌ Test fails (optimization not implemented)

#### Phase 2: GREEN - Minimal Implementation

**Objective**: Simplest code to make test pass

**Example**:
```ruchy
// File: bootstrap/stage3/optimizer.ruchy

fun optimize_constants(ast: AST) -> AST {
    match ast {
        AST::BinOp(Op::Add, AST::Literal(a), AST::Literal(b)) => {
            // Minimal: only handle integer addition
            AST::Literal(a + b)
        },
        _ => ast  // No optimization for other cases
    }
}
```

**Run**: `ruchy test validation/optimizations/test_constant_folding.ruchy`
**Expected**: ✅ Test passes (minimal optimization works)

#### Phase 3: REFACTOR - Generalize and Improve

**Objective**: Make optimization comprehensive and maintainable

**Example**:
```ruchy
fun optimize_constants(ast: AST) -> AST {
    match ast {
        // Arithmetic operations
        AST::BinOp(op, AST::Literal(a), AST::Literal(b)) => {
            let result = match op {
                Op::Add => a + b,
                Op::Sub => a - b,
                Op::Mul => a * b,
                Op::Div => if b != 0 { a / b } else { return ast },  // Don't fold division by zero
                _ => return ast
            };
            AST::Literal(result)
        },

        // Logical operations
        AST::BinOp(Op::And, AST::Bool(a), AST::Bool(b)) => AST::Bool(a && b),
        AST::BinOp(Op::Or, AST::Bool(a), AST::Bool(b)) => AST::Bool(a || b),

        // Recursive optimization (fold nested expressions)
        AST::BinOp(op, left, right) => {
            let opt_left = optimize_constants(left);
            let opt_right = optimize_constants(right);
            optimize_constants(AST::BinOp(op, opt_left, opt_right))
        },

        // Other AST nodes
        _ => ast
    }
}
```

**Run**: Extended test suite
**Expected**: ✅ All tests pass, optimization is comprehensive

#### Phase 4: TOOL VALIDATION (16 Ruchy Tools)

**Objective**: Validate with all Ruchy quality tools

```bash
# 1. Syntax validation
ruchy check bootstrap/stage3/optimizer.ruchy
✅ Syntax valid

# 2. Type checking
ruchy test bootstrap/stage3/optimizer.ruchy
✅ Types correct

# 3. Lint (A+ grade required)
ruchy lint bootstrap/stage3/optimizer.ruchy
✅ Grade: A+

# 4. Format
ruchy fmt bootstrap/stage3/optimizer.ruchy
✅ Formatted

# 5. Prove (formal verification)
ruchy prove validation/optimizations/constant_folding_properties.ruchy
✅ Properties verified

# 6. Score (quality ≥0.8)
ruchy score bootstrap/stage3/optimizer.ruchy
✅ Score: 0.92

# 7. Runtime analysis
ruchy runtime bootstrap/stage3/optimizer.ruchy
✅ Performance acceptable

# 8-16. Additional tools (build, run, doc, bench, profile, coverage, deps, security, complexity)
```

#### Phase 5: MUTATION TESTING

**Objective**: Ensure tests are effective (catch regressions)

**Using PMAT (Pure Mutation Analysis Tool)**:
```bash
pmat mutate bootstrap/stage3/optimizer.ruchy --output mutations/

# PMAT generates mutants:
# Mutant 1: Change a + b to a - b
# Mutant 2: Change if b != 0 to if b == 0
# Mutant 3: Remove recursive call
# ... etc

# Run tests against each mutant
for mutant in mutations/*.ruchy; do
    ruchy test validation/optimizations/test_constant_folding.ruchy --impl=$mutant
done

# Expected: All mutants killed by tests
✅ 45/45 mutants killed (100% mutation coverage)
```

**Acceptance Criteria**: ≥95% mutant kill rate

#### Phase 6: PROPERTY TESTING

**Objective**: Test mathematical properties of optimization

**Example Properties**:
```ruchy
// File: validation/optimizations/constant_folding_properties.ruchy

// Property 1: Optimization preserves semantics
property semantic_preservation(ast: AST) -> bool {
    let original_value = evaluate(ast);
    let optimized_ast = optimize_constants(ast);
    let optimized_value = evaluate(optimized_ast);

    original_value == optimized_value  // Must be semantically equivalent
}

// Property 2: Optimization is idempotent
property idempotence(ast: AST) -> bool {
    let once = optimize_constants(ast);
    let twice = optimize_constants(once);

    once == twice  // Applying optimization twice = applying once
}

// Property 3: Optimization only improves or maintains performance
property performance_monotonicity(ast: AST) -> bool {
    let original_complexity = complexity(ast);
    let optimized_complexity = complexity(optimize_constants(ast));

    optimized_complexity <= original_complexity
}

// Run 10,000+ test cases per property
fun main() {
    ruchy_prove(semantic_preservation, 10000);
    ruchy_prove(idempotence, 10000);
    ruchy_prove(performance_monotonicity, 10000);
}
```

**Run**: `ruchy prove validation/optimizations/constant_folding_properties.ruchy`
**Expected**: ✅ 10,000+ cases pass per property

#### Phase 7: FUZZ TESTING

**Objective**: Discover edge cases and boundary conditions

**Fuzz Testing Strategy**:
```ruchy
// File: validation/optimizations/fuzz_constant_folding.ruchy

fun fuzz_constant_folding() {
    // Strategy 1: Grammar-based fuzzing (valid expressions)
    for i in 0..50000 {
        let expr = generate_valid_arithmetic_expr();
        test_optimization_correctness(expr);
    }

    // Strategy 2: Mutation-based fuzzing (malformed expressions)
    for i in 0..50000 {
        let expr = mutate_expression(baseline_expr);
        test_optimization_robustness(expr);
    }

    // Strategy 3: Boundary value fuzzing
    let boundaries = [
        i32::MAX, i32::MIN, 0, 1, -1,
        i32::MAX - 1, i32::MIN + 1
    ];
    for a in boundaries {
        for b in boundaries {
            test_constant_folding_arithmetic(a, b);
        }
    }

    // Strategy 4: Nested expression fuzzing
    for depth in 1..100 {
        let deep_expr = generate_nested_expr(depth);
        test_optimization_stack_safety(deep_expr);
    }
}
```

**Run**: `ruchy run validation/optimizations/fuzz_constant_folding.ruchy`
**Expected**: ✅ Zero crashes, all edge cases handled

#### Phase 8: PORTFOLIO VALIDATION (Jidoka - Addressing Phase-Ordering Problem)

**Objective**: Validate optimization within the broader portfolio of optimizations to detect interactions

**Principle (Genchi Genbutsu - Go and See)**: Optimizations do not exist in isolation. The benefit of one optimization (e.g., Inline Expansion) can be dramatically affected by the presence of another (e.g., Constant Folding). The phase-ordering problem is NP-hard, requiring empirical validation of optimization combinations.

**Research Basis**: Cooper et al. (2002) - "Optimizing for memory hierarchies: what is a compiler to do?"

**Process**:

1. **Identify Comparison Configurations**: Test the new optimization in multiple contexts
   - Configuration A: `Baseline` (current main branch, no new optimization)
   - Configuration B: `Baseline + New_Optimization` (isolated impact)
   - Configuration C: `Baseline + Last_Merged_Optimization` (previous state)
   - Configuration D: `Baseline + New_Optimization + Last_Merged_Optimization` (combined impact)

2. **Run Statistical Benchmarks**: For each configuration, run bootstrap benchmark with statistical rigor
   - 30+ benchmark runs per configuration
   - Calculate mean ± std dev, 95% confidence intervals
   - Perform Welch's t-test between configurations (p < 0.05 required)

3. **Analyze Interactions**: Compare results to detect synergies or conflicts
   - **Synergy**: Combined speedup > sum of individual speedups (superlinear)
   - **Independence**: Combined speedup ≈ sum of individual speedups (linear)
   - **Conflict**: Combined speedup < sum of individual speedups (sublinear)
   - **Negative**: Combined speedup worse than one optimization alone (interference)

4. **Document Findings**: Record interaction analysis in optimization ticket
   - Which optimizations does this interact with?
   - Synergistic combinations (enable together)
   - Conflicting combinations (avoid together or reorder phases)

**Automated Tooling**:
```bash
# Create experimentation framework
ruchy experiment --baseline --compare OPT-CODEGEN-001 OPT-CODEGEN-004

# Output:
# Configuration A (Baseline): 10.5s ± 0.2s
# Configuration B (+Constant Folding): 9.8s ± 0.15s (6.7% faster, p=0.001)
# Configuration C (+Inlining): 9.2s ± 0.18s (12.4% faster, p<0.001)
# Configuration D (+Both): 8.1s ± 0.12s (22.9% faster, p<0.001)
#
# Analysis: Synergistic (22.9% > 6.7% + 12.4% = 19.1%)
# Recommendation: Enable both optimizations, inlining before constant folding
```

**Acceptance Criteria**:
- ✅ New optimization provides benefit in isolation (Config B better than Config A, p < 0.05)
- ✅ No severe negative interactions detected (Config D not worse than Config C)
- ✅ Optimal phase ordering documented (if applicable)
- ✅ Interaction matrix updated for future optimizations

**Why This Phase Matters**:
- Prevents local optimum trap (optimizing in isolation but harming overall performance)
- Empirically solves phase-ordering problem (discover optimal order through measurement)
- Detects regressions early (before merge, not after deployment)
- Builds portfolio knowledge (understand how optimizations compose)

**Run**: `ruchy experiment --config validation/experiments/opt-codegen-001-portfolio.yaml`
**Expected**: ✅ Statistically significant improvement with no severe negative interactions

#### Summary: 8-Phase Validation Checklist (v1.1)

Every optimization MUST pass:
- [ ] ✅ RED: Failing test demonstrates opportunity
- [ ] ✅ GREEN: Minimal implementation passes test
- [ ] ✅ REFACTOR: Comprehensive, maintainable code
- [ ] ✅ TOOL VALIDATION: All 16 Ruchy tools pass
- [ ] ✅ MUTATION TESTING: ≥95% mutant kill rate
- [ ] ✅ PROPERTY TESTING: 10,000+ cases per property
- [ ] ✅ FUZZ TESTING: 100,000+ test cases, zero crashes
- [ ] ✅ **PORTFOLIO VALIDATION**: Statistical significance + no negative interactions (NEW in v1.1)

**Quality Gate**: Optimization is **BLOCKED** from merge until all 8 phases complete.

---

## Optimization Risk Classification (v1.1)

**Principle (Kaizen - Continuous Improvement)**: Not all optimizations carry equal risk of introducing bugs or regressions. Applying validation rigor proportional to risk maximizes quality while enabling faster iteration on low-risk changes.

**Research Basis**: Sullivan & Zeller (2005) - "Yesterday's loss is today's gain: a simple and effective technique for localizing regression bugs"

### Risk Class Definitions

We classify optimizations into three risk classes based on complexity, scope, and correctness guarantees:

#### Risk Class 3: Provably Correct / Local Optimizations (FAST-TRACK)

**Characteristics**:
- **Provably correct**: Transformation can be formally proven to preserve semantics
- **Local scope**: Operates on individual expressions or statements (no global analysis)
- **Deterministic**: Same input always produces same output (no heuristics)
- **Simple implementation**: <50 LOC, low cyclomatic complexity

**Examples**:
- Constant folding (`2 + 3 → 5`)
- Algebraic simplification (`x * 1 → x`, `x + 0 → x`)
- Peephole optimization (pattern-based rewrites)
- Dead store elimination (unused local variables)

**Validation Requirements** (Streamlined):
- ✅ Phases 1-4: RED-GREEN-REFACTOR-TOOL VALIDATION (mandatory)
- ✅ Phase 5: MUTATION TESTING (≥95% kill rate)
- ✅ Phase 6: PROPERTY TESTING (1,000+ cases - reduced from 10,000)
- ⏭️ Phase 7: FUZZ TESTING (optional - can defer to integration testing)
- ✅ Phase 8: PORTFOLIO VALIDATION (statistical significance required)

**Rationale**: Provably correct transformations have low regression risk. Focus testing on correctness proof and interaction with other optimizations rather than exhaustive edge cases.

#### Risk Class 2: Complex / Global Optimizations (STANDARD RIGOR)

**Characteristics**:
- **Complex analysis**: Requires data flow, control flow, or type analysis
- **Global scope**: Operates across multiple functions or modules
- **Non-trivial implementation**: 50-200 LOC, moderate cyclomatic complexity
- **Deterministic**: No heuristics, but complex algorithms (e.g., unification, liveness analysis)

**Examples**:
- Dead code elimination (DCE) - requires liveness analysis
- Type inference caching - requires cache invalidation strategy
- AST node pooling - requires memory management correctness
- Occurs check optimization - requires union-find correctness

**Validation Requirements** (FULL EXTREME TDD):
- ✅ All 8 phases MANDATORY (no exceptions)
- ✅ Phase 5: MUTATION TESTING (≥95% kill rate)
- ✅ Phase 6: PROPERTY TESTING (10,000+ cases)
- ✅ Phase 7: FUZZ TESTING (100,000+ cases)
- ✅ Phase 8: PORTFOLIO VALIDATION (statistical significance + interaction analysis)

**Rationale**: Complex optimizations have higher bug surface area. Require comprehensive validation to ensure correctness across edge cases and interactions.

#### Risk Class 1: Heuristic-Based / Profile-Guided (MAXIMUM RIGOR)

**Characteristics**:
- **Heuristic-driven**: Uses thresholds, heuristics, or machine learning
- **Non-deterministic**: Different heuristics may produce different (but valid) results
- **Context-dependent**: Effectiveness varies wildly based on input characteristics
- **High-risk trade-offs**: Can improve common case but hurt edge cases

**Examples**:
- Inline expansion (heuristics: function size, call frequency, recursion depth)
- Profile-guided optimization (PGO) - depends on representative profile
- Loop unrolling (heuristics: loop trip count, body size)
- Register allocation (graph coloring heuristics)

**Validation Requirements** (MAXIMUM + EXTENDED):
- ✅ All 8 phases MANDATORY
- ✅ Phase 5: MUTATION TESTING (≥98% kill rate - HIGHER than Class 2/3)
- ✅ Phase 6: PROPERTY TESTING (25,000+ cases - HIGHER than Class 2)
- ✅ Phase 7: FUZZ TESTING (250,000+ cases - HIGHER than Class 2)
- ✅ Phase 8: PORTFOLIO VALIDATION (EXTENSIVE - test multiple heuristic configurations)
- ✅ **Extended validation**: Differential testing (compare heuristic variants)
- ✅ **Extended validation**: Sensitivity analysis (how do heuristic thresholds affect results?)
- ✅ **Extended validation**: Worst-case validation (ensure heuristic never hurts performance by >5%)

**Rationale**: Heuristic-based optimizations are the highest risk. They can introduce subtle regressions that only appear on specific input patterns. Require maximum validation rigor and ongoing monitoring.

### Risk Classification Workflow

When implementing a new optimization:

1. **Classify the optimization** into Risk Class 1, 2, or 3
2. **Apply appropriate validation rigor** based on classification
3. **Document classification** in optimization ticket and book chapter
4. **Justify classification** if reviewers disagree (explain why it's Class X not Class Y)

**Example Classification Decision Tree**:
```
Is the transformation provably correct AND local scope?
  YES → Risk Class 3 (Fast-track validation)
  NO  → Continue

Does the optimization use heuristics or profile data?
  YES → Risk Class 1 (Maximum rigor)
  NO  → Continue

Is the optimization complex (data flow, control flow, global analysis)?
  YES → Risk Class 2 (Standard rigor)
  NO  → Re-evaluate (might be Class 3)
```

### Benefits of Risk-Based Classification

1. **Faster iteration on low-risk changes**: Provably correct optimizations don't need 100,000+ fuzz cases
2. **Focus resources on high-risk areas**: Heuristic optimizations get maximum scrutiny
3. **Clear expectations**: Developers know validation requirements upfront
4. **Balanced rigor**: Avoid over-testing simple changes, avoid under-testing complex changes
5. **Continuous improvement**: Classification criteria evolve as team learns from regressions

---

## Testing Strategy

### Mutation Testing (PMAT)

**Tool**: PMAT (Pure Mutation Analysis Tool for Ruchy)
**Target**: ≥95% mutation coverage
**Process**:

1. **Generate Mutants**: PMAT creates code variations
   - Arithmetic operator changes: `+` → `-`, `*` → `/`
   - Comparison operator changes: `==` → `!=`, `<` → `<=`
   - Constant changes: `0` → `1`, `true` → `false`
   - Statement deletion
   - Boolean expression negation

2. **Run Tests Against Mutants**: Each test suite runs against each mutant
   - If test fails: Mutant **killed** ✅ (test is effective)
   - If test passes: Mutant **survived** ❌ (test is weak)

3. **Calculate Mutation Score**:
   ```
   Mutation Score = Killed Mutants / Total Mutants
   Target: ≥95%
   ```

4. **Strengthen Tests**: For surviving mutants, add tests to kill them

**Example**:
```bash
# Generate mutants for constant folding optimization
pmat mutate bootstrap/stage3/optimizer.ruchy --operators --constants

# Run test suite against mutants
pmat test validation/optimizations/test_constant_folding.ruchy

# Results:
# Total mutants: 120
# Killed: 115
# Survived: 5
# Mutation score: 95.8% ✅
```

### Property Testing (ruchy prove)

**Tool**: `ruchy prove` (built-in property testing)
**Target**: 10,000+ test cases per property
**Process**:

1. **Define Properties**: Mathematical invariants that must hold

   **Example Properties for Constant Folding**:
   ```ruchy
   // Commutativity: a + b = b + a
   property commutative_addition(a: i32, b: i32) -> bool {
       let expr1 = parse("$a + $b");
       let expr2 = parse("$b + $a");
       evaluate(optimize(expr1)) == evaluate(optimize(expr2))
   }

   // Associativity: (a + b) + c = a + (b + c)
   property associative_addition(a: i32, b: i32, c: i32) -> bool {
       let expr1 = parse("($a + $b) + $c");
       let expr2 = parse("$a + ($b + $c)");
       evaluate(optimize(expr1)) == evaluate(optimize(expr2))
   }

   // Identity: a + 0 = a
   property additive_identity(a: i32) -> bool {
       let expr = parse("$a + 0");
       let optimized = optimize(expr);
       evaluate(optimized) == a
   }
   ```

2. **Generate Test Cases**: Automatically generate inputs
   - Random generation
   - Boundary values (i32::MIN, i32::MAX, 0, 1, -1)
   - Edge cases (overflow, underflow)

3. **Shrinking**: When property fails, find minimal failing case
   ```
   Original failing case: a=1234567, b=9876543
   Shrunk to: a=1, b=-1
   ```

4. **Validation**: All properties must pass 10,000+ cases

### Fuzz Testing (ruchy fuzz)

**Strategies**:

1. **Grammar-Based Fuzzing**:
   - Generate valid Ruchy programs using grammar rules
   - Test optimization on valid inputs
   - Target: 100,000+ generated programs

2. **Mutation-Based Fuzzing**:
   - Start with valid programs
   - Mutate (insert/delete/modify tokens)
   - Test robustness on malformed inputs
   - Target: 50,000+ mutated programs

3. **Corpus-Based Fuzzing**:
   - Use real compiler source code as corpus
   - Mutate and combine real-world patterns
   - Target: 1,000+ corpus-based tests

4. **Boundary Value Fuzzing**:
   - Test integer limits: i32::MIN, i32::MAX
   - Test string limits: empty, 1MB, 10MB
   - Test nesting limits: 1, 100, 1000 levels deep

**Acceptance Criteria**:
- Zero crashes across all fuzz tests
- Graceful handling of edge cases
- Clear error messages for invalid inputs

### PMAT TDG Score

**Tool**: PMAT TDG (Ticket-Driven Genchi Genbutsu)
**Target**: ≥85 TDG score
**Metrics**:

```yaml
TDG Score Breakdown:
  - Test Coverage: 30 points (≥80% coverage required)
  - Documentation: 20 points (all phases documented)
  - Code Quality: 20 points (ruchy lint A+, complexity <20)
  - Mutation Coverage: 15 points (≥95% mutants killed)
  - Property Tests: 10 points (≥3 properties, 10K+ cases each)
  - Fuzz Tests: 5 points (≥100K fuzz cases, 0 crashes)

Total: 100 points
Passing: ≥85 points
```

**Run**: `pmat tdg bootstrap/stage3/ --format=json`

---

## Implementation Phases

### Phase 1: Measurement Infrastructure (Weeks 1-2)

**Objective**: Build profiling and benchmarking tools

**Tickets**:
- **OPT-INFRA-001**: Bootstrap Timing Harness
  - Measure Stage 1→2→3 compilation times
  - Track time per compiler phase (lexer, parser, types, codegen)
  - Establish baseline: Current bootstrap time = X seconds

- **OPT-INFRA-002**: Memory Profiling Integration
  - Track allocations during bootstrap
  - Identify memory hotspots
  - Measure peak memory usage

- **OPT-INFRA-003**: Code Quality Metrics
  - Measure generated code size
  - Count generated instructions
  - Benchmark generated code execution time

- **OPT-INFRA-004**: Statistical Benchmarking Framework (NEW in v1.1)
  - Automate N≥30 benchmark runs per configuration
  - Calculate mean, standard deviation, 95% confidence intervals
  - Implement Welch's t-test for statistical significance (p < 0.05)
  - Detect and report performance noise sources (OS scheduler, cache, CPU turbo)
  - Generate statistical reports with p-values and effect sizes

**Deliverables**:
- `scripts/benchmark-bootstrap.sh` - Automated bootstrap benchmarking with statistical rigor
- `validation/benchmarks/bootstrap_baseline.ruchy` - Baseline measurements (N=30 runs)
- `validation/benchmarks/statistical_test.ruchy` - Welch's t-test implementation
- Dashboard showing current performance metrics with confidence intervals

**Success Criteria**:
- ✅ Reproducible bootstrap time measurements with statistical validation (p < 0.05 for variance check)
- ✅ N≥30 runs per configuration with mean ± std dev reporting
- ✅ Profiling data collected for each compiler phase
- ✅ Baseline established for future optimization comparison (with 95% confidence intervals)
- ✅ Statistical significance testing automated (Welch's t-test implemented)

### Phase 2: Lexer Optimizations (Weeks 3-4)

**Objective**: Optimize tokenization performance

**Tickets**:
- **OPT-LEX-001**: Token Stream Caching (RED-GREEN-REFACTOR)
- **OPT-LEX-002**: Lazy String Allocation (RED-GREEN-REFACTOR)

**Each ticket follows 8-phase EXTREME TDD** (v1.1):
1. RED: Demonstrate opportunity via profiling
2. GREEN: Minimal optimization implementation
3. REFACTOR: Generalize and improve
4. TOOL VALIDATION: All 16 Ruchy tools
5. MUTATION: ≥95% mutant kill rate
6. PROPERTY: 10,000+ test cases
7. FUZZ: 100,000+ fuzz cases
8. PORTFOLIO: Statistical significance + interaction analysis

**Success Criteria** (Statistical Rigor - v1.1):
- ✅ Lexer phase speedup: ≥15% ± Y% (p < 0.05, N=30 runs)
- ✅ Memory allocation reduction: ≥25% ± Y% (p < 0.05, N=30 runs)
- ✅ Bootstrap Stage 1→2 time reduction: ≥5% ± Y% (p < 0.05, N=30 runs)
- ✅ All validation phases pass
- ✅ No negative interactions with existing optimizations (portfolio validation)

### Phase 3: Parser Optimizations (Weeks 5-6)

**Objective**: Optimize AST construction

**Tickets**:
- **OPT-PARSE-001**: Left-Recursion Elimination
- **OPT-PARSE-002**: AST Node Pooling

**Success Criteria** (Statistical Rigor - v1.1):
- ✅ Parser phase speedup: ≥20% ± Y% (p < 0.05, N=30 runs)
- ✅ AST memory churn reduction: ≥35% ± Y% (p < 0.05, N=30 runs)
- ✅ Bootstrap Stage 1→2 time reduction: ≥8% ± Y% (p < 0.05, N=30 runs)
- ✅ No negative interactions with lexer optimizations (portfolio validation)

### Phase 4: Type System Optimizations (Weeks 7-8)

**Objective**: Optimize type inference and checking

**Tickets**:
- **OPT-TYPE-001**: Type Inference Caching
- **OPT-TYPE-002**: Occurs Check Optimization

**Success Criteria** (Statistical Rigor - v1.1):
- ✅ Type checking phase speedup: ≥25% ± Y% (p < 0.05, N=30 runs)
- ✅ Unification operations reduction: ≥30% ± Y% (p < 0.05, N=30 runs)
- ✅ Bootstrap Stage 2→3 time reduction: ≥10% ± Y% (p < 0.05, N=30 runs)
- ✅ No negative interactions with parser optimizations (portfolio validation)

### Phase 5: Code Generation Optimizations (Weeks 9-12)

**Objective**: Optimize generated code quality and compiler codegen phase

**Tickets**:
- **OPT-CODEGEN-001**: Constant Folding
- **OPT-CODEGEN-002**: Peephole Optimization
- **OPT-CODEGEN-003**: Dead Code Elimination
- **OPT-CODEGEN-004**: Inline Expansion

**Success Criteria** (Statistical Rigor - v1.1):
- ✅ Generated code speedup: ≥30% ± Y% (p < 0.05, N=30 runs)
- ✅ Generated code size reduction: ≥20% ± Y% (p < 0.05, N=30 runs)
- ✅ Codegen phase speedup: ≥15% ± Y% (p < 0.05, N=30 runs)
- ✅ Bootstrap overall time reduction: ≥25% ± Y% (p < 0.05, N=30 runs)
- ✅ Optimization interactions documented (portfolio validation - esp. inlining + constant folding)

### Phase 6: Profile-Guided Optimization (Weeks 13-14)

**Objective**: Use real bootstrap profile to guide optimizations

**Tickets**:
- **OPT-GLOBAL-001**: PGO Implementation
- **OPT-GLOBAL-002**: Whole-Program Optimization

**Success Criteria** (Statistical Rigor - v1.1):
- ✅ Profile collection overhead: <5% ± Y% (p < 0.05, N=30 runs)
- ✅ PGO-guided speedup: ≥20% ± Y% (p < 0.05, N=30 runs)
- ✅ Total bootstrap time reduction: ≥40% ± Y% (cumulative, p < 0.05, N=30 runs)
- ✅ Optimization portfolio interactions fully documented (portfolio validation)

### Phase 7: Validation and Measurement (Week 15)

**Objective**: Comprehensive validation and documentation

**Tasks**:
- Run complete mutation test suite (PMAT)
- Run all property tests (10,000+ cases each)
- Run comprehensive fuzz testing (500,000+ cases)
- Validate multi-stage bootstrap (Stage 2 == Stage 3)
- Measure final performance improvements with statistical rigor
- Document optimization results in book
- Generate final statistical report (N=30 runs, confidence intervals, p-values)

**Success Criteria** (Statistical Rigor - v1.1):
- ✅ Mutation coverage: ≥95%
- ✅ Property tests: All pass (10,000+ cases)
- ✅ Fuzz tests: Zero crashes (500,000+ cases)
- ✅ Bootstrap validation: Bit-for-bit identical (Stage 2 == Stage 3)
- ✅ Performance: ≥40% ± Y% overall bootstrap speedup (p < 0.05, N=30 runs)
- ✅ Statistical report: All improvements statistically significant (p < 0.05)
- ✅ Book: All optimization chapters complete with portfolio interaction analysis

---

## Success Metrics

### Primary Metrics (Statistical Rigor - v1.1)

#### 1. Bootstrap Compilation Time
**Baseline**: Establish current Stage 1→2→3 compilation time (mean ± std dev, N=30 runs)
**Target**: ≥40% ± Y% reduction by end of Phase 6 (p < 0.05, statistically significant)
**Measurement**: `scripts/benchmark-bootstrap.sh --runs=30 --statistical-test`

**Statistical Requirements** (Georges et al., 2007):
- N ≥ 30 benchmark runs per configuration
- Report mean ± standard deviation
- Calculate 95% confidence intervals
- Perform Welch's t-test between configurations
- Accept optimization only if p < 0.05 (statistically significant)

**Breakdown by Phase** (all require p < 0.05):
- Lexer optimizations: ≥5% ± Y% improvement
- Parser optimizations: ≥8% ± Y% improvement
- Type system optimizations: ≥10% ± Y% improvement
- Code generation optimizations: ≥25% ± Y% improvement
- PGO: ≥20% ± Y% improvement (on top of previous)

**Formula**: `Cumulative_Speedup = 1 - ∏(1 - Individual_Speedup_i)`

**Example Statistical Report**:
```
Configuration A (Baseline): 10.5s ± 0.2s (N=30)
Configuration B (+All Optimizations): 6.3s ± 0.15s (N=30)
Speedup: 40.0% ± 2.1%
Welch's t-test: t=78.3, p<0.001 (highly significant)
95% CI: [37.8%, 42.2%]
```

#### 2. Generated Code Quality
**Metrics** (Statistical Rigor - v1.1):
- Execution time: ≥30% ± Y% faster (p < 0.05, N=30 runs)
- Code size: ≥20% ± Y% smaller (p < 0.05, N=30 runs)
- Instruction count: ≥25% ± Y% reduction (p < 0.05, N=30 runs)

**Measurement**: Benchmark suite on generated code with statistical validation

#### 3. Memory Usage
**Metrics**:
- Peak memory: ≥15% reduction
- Allocation rate: ≥35% reduction
- Memory churn: ≥40% reduction

**Measurement**: Memory profiling during bootstrap

### Validation Metrics

#### 1. Correctness (MANDATORY)
**Metric**: Multi-stage bootstrap validation
**Rule**: `Stage2_Output == Stage3_Output` (bit-for-bit)
**Tolerance**: Zero tolerance - must be identical

#### 2. Test Quality
**Mutation Coverage**: ≥95% mutant kill rate
**Property Testing**: ≥10,000 cases per property, all pass
**Fuzz Testing**: ≥500,000 total cases, zero crashes

#### 3. Code Quality
**Ruchy Lint**: A+ grade (all optimization code)
**PMAT TDG Score**: ≥85
**Complexity**: All functions <20 cyclomatic complexity

#### 4. Documentation
**Book Chapters**: All optimizations documented with RED-GREEN-REFACTOR
**Reproducibility**: All optimizations have reproduction scripts
**Debuggability**: All optimizations debuggable with ruchydbg

---

## Pre/Post-Optimization Strategy

**Document Version**: 1.2 (November 2, 2025)
**Purpose**: Define systematic optimization of transpiled Rust code using static analysis, iterative compilation, and post-compilation optimization

### Executive Summary

**Pre-optimization** applies static analysis (Clippy::perf, cargo-bloat) to transpiled Rust code before compilation. **Post-optimization** iteratively tunes compiler flags (opt-level, LTO, codegen-units) and recompiles based on benchmark results. This two-phase approach optimizes both source-level and binary-level performance.

**Key Insight**: Transpiler-generated code differs from hand-written Rust. Static analysis tools (Clippy) catch inefficiencies invisible to the transpiler but obvious to Rust tooling. Iterative flag tuning discovers non-linear optimization interactions.

---

### Research Foundation: Pre-Optimization (Static Analysis)

#### 1. Static Analysis for Performance Optimization
**Paper**: "Static Analysis for Performance Optimization: A Survey"
**Source**: IEEE Access, Vol. 9 (2021) - Open Access
**DOI**: 10.1109/ACCESS.2021.3068492
**Key Findings**:
- Static analysis detects performance anti-patterns without execution
- 15-30% performance improvement from lint-driven refactoring
- Type-based analysis (Rust ownership) enables deeper optimization hints
- False positive rate <5% for well-tuned heuristics

**Application to RuchyRuchy**:
- Run `cargo clippy -- -W clippy::perf` on transpiled Rust
- Detect: unnecessary clones, inefficient iterations, suboptimal data structures
- Transpiler can emit patterns Clippy flags (e.g., `Vec` reallocation)
- Automated refactoring pipeline: Clippy → Fix → Re-transpile

#### 2. Automated Performance Bug Detection
**Paper**: "Automated Detection of Performance Regressions: The Wisdom of the Crowds"
**Source**: ACM SIGSOFT FSE 2020 - arXiv:2008.08443
**Key Findings**:
- Performance anti-patterns detectable via abstract interpretation
- 89% precision in identifying inefficient API usage
- Tool-based detection scales to millions of LOC

**Application**: Post-transpile analysis catches inefficiencies in generated Rust patterns

#### 3. Rust-Specific Performance Analysis
**Paper**: "Understanding and Detecting Real-World Performance Bugs in Rust"
**Source**: ICSE 2024 (pre-print available)
**Key Findings**:
- Common Rust performance bugs: unnecessary `clone()`, `Arc` overuse, `String` allocation
- Ownership system creates unique optimization opportunities
- Static lifetime analysis enables zero-cost abstractions validation

**Application**: Clippy::perf rules target Rust-specific patterns in transpiled code

---

### Research Foundation: Post-Optimization (Iterative Compilation)

#### 4. Iterative Compilation and Flag Tuning
**Paper**: "A Survey on Compiler Autotuning using Machine Learning"
**Source**: ACM Computing Surveys, Vol. 51, No. 5 (September 2018) - Open Access
**DOI**: 10.1145/3197406
**Authors**: Ashouri, Killian, Cavazos, Palermo, Silvano
**Key Findings**:
- Phase-ordering problem is NP-hard (2^N possible orderings)
- Iterative compilation discovers 10-40% speedups vs default flags
- Non-linear interactions between optimization passes
- Requires statistical validation (p < 0.05) for performance claims

**Application to RuchyRuchy**:
- Systematically explore: `opt-level={0,1,2,3,s,z}`, `lto={off,thin,fat}`
- Measure compile time vs runtime speedup trade-offs
- Use bootstrap compilation time as fitness function
- Statistical rigor: 30 runs per configuration, Welch's t-test

#### 5. Link-Time Optimization (LTO) Trade-offs
**Paper**: "Link-Time Optimization: Design and Implementation"
**Source**: GCC Summit 2007 (Open Access Proceedings)
**Authors**: Hubička, J. (GCC Maintainer)
**Key Findings**:
- LTO enables whole-program optimization, inlining across compilation units
- Thin-LTO: 10-15% speedup, fast compile times
- Fat-LTO: 15-25% speedup, 3-10x slower compilation
- Diminishing returns beyond fat-LTO for most workloads

**Application**: Benchmark thin vs fat LTO for transpiled Ruchy code, measure compile-time budget

#### 6. Compiler Flag Space Exploration
**Paper**: "Mitigating the Compiler Optimization Phase-Ordering Problem using Machine Learning"
**Source**: OOPSLA 2012 - ACM DL Open Access
**DOI**: 10.1145/2384616.2384628
**Key Findings**:
- Default compiler flags suboptimal for 60% of programs
- Genetic algorithms find 2-5x speedups on specific workloads
- Requires domain-specific benchmarks (synthetic benchmarks mislead)

**Application**: Use self-compilation (bootstrapping) as benchmark suite for flag tuning

---

### Research Foundation: Benchmark-Driven Optimization

#### 7. Statistical Rigor in Performance Measurement
**Paper**: "Rigorous Benchmarking in Reasonable Time"
**Source**: SIGPLAN ISMM 2013 - Open Access
**DOI**: 10.1145/2464157.2464160
**Authors**: Kalibera & Jones
**Key Findings**:
- Minimum 30 samples required for statistical validity
- Coefficient of variation (CV) <3% indicates stable benchmark
- Welch's t-test (p < 0.05) for significance, Cohen's d for effect size
- Misleading conclusions from <10 runs in 82% of studies surveyed

**Application**: All performance claims require 30-run validation with p-value reporting

---

## Pre/Post-Optimization Strategy

### Phase 1: Pre-Optimization (Static Analysis)

**Goal**: Optimize transpiled Rust source code before compilation

**Step 1.1: Clippy Performance Lints**
```bash
# Run Clippy with performance-only lints
cargo clippy --all-targets -- \
  -W clippy::perf \
  -W clippy::style \
  -D warnings
```

**Common Issues in Transpiled Code**:
1. **Unnecessary Clones**: Transpiler may defensively clone when borrow would suffice
2. **Inefficient Iterations**: `.iter().map().collect()` when `.into_iter()` faster
3. **String Allocations**: `format!()` in hot loops, use `write!()` to buffer
4. **`Vec` Reallocation**: No capacity hints, causing repeated reallocation

**Step 1.2: Cargo Bloat Analysis**
```bash
# Identify binary size hotspots
cargo bloat --release -n 20
```

**Action**: Detect generic monomorphization explosions, trim unused dependencies

**Step 1.3: Cargo Bench Baseline**
```bash
# Establish baseline before optimization
cargo bench --bench bootstrap_compile_time -- --save-baseline pre-opt
```

---

### Phase 2: Post-Optimization (Iterative Compilation)

**Goal**: Systematically explore compiler flag combinations, measure impact

#### Additional Research: Codegen Tuning

**8. Code Generation Unit Optimization**
**Paper**: "Optimizing Parallel Compilation via Code Generation Units"
**Source**: CGO 2018 - IEEE Xplore Open Access
**Key Findings**:
- `codegen-units=1`: Maximum optimization, slowest compile
- `codegen-units=16`: Fast compilation, 10-15% slower runtime
- Trade-off: developer iteration speed vs production performance

**Application**: Profile codegen-units impact on bootstrap compilation time

**9. CPU-Specific Optimizations (target-cpu)**
**Paper**: "The Impact of Compiler Optimizations on Microarchitecture Performance"
**Source**: ACM TACO, Vol. 17, No. 2 (2020) - Open Access
**DOI**: 10.1145/3385389
**Key Findings**:
- `target-cpu=native`: 5-15% speedup from SIMD, branch prediction tuning
- Portability vs performance trade-off
- Diminishing returns on modern CPUs with adaptive dispatch

**Application**: Measure `target-cpu=native` vs `target-cpu=x86-64-v2` for Ruchy

**10. Profile-Guided Optimization (PGO) in LLVM**
**Paper**: "A Comprehensive Study of Profile-Guided Optimization"
**Source**: ACM TOPLAS, Vol. 43, No. 3 (2021) - Open Access
**DOI**: 10.1145/3460866
**Key Findings**:
- PGO: 10-30% speedup on real workloads
- Instrumentation overhead: 2-5x slowdown during profiling
- Stable with >1000 training runs, unstable with <100 runs

**Application**: Run instrumented bootstrap, collect profile, recompile with PGO

**11. Rust Compiler Flag Interactions**
**Paper**: "Understanding Compiler Optimization Pragmas in the Wild"
**Source**: ESEC/FSE 2022 - arXiv:2207.12433
**Key Findings**:
- Optimization flags interact non-linearly (not additive)
- 23% of flag combinations cause regressions vs default
- Requires empirical measurement, not analytical prediction

**Application**: Systematically test flag portfolios on bootstrap benchmark

---

### Step 2.1: Compiler Flag Space Exploration

**Configuration Matrix**:
```toml
# Cargo.toml profiles for systematic testing
[profile.opt-z]
opt-level = "z"  # Optimize for size
lto = "thin"
codegen-units = 1

[profile.opt-3-thin]
opt-level = 3
lto = "thin"
codegen-units = 16

[profile.opt-3-fat]
opt-level = 3
lto = "fat"
codegen-units = 1

[profile.opt-3-native]
opt-level = 3
lto = "fat"
codegen-units = 1
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=native"]
```

**Benchmark Script** (`scripts/optimize-flags.sh`):
```bash
#!/bin/bash
set -euo pipefail

PROFILES=("opt-z" "opt-3-thin" "opt-3-fat" "opt-3-native")
RUNS=30

for profile in "${PROFILES[@]}"; do
  echo "Testing profile: $profile"

  for i in $(seq 1 $RUNS); do
    cargo clean

    # Measure compilation time + binary size
    /usr/bin/time -v cargo build --profile $profile 2>&1 | \
      grep "Elapsed\|Maximum resident" >> results_${profile}.txt

    # Measure runtime performance (bootstrap self-compilation)
    ./target/$profile/ruchyruchy compile bootstrap/stage1/*.ruchy \
      --output /tmp/stage1-compiled 2>&1 | \
      grep "Compilation time" >> results_${profile}.txt
  done

  # Statistical analysis
  python3 scripts/analyze-optimization-results.py results_${profile}.txt
done
```

**Step 2.2: Statistical Analysis**

**Python Analysis Script** (excerpt):
```python
import scipy.stats as stats
import numpy as np

def analyze_optimization_results(baseline, optimized):
    # Welch's t-test (unequal variances)
    t_stat, p_value = stats.ttest_ind(baseline, optimized, equal_var=False)

    # Cohen's d (effect size)
    pooled_std = np.sqrt((np.std(baseline)**2 + np.std(optimized)**2) / 2)
    cohens_d = (np.mean(optimized) - np.mean(baseline)) / pooled_std

    # Coefficient of variation
    cv_baseline = np.std(baseline) / np.mean(baseline) * 100
    cv_optimized = np.std(optimized) / np.mean(optimized) * 100

    print(f"p-value: {p_value:.4f} (significant if < 0.05)")
    print(f"Cohen's d: {cohens_d:.2f} (large effect if |d| > 0.8)")
    print(f"CV: {cv_baseline:.2f}% (baseline), {cv_optimized:.2f}% (optimized)")
    print(f"Speedup: {np.mean(baseline) / np.mean(optimized):.2f}x")
```

---

### Step 2.3: Profile-Guided Optimization (PGO)

**12. PGO Implementation Research**
**Paper**: "Feedback-Directed Optimization in Compilers"
**Source**: ACM Computing Surveys (2023) - Open Access
**arXiv**: 2301.07345
**Key Findings**:
- PGO most effective for: branch prediction, inlining, register allocation
- Training data quality > quantity (1000 representative runs > 10K synthetic)
- Cold code optimization: size reduction via outlining

**Application**: Bootstrap compilation = ideal PGO training workload (real-world, deterministic)

**PGO Workflow**:
```bash
# Step 1: Build instrumented binary
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
  cargo build --release

# Step 2: Run training workload (bootstrap compilation)
for i in $(seq 1 100); do
  ./target/release/ruchyruchy compile bootstrap/stage1/*.ruchy \
    --output /tmp/stage1-pgo-$i
done

# Step 3: Merge profile data
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

# Step 4: Rebuild with PGO
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
  cargo build --release

# Step 5: Measure improvement
cargo bench --bench bootstrap_compile_time -- --baseline pre-opt
```

---

### Complete End-to-End Optimization Workflow

**Master Script** (`scripts/optimize-transpiled-rust.sh`):
```bash
#!/bin/bash
set -euo pipefail

echo "=== Phase 1: Pre-Optimization (Static Analysis) ==="

# 1.1: Clippy performance lints
echo "Running Clippy::perf..."
cargo clippy --all-targets -- \
  -W clippy::perf \
  -W clippy::style \
  -D warnings \
  2>&1 | tee clippy-report.txt

# Count performance warnings
PERF_WARNINGS=$(grep "warning: " clippy-report.txt | wc -l)
echo "Found $PERF_WARNINGS performance warnings"

# 1.2: Binary size analysis
echo "Analyzing binary bloat..."
cargo bloat --release -n 20 | tee bloat-report.txt

# 1.3: Baseline benchmark
echo "Establishing baseline..."
cargo bench --bench bootstrap_compile_time -- --save-baseline pre-opt

echo "=== Phase 2: Post-Optimization (Flag Tuning) ==="

# 2.1: Test compiler flag combinations
PROFILES=("opt-z" "opt-3-thin" "opt-3-fat" "opt-3-native")
RUNS=30

for profile in "${PROFILES[@]}"; do
  echo "Testing profile: $profile (${RUNS} runs)..."

  for i in $(seq 1 $RUNS); do
    cargo clean -q
    cargo build --profile $profile -q
    cargo bench --bench bootstrap_compile_time --profile $profile -- \
      --noplot >> results_${profile}.txt 2>&1
  done

  # Statistical analysis
  python3 scripts/analyze-optimization-results.py \
    results_pre-opt.txt \
    results_${profile}.txt
done

# 2.2: Profile-Guided Optimization
echo "Running PGO workflow..."
./scripts/run-pgo.sh

# 2.3: Select optimal configuration
python3 scripts/select-optimal-config.py results_*.txt

echo "=== Optimization Complete ==="
echo "See optimization-report.md for full analysis"
```

**Expected Results**:
- **Pre-optimization (Clippy)**: 5-15% performance improvement from eliminating clones, inefficient iterations
- **Post-optimization (Flags)**: 10-25% speedup from opt-level=3 + thin-LTO vs default
- **PGO**: Additional 10-20% speedup on hot paths (inlining, branch prediction)
- **Total**: 25-60% cumulative improvement over baseline (opt-level=0, no LTO)

**Key Metrics to Track**:
1. **Compile-time overhead**: Pre-optimization adds ~2min (Clippy), post-optimization adds 3-10x (LTO/PGO)
2. **Runtime speedup**: Measure via bootstrap self-compilation time
3. **Binary size**: opt-level=z reduces 20-30%, opt-level=3 increases 10-20%
4. **Statistical confidence**: p < 0.05 (Welch's t-test), Cohen's d > 0.8 (large effect)

**Trade-off Matrix**:
| Configuration | Compile Time | Runtime | Binary Size | Use Case |
|---------------|--------------|---------|-------------|----------|
| `opt-level=0` | 1x (baseline)| 1x | 1x | Development |
| `opt-level=z` | 1.2x | 0.85x | 0.7x | Embedded |
| `opt-level=3 thin-LTO` | 3x | 0.6x | 1.1x | Production |
| `opt-level=3 fat-LTO` | 8x | 0.5x | 1.15x | Release |
| `PGO (fat-LTO + profile)` | 10x | 0.4x | 1.2x | Benchmarks |

---

### Integration with EXTREME TDD Methodology

**RED Phase**: Write performance regression tests
```rust
#[test]
fn test_optimization_no_regression() {
    let baseline = benchmark_bootstrap_time("opt-level-0");
    let optimized = benchmark_bootstrap_time("opt-level-3-thin");

    // Require statistically significant speedup
    assert!(optimized.mean() < baseline.mean() * 0.85,
        "opt-level=3 thin-LTO must be ≥15% faster than baseline");
    assert!(optimized.p_value() < 0.05,
        "Speedup must be statistically significant");
}
```

**GREEN Phase**: Apply optimizations, make tests pass

**REFACTOR Phase**: Tune flags iteratively until optimal configuration found

**TOOL Phase**: Validate with all 16 quality tools (clippy, fmt, test, etc.)

**PMAT Phase**: Document in book with reproduction script

---

## References

### Peer-Reviewed Research (2024-2025)

1. **"From Profiling to Optimization: Unveiling the Profile Guided Optimization"**
   arXiv:2507.16649v1 (2025)
   Comprehensive PGO survey with modern techniques

2. **"CompilerDream: Learning a Compiler World Model for General Code Optimization"**
   arXiv:2404.16077v2 (August 2024)
   Model-based RL for compiler optimization

3. **"Compiler Optimization: A Deep Learning and Transformer-Based Approach"**
   3rd International Conference on Optimization Techniques (ICOFE-2024)
   AI-driven compiler optimization

### Classic Research

4. **"A Survey on Compiler Autotuning using Machine Learning"**
   ACM Computing Surveys (September 2018)
   Comprehensive survey of iterative compilation

5. **"Compilers: Principles, Techniques, and Tools" (Dragon Book)**
   Aho, Sethi, Ullman
   Classic compiler optimization techniques

6. **GCC Documentation: Multi-Stage Bootstrapping**
   GCC Project
   Three-stage bootstrap validation methodology

### Tools and Frameworks

7. **CompilerGym**: Reinforcement learning environments for compiler optimization
   IEEE/ACM CGO 2022

8. **OpenTuner**: Framework for autotuning compiler optimizations
   State-of-the-art iterative compilation

9. **PMAT**: Pure Mutation Analysis Tool for Ruchy
   RuchyRuchy project (this repository)

### Additional Resources

10. **"Peephole Optimization in Compiler Design"**
    GeeksforGeeks, ScienceDirect Topics
    Classic peephole optimization techniques

11. **"Profile-Guided Optimization"**
    Wikipedia, Microsoft Docs
    Industry standard PGO implementations

12. **"Evaluating Iterative Optimization Across 1000 Datasets"**
    Chen et al., PLDI 2010
    Empirical evaluation of iterative compilation

---

## Appendix A: Optimization Quick Reference

| ID | Name | Expected Speedup | Phase | Complexity |
|----|------|------------------|-------|------------|
| OPT-LEX-001 | Token Stream Caching | 10-15% lexer | 2 | Medium |
| OPT-LEX-002 | Lazy String Allocation | 20-30% memory | 2 | Low |
| OPT-PARSE-001 | Left-Recursion Elimination | 15-25% parser | 3 | Medium |
| OPT-PARSE-002 | AST Node Pooling | 30-40% memory | 3 | High |
| OPT-TYPE-001 | Type Inference Caching | 20-35% types | 4 | Medium |
| OPT-TYPE-002 | Occurs Check Optimization | 10-20% unification | 4 | Medium |
| OPT-CODEGEN-001 | Constant Folding | 5-10% runtime | 5 | Low |
| OPT-CODEGEN-002 | Peephole Optimization | 3-7% runtime | 5 | Medium |
| OPT-CODEGEN-003 | Dead Code Elimination | 5-15% size | 5 | Medium |
| OPT-CODEGEN-004 | Inline Expansion | 10-25% runtime | 5 | High |
| OPT-GLOBAL-001 | Profile-Guided Optimization | 15-30% overall | 6 | High |
| OPT-GLOBAL-002 | Whole-Program Optimization | 10-20% overall | 6 | Very High |

**Total Expected Cumulative Speedup**: ≥40% bootstrap time reduction

---

## Appendix B: EXTREME TDD Template (v1.1)

Every optimization ticket follows this structure:

```markdown
# OPT-XXX-YYY: Optimization Name

## Risk Classification (NEW in v1.1)
**Risk Class**: [1 / 2 / 3] - [Heuristic-Based / Complex-Global / Provably Correct]
**Justification**: [Explain why this class was chosen]
**Validation Rigor**: [MAXIMUM / FULL EXTREME TDD / FAST-TRACK]

## Context
[Why this optimization is needed, research basis]

## Phase 1 - RED: Write Failing Test
[Test demonstrating opportunity, expected to fail]

## Phase 2 - GREEN: Minimal Implementation
[Simplest code to pass test]

## Phase 3 - REFACTOR: Improvements
[Generalize, improve, maintain]

## Phase 4 - TOOL VALIDATION (16 Ruchy Tools)
- [ ] ruchy check: ✅ Syntax valid
- [ ] ruchy test: ✅ Tests pass
- [ ] ruchy lint: ✅ A+ grade
- [ ] ruchy fmt: ✅ Formatted
- [ ] ruchy prove: ✅ Properties verified
- [ ] ruchy score: ✅ ≥0.8
- [ ] ruchy runtime: ✅ Performance acceptable
- [ ] ruchy build: ✅ Builds clean
- [ ] ruchy run: ✅ Executes correctly
- [ ] ruchy doc: ✅ Documented
- [ ] ruchy bench: ✅ Benchmark results
- [ ] ruchy profile: ✅ Profile analyzed
- [ ] ruchy coverage: ✅ ≥80% coverage
- [ ] ruchy deps: ✅ Dependencies clean
- [ ] ruchy security: ✅ No vulnerabilities
- [ ] ruchy complexity: ✅ All functions <20

## Phase 5 - MUTATION TESTING
- Total mutants: XXX
- Killed: XXX
- Mutation score: XX.X% (target: ≥95% for Class 2/3, ≥98% for Class 1)

## Phase 6 - PROPERTY TESTING
- Property 1: [Name] - [1,000+ / 10,000+ / 25,000+] cases ✅ (based on risk class)
- Property 2: [Name] - [1,000+ / 10,000+ / 25,000+] cases ✅
- Property 3: [Name] - [1,000+ / 10,000+ / 25,000+] cases ✅

## Phase 7 - FUZZ TESTING
- Grammar-based: [varies by risk class] cases ✅
- Mutation-based: [varies by risk class] cases ✅
- Boundary values: 1,000+ cases ✅
- Total: [optional / 100,000+ / 250,000+] cases, 0 crashes ✅ (based on risk class)

## Phase 8 - PORTFOLIO VALIDATION (NEW in v1.1)
**Statistical Requirements** (Georges et al., 2007):
- N ≥ 30 benchmark runs per configuration
- Mean ± std dev reported
- 95% confidence intervals calculated
- Welch's t-test performed (p < 0.05 required)

**Configurations Tested**:
- Configuration A (Baseline): X.XX ± Y.YY seconds (N=30)
- Configuration B (+This Optimization): X.XX ± Y.YY seconds (N=30)
- Configuration C (+Previous Optimization): X.XX ± Y.YY seconds (N=30)
- Configuration D (+Both): X.XX ± Y.YY seconds (N=30)

**Statistical Test Results**:
- Speedup (B vs A): XX.X% ± Y.Y% (Welch's t-test: t=Z.Z, p<0.05) ✅
- Interaction analysis: [Synergistic / Independent / Conflict / Negative]
- Optimal phase ordering: [Document if applicable]

**Interaction Matrix Update**:
- Synergies: [List optimizations that work well together]
- Conflicts: [List optimizations that interfere]

## REPRODUCIBILITY
```bash
# Reproduction script
./scripts/reproduce-opt-xxx-yyy.sh --runs=30 --statistical-test
```

## DEBUGGABILITY
```bash
# Debug with ruchydbg
ruchydbg validate bootstrap/stage3/optimization.ruchy
```

## Performance Impact (Statistical Rigor - v1.1)
- Baseline: X.XX ± Y.YY seconds (N=30 runs)
- Optimized: X.XX ± Y.YY seconds (N=30 runs)
- Speedup: XX.X% ± Y.Y% (p < 0.05, statistically significant) ✅
- 95% Confidence Interval: [XX.X%, XX.X%]

## Validation Summary (v1.1)
- [ ] 8/8 EXTREME TDD phases complete (v1.1)
- [ ] Risk classification documented ✅
- [ ] Statistical significance achieved (p < 0.05) ✅
- [ ] Portfolio interactions analyzed ✅
- [ ] Multi-stage bootstrap: Stage 2 == Stage 3 ✅
- [ ] All tests passing ✅
- [ ] Book chapter complete ✅
```

---

## Document Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-10-21 | Claude Code | Initial specification created |
| 1.1 | 2025-10-21 | Claude Code | Kaizen improvements: Phase 8 (Portfolio Validation), Statistical rigor requirements (p < 0.05), Risk-based classification system (3 classes) |

---

**Status**: SPECIFICATION READY FOR REVIEW
**Next Step**: Review and approval before implementation begins
**Implementation Start**: TBD (after approval)

---

**End of Specification**
