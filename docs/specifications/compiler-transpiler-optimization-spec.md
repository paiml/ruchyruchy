# Compiler/Transpiler Optimization Specification via Self-Hosting

**Project**: RuchyRuchy Bootstrap Compiler
**Document Version**: 1.0
**Date**: October 21, 2025
**Status**: SPECIFICATION - PENDING IMPLEMENTATION
**Methodology**: EXTREME TDD + Mutation/Property/Fuzz/PMAT Testing

---

## Executive Summary

This specification defines a **scientific, peer-reviewed approach** to discovering and implementing compiler/transpiler optimizations through self-hosting and bootstrapping techniques. By compiling the compiler with itself and systematically measuring performance, we can discover optimization opportunities empirically rather than theoretically.

**Core Principle**: Use the bootstrap compiler as its own benchmark suite to discover real-world optimization opportunities through profiling, measurement, and iterative improvement.

---

## Table of Contents

1. [Research Foundation](#research-foundation)
2. [Self-Hosting Optimization Strategy](#self-hosting-optimization-strategy)
3. [Optimization Techniques Catalog](#optimization-techniques-catalog)
4. [EXTREME TDD Implementation Methodology](#extreme-tdd-implementation-methodology)
5. [Testing Strategy: Mutation/Property/Fuzz/PMAT](#testing-strategy)
6. [Implementation Phases](#implementation-phases)
7. [Success Metrics](#success-metrics)
8. [References](#references)

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
**Research Basis**: Memoization techniques in compiler construction
**Opportunity**: Bootstrap compiler re-tokenizes common patterns
**Implementation**:
- Cache tokenized input for identical source fragments
- Use hash-based lookup for token stream retrieval
- Invalidate cache on source modification

**Expected Impact**: 10-15% lexer speedup on repeated patterns
**Measurement**: Profile token generation time during bootstrap
**Validation**: Property test: `cached_tokenize(s) == tokenize(s)`

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
**Research Basis**: Classic compiler optimization (Dragon Book §9.1)
**Opportunity**: Compile-time evaluation of constant expressions
**Implementation**:
- Evaluate arithmetic/logical ops on constants at compile-time
- Fold constant branches (dead code elimination)
- Propagate constants across assignments

**Expected Impact**: 5-10% runtime speedup of generated code
**Measurement**: Compare generated code size and execution time
**Validation**: Property test: Semantic equivalence

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
**Research Basis**: Data flow analysis (Dragon Book §9.2)
**Opportunity**: Unreachable code from constant folding
**Implementation**:
- Compute liveness analysis on generated code
- Remove unused variable assignments
- Eliminate unreachable basic blocks

**Expected Impact**: 5-15% generated code size reduction
**Measurement**: Compare bytecode/assembly size
**Validation**: Property test: Behavior preservation

#### OPT-CODEGEN-004: Inline Expansion
**Research Basis**: Procedure integration (Classic optimization)
**Opportunity**: Function call overhead in hot paths
**Implementation**:
- Inline small, frequently-called functions
- Use heuristics: size threshold, call frequency
- Avoid inlining recursive functions

**Expected Impact**: 10-25% runtime speedup (context-dependent)
**Measurement**: Profile function call overhead during bootstrap
**Validation**: Benchmark suite + semantic equivalence

### Category 5: Cross-Cutting Optimizations

#### OPT-GLOBAL-001: Profile-Guided Optimization (PGO)
**Research Basis**: "From Profiling to Optimization" (2025)
**Opportunity**: Optimize based on real bootstrap execution profile
**Implementation**:
- Instrument bootstrap compiler to collect execution profiles
- Identify hot paths (80/20 rule: 80% time in 20% code)
- Apply targeted optimizations to hot paths
- Use branch prediction hints based on profile

**Expected Impact**: 15-30% bootstrap speedup
**Measurement**: Compare instrumented vs optimized bootstrap time
**Validation**: Multi-stage bootstrap validation (Stage 2 == Stage 3)

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

### 7-Phase EXTREME TDD for Each Optimization

Every optimization must follow this **mandatory** process:

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

#### Summary: 7-Phase Validation Checklist

Every optimization MUST pass:
- [ ] ✅ RED: Failing test demonstrates opportunity
- [ ] ✅ GREEN: Minimal implementation passes test
- [ ] ✅ REFACTOR: Comprehensive, maintainable code
- [ ] ✅ TOOL VALIDATION: All 16 Ruchy tools pass
- [ ] ✅ MUTATION TESTING: ≥95% mutant kill rate
- [ ] ✅ PROPERTY TESTING: 10,000+ cases per property
- [ ] ✅ FUZZ TESTING: 100,000+ test cases, zero crashes

**Quality Gate**: Optimization is **BLOCKED** from merge until all phases complete.

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

**Deliverables**:
- `scripts/benchmark-bootstrap.sh` - Automated bootstrap benchmarking
- `validation/benchmarks/bootstrap_baseline.ruchy` - Baseline measurements
- Dashboard showing current performance metrics

**Success Criteria**:
- ✅ Reproducible bootstrap time measurements (±1% variance)
- ✅ Profiling data collected for each compiler phase
- ✅ Baseline established for future optimization comparison

### Phase 2: Lexer Optimizations (Weeks 3-4)

**Objective**: Optimize tokenization performance

**Tickets**:
- **OPT-LEX-001**: Token Stream Caching (RED-GREEN-REFACTOR)
- **OPT-LEX-002**: Lazy String Allocation (RED-GREEN-REFACTOR)

**Each ticket follows 7-phase EXTREME TDD**:
1. RED: Demonstrate opportunity via profiling
2. GREEN: Minimal optimization implementation
3. REFACTOR: Generalize and improve
4. TOOL VALIDATION: All 16 Ruchy tools
5. MUTATION: ≥95% mutant kill rate
6. PROPERTY: 10,000+ test cases
7. FUZZ: 100,000+ fuzz cases

**Success Criteria**:
- ✅ Lexer phase speedup: ≥15%
- ✅ Memory allocation reduction: ≥25%
- ✅ Bootstrap Stage 1→2 time reduction: ≥5%
- ✅ All validation phases pass

### Phase 3: Parser Optimizations (Weeks 5-6)

**Objective**: Optimize AST construction

**Tickets**:
- **OPT-PARSE-001**: Left-Recursion Elimination
- **OPT-PARSE-002**: AST Node Pooling

**Success Criteria**:
- ✅ Parser phase speedup: ≥20%
- ✅ AST memory churn reduction: ≥35%
- ✅ Bootstrap Stage 1→2 time reduction: ≥8%

### Phase 4: Type System Optimizations (Weeks 7-8)

**Objective**: Optimize type inference and checking

**Tickets**:
- **OPT-TYPE-001**: Type Inference Caching
- **OPT-TYPE-002**: Occurs Check Optimization

**Success Criteria**:
- ✅ Type checking phase speedup: ≥25%
- ✅ Unification operations reduction: ≥30%
- ✅ Bootstrap Stage 2→3 time reduction: ≥10%

### Phase 5: Code Generation Optimizations (Weeks 9-12)

**Objective**: Optimize generated code quality and compiler codegen phase

**Tickets**:
- **OPT-CODEGEN-001**: Constant Folding
- **OPT-CODEGEN-002**: Peephole Optimization
- **OPT-CODEGEN-003**: Dead Code Elimination
- **OPT-CODEGEN-004**: Inline Expansion

**Success Criteria**:
- ✅ Generated code speedup: ≥30%
- ✅ Generated code size reduction: ≥20%
- ✅ Codegen phase speedup: ≥15%
- ✅ Bootstrap overall time reduction: ≥25%

### Phase 6: Profile-Guided Optimization (Weeks 13-14)

**Objective**: Use real bootstrap profile to guide optimizations

**Tickets**:
- **OPT-GLOBAL-001**: PGO Implementation
- **OPT-GLOBAL-002**: Whole-Program Optimization

**Success Criteria**:
- ✅ Profile collection overhead: <5%
- ✅ PGO-guided speedup: ≥20%
- ✅ Total bootstrap time reduction: ≥40% (cumulative)

### Phase 7: Validation and Measurement (Week 15)

**Objective**: Comprehensive validation and documentation

**Tasks**:
- Run complete mutation test suite (PMAT)
- Run all property tests (10,000+ cases each)
- Run comprehensive fuzz testing (500,000+ cases)
- Validate multi-stage bootstrap (Stage 2 == Stage 3)
- Measure final performance improvements
- Document optimization results in book

**Success Criteria**:
- ✅ Mutation coverage: ≥95%
- ✅ Property tests: All pass (10,000+ cases)
- ✅ Fuzz tests: Zero crashes (500,000+ cases)
- ✅ Bootstrap validation: Bit-for-bit identical (Stage 2 == Stage 3)
- ✅ Performance: ≥40% overall bootstrap speedup
- ✅ Book: All optimization chapters complete

---

## Success Metrics

### Primary Metrics

#### 1. Bootstrap Compilation Time
**Baseline**: Establish current Stage 1→2→3 compilation time
**Target**: ≥40% reduction by end of Phase 6
**Measurement**: `time scripts/benchmark-bootstrap.sh`

**Breakdown by Phase**:
- Lexer optimizations: ≥5% improvement
- Parser optimizations: ≥8% improvement
- Type system optimizations: ≥10% improvement
- Code generation optimizations: ≥25% improvement
- PGO: ≥20% improvement (on top of previous)

**Formula**: `Cumulative_Speedup = 1 - ∏(1 - Individual_Speedup_i)`

#### 2. Generated Code Quality
**Metrics**:
- Execution time: ≥30% faster
- Code size: ≥20% smaller
- Instruction count: ≥25% reduction

**Measurement**: Benchmark suite on generated code

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

## Appendix B: EXTREME TDD Template

Every optimization ticket follows this structure:

```markdown
# OPT-XXX-YYY: Optimization Name

## Context
[Why this optimization is needed, research basis]

## RED Phase: Write Failing Test
[Test demonstrating opportunity, expected to fail]

## GREEN Phase: Minimal Implementation
[Simplest code to pass test]

## REFACTOR Phase: Improvements
[Generalize, improve, maintain]

## TOOL VALIDATION (16 Ruchy Tools)
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

## MUTATION TESTING
- Total mutants: XXX
- Killed: XXX
- Mutation score: XX.X% (target: ≥95%)

## PROPERTY TESTING
- Property 1: [Name] - 10,000+ cases ✅
- Property 2: [Name] - 10,000+ cases ✅
- Property 3: [Name] - 10,000+ cases ✅

## FUZZ TESTING
- Grammar-based: 50,000 cases ✅
- Mutation-based: 50,000 cases ✅
- Boundary values: 1,000 cases ✅
- Total: 100,000+ cases, 0 crashes ✅

## REPRODUCIBILITY
```bash
# Reproduction script
./scripts/reproduce-opt-xxx-yyy.sh
```

## DEBUGGABILITY
```bash
# Debug with ruchydbg
ruchydbg validate bootstrap/stage3/optimization.ruchy
```

## Performance Impact
- Baseline: X.XX seconds
- Optimized: X.XX seconds
- Speedup: XX.X%

## Validation Summary
- [ ] 7/7 EXTREME TDD phases complete
- [ ] Multi-stage bootstrap: Stage 2 == Stage 3 ✅
- [ ] All tests passing ✅
- [ ] Book chapter complete ✅
```

---

## Document Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-10-21 | Claude Code | Initial specification created |

---

**Status**: SPECIFICATION READY FOR REVIEW
**Next Step**: Review and approval before implementation begins
**Implementation Start**: TBD (after approval)

---

**End of Specification**
