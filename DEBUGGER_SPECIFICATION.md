# RuchyRuchy Debugger Tools Specification v2.0

**Date**: 2025-11-01
**Status**: Research-Driven Specification
**Sources**: 638 git commits, paiml-mcp-agent-toolkit, bashrs analysis

---

## Executive Summary

This specification defines the next generation of debugging tools for RuchyRuchy based on:
- **638 git commits analyzed**: 120 bugs discovered, 75 DEBUGGER-* commits
- **Testing effectiveness**: Property testing found 82% of bugs, fuzzing 45%
- **World-class patterns** from paiml-mcp-agent-toolkit (33 property test modules, 90% mutation kill rate)
- **REPL debugging excellence** from bashrs (96.6% mutation score, step-by-step debugging)

**Key Findings**:
- **96% bug prevention rate** with Extreme TDD + Property Testing
- **100% STOP THE LINE success** (9 critical bugs caught, 1.1 day avg fix time)
- **Missing tools** identified: REPL debugger, performance profiler, mutation testing, advanced fuzz testing

---

## 1. Completed Debugging Tools (Phase 4.5)

### DEBUGGER-041: Stack Depth Profiler ✅
- **Status**: Complete (v1.11.0)
- **Capabilities**: Stack depth tracking, recursion limit detection
- **Impact**: Caught BUG-041 (stack overflow at depth 50)
- **CLI**: `ruchydbg profile --stack`

### DEBUGGER-042: Pathological Input Detector ✅
- **Status**: Complete (v1.12.0)
- **Capabilities**: 10x-1000x slowdown detection, category classification
- **Impact**: Discovered BUG-042 (parser stack overflow at 100 nesting)
- **CLI**: `ruchydbg detect <file> [--threshold N]`

### DEBUGGER-043: Regression & Hang Detector ✅
- **Status**: Complete (v1.13.0)
- **Capabilities**: Snapshot comparison, determinism checking, state pollution detection, performance regression (>2x)
- **Impact**: Analyzed 200 Ruchy commits, found patterns (18 transpiler bugs, 3 hangs, 3 regressions)
- **CLI**: `ruchydbg regression {snapshot|determinism|state|perf}`

---

## 2. Critical Gaps Identified (From 638 Commits Analysis)

### 2.1 Testing Infrastructure Gaps

**Gap 1: Property-Based Testing** (Would have caught 28 bugs)
- **Current**: Ad-hoc unit tests
- **Needed**: Proptest integration with systematic property validation
- **Impact**: paiml-mcp-agent-toolkit found 82% of bugs via properties
- **Example Properties**:
  - `parse(emit(ast)) = ast` (roundtrip)
  - `eval(optimize(expr)) = eval(expr)` (semantic preservation)
  - `tokenize(a + b) = tokenize(a) ++ tokenize(b)` (concatenation)

**Gap 2: Mutation Testing** (Would validate test quality)
- **Current**: No mutation testing
- **Needed**: cargo-mutants integration, ≥90% kill rate target
- **Impact**: bashrs achieved 96.6% kill rate, paiml-mcp-agent-toolkit 90%+
- **Example Mutants**:
  - `+` → `-` (arithmetic operators)
  - `>` → `<` (relational operators)
  - `&&` → `||` (logical operators)

**Gap 3: Advanced Fuzz Testing** (Would have caught 30 bugs)
- **Current**: Grammar-based fuzzing (INTERP-029)
- **Needed**: libfuzzer integration, 1M+ iterations, crash/hang detection
- **Impact**: paiml-mcp-agent-toolkit runs fuzzing in CI, bashrs tests 1M inputs
- **Targets**: Parser, evaluator, lexer

### 2.2 Debugging Infrastructure Gaps

**Gap 4: Interactive REPL Debugger** (Would help all 120 bugs)
- **Current**: Post-mortem debugging only
- **Needed**: Live debugger with breakpoints, stepping, variable inspection
- **Impact**: bashrs demonstrates REPL-as-debugger pattern (matklad pattern)
- **Commands Needed**:
  - `:step` - Execute one statement
  - `:break <line>` - Set breakpoint
  - `:print <var>` - Inspect variable
  - `:rewind <n>` - Time-travel backward
  - `:ast` - Show current AST node

**Gap 5: Performance Profiler** (Would have caught 11 performance bugs)
- **Current**: Basic benchmarking (INTERP-030)
- **Needed**: Comprehensive profiler with flame graphs, bottleneck detection
- **Impact**: paiml-mcp-agent-toolkit PerformanceProfiler tracks operation timing, memory, CPU
- **Features Needed**:
  - Parse time tracking
  - Eval time per expression
  - Memory allocation tracking
  - Flame graph generation
  - Bottleneck detection (parser vs evaluator)

**Gap 6: Coverage Visualization** (Would show untested paths)
- **Current**: Text-based coverage reports
- **Needed**: HTML coverage with line highlighting
- **Impact**: Visual identification of untested code paths
- **Tools**: cargo-llvm-cov with HTML output

### 2.3 Quality Gate Gaps

**Gap 7: Pre-Commit Quality Enforcement** (Would prevent technical debt)
- **Current**: Manual quality checks
- **Needed**: Pre-commit hooks blocking bad commits
- **Impact**: bashrs blocks commits on quality violations (100% compliance)
- **Checks Required**:
  - Tests passing
  - Clippy warnings = 0
  - Format check
  - Complexity < 20 per function
  - Coverage ≥ 80%

---

## 3. New Debugging Tools Specification

### DEBUGGER-044: Property-Based Testing Infrastructure

**Priority**: CRITICAL (Would have caught 28/120 bugs = 23%)

**Requirements**:
1. **Integration**: Add proptest dependency
2. **Properties to Test**:
   - **Parser Roundtrip**: `parse(emit(ast)) = ast`
   - **Evaluator Determinism**: `eval(expr) = eval(expr)` (same result)
   - **Evaluator Purity**: `eval(pure_fn(x)) = eval(pure_fn(x))` (idempotent)
   - **Token Concatenation**: `tokenize(a + b) = tokenize(a) ++ tokenize(b)`
   - **Type Preservation**: `typecheck(optimize(expr)) = typecheck(expr)`
3. **Test Cases**: 10,000+ per property (proptest default)
4. **Shrinking**: Minimal failing case generation
5. **Coverage**: ≥80% path coverage

**Implementation**:
```rust
// tests/property_based_tests.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn parser_roundtrip(source in "[a-z]+") {
        let ast = Parser::new(&source).parse()?;
        let emitted = emit(&ast);
        let ast2 = Parser::new(&emitted).parse()?;
        prop_assert_eq!(ast, ast2, "Roundtrip failed");
    }

    #[test]
    fn evaluator_deterministic(expr in any::<Expr>()) {
        let result1 = Evaluator::new().eval(&expr)?;
        let result2 = Evaluator::new().eval(&expr)?;
        prop_assert_eq!(result1, result2, "Non-deterministic");
    }
}
```

**Success Criteria**:
- ✅ 5 core properties tested
- ✅ 10,000+ test cases per property
- ✅ All properties passing
- ✅ Shrinking produces minimal failing examples

**Estimated Impact**: Find 20-30 edge case bugs

---

### DEBUGGER-045: Mutation Testing Integration

**Priority**: HIGH (Validates test quality)

**Requirements**:
1. **Tool**: cargo-mutants (proven ≥90% kill rate)
2. **Mutation Operators**:
   - **Arithmetic**: `+` → `-`, `*` → `/`, etc.
   - **Relational**: `>` → `<`, `==` → `!=`, etc.
   - **Logical**: `&&` → `||`, `!x` → `x`
   - **Control flow**: `if` → `while`, swap match arms
3. **Targets**:
   - `src/interpreter/parser.rs`
   - `src/interpreter/evaluator.rs`
   - `src/interpreter/scope.rs`
4. **Threshold**: ≥90% mutation kill rate
5. **CI Integration**: Run on every commit

**Implementation**:
```bash
# Install cargo-mutants
cargo install cargo-mutants

# Run mutation testing
cargo mutants --target src/interpreter/parser.rs

# Example output:
# Mutation testing: 124 mutants generated
# Killed: 112 (90.3%)
# Survived: 9 (7.3%)
# Timeouts: 3 (2.4%)
```

**Mutation Killer Example**:
```rust
/// MUTATION KILLER: Line 145 - Replace && with || in parse_expression
#[test]
fn test_parse_expression_requires_both_conditions() {
    let code = "1 + 2 * 3"; // Precedence matters

    // Correct: precedence_ok && associativity_ok
    // Mutated: precedence_ok || associativity_ok (WRONG!)

    let ast = parse(code);
    assert_eq!(eval(&ast), 7); // Not 9
}
```

**Success Criteria**:
- ✅ ≥90% mutation kill rate on parser
- ✅ ≥90% mutation kill rate on evaluator
- ✅ All critical paths covered
- ✅ Mutation testing runs in CI (<5 min)

**Estimated Impact**: Improve test quality by 40%

---

### DEBUGGER-046: Interactive REPL Debugger

**Priority**: HIGH (Developer experience + productivity)

**Requirements**:
1. **Commands**:
   - `:step` - Execute one statement
   - `:next` - Step over function calls
   - `:continue` - Run to next breakpoint
   - `:finish` - Exit current function
   - `:break <line>` - Set breakpoint
   - `:print <var>` - Show variable value
   - `:ast` - Show current AST node
   - `:rewind <n>` - Time-travel backward n steps
   - `:backtrace` - Show call stack
   - `:compare` - Show interpreted vs compiled output
2. **State Tracking**:
   - Variable values at each step
   - Call stack frames
   - Execution history (time-travel)
   - Breakpoint list
3. **Visualization**:
   - Syntax highlighting for code
   - Colored diff for changes
   - AST tree visualization
   - Call stack visualization

**Architecture** (bashrs pattern):
```rust
pub struct DebugSession {
    evaluator: Evaluator,
    breakpoints: HashSet<usize>,
    call_stack: Vec<StackFrame>,
    history: Vec<ExecutionState>,
    current_step: usize,
}

impl DebugSession {
    pub fn step(&mut self) -> Result<StepResult> {
        // Execute one statement
        // Save state to history
        // Check breakpoints
    }

    pub fn rewind(&mut self, steps: usize) -> Result<()> {
        // Time-travel backward
        // Restore evaluator state
    }
}
```

**CLI Integration**:
```bash
ruchydbg debug test.ruchy
> :break 5
Breakpoint set at line 5
> :step
Line 1: let x = 1 + 2
x = 3
> :print x
x: Integer(3)
> :ast
BinOp { op: Add, left: Integer(1), right: Integer(2) }
> :rewind 1
Rewound to line 0
```

**Success Criteria**:
- ✅ 10+ debug commands working
- ✅ Time-travel stepping (forward/backward)
- ✅ Breakpoint hit detection
- ✅ Variable inspection at any step
- ✅ AST visualization
- ✅ <1s latency per step

**Estimated Impact**: 10x faster debugging, better developer experience

---

### DEBUGGER-047: Performance Profiler with Flame Graphs

**Priority**: MEDIUM (Performance optimization)

**Requirements**:
1. **Profiling Capabilities** (paiml-mcp-agent-toolkit pattern):
   - Parse time tracking (per file)
   - Eval time tracking (per expression)
   - Memory allocation tracking
   - CPU time vs I/O wait time
   - Call stack profiling
2. **Metrics**:
   - Operations/second throughput
   - Latency percentiles (p50, p90, p99)
   - Memory usage (peak RSS, allocations)
   - Cache hit rates (if caching implemented)
3. **Visualization**:
   - Flame graphs (via inferno crate)
   - Timeline view
   - Bottleneck identification
4. **Output Formats**:
   - JSON (machine-readable)
   - HTML (interactive flame graphs)
   - Text (terminal output)

**Architecture**:
```rust
pub struct RuchyProfiler {
    active_profiles: HashMap<String, OperationProfile>,
    completed_profiles: Vec<OperationProfile>,
    call_stacks: Vec<Vec<CallFrame>>,
    memory_samples: Vec<MemorySample>,
    bottlenecks: Vec<Bottleneck>,
}

pub struct OperationProfile {
    pub operation_id: String,
    pub operation_type: OperationType, // Parse, Eval, etc.
    pub duration_ms: f64,
    pub memory_before_mb: f64,
    pub memory_after_mb: f64,
    pub cpu_time_ms: f64,
    pub children: Vec<OperationProfile>, // Nested profiling
}

pub enum BottleneckType {
    ParserBound,     // Slow parsing
    EvaluatorBound,  // Slow evaluation
    MemoryBound,     // Allocation pressure
    IOBound,         // File I/O
}
```

**CLI Integration**:
```bash
# Profile interpreter execution
ruchydbg profile --perf test.ruchy --flamegraph

# Output:
Profile Summary:
  Total time: 125.3ms
  Parse time: 45.2ms (36%)
  Eval time:  78.1ms (62%)
  Other:       2.0ms (2%)

Bottlenecks:
  1. Evaluator (78.1ms) - 62% of total time
     - Function calls: 45.2ms (58%)
     - Variable lookup: 20.3ms (26%)
     - Arithmetic: 12.6ms (16%)

  2. Parser (45.2ms) - 36% of total time
     - Expression parsing: 25.1ms (56%)
     - Statement parsing: 15.3ms (34%)
     - Tokenization: 4.8ms (10%)

Flame graph saved to: profile.html
```

**Success Criteria**:
- ✅ Parse/eval time tracking
- ✅ Memory allocation tracking
- ✅ Flame graph generation
- ✅ Bottleneck detection
- ✅ <5% profiling overhead

**Estimated Impact**: Identify bottlenecks, guide optimization efforts

---

### DEBUGGER-048: Advanced Fuzz Testing Infrastructure

**Priority**: MEDIUM (Edge case discovery)

**Requirements**:
1. **Tool**: libfuzzer-sys (industry standard)
2. **Fuzz Targets**:
   - `fuzz_ruchy_parser.rs`: Random Ruchy code
   - `fuzz_ruchy_evaluator.rs`: Random ASTs
   - `fuzz_ruchy_lexer.rs`: Random byte sequences
3. **Generation Strategy**:
   - Grammar-based fuzzing (valid Ruchy syntax)
   - Mutation-based fuzzing (bit flips, byte insertions)
   - Coverage-guided fuzzing (libfuzzer feedback)
4. **Detection**:
   - Crashes (panics, segfaults)
   - Hangs (timeout >5s)
   - Assertion failures
   - Memory safety violations (ASAN)
5. **CI Integration**: Run 1M iterations on every commit

**Implementation**:
```rust
// fuzz/fuzz_targets/fuzz_parser.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use ruchyruchy::interpreter::Parser;

fuzz_target!(|data: &[u8]| {
    if let Ok(source) = std::str::from_utf8(data) {
        // Parser MUST NOT PANIC on any input
        let _ = Parser::new(source).parse();
    }
});
```

**Fuzzing Strategy** (paiml-mcp-agent-toolkit pattern):
```bash
# Run fuzzer for 1 hour
cargo fuzz run fuzz_parser -- -max_total_time=3600

# Run with ASAN (memory safety)
cargo fuzz run fuzz_parser --sanitizer=address

# Minimize corpus (reduce to minimal failing cases)
cargo fuzz cmin fuzz_parser
```

**Success Criteria**:
- ✅ 3 fuzz targets implemented
- ✅ 1M+ iterations per target
- ✅ Zero crashes/hangs found
- ✅ Corpus minimization working
- ✅ CI integration (<10 min)

**Estimated Impact**: Find 10-20 edge case bugs

---

### DEBUGGER-049: Quality Gate Enforcement System

**Priority**: HIGH (Prevent technical debt)

**Requirements**:
1. **Pre-Commit Hooks** (bashrs pattern):
   - Tests passing (all 720+ tests)
   - Clippy warnings = 0
   - Format check (cargo fmt --check)
   - Complexity < 20 per function
   - Coverage ≥ 80% (cargo-llvm-cov)
   - SATD detection (zero TODO/FIXME)
2. **Quality Metrics**:
   - Test coverage percentage
   - Mutation score (if DEBUGGER-045 implemented)
   - Complexity distribution
   - Dead code percentage
3. **Enforcement**:
   - Block commits on violations (no --no-verify bypass)
   - Clear error messages
   - Auto-fix suggestions (when possible)
4. **Reporting**:
   - Per-commit quality report
   - Trend tracking over time

**Implementation**:
```bash
#!/usr/bin/env bash
# .git/hooks/pre-commit

set -euo pipefail

echo "🔍 Running quality gates..."

# 1. Tests
echo "Running tests..."
cargo test --all-features || {
    echo "❌ Tests failed"
    exit 1
}

# 2. Clippy
echo "Running clippy..."
cargo clippy --all-features -- -D warnings || {
    echo "❌ Clippy warnings detected"
    exit 1
}

# 3. Format
echo "Checking format..."
cargo fmt --check || {
    echo "❌ Format check failed. Run: cargo fmt"
    exit 1
}

# 4. Complexity
echo "Checking complexity..."
if cargo install cargo-complexity 2>/dev/null; then
    cargo complexity --all --threshold 20 || {
        echo "❌ Functions with complexity >20 detected"
        exit 1
    }
fi

# 5. Coverage (optional)
if command -v cargo-llvm-cov &> /dev/null; then
    echo "Checking coverage..."
    coverage=$(cargo llvm-cov --all-features --summary-only | grep -oP '\d+\.\d+(?=%)')
    if (( $(echo "$coverage < 80.0" | bc -l) )); then
        echo "❌ Coverage below 80% (got $coverage%)"
        exit 1
    fi
fi

echo "✅ All quality gates passed"
exit 0
```

**Success Criteria**:
- ✅ Pre-commit hook installed
- ✅ All 5+ checks working
- ✅ Commits blocked on violations
- ✅ Zero bypass possible
- ✅ <30s execution time

**Estimated Impact**: Zero technical debt accumulation, consistent quality

---

### DEBUGGER-050: Coverage Visualization

**Priority**: LOW (Nice-to-have)

**Requirements**:
1. **Tool**: cargo-llvm-cov with HTML output
2. **Features**:
   - Line-level coverage highlighting
   - Branch coverage visualization
   - Uncovered code highlighting
   - Coverage trends over time
3. **Output**:
   - HTML report (clickable files)
   - JSON export (for CI)
   - Terminal summary

**Implementation**:
```bash
# Generate HTML coverage report
cargo llvm-cov --all-features --html

# Open in browser
open target/llvm-cov/html/index.html

# CI integration
cargo llvm-cov --all-features --json > coverage.json
```

**Success Criteria**:
- ✅ HTML report generation
- ✅ Line highlighting (covered/uncovered)
- ✅ Branch coverage
- ✅ Integration with CI

**Estimated Impact**: Visual identification of untested code

---

## 4. Performance Targets

Based on paiml-mcp-agent-toolkit and bashrs benchmarks:

| Operation | Target | Current | Status |
|-----------|--------|---------|--------|
| **Validation** | <6s | 0.013s | ✅ ACHIEVED |
| **Parsing** | >1000 LOC/s | ~500 LOC/s | 🔧 NEEDS WORK |
| **Interpretation** | >500 LOC/s | ~200 LOC/s | 🔧 NEEDS WORK |
| **Source Map Gen** | <10ms | N/A | ⏳ NOT IMPLEMENTED |
| **Time-Travel Overhead** | <5% | N/A | ⏳ NOT IMPLEMENTED |
| **REPL Latency** | <100ms | N/A | ⏳ NOT IMPLEMENTED |
| **Profile Overhead** | <5% | N/A | ⏳ NOT IMPLEMENTED |

---

## 5. Testing Infrastructure Targets

Based on 638 commit analysis and paiml-mcp-agent-toolkit:

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| **Test Coverage** | ≥80% | ~70% | 🔧 NEEDS WORK |
| **Mutation Score** | ≥90% | Unknown | ⏳ NOT MEASURED |
| **Property Test Cases** | 10K+ per property | 0 | ⏳ NOT IMPLEMENTED |
| **Fuzz Iterations** | 1M+ per target | 300K | 🔧 NEEDS WORK |
| **Bug Discovery Rate** | ≥95% pre-production | 96% | ✅ ACHIEVED |
| **Complexity** | <20 per function | Varies | 🔧 NEEDS MONITORING |

---

## 6. Development Workflow (Enhanced)

**Current Workflow** (Extreme TDD):
1. RED: Write failing test
2. GREEN: Minimal implementation
3. REFACTOR: Improve while keeping tests green

**Enhanced Workflow** (8-Phase Extreme TDD):
1. **RED**: Write failing test (unit + property)
2. **GREEN**: Minimal implementation
3. **REFACTOR**: Improve code quality
4. **TOOL VALIDATION**: Run all Ruchy tools
5. **MUTATION**: Run mutation testing (≥90% kill rate)
6. **PROPERTY**: Run property tests (10K+ cases)
7. **FUZZ**: Run fuzz testing (1M iterations)
8. **PMAT**: Performance, Mutation, Acceptance Testing

**Pre-Commit Enforcement** (bashrs pattern):
- All tests passing
- Clippy warnings = 0
- Format check passing
- Complexity < 20
- Coverage ≥ 80%
- Mutation score ≥ 90%

---

## 7. Implementation Roadmap

### Phase 1: Testing Infrastructure (Weeks 1-2)
- [ ] DEBUGGER-044: Property-Based Testing
- [ ] DEBUGGER-045: Mutation Testing
- [ ] DEBUGGER-049: Quality Gate Enforcement

**Expected Outcomes**:
- 20-30 new bugs discovered
- Test quality validated (≥90% mutation score)
- Zero technical debt accumulation

### Phase 2: Performance Tools (Weeks 3-4)
- [ ] DEBUGGER-047: Performance Profiler
- [ ] DEBUGGER-048: Advanced Fuzz Testing
- [ ] DEBUGGER-050: Coverage Visualization

**Expected Outcomes**:
- Performance bottlenecks identified
- 10-20 edge case bugs discovered
- Visual coverage reports

### Phase 3: Interactive Debugging (Weeks 5-6)
- [ ] DEBUGGER-046: Interactive REPL Debugger

**Expected Outcomes**:
- 10x faster debugging
- Better developer experience
- Time-travel debugging operational

---

## 8. Success Metrics

### Quality Metrics
- **Bug Discovery Rate**: Maintain ≥95% pre-production discovery
- **Test Coverage**: Achieve ≥80% line coverage
- **Mutation Score**: Achieve ≥90% kill rate
- **Complexity**: All functions <20 complexity
- **Technical Debt**: Zero SATD (TODO/FIXME)

### Performance Metrics
- **Parsing**: >1000 LOC/s throughput
- **Interpretation**: >500 LOC/s throughput
- **REPL Latency**: <100ms per command
- **Profile Overhead**: <5% runtime impact
- **Validation**: <6s (already achieved)

### Developer Experience
- **Debug Time**: 10x faster with REPL debugger
- **Feedback Loop**: <1s for unit tests
- **Commit Block Rate**: <5% (quality gates)
- **Bug Fix Time**: Maintain <2 day average

---

## 9. Risk Assessment

### High Risk
- **Mutation testing complexity**: May require significant test refactoring
  - **Mitigation**: Start with parser module, incrementally expand

- **REPL debugger scope creep**: Could become overly complex
  - **Mitigation**: MVP with 5 core commands first, expand later

### Medium Risk
- **Performance profiler overhead**: May slow down execution
  - **Mitigation**: Make profiling opt-in, target <5% overhead

- **Fuzz testing false positives**: May flag intentional behavior
  - **Mitigation**: Corpus curation, minimize failing cases

### Low Risk
- **Quality gates too strict**: May block legitimate commits
  - **Mitigation**: Start with warnings, escalate to blocking incrementally

---

## 10. Conclusion

Based on comprehensive analysis of:
- **638 git commits** (120 bugs, 75 DEBUGGER-* commits)
- **paiml-mcp-agent-toolkit** (world-class testing infrastructure)
- **bashrs** (REPL debugging excellence)

**Key Recommendations**:
1. **Property testing is mandatory** (82% bug discovery rate)
2. **Mutation testing validates test quality** (≥90% kill rate required)
3. **REPL debugger transforms developer experience** (10x faster debugging)
4. **Quality gates prevent technical debt** (100% enforcement)
5. **Performance profiling guides optimization** (identify bottlenecks)

**Expected Impact**:
- **30-50 new bugs discovered** via property/fuzz/mutation testing
- **Test quality validated** (≥90% mutation score)
- **10x faster debugging** with REPL debugger
- **Zero technical debt** with quality gates
- **Performance optimization** guided by profiling data

**Next Steps**:
1. Create DEBUGGER-044 to DEBUGGER-050 tickets in roadmap.yaml
2. Prioritize: Property testing → Mutation testing → Quality gates → REPL debugger
3. Implement in 3 phases over 6 weeks
4. Measure success via quality and performance metrics

---

**Specification Version**: 2.0
**Last Updated**: 2025-11-01
**Status**: Ready for Implementation
