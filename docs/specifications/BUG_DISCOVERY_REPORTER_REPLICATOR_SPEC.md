# Bug Discovery, Reporter, and Replicator System - Specification

**Version**: 1.0.0
**Date**: 2025-10-29
**Status**: Specification - Ready for Implementation
**Project**: RuchyRuchy Bug Discovery & Analysis System

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [System Overview](#system-overview)
3. [Historical Bug Analysis](#historical-bug-analysis)
4. [Core Components](#core-components)
5. [Bug Discovery Module](#bug-discovery-module)
6. [Bug Replicator Module](#bug-replicator-module)
7. [Bug Reporter Module](#bug-reporter-module)
8. [Quantitative Analysis Framework](#quantitative-analysis-framework)
9. [Report Template & GitHub Integration](#report-template--github-integration)
10. [Implementation Plan](#implementation-plan)
11. [Testing & Validation](#testing--validation)

---

## 1. Executive Summary

### Purpose

Create a comprehensive, automated bug discovery, replication, and reporting system that:
1. **Discovers** bugs through systematic testing
2. **Replicates** bugs with minimal reproducible examples
3. **Reports** bugs with extreme quantitative detail
4. **Prevents** future bugs through deep root cause analysis

### Key Innovation

**Extreme Quantitative Analysis with Confidence Scoring**: Every bug report includes:
- **Confidence Score** (0.0-1.0): Automation certainty for prioritization (prevents alert fatigue)
- Complexity metrics (cyclomatic, cognitive, loop nesting depth)
- SATD (Self-Admitted Technical Debt) analysis with NLP enhancement
- Code churn analysis (commits, changes, hot spots, semantic analysis)
- Formalization hints (formal verification candidates, not scores)
- Linked files (dependency graph, call chains)
- TDD fix workflow with RED-GREEN-REFACTOR
- Five-Whys assisted analysis (data-driven hypotheses)
- Regression testing strategy with statistical analysis
- Property/Fuzz/Mutation testing recommendations
- Prevention strategy for similar bugs

**Jidoka Principle**: Automation with human touch - system provides high-quality data and hypotheses, humans make final causal judgments

### Historical Context

**Analyzed Bugs**:
- **Ruchy compiler**: 79 issues (31 open, 48 closed)
- **ubuntu-config-scripts**: 5/9 conversions broken by bugs (56% failure rate)
- **Critical patterns**: Runtime hangs, parser errors, formatter bugs

**Impact**:
- **62.5%** of production work blocked by bugs
- **20 developer days** lost to debugging
- **100%** detection rate possible with proper tooling (Code Churn + ML Predict)

---

## 2. System Overview

### 2.1 Confidence Scoring System (Jidoka Principle)

**Purpose**: Prevent alert fatigue by ranking automated findings by confidence level.

**Confidence Score Formula**:
```rust
struct BugReportConfidence {
    discovery_method_weight: f64,  // 0.0-1.0
    reproducibility_score: f64,     // 0.0-1.0
    quantitative_evidence: f64,     // 0.0-1.0
    root_cause_clarity: f64,        // 0.0-1.0
}

impl BugReportConfidence {
    fn calculate_overall_confidence(&self) -> f64 {
        let weights = [0.35, 0.30, 0.20, 0.15];  // Discovery, Repro, Quant, Root Cause

        weights[0] * self.discovery_method_weight +
        weights[1] * self.reproducibility_score +
        weights[2] * self.quantitative_evidence +
        weights[3] * self.root_cause_clarity
    }
}
```

**Discovery Method Weights**:
| Discovery Method | Confidence Weight | Rationale |
|------------------|------------------|-----------|
| Differential Test (Version Regression) | **1.0** | Clear before/after comparison, deterministic |
| Differential Test (Target Mismatch) | **0.9** | Clear discrepancy, but may be expected |
| Property Test Violation | **0.95** | Mathematical invariant violated |
| Grammar Fuzz (Crash/Hang) | **0.85** | Clear failure mode, but may be edge case |
| Grammar Fuzz (Incorrect Output) | **0.70** | May be spec ambiguity |
| Mutation Fuzz (Crash) | **0.75** | Invalid input expected to fail gracefully |
| Code Churn Hot Spot | **0.60** | Predictive, not deterministic bug |

**Reproducibility Score**:
- **1.0**: 100% reproducible with minimized test case
- **0.9**: 100% reproducible, but large test case
- **0.7**: Intermittent (>50% failure rate)
- **0.5**: Intermittent (<50% failure rate)
- **0.3**: Non-deterministic/flaky

**Quantitative Evidence Score**:
- **1.0**: All metrics collected (complexity, churn, SATD, dependencies)
- **0.8**: Partial metrics (missing 1-2 categories)
- **0.6**: Limited metrics (only complexity or churn)
- **0.4**: No quantitative data

**Root Cause Clarity Score**:
- **1.0**: Single, obvious root cause with fix
- **0.8**: Primary root cause identified, secondary factors present
- **0.6**: Multiple plausible root causes
- **0.4**: Unclear root cause, hypothesis only
- **0.2**: No root cause identified

**Usage Example**:
```rust
let bug = DiscoveredBug {
    discovery_method: DiscoveryMethod::DifferentialTestVersionRegression,
    reproducibility: Reproducibility::Always,
    quantitative_data: QuantitativeData::Complete,
    root_cause: RootCause::Identified("Infinite loop in Vec::new()"),
};

let confidence = BugReportConfidence {
    discovery_method_weight: 1.0,   // Version regression
    reproducibility_score: 1.0,     // 100% reproducible
    quantitative_evidence: 1.0,     // All metrics
    root_cause_clarity: 1.0,        // Clear root cause
};

let overall = confidence.calculate_overall_confidence();  // 1.0
// Result: HIGH CONFIDENCE - File immediately to GitHub
```

**Prioritization Thresholds**:
- **0.85-1.0**: 🔴 CRITICAL - File immediately, block release
- **0.70-0.84**: 🟠 HIGH - File within 24 hours, investigate
- **0.50-0.69**: 🟡 MEDIUM - File within 1 week, triage
- **0.30-0.49**: 🔵 LOW - Review manually before filing
- **<0.30**: ⚪ NOISE - Suppress or flag for human review

**Toyota Way Connection**: This embodies **Jidoka** - automation with a human touch. The system automates detection and analysis but provides confidence scores to guide human decision-making, preventing wasted effort on low-confidence findings.

---

### 2.2 Architecture

```
┌─────────────────────────────────────────────────────────┐
│           BUG DISCOVERY & REPORTING SYSTEM              │
└─────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│              │  │              │  │              │
│  DISCOVERY   │  │  REPLICATOR  │  │   REPORTER   │
│    MODULE    │  │    MODULE    │  │    MODULE    │
│              │  │              │  │              │
└──────────────┘  └──────────────┘  └──────────────┘
        │                  │                  │
        │                  │                  │
        ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────┐
│          QUANTITATIVE ANALYSIS FRAMEWORK                │
│                                                          │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  │Complexity│ │   SATD   │ │  Churn   │ │Big-O/Prov│  │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │
│                                                          │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  │5-Whys   │ │   TDD    │ │Regression│ │Prevention│  │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│         GITHUB INTEGRATION & MARKDOWN REPORTS            │
│                                                          │
│  • Auto-generate markdown reports                       │
│  • File directly to GitHub Issues                       │
│  • Link to related issues and PRs                       │
│  • Track bug lifecycle                                  │
└─────────────────────────────────────────────────────────┘
```

### Data Flow

1. **Discovery** → Finds potential bugs through systematic testing
2. **Replication** → Creates minimal reproducible examples
3. **Analysis** → Runs comprehensive quantitative analysis
4. **Reporting** → Generates detailed markdown reports
5. **Filing** → Automatically creates GitHub issues
6. **Tracking** → Monitors bug lifecycle until resolution

---

## 3. Historical Bug Analysis

### 3.1 Bug Categories (Ruchy Compiler)

From analysis of 79 GitHub issues:

| Category | Count | % | Examples |
|----------|-------|---|----------|
| **Runtime Hangs** | 12 | 15% | #76 (Vec::new), #75 (Command.output), #74 (vec! macro) |
| **Parser Errors** | 18 | 23% | #71 (&mut), #67 (while+HashMap), #65 (misleading errors) |
| **Formatter Bugs** | 8 | 10% | #72 (macro conversion), #64 (data loss), #60 (fun→fn) |
| **Type System** | 7 | 9% | #68 (&str+String), #35 (incorrect inference) |
| **Lint False Positives** | 9 | 11% | #69 (forward refs), #34 (built-ins), #15 (used vars) |
| **WASM** | 6 | 8% | #27 (100% failure), #53/#52/#51 (syntax) |
| **Missing Features** | 12 | 15% | #70 (fn() type), #16 (ruchy doc), #14 (ruchy fmt) |
| **Test Tool Bugs** | 7 | 9% | #37 (false PASS), #36 (coverage), #33 (@test) |

### 3.2 Critical Patterns

**Pattern 1: Runtime Hangs (Most Critical)**
- **Frequency**: 12 occurrences (15%)
- **Severity**: CRITICAL - Blocks all work
- **Common causes**:
  - Vec operations (#76, #74, #62)
  - Command execution (#75)
  - Boolean negation (#54)
  - Pattern matching in loops (#40)
- **Detection**: Code Churn Analysis (18 commits in parser.rs = 8 bugs)

**Pattern 2: Parser Regressions**
- **Frequency**: 18 occurrences (23%)
- **Severity**: HIGH - Breaks existing code
- **Common causes**:
  - Complex syntax combinations
  - New features breaking old patterns
  - Misleading error messages
- **Detection**: Differential testing (compare versions)

**Pattern 3: Formatter Data Loss**
- **Frequency**: 8 occurrences (10%)
- **Severity**: CRITICAL - Corrupts source code
- **Common causes**:
  - AST debug output instead of code (#31, #14)
  - Macro transformations (#72)
  - Keyword changes (#60: fun→fn)
- **Detection**: Roundtrip testing (format(parse(code)) == code)

### 3.3 Code Churn Hot Spots

From git history analysis:

| File | Commits (last 100) | Bugs | Bugs/Commit | Risk Level |
|------|-------------------|------|-------------|------------|
| `parser.rs` | 18 | 8 | **0.44** | 🔴 CRITICAL |
| `formatter.rs` | 15 | 4 | 0.27 | 🟠 HIGH |
| `lexer.rs` | 12 | 3 | 0.25 | 🟠 HIGH |
| `type_checker.rs` | 10 | 2 | 0.20 | 🟡 MEDIUM |
| `runtime.rs` | 14 | 5 | **0.36** | 🔴 CRITICAL |

**Conclusion**: Files with >10 commits and >0.3 bugs/commit are **CRITICAL** risk

### 3.4 Real-World Impact (ubuntu-config-scripts)

**Conversion Project Statistics**:
- **Files converted**: 9 TypeScript → Ruchy
- **Files created**: 54 Ruchy files (1,200+ LOC)
- **Tests written**: 60+ tests
- **Bugs encountered**: 3 distinct bugs (#76, #75, #73)
- **Conversions broken**: 5/9 (56% failure rate)
- **Tests blocked**: 32/60 (53%)
- **Time lost**: 3+ weeks waiting for fixes

**Bug Impact Breakdown**:
- **Issue #76** (Vec::new hang): Broke RUCHY-001, 002, 003 (3 files, 30 tests)
- **Issue #75** (Command.output hang): Broke RUCHY-006, 007 (2 files, 2 tests)
- **Issue #73** (Command parameter name): Contributed to parsing issues

**Prevention Analysis**:
- **Code Churn** would have flagged parser.rs (18 commits)
- **ML Defect Prediction** would have predicted 95% bug probability
- **Mutation Testing** would have shown 45% test coverage for Command path
- **Result**: 62.5% of bugs preventable with QUALITY tools

---

## 4. Core Components

### 4.1 Component Responsibilities

**Discovery Module**:
- Systematic bug hunting (fuzz, property, differential)
- Pattern detection (hangs, crashes, errors)
- Regression detection (version comparisons)
- Coverage analysis (find untested code paths)

**Replicator Module**:
- Minimize reproduction cases
- Generate standalone test files
- Document exact steps to reproduce
- Create TDD test workflow (RED phase)

**Reporter Module**:
- Generate comprehensive markdown reports
- Run quantitative analysis (all metrics)
- Apply Five-Whys root cause analysis
- Create prevention strategies
- File GitHub issues automatically

**Analysis Framework** (Shared):
- Complexity metrics (cyclomatic, cognitive, Big-O)
- SATD detection and categorization
- Code churn analysis (git history)
- Provability analysis (formal verification potential)
- Dependency graph generation
- Call chain analysis

---

## 5. Bug Discovery Module

### 5.1 Discovery Techniques

**Technique 1: Differential Testing**
```
Goal: Compare behavior across versions, targets, optimization levels

Test Matrix:
┌────────────────┬─────────┬─────────┬─────────┐
│                │ v3.146  │ v3.147  │ v3.148  │
├────────────────┼─────────┼─────────┼─────────┤
│ Debug build    │  PASS   │  HANG   │  PASS   │  ← Regression in v3.147!
│ Release build  │  PASS   │  HANG   │  PASS   │
│ WASM target    │  PASS   │  PASS   │  PASS   │
└────────────────┴─────────┴─────────┴─────────┘

Detection: Version v3.147 introduces runtime hang
```

**Implementation with Statistical Analysis**:
```rust
struct DifferentialTester {
    versions: Vec<CompilerVersion>,  // v3.146, v3.147, v3.148
    targets: Vec<CompilationTarget>,  // debug, release, wasm
    test_suite: Vec<TestCase>,
}

#[derive(Debug)]
struct TestResult {
    status: TestStatus,           // Pass, Hang, Crash, WrongOutput
    execution_time_ms: Option<f64>,
    memory_usage_mb: Option<f64>,
    output: Option<String>,
}

#[derive(Debug, PartialEq)]
enum TestStatus {
    Pass,
    Hang(Duration),      // Timeout duration
    Crash(String),       // Error message
    WrongOutput(String), // Diff
    PerfRegression { slowdown_factor: f64, p_value: f64 },  // Statistical
}

impl DifferentialTester {
    fn find_regressions(&self) -> Vec<RegressionBug> {
        let mut bugs = vec![];

        for test in &self.test_suite {
            let results = self.run_across_versions_with_stats(test);

            // Detect functional regressions (crashes, hangs, wrong output)
            if results.has_new_failures() {
                bugs.push(RegressionBug {
                    test_case: test.clone(),
                    working_version: results.last_working_version(),
                    broken_version: results.first_failing_version(),
                    failure_mode: results.failure_type(),
                    confidence: 0.95,  // High confidence for functional failures
                });
            }

            // Detect performance regressions (statistical analysis)
            if let Some(perf_regression) = self.detect_performance_regression(&results) {
                bugs.push(RegressionBug {
                    test_case: test.clone(),
                    working_version: perf_regression.baseline_version,
                    broken_version: perf_regression.regressed_version,
                    failure_mode: FailureMode::PerformanceRegression {
                        slowdown: perf_regression.slowdown_factor,
                        p_value: perf_regression.p_value,
                        baseline_mean_ms: perf_regression.baseline_mean,
                        regressed_mean_ms: perf_regression.regressed_mean,
                    },
                    confidence: self.perf_regression_confidence(&perf_regression),
                });
            }
        }

        bugs
    }

    /// Detect performance regressions using Welch's t-test
    /// References: Kalibera & Jones (2013) "Quantifying Performance Changes with Effect Size Confidence Intervals"
    fn detect_performance_regression(&self, results: &VersionResults) -> Option<PerfRegression> {
        const SIGNIFICANCE_LEVEL: f64 = 0.05;  // 95% confidence
        const MIN_SLOWDOWN: f64 = 1.2;  // 20% slowdown threshold

        for i in 0..results.versions.len() - 1 {
            let baseline = &results.timings[i];
            let current = &results.timings[i + 1];

            // Run each test 30 times for statistical power
            let baseline_samples = self.run_multiple_times(&results.versions[i], 30);
            let current_samples = self.run_multiple_times(&results.versions[i + 1], 30);

            let (t_stat, p_value) = welchs_t_test(&baseline_samples, &current_samples);
            let slowdown = mean(&current_samples) / mean(&baseline_samples);

            if p_value < SIGNIFICANCE_LEVEL && slowdown > MIN_SLOWDOWN {
                return Some(PerfRegression {
                    baseline_version: results.versions[i].clone(),
                    regressed_version: results.versions[i + 1].clone(),
                    slowdown_factor: slowdown,
                    p_value,
                    baseline_mean: mean(&baseline_samples),
                    regressed_mean: mean(&current_samples),
                    effect_size: cohens_d(&baseline_samples, &current_samples),
                });
            }
        }

        None
    }

    /// Calculate confidence based on statistical strength
    fn perf_regression_confidence(&self, regression: &PerfRegression) -> f64 {
        // Higher confidence for:
        // - Lower p-values (more significant)
        // - Larger effect sizes (more noticeable)
        // - Larger slowdowns (more impactful)

        let p_score = (0.05 - regression.p_value) / 0.05;  // 0.0-1.0
        let effect_score = (regression.effect_size.abs() / 2.0).min(1.0);  // Large effect > 0.8
        let slowdown_score = ((regression.slowdown_factor - 1.0) / 2.0).min(1.0);  // 2x = 0.5

        0.5 * p_score + 0.3 * effect_score + 0.2 * slowdown_score
    }
}

/// Welch's t-test for samples with potentially unequal variance
/// More robust than Student's t-test for real-world performance data
fn welchs_t_test(sample1: &[f64], sample2: &[f64]) -> (f64, f64) {
    let mean1 = mean(sample1);
    let mean2 = mean(sample2);
    let var1 = variance(sample1);
    let var2 = variance(sample2);
    let n1 = sample1.len() as f64;
    let n2 = sample2.len() as f64;

    let t_stat = (mean1 - mean2) / ((var1 / n1) + (var2 / n2)).sqrt();

    // Welch-Satterthwaite degrees of freedom
    let df = ((var1 / n1) + (var2 / n2)).powi(2) /
             ((var1 / n1).powi(2) / (n1 - 1.0) + (var2 / n2).powi(2) / (n2 - 1.0));

    let p_value = students_t_cdf(t_stat.abs(), df);

    (t_stat, p_value)
}

/// Cohen's d effect size (small: 0.2, medium: 0.5, large: 0.8+)
fn cohens_d(sample1: &[f64], sample2: &[f64]) -> f64 {
    let mean1 = mean(sample1);
    let mean2 = mean(sample2);
    let pooled_sd = ((variance(sample1) + variance(sample2)) / 2.0).sqrt();

    (mean2 - mean1) / pooled_sd
}
```

**Key Improvements**:
1. **Statistical rigor**: Uses Welch's t-test instead of binary pass/fail
2. **Effect size**: Reports Cohen's d to quantify magnitude of slowdown
3. **Confidence scoring**: Lower confidence for borderline performance issues
4. **Multiple samples**: Runs each test 30 times for statistical power
5. **Thresholds**: 20% slowdown + p<0.05 = regression (reduces false positives)

**Technique 2: Grammar-Based Fuzzing**
```
Goal: Generate syntactically valid Ruchy code to find parser/runtime bugs

Grammar:
program → stmt+
stmt → let_stmt | fun_stmt | expr_stmt
let_stmt → "let" IDENT "=" expr
fun_stmt → "fun" IDENT "(" params? ")" "{" stmt+ "}"
expr → literal | call | binary_op | ...

Fuzzing Strategy:
1. Generate 10,000+ programs from grammar
2. Test each with: ruchy check, ruchy run, ruchy fmt
3. Detect: crashes, hangs (timeout), incorrect output
4. Minimize: Reduce failing case to smallest example
```

**Implementation**:
```rust
struct GrammarFuzzer {
    grammar: RuchyGrammar,
    generation_limit: usize,  // 10,000
    timeout: Duration,         // 5 seconds per test
}

impl GrammarFuzzer {
    fn generate_test_program(&self) -> String {
        // Generate random valid Ruchy program
        self.grammar.generate_from_rule("program")
    }

    fn fuzz_compiler(&self) -> Vec<FuzzBug> {
        let mut bugs = vec![];

        for i in 0..self.generation_limit {
            let program = self.generate_test_program();

            // Test compilation
            match self.compile_with_timeout(&program, self.timeout) {
                CompileResult::Hang => {
                    bugs.push(FuzzBug::Hang {
                        program: self.minimize(program),
                        phase: "compilation",
                    });
                }
                CompileResult::Crash(error) => {
                    bugs.push(FuzzBug::Crash {
                        program: self.minimize(program),
                        error_message: error,
                    });
                }
                CompileResult::Success(executable) => {
                    // Test execution
                    match self.run_with_timeout(&executable, self.timeout) {
                        RunResult::Hang => {
                            bugs.push(FuzzBug::Hang {
                                program: self.minimize(program),
                                phase: "runtime",
                            });
                        }
                        RunResult::Crash(error) => {
                            bugs.push(FuzzBug::Crash {
                                program: self.minimize(program),
                                error_message: error,
                            });
                        }
                        RunResult::Success => {} // OK
                    }
                }
            }
        }

        bugs
    }

    fn minimize(&self, program: String) -> String {
        // Delta debugging: Remove lines until bug disappears
        // Return smallest program that still triggers bug
        delta_debug(&program, |code| self.reproduces_bug(code))
    }
}
```

**Technique 2B: Schema-Based Runtime Property Fuzzing** ⭐ **CRITICAL FOR RUNTIME HANGS**
```
Goal: Move beyond syntax-level fuzzing to behavioral/semantic fuzzing
Problem: Current bugs (#76 Vec::new, #75 Command.output) are RUNTIME hangs, not parse errors
Solution: Model valid states and transitions of runtime objects

Key Insight: A syntax fuzzer tests the compiler frontend by generating valid TEXT.
            A runtime fuzzer tests the compiled program BEHAVIOR by generating valid
            SEQUENCES OF ACTIONS on stateful objects.

Runtime Schema (The "Grammar" of Behavior):
-------------------------------------------

objects:
  - name: Vec
    state:
      - name: len
        type: int
        initial: 0
      - name: is_empty
        type: bool
        initial: true
    operations:
      - name: push
        params: [Int]
        transitions:
          - "state.len = old.len + 1"
          - "state.is_empty = false"
      - name: pop
        preconditions:
          - "state.len > 0"  # Cannot pop from empty vec
        returns: Int
        transitions:
          - "state.len = old.len - 1"
          - "state.is_empty = (state.len == 0)"
      - name: clear
        transitions:
          - "state.len = 0"
          - "state.is_empty = true"

  - name: Command
    state:
      - name: has_run
        type: bool
        initial: false
    operations:
      - name: new
        params: [String]
        is_constructor: true
        properties:
          - "must_complete_within(100ms)"  # CRITICAL: Detect hangs in constructors
      - name: arg
        params: [String]
        preconditions:
          - "state.has_run == false"  # Cannot add args after running
      - name: output
        properties:
          - "must_complete_within(1000ms)"  # CRITICAL: #75 hang detection
        transitions:
          - "state.has_run = true"

Stateful Fuzzing Engine:
-------------------------
1. Initialization: Start with empty set of active objects
2. Action Selection:
   - Create new object (call constructor)
   - Call method on existing object
3. Operation Generation:
   - Get valid operations from schema
   - Filter out operations with unmet preconditions
   - Select random valid operation
   - Generate parameters
4. Code Generation: Emit Ruchy code for action
5. Shadow State Update: Update internal state per schema transitions
6. Property Injection: Insert timeout checks and assertions
7. Loop: Repeat for N operations (e.g., 100 operations per test case)

Example Generated Test Case:
-----------------------------
```ruchy
// Generated by schema-based fuzzer
fun test_runtime_behavior_001() {
    // Step 1: Create Vec (shadow: {len:0, is_empty:true})
    let v0 = Vec::new();
    assert_eq!(v0.len(), 0);
    assert!(v0.is_empty());

    // Step 2: Push (shadow: {len:1, is_empty:false})
    v0.push(42);
    assert_eq!(v0.len(), 1);
    assert!(!v0.is_empty());

    // Step 3: Create Command (shadow: {has_run:false})
    let start_time = now();
    let cmd = Command::new("ls");  // TIMEOUT CHECK INJECTED
    let elapsed = now() - start_time;
    assert!(elapsed < 100, "Command::new() took {}ms (>100ms)", elapsed);

    // Step 4: Pop (shadow: {len:0, is_empty:true})
    let val = v0.pop();
    assert_eq!(val, 42);
    assert!(v0.is_empty());

    // Step 5: Output with timeout (CRITICAL FOR #75)
    let start_time = now();
    let output = cmd.output();  // This is where #75 hangs
    let elapsed = now() - start_time;
    assert!(elapsed < 1000, "Command.output() hung for {}ms", elapsed);
}
```

Behavioral Properties (The "Oracles"):
---------------------------------------

Type 1: State Invariants (Auto-generated from schema)
  - assert_eq!(v0.len(), shadow_state.len)
  - assert_eq!(v0.is_empty(), shadow_state.is_empty)

Type 2: Timeout/Hang Detection (CRITICAL for #76, #75, #74)
  - Wrap every operation in timing check
  - Fail test if operation exceeds threshold
  - Example: Vec::new() taking >100ms = BUG

Type 3: Roundtrip Properties
  - serialize(deserialize(obj)) == obj
  - format(parse(code)) == code

Type 4: Metamorphic Properties
  - list.sort().reverse() == list.sort_descending()
  - Two different code paths should produce same result

Minimization Strategy:
----------------------
When a test fails (e.g., timeout detected), use delta debugging:
1. Start with failing sequence: [Op1, Op2, ..., Op100]
2. Binary search for minimal failing subset
3. Example: #76 Vec::new() hang reduces to single line:
   ```ruchy
   let v = Vec::new();  // Hangs forever
   ```

Implementation Priority:
------------------------
1. Phase 1: Schema parser (YAML/JSON → Rust structs)
2. Phase 2: Stateful test generator (operation sequences)
3. Phase 3: Property injection (timeout checks, assertions)
4. Phase 4: Test execution + minimization

Why This Solves Runtime Hangs:
-------------------------------
- Current grammar fuzzer: Generates SYNTAX (strings of code)
- Schema fuzzer: Generates BEHAVIOR (sequences of operations)
- Timeout properties: Detect hangs (#75, #76, #74) immediately
- Preconditions: Only generate VALID operation sequences
- Shadow state: Track expected state, catch state bugs
- Minimization: Reduce "Vec::new() hangs" from 100-line test to 1 line
```

**Technique 3: Property-Based Testing**
```
Goal: Test compiler invariants (properties that should always hold)

Property 1: Parse Roundtrip
  ∀ code: parse(emit(parse(code))) == parse(code)

Property 2: Format Idempotence
  ∀ code: format(format(code)) == format(code)

Property 3: Optimization Correctness
  ∀ code: run(optimize(code)) == run(code)

Property 4: Type Safety
  ∀ code: type_check(code) = OK → run(code) ≠ type_error
```

**Implementation**:
```rust
#[property_test]
fn parse_roundtrip(code: ValidRuchyCode) {
    let ast1 = parse(&code);
    let emitted = emit(&ast1);
    let ast2 = parse(&emitted);

    assert_eq!(ast1, ast2, "Parse roundtrip failed");
}

#[property_test]
fn format_idempotence(code: ValidRuchyCode) {
    let formatted1 = format(&code);
    let formatted2 = format(&formatted1);

    assert_eq!(formatted1, formatted2, "Format not idempotent");
}

#[property_test]
fn optimization_correctness(code: ValidRuchyCode) {
    let output1 = run(&code);
    let optimized = optimize(&code);
    let output2 = run(&optimized);

    assert_eq!(output1, output2, "Optimization changed behavior");
}
```

**Technique 4: Code Churn Analysis**
```
Goal: Identify bug-prone files from git history

Algorithm:
1. git log --numstat --since="100 commits ago"
2. Count commits per file
3. Cross-reference with closed bug issues
4. Calculate bugs/commit ratio
5. Flag files with ratio >0.3 as HIGH RISK

Output:
parser.rs: 18 commits, 8 bugs → 0.44 bugs/commit → 🔴 CRITICAL
formatter.rs: 15 commits, 4 bugs → 0.27 bugs/commit → 🟠 HIGH
```

**Implementation**:
```rust
struct ChurnAnalyzer {
    git_repo: Repository,
    issue_tracker: IssueTracker,
}

impl ChurnAnalyzer {
    fn analyze_churn(&self, commit_limit: usize) -> Vec<ChurnReport> {
        let commits = self.git_repo.last_n_commits(commit_limit);
        let mut file_churn: HashMap<String, FileChurn> = HashMap::new();

        for commit in commits {
            for file in commit.changed_files() {
                file_churn.entry(file.path.clone())
                    .or_insert(FileChurn::default())
                    .commit_count += 1;
            }
        }

        // Cross-reference with bugs
        for issue in self.issue_tracker.closed_bugs() {
            if let Some(files) = issue.affected_files() {
                for file in files {
                    if let Some(churn) = file_churn.get_mut(&file) {
                        churn.bug_count += 1;
                    }
                }
            }
        }

        // Calculate risk
        file_churn.into_iter()
            .map(|(path, churn)| ChurnReport {
                file: path,
                commits: churn.commit_count,
                bugs: churn.bug_count,
                bugs_per_commit: churn.bug_count as f64 / churn.commit_count as f64,
                risk_level: if churn.bugs_per_commit > 0.3 { RiskLevel::Critical }
                           else if churn.bugs_per_commit > 0.2 { RiskLevel::High }
                           else { RiskLevel::Medium },
            })
            .collect()
    }
}
```

### 5.2 Discovery Output

**Bug Discovery Report Format**:
```yaml
bug_id: BUG-DISC-001
discovery_date: 2025-10-29
discovery_technique: grammar_fuzzing
bug_type: runtime_hang
severity: critical
test_case:
  input_program: |
    fun test() {
        let v = Vec::new()
        println("done")
    }
  expected: "done"
  actual: "[infinite hang]"
  timeout: 5s
minimal_reproduction: true
affects_versions: [v3.147.0, v3.147.1]
working_versions: [v3.146.0]
```

---

## 6. Bug Replicator Module

### 6.1 Minimization Strategy

**Delta Debugging Algorithm**:
```
Goal: Find smallest program that reproduces bug

Algorithm:
1. Start with failing program P
2. Split P into chunks: [C1, C2, C3, ...]
3. For each chunk Ci:
   a. Try removing Ci: P' = P - Ci
   b. If P' still fails: P = P', goto 2
4. When no chunk can be removed: P is minimal

Example:
Original (30 lines) → Remove half (15 lines) → Still fails
                   → Remove half (7 lines) → Still fails
                   → Remove half (3 lines) → Still fails
                   → Cannot remove more → MINIMAL (3 lines)
```

**Implementation: Line-Based Delta Debugging** (fallback for unparseable code):
```rust
fn delta_debug_lines(program: &str, test_fn: impl Fn(&str) -> bool) -> String {
    let mut lines: Vec<&str> = program.lines().collect();

    loop {
        let chunk_size = lines.len() / 2;
        if chunk_size == 0 { break; }

        let mut made_progress = false;

        for i in 0..lines.len() {
            // Try removing chunk starting at i
            let candidate: Vec<&str> = lines.iter()
                .enumerate()
                .filter(|(idx, _)| *idx < i || *idx >= i + chunk_size)
                .map(|(_, line)| *line)
                .collect();

            let candidate_program = candidate.join("\n");

            // Does smaller version still trigger bug?
            if test_fn(&candidate_program) {
                lines = candidate;
                made_progress = true;
                break;  // Start over with smaller program
            }
        }

        if !made_progress { break; }
    }

    lines.join("\n")
}
```

**Implementation: Tree-Based AST Minimization** (preferred for structured code):
```rust
/// Hierarchical Delta Debugging on Abstract Syntax Tree
/// References: Zeller & Hildebrandt (2002) "Simplifying and Isolating Failure-Inducing Input"
///            Misherghi & Su (2006) "HDD: Hierarchical Delta Debugging"
fn delta_debug_ast(program: &str, test_fn: impl Fn(&str) -> bool) -> String {
    // Parse into AST
    let ast = match parse_ruchy(program) {
        Ok(ast) => ast,
        Err(_) => return delta_debug_lines(program, test_fn),  // Fallback to line-based
    };

    let minimized_ast = minimize_ast_node(ast, &test_fn);

    emit_ruchy(&minimized_ast)
}

/// Recursively minimize AST nodes while preserving bug
fn minimize_ast_node(node: ASTNode, test_fn: &impl Fn(&str) -> bool) -> ASTNode {
    match node {
        ASTNode::Program(items) => {
            let minimized_items = minimize_items(items, test_fn);
            ASTNode::Program(minimized_items)
        },

        ASTNode::Function { name, params, body, .. } => {
            // Try removing the entire function
            if test_fn(&emit_ruchy(&ASTNode::Program(vec![]))) {
                return ASTNode::Program(vec![]);  // Function not needed
            }

            // Try minimizing function body
            let minimized_body = minimize_statements(body, test_fn, &name);
            ASTNode::Function { name, params, body: minimized_body }
        },

        ASTNode::Block(stmts) => {
            let minimized_stmts = minimize_statements(stmts, test_fn, "block");
            ASTNode::Block(minimized_stmts)
        },

        ASTNode::If { condition, then_branch, else_branch } => {
            // Try replacing If with just then_branch
            if test_fn(&emit_ruchy(&then_branch)) {
                return minimize_ast_node(*then_branch, test_fn);
            }

            // Try replacing If with just else_branch
            if let Some(else_b) = else_branch {
                if test_fn(&emit_ruchy(&else_b)) {
                    return minimize_ast_node(*else_b, test_fn);
                }
            }

            // Recursively minimize branches
            let min_then = minimize_ast_node(*then_branch, test_fn);
            let min_else = else_branch.map(|e| Box::new(minimize_ast_node(*e, test_fn)));

            ASTNode::If {
                condition,
                then_branch: Box::new(min_then),
                else_branch: min_else,
            }
        },

        _ => node,  // Leaf nodes (literals, identifiers) cannot be minimized
    }
}

/// Minimize a list of statements/items using delta debugging
fn minimize_statements(
    stmts: Vec<ASTNode>,
    test_fn: &impl Fn(&str) -> bool,
    context: &str
) -> Vec<ASTNode> {
    let mut current = stmts;

    loop {
        let chunk_size = (current.len() / 2).max(1);
        let mut made_progress = false;

        for i in 0..current.len() {
            // Try removing chunk [i..i+chunk_size]
            let candidate: Vec<ASTNode> = current.iter()
                .enumerate()
                .filter(|(idx, _)| *idx < i || *idx >= i + chunk_size)
                .map(|(_, node)| node.clone())
                .collect();

            if candidate.is_empty() {
                continue;  // Must keep at least one statement
            }

            // Build test program with reduced statements
            let test_program = build_test_program(&candidate, context);

            if test_fn(&test_program) {
                current = candidate;
                made_progress = true;
                break;
            }
        }

        if !made_progress { break; }
    }

    // Recursively minimize each remaining statement
    current.into_iter()
        .map(|stmt| minimize_ast_node(stmt, test_fn))
        .collect()
}

/// Build executable test program from AST fragments
fn build_test_program(nodes: &[ASTNode], context: &str) -> String {
    match context {
        "block" => {
            let stmts = nodes.iter().map(|n| emit_ruchy(n)).collect::<Vec<_>>().join("\n");
            format!("fun test() {{\n{}\n}}", stmts)
        },
        name if name.starts_with("fun_") => {
            // Already a function body
            nodes.iter().map(|n| emit_ruchy(n)).collect::<Vec<_>>().join("\n")
        },
        _ => {
            // Top-level items
            nodes.iter().map(|n| emit_ruchy(n)).collect::<Vec<_>>().join("\n\n")
        }
    }
}
```

**Key Advantages of AST-Based Minimization**:
1. **Preserves syntactic correctness**: Never generates invalid syntax
2. **More efficient**: O(log n) complexity vs O(n²) for line-based
3. **Semantic awareness**: Understands function boundaries, block structure
4. **Better minimization**: Can remove entire functions, if/else branches, loop bodies
5. **Graceful degradation**: Falls back to line-based if parsing fails

**Example Comparison**:
```rust
// Original (15 lines, parse error)
fun main() {
    let x = 5
    let y = 10
    if x < y {
        println("x is less")
        let z = Vec::new()  // ← BUG
        println("done")
    } else {
        println("x is greater")
    }
}

// Line-based minimization (12 lines - might break syntax)
fun main() {
    let x = 5
    if x < y {
        let z = Vec::new()  // ← BUG
        println("done")
    }
}

// AST-based minimization (4 lines - preserves syntax)
fun main() {
    let z = Vec::new()  // ← BUG
}
```

### 6.2 Standalone Test Generation

**Test File Template**:
```ruchy
// AUTO-GENERATED: Bug reproduction test for BUG-DISC-001
// Generated: 2025-10-29
// Issue: Runtime hang on Vec::new()
// Minimal reproduction: 3 lines

fun test_bug_disc_001() {
    // ARRANGE: Setup test conditions
    let expected = "done"

    // ACT: Trigger the bug
    let v = Vec::new()
    println("done")

    // ASSERT: This line is never reached due to hang
    assert_eq!(get_output(), expected)
}

// Run with: ruchy test test_bug_disc_001.ruchy
// Expected: Test should complete in <1s
// Actual: Hangs indefinitely on Vec::new()
```

### 6.3 TDD Workflow Generation

**RED Phase (Failing Test)**:
```ruchy
// RED: Write test that exposes the bug

#[test]
fun test_vec_new_should_not_hang() {
    let start_time = now()
    let v = Vec::new()  // BUG: Hangs here
    let elapsed = now() - start_time

    assert!(elapsed < 1000, "Vec::new() should complete in <1s")
}

// Result: ❌ FAIL (timeout after 5s)
```

**GREEN Phase (Fix Specification)**:
```rust
// GREEN: Minimal fix to make test pass

// In runtime/vec.rs:
impl Vec {
    pub fn new() -> Self {
        // OLD (buggy):
        // loop { /* infinite loop bug */ }

        // NEW (fixed):
        Vec {
            data: Vec::new(),  // Use Rust's Vec
            len: 0,
        }
    }
}

// Result: ✅ PASS (completes in <1ms)
```

**REFACTOR Phase (Improve)**:
```rust
// REFACTOR: Improve implementation while keeping tests green

impl Vec {
    pub fn new() -> Self {
        Self::with_capacity(0)  // Delegate to with_capacity
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vec {
            data: Vec::with_capacity(capacity),
            len: 0,
        }
    }
}

// Result: ✅ PASS (all tests still passing)
```

---

## 7. Bug Reporter Module

### 7.1 Report Structure

**Markdown Template** (see Section 9 for full template)

**Key Sections**:
1. **Executive Summary** - TL;DR for busy developers
2. **Bug Details** - Reproduction steps, expected vs. actual
3. **Quantitative Analysis** - All metrics (complexity, SATD, churn, etc.)
4. **Root Cause Analysis** - Five-Whys, call chains, dependencies
5. **Fix Strategy** - TDD workflow, test recommendations
6. **Prevention** - How to prevent similar bugs
7. **Related Issues** - Links to similar bugs

### 7.2 Quantitative Metrics

**Metric 1: Complexity Analysis**
```rust
struct ComplexityMetrics {
    cyclomatic_complexity: u32,      // Number of decision points
    cognitive_complexity: u32,        // Human perceived difficulty
    max_nesting_depth: u32,          // Deepest nested block
    function_length: u32,            // Lines of code
    big_o_time: Complexity,          // O(1), O(n), O(n²), etc.
    big_o_space: Complexity,         // Memory complexity
}

// Example analysis:
parser.rs::parse_expression():
  Cyclomatic: 47 (🔴 CRITICAL - threshold: 20)
  Cognitive: 62 (🔴 CRITICAL - threshold: 30)
  Nesting: 8 levels (🟠 HIGH - threshold: 5)
  Length: 450 LOC (🔴 CRITICAL - threshold: 200)
  Time: O(n²) (🟠 HIGH - should be O(n))

Recommendation: REFACTOR REQUIRED - Extract helper functions
```

**Metric 2: SATD (Self-Admitted Technical Debt)**
```rust
enum SATDType {
    TODO,
    FIXME,
    HACK,
    XXX,
    BUG,
    REFACTOR,
    OPTIMIZE,
}

struct SATDAnalysis {
    total_count: u32,
    by_type: HashMap<SATDType, u32>,
    by_file: HashMap<String, u32>,
    oldest: SATDComment,  // Date, age in days
}

// Example analysis:
parser.rs:
  TOTAL SATD: 23 comments
  TODO: 12 (52%)
  FIXME: 7 (30%)
  HACK: 4 (17%)

  Oldest: "// TODO: Handle edge case" (347 days old)

  CORRELATION: Files with >15 SATD have 3x more bugs
```

**Metric 3: Code Churn**
```rust
struct ChurnMetrics {
    commits_last_100: u32,
    lines_added: u32,
    lines_deleted: u32,
    churn_rate: f64,  // (added + deleted) / total_lines
    bug_count: u32,
    bugs_per_commit: f64,
}

// Example:
parser.rs (last 100 commits):
  Commits: 18
  Added: 3,450 lines
  Deleted: 2,100 lines
  Churn rate: 0.85 (🔴 CRITICAL - threshold: 0.5)
  Bugs: 8
  Bugs/commit: 0.44 (🔴 CRITICAL - threshold: 0.2)

Analysis: HIGHEST RISK FILE - High churn + high bug rate
```

**Metric 4: Formalization Hints for Verification** (not a "provability score")
```rust
/// Identify functions that are good candidates for formal verification
/// and generate actionable hints for formalization
///
/// References: Leino (2010) "Dafny: An automatic program verifier for functional correctness"
///            Filliâtre & Paskevich (2013) "Why3 — Where Programs Meet Provers"
struct FormalizationAnalysis {
    good_candidates: Vec<FunctionInfo>,      // Pure functions, simple logic
    poor_candidates: Vec<FunctionInfo>,      // I/O, mutable state, complex
    formalization_hints: Vec<FormalizationHint>,
}

struct FunctionInfo {
    name: String,
    is_pure: bool,                    // No side effects
    uses_mutable_state: bool,
    has_io: bool,
    loop_count: usize,
    branch_count: usize,
    candidacy_score: CandidacyScore,
}

#[derive(Debug)]
enum CandidacyScore {
    Excellent { reason: &'static str },  // Pure, simple logic
    Good { reason: &'static str },       // Minor issues (e.g., 1-2 loops)
    Poor { reason: &'static str },       // Significant barriers
    Impractical { reason: &'static str }, // Not worth formalizing
}

#[derive(Debug)]
enum FormalizationHint {
    // Pre/postconditions
    AddPrecondition { function: String, hint: String },
    AddPostcondition { function: String, hint: String },
    AddLoopInvariant { function: String, loop_line: usize, hint: String },

    // Simplification needed
    ExtractPureFunction { function: String, suggestion: String },
    RemoveSideEffects { function: String, issue: String },
    SimplifyControlFlow { function: String, branch_count: usize },

    // Formal methods tool suggestions
    ToolSuggestion { function: String, tool: &'static str, rationale: String },
}

/// Analyze function for formal verification candidacy
fn analyze_formalization_candidacy(func: &FunctionAST) -> FunctionInfo {
    let is_pure = !has_side_effects(func) && !has_io(func);
    let uses_mutable_state = has_mutable_state(func);
    let has_io_ops = has_io(func);
    let loops = count_loops(func);
    let branches = count_branches(func);

    let candidacy = classify_candidacy(is_pure, uses_mutable_state, has_io_ops, loops, branches);

    FunctionInfo {
        name: func.name.clone(),
        is_pure,
        uses_mutable_state,
        has_io: has_io_ops,
        loop_count: loops,
        branch_count: branches,
        candidacy_score: candidacy,
    }
}

fn classify_candidacy(
    is_pure: bool,
    mutable_state: bool,
    has_io: bool,
    loops: usize,
    branches: usize
) -> CandidacyScore {
    use CandidacyScore::*;

    // Impractical: I/O-heavy, extensive mutation
    if has_io {
        return Impractical { reason: "Function performs I/O - formal verification not practical" };
    }

    if mutable_state && loops > 3 {
        return Impractical { reason: "Extensive mutable state + complex loops - manual invariants too difficult" };
    }

    // Excellent: Pure functions with simple logic
    if is_pure && branches < 5 && loops <= 1 {
        return Excellent { reason: "Pure function with simple control flow - ideal for Dafny or F*" };
    }

    // Good: Minor issues, but formalizable
    if is_pure && branches < 15 && loops <= 3 {
        return Good { reason: "Pure function, moderate complexity - consider using Why3 or Dafny" };
    }

    // Poor: Significant barriers, may not be worth effort
    if mutable_state {
        return Poor { reason: "Mutable state requires complex invariants - consider refactoring to pure functions first" };
    }

    Poor { reason: "Complex control flow - simplification recommended before formalization" }
}

/// Generate actionable formalization hints
fn generate_formalization_hints(func: &FunctionAST, info: &FunctionInfo) -> Vec<FormalizationHint> {
    let mut hints = vec![];

    match info.candidacy_score {
        CandidacyScore::Excellent { .. } | CandidacyScore::Good { .. } => {
            // Generate pre/postconditions
            if let Some(precond) = infer_precondition(func) {
                hints.push(FormalizationHint::AddPrecondition {
                    function: func.name.clone(),
                    hint: precond,
                });
            }

            if let Some(postcond) = infer_postcondition(func) {
                hints.push(FormalizationHint::AddPostcondition {
                    function: func.name.clone(),
                    hint: postcond,
                });
            }

            // Generate loop invariants for each loop
            for (line, invariant) in infer_loop_invariants(func) {
                hints.push(FormalizationHint::AddLoopInvariant {
                    function: func.name.clone(),
                    loop_line: line,
                    hint: invariant,
                });
            }

            // Suggest appropriate tool
            let tool = if info.is_pure && info.loop_count <= 1 {
                "Dafny (automated verification, great for pure functions)"
            } else if info.loop_count <= 3 {
                "Why3 (SMT-based, good for moderate complexity)"
            } else {
                "F* (dependent types, handles complex invariants)"
            };

            hints.push(FormalizationHint::ToolSuggestion {
                function: func.name.clone(),
                tool,
                rationale: format!("Pure: {}, Loops: {}, Branches: {}",
                                   info.is_pure, info.loop_count, info.branch_count),
            });
        },

        CandidacyScore::Poor { reason } => {
            // Suggest refactoring before formalization
            if info.uses_mutable_state {
                hints.push(FormalizationHint::RemoveSideEffects {
                    function: func.name.clone(),
                    issue: "Function uses mutable state - refactor to pure functions first".to_string(),
                });
            }

            if info.branch_count > 15 {
                hints.push(FormalizationHint::SimplifyControlFlow {
                    function: func.name.clone(),
                    branch_count: info.branch_count,
                });
            }
        },

        CandidacyScore::Impractical { .. } => {
            // No hints - not worth formalizing
        },

        _ => {},
    }

    hints
}

// Example Output:
parse_expression():
  Candidacy: POOR - "Mutable state requires complex invariants"

  Formalization Hints:
  1. [REFACTORING NEEDED] Remove side effects:
     - Extract pure helper: parse_primary_expression()
     - Extract pure helper: parse_binary_operator()
     - Keep mutable state isolated in top-level function

  2. [AFTER REFACTORING] Add preconditions:
     - requires tokens.is_valid()
     - requires tokens.current_position() < tokens.length()

  3. [AFTER REFACTORING] Add postconditions:
     - ensures result.is_ok() ==> result.unwrap().is_valid_ast()
     - ensures tokens.current_position() >= old(tokens.current_position())

  4. [NOT RECOMMENDED] Due to complexity (47 branches, mutable state),
     formal verification not practical without significant refactoring
```

**Key Improvements**:
1. **Honest framing**: "Formalization Hints", not "Provability Score"
2. **Candidacy classification**: Excellent/Good/Poor/Impractical with rationale
3. **Actionable hints**: Specific pre/post/invariant suggestions
4. **Tool recommendations**: Dafny/Why3/F* based on function characteristics
5. **Refactoring guidance**: Suggests refactoring before formalization when needed

**Metric 5: Dependency Analysis**
```rust
struct DependencyMetrics {
    fanin: u32,   // How many files depend on this file
    fanout: u32,  // How many files this file depends on
    instability: f64,  // fanout / (fanin + fanout)
    abstractness: f64,  // interfaces / total_types
    distance_from_main_sequence: f64,  // |abstractness + instability - 1|
}

// Example:
parser.rs:
  Fan-in: 23 files depend on parser.rs (🟠 HIGH)
  Fan-out: 7 files (lexer, ast, error, etc.)
  Instability: 0.23 (stable - hard to change)
  Abstractness: 0.1 (concrete - few interfaces)
  Distance: 0.67 (🔴 "Zone of Pain" - stable + concrete)

Risk: Changes to parser.rs affect 23 files - high blast radius
```

### 7.3 Root Cause Analysis (Assisted Five-Whys)

**Important**: The system provides *data-driven hypotheses* for each "Why", but humans must validate the causal links. Automated root cause analysis cannot determine intent or make judgment calls about process failures.

**Five-Whys Assisted Analysis Framework**:
```rust
/// Assist (not automate) Five-Whys root cause analysis
/// System provides data, developer validates causal reasoning
struct AssistedFiveWhys {
    problem_statement: String,
    whys: Vec<WhyHypothesis>,
    root_cause_hypothesis: String,  // Requires human validation
    prevention_strategies: Vec<PreventionStrategy>,
}

struct WhyHypothesis {
    question: String,
    data_points: Vec<DataPoint>,     // System provides this
    answer_hypothesis: String,        // System suggests, human validates
    confidence: HypothesisConfidence,
}

#[derive(Debug)]
enum DataPoint {
    // Objective data the system can provide
    ComplexityMetric { name: String, value: f64, threshold: f64 },
    ChurnMetric { commits: u32, bugs: u32, rate: f64 },
    TestCoverage { coverage_pct: f64, mutation_score: f64 },
    CommitInfo { hash: String, message: String, author: String, date: String },
    SATDComment { message: String, age_days: u32 },
    DiffInfo { files_changed: Vec<String>, lines_added: u32, lines_deleted: u32 },
}

#[derive(Debug)]
enum HypothesisConfidence {
    High { reason: &'static str },    // Strong evidence from data
    Medium { reason: &'static str },  // Some evidence, needs validation
    Low { reason: &'static str },     // Speculation based on patterns
}

impl AssistedFiveWhys {
    /// Generate Five-Whys template with data-driven hypotheses
    fn generate_assisted_analysis(bug: &Bug, metrics: &QuantitativeMetrics) -> Self {
        let mut whys = vec![];

        // Why 1: What is the immediate technical cause?
        whys.push(WhyHypothesis {
            question: format!("Why does {} occur?", bug.symptom),
            data_points: vec![
                DataPoint::CommitInfo {
                    hash: metrics.last_changed_commit.clone(),
                    message: metrics.commit_message.clone(),
                    author: metrics.author.clone(),
                    date: metrics.commit_date.clone(),
                },
                DataPoint::DiffInfo {
                    files_changed: metrics.files_in_commit.clone(),
                    lines_added: metrics.lines_added,
                    lines_deleted: metrics.lines_deleted,
                },
            ],
            answer_hypothesis: format!("Implementation contains {} (based on commit {} diff analysis)",
                                       metrics.likely_defect_type, metrics.last_changed_commit),
            confidence: HypothesisConfidence::High {
                reason: "Direct evidence from git blame and diff analysis"
            },
        });

        // Why 2: Why was the defect introduced?
        whys.push(WhyHypothesis {
            question: "Why was the defective code introduced?",
            data_points: vec![
                DataPoint::ComplexityMetric {
                    name: "Cyclomatic Complexity".to_string(),
                    value: metrics.cyclomatic as f64,
                    threshold: 20.0,
                },
                DataPoint::ComplexityMetric {
                    name: "Cognitive Complexity".to_string(),
                    value: metrics.cognitive as f64,
                    threshold: 30.0,
                },
            ],
            answer_hypothesis: if metrics.cyclomatic > 40 {
                format!("High complexity (cyclomatic: {}) makes logic errors likely", metrics.cyclomatic)
            } else {
                "Hypothesis: Refactoring or feature addition introduced edge case".to_string()
            },
            confidence: if metrics.cyclomatic > 40 {
                HypothesisConfidence::High { reason: "Complexity strongly correlated with defects" }
            } else {
                HypothesisConfidence::Medium { reason: "Requires commit message and code review analysis" }
            },
        });

        // Why 3: Why didn't testing catch it?
        whys.push(WhyHypothesis {
            question: "Why didn't testing catch the defect before merge?",
            data_points: vec![
                DataPoint::TestCoverage {
                    coverage_pct: metrics.test_coverage,
                    mutation_score: metrics.mutation_score,
                },
            ],
            answer_hypothesis: if metrics.mutation_score < 0.6 {
                format!("Low mutation score ({:.0}%) indicates weak tests - defect not exercised",
                        metrics.mutation_score * 100.0)
            } else {
                "Hypothesis: Edge case not covered by existing test suite".to_string()
            },
            confidence: if metrics.mutation_score < 0.6 {
                HypothesisConfidence::High { reason: "Mutation score directly measures test quality" }
            } else {
                HypothesisConfidence::Medium { reason: "May be novel edge case" }
            },
        });

        // Why 4: Why was test coverage insufficient?
        whys.push(WhyHypothesis {
            question: "Why was test coverage/quality insufficient?",
            data_points: vec![
                DataPoint::ChurnMetric {
                    commits: metrics.churn_commits,
                    bugs: metrics.churn_bugs,
                    rate: metrics.bugs_per_commit,
                },
                DataPoint::SATDComment {
                    message: metrics.oldest_satd.message.clone(),
                    age_days: metrics.oldest_satd.age_days,
                },
            ],
            answer_hypothesis: if metrics.churn_commits > 10 && metrics.bugs_per_commit > 0.3 {
                format!("High churn ({} commits, {:.2} bugs/commit) suggests rushed changes without TDD",
                        metrics.churn_commits, metrics.bugs_per_commit)
            } else if !metrics.oldest_satd.message.is_empty() {
                format!("SATD debt ({} days old: '{}') indicates deferred quality work",
                        metrics.oldest_satd.age_days, metrics.oldest_satd.message)
            } else {
                "Hypothesis: Code added without TDD workflow (test-first approach)".to_string()
            },
            confidence: if metrics.bugs_per_commit > 0.3 {
                HypothesisConfidence::High { reason: "High bugs/commit strongly indicates quality issues" }
            } else {
                HypothesisConfidence::Low { reason: "Speculation - requires process review" }
            },
        });

        // Why 5: Why wasn't quality process followed?
        whys.push(WhyHypothesis {
            question: "Why wasn't quality process (TDD, mutation testing) enforced?",
            data_points: vec![], // No quantitative data - requires human input
            answer_hypothesis: "Hypothesis: No enforcement of TDD workflow in CI/CD (HUMAN VALIDATION REQUIRED)".to_string(),
            confidence: HypothesisConfidence::Low {
                reason: "Process questions require human judgment about team practices and policies"
            },
        });

        let root_cause_hypothesis = format!(
            "HYPOTHESIS (requires validation): {} + {} + {}",
            if metrics.cyclomatic > 40 { "High complexity" } else { "Complex logic" },
            if metrics.mutation_score < 0.6 { "Weak tests" } else { "Edge case" },
            if metrics.bugs_per_commit > 0.3 { "Rushed development" } else { "Process gap" }
        );

        AssistedFiveWhys {
            problem_statement: bug.description.clone(),
            whys,
            root_cause_hypothesis,
            prevention_strategies: generate_prevention_strategies(metrics),
        }
    }

    /// Format for markdown report
    fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("## 🔍 Root Cause Analysis: Five-Whys (Assisted)\n\n");
        md.push_str("**⚠️ IMPORTANT**: Answers are *data-driven hypotheses*. ");
        md.push_str("Human validation required for causal links.\n\n");
        md.push_str(&format!("**Problem**: {}\n\n", self.problem_statement));

        for (i, why) in self.whys.iter().enumerate() {
            md.push_str(&format!("**Why {}**: {}\n", i + 1, why.question));

            // Show data points
            if !why.data_points.is_empty() {
                md.push_str("**Data Points**:\n");
                for dp in &why.data_points {
                    md.push_str(&format!("- {}\n", format_data_point(dp)));
                }
            }

            // Show hypothesis
            md.push_str(&format!("**Hypothesis**: {}\n", why.answer_hypothesis));
            md.push_str(&format!("**Confidence**: {:?}\n\n", why.confidence));
        }

        md.push_str(&format!("**ROOT CAUSE HYPOTHESIS**: {}\n\n", self.root_cause_hypothesis));
        md.push_str("**👤 ACTION REQUIRED**: Developer must validate causal chain and refine root cause.\n\n");

        md
    }
}
```

**Example Output**:
```markdown
## 🔍 Root Cause Analysis: Five-Whys (Assisted)

**⚠️ IMPORTANT**: Answers are *data-driven hypotheses*. Human validation required for causal links.

**Problem**: Vec::new() causes infinite hang in v3.147.0

**Why 1**: Why does Vec::new() hang?
**Data Points**:
- Commit: abc123 "Refactor Vec implementation" by alice, 2025-10-15
- Diff: runtime/vec.rs (+45 lines, -12 lines)
**Hypothesis**: Implementation contains infinite loop (based on commit abc123 diff analysis)
**Confidence**: High - Direct evidence from git blame and diff analysis

**Why 2**: Why was the defective code introduced?
**Data Points**:
- Cyclomatic Complexity: 47 (threshold: 20)
- Cognitive Complexity: 62 (threshold: 30)
**Hypothesis**: High complexity (cyclomatic: 47) makes logic errors likely
**Confidence**: High - Complexity strongly correlated with defects

**Why 3**: Why didn't testing catch the defect before merge?
**Data Points**:
- Test Coverage: 73%
- Mutation Score: 45%
**Hypothesis**: Low mutation score (45%) indicates weak tests - defect not exercised
**Confidence**: High - Mutation score directly measures test quality

**Why 4**: Why was test coverage/quality insufficient?
**Data Points**:
- Churn: 18 commits, 8 bugs, 0.44 bugs/commit
- SATD: "TODO: Add proper Vec tests" (347 days old)
**Hypothesis**: High churn (18 commits, 0.44 bugs/commit) suggests rushed changes without TDD
**Confidence**: High - High bugs/commit strongly indicates quality issues

**Why 5**: Why wasn't quality process (TDD, mutation testing) enforced?
**Data Points**: (none - requires human input)
**Hypothesis**: No enforcement of TDD workflow in CI/CD (HUMAN VALIDATION REQUIRED)
**Confidence**: Low - Process questions require human judgment

**ROOT CAUSE HYPOTHESIS**: High complexity + Weak tests + Rushed development

**👤 ACTION REQUIRED**: Developer must validate causal chain and refine root cause.
```

**Key Principles** (Jidoka):
1. **System provides data**: Objective metrics from git, complexity analysis, test results
2. **System suggests hypotheses**: Based on research correlations (complexity→bugs, churn→bugs)
3. **Human validates causality**: Only developer can confirm *why* process wasn't followed
4. **Confidence scoring**: Helps developer know which hypotheses are strong vs. speculative
```

### 7.4 Fix Strategy

**TDD Workflow Specification**:
```markdown
## Fix Strategy

### Phase 1: RED (Write Failing Test)

\`\`\`ruchy
#[test]
fun test_vec_new_completes_quickly() {
    let start = now()
    let v = Vec::new()
    let elapsed = now() - start
    assert!(elapsed < 100, "Vec::new() should complete in <100ms")
}
\`\`\`

**Expected**: ❌ FAIL (timeout or assertion)

### Phase 2: GREEN (Minimal Fix)

\`\`\`rust
// In runtime/vec.rs
impl Vec {
    pub fn new() -> Self {
        Vec {
            data: std::vec::Vec::new(),
            len: 0,
        }
    }
}
\`\`\`

**Expected**: ✅ PASS

### Phase 3: REFACTOR (Improve)

\`\`\`rust
impl Vec {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vec {
            data: std::vec::Vec::with_capacity(capacity),
            len: 0,
        }
    }
}
\`\`\`

**Expected**: ✅ PASS (all tests still green)

### Phase 4: VALIDATION

Run comprehensive test suite:
- Unit tests: ✅ 12/12 passing
- Mutation tests: ✅ 95% score (18/19 mutations killed)
- Property tests: ✅ 8/8 properties verified
- Fuzz tests: ✅ 10,000 inputs, 0 crashes
- Regression tests: ✅ All previous bugs still fixed
```

### 7.5 Prevention Strategy

**Template**:
```markdown
## Prevention Strategy

### Immediate Actions
1. **Add Regression Test**: Create test_vec_new_no_hang.ruchy
2. **Update Documentation**: Document Vec::new() behavior
3. **Code Review**: Require review for runtime/* changes

### Short-Term (Week 1-2)
1. **Increase Coverage**: Raise Vec module coverage to >80%
2. **Add Mutation Tests**: Achieve >90% mutation score
3. **Property Tests**: Add 5 properties for Vec operations

### Long-Term (Month 1+)
1. **Enforce TDD**: Pre-commit hook requires RED test first
2. **CI/CD Integration**: Block merges with <80% mutation score
3. **Code Churn Monitoring**: Alert on files with >0.3 bugs/commit

### Similar Bug Prevention
**Pattern**: Runtime hangs on Vec operations

**Detection**:
- Code Churn: Flag runtime/* files with >10 commits
- Complexity: Alert on functions with CC >20
- Mutation: Require >90% score for runtime code

**Prevention**:
- Timeout tests: All runtime ops must complete in <1s
- Fuzz testing: Generate random Vec operations
- Differential: Compare with Rust std::vec::Vec behavior
```

---

## 8. Quantitative Analysis Framework

### 8.1 Metric Calculation Methods

**Cyclomatic Complexity**:
```rust
fn calculate_cyclomatic_complexity(ast: &FunctionAST) -> u32 {
    let mut complexity = 1;  // Base complexity

    ast.visit_nodes(|node| {
        match node {
            Node::If(_) => complexity += 1,
            Node::While(_) => complexity += 1,
            Node::For(_) => complexity += 1,
            Node::Match(arms) => complexity += arms.len() as u32,
            Node::LogicalAnd | Node::LogicalOr => complexity += 1,
            _ => {}
        }
    });

    complexity
}

// Thresholds:
// 1-10: Simple (GREEN)
// 11-20: Moderate (YELLOW)
// 21-50: Complex (ORANGE)
// 50+: Very Complex (RED)
```

**Cognitive Complexity** (human-perceived difficulty):
```rust
fn calculate_cognitive_complexity(ast: &FunctionAST) -> u32 {
    let mut complexity = 0;
    let mut nesting_level = 0;

    ast.visit_nodes_with_depth(|node, depth| {
        match node {
            Node::If(_) | Node::While(_) | Node::For(_) => {
                complexity += 1 + depth;  // Nested control flow is harder
                nesting_level = depth + 1;
            }
            Node::Match(_) => complexity += 1 + depth,
            Node::LogicalAnd | Node::LogicalOr => {
                if depth > 0 {
                    complexity += 1;  // In nested context
                }
            }
            Node::Continue | Node::Break => complexity += 1,
            _ => {}
        }
    });

    complexity
}

// Thresholds:
// 1-15: Understandable (GREEN)
// 16-30: Moderate (YELLOW)
// 31-60: Difficult (ORANGE)
// 60+: Very Difficult (RED)
```

**Loop Nesting Depth Heuristic** (NOT true Big-O analysis):
```rust
/// IMPORTANT: This is a HEURISTIC only, not formal complexity analysis
/// True algorithmic complexity analysis is undecidable (Halting Problem)
/// This provides a rough indicator for human review
///
/// For more accurate analysis, explore abstract interpretation techniques
/// or manual proof of complexity bounds with loop invariants
fn estimate_loop_complexity_heuristic(ast: &FunctionAST) -> LoopComplexityHeuristic {
    let mut max_nesting = 0;
    let mut total_loops = 0;
    let mut has_recursion = false;

    ast.visit_nodes_with_depth(|node, depth| {
        match node {
            Node::While(_) | Node::For(_) => {
                total_loops += 1;
                max_nesting = max_nesting.max(depth);
            }
            Node::FunctionCall { name, .. } if name == ast.function_name => {
                has_recursion = true;
            }
            _ => {}
        }
    });

    LoopComplexityHeuristic {
        max_nesting_depth: max_nesting,
        total_loop_count: total_loops,
        has_recursion,
        likely_class: classify_loop_pattern(max_nesting, total_loops, has_recursion),
    }
}

#[derive(Debug)]
struct LoopComplexityHeuristic {
    max_nesting_depth: usize,
    total_loop_count: usize,
    has_recursion: bool,
    likely_class: ComplexityClass,
}

#[derive(Debug)]
enum ComplexityClass {
    Constant { confidence: &'static str },        // No loops
    Linear { confidence: &'static str },          // Single loop
    Linearithmic { confidence: &'static str },    // Divide-and-conquer pattern
    Quadratic { confidence: &'static str },       // Nested loops (2 levels)
    Cubic { confidence: &'static str },           // Nested loops (3 levels)
    Unknown { reason: &'static str },             // Cannot determine
}

fn classify_loop_pattern(max_nesting: usize, total_loops: usize, has_recursion: bool) -> ComplexityClass {
    use ComplexityClass::*;

    if has_recursion {
        return Unknown { reason: "Recursion detected - requires recurrence relation analysis" };
    }

    match (max_nesting, total_loops) {
        (0, 0) => Constant { confidence: "HIGH - No loops detected" },
        (0, 1) => Linear { confidence: "MEDIUM - Single non-nested loop (assumes O(n) iteration)" },
        (1, 2..) => {
            // Multiple loops at same level - could be sequential O(n) or O(n²)
            Unknown { reason: "Multiple loops - need data flow analysis to determine if sequential or dependent" }
        }
        (1, _) => Quadratic { confidence: "LOW - Nested loops (2 levels), but actual complexity depends on iteration bounds" },
        (2, _) => Cubic { confidence: "LOW - Nested loops (3 levels), but actual complexity depends on iteration bounds" },
        _ => Unknown { reason: "Deep nesting or complex control flow - manual analysis required" },
    }
}

impl LoopComplexityHeuristic {
    fn to_report_string(&self) -> String {
        match &self.likely_class {
            ComplexityClass::Constant { confidence } =>
                format!("Likely O(1) - {} (Max Nesting: {}, Total Loops: {})",
                        confidence, self.max_nesting_depth, self.total_loop_count),
            ComplexityClass::Linear { confidence } =>
                format!("Likely O(n) - {} (Max Nesting: {}, Total Loops: {})",
                        confidence, self.max_nesting_depth, self.total_loop_count),
            ComplexityClass::Quadratic { confidence } =>
                format!("Likely O(n²) - {} (Max Nesting: {}, Total Loops: {})",
                        confidence, self.max_nesting_depth, self.total_loop_count),
            ComplexityClass::Cubic { confidence } =>
                format!("Likely O(n³) - {} (Max Nesting: {}, Total Loops: {})",
                        confidence, self.max_nesting_depth, self.total_loop_count),
            ComplexityClass::Unknown { reason } =>
                format!("Unknown - {} (Max Nesting: {}, Total Loops: {})",
                        reason, self.max_nesting_depth, self.total_loop_count),
            _ => format!("Unknown complexity pattern"),
        }
    }
}
```

**Key Improvements**:
1. **Honest labeling**: "Loop Nesting Depth Heuristic", not "Big-O Analysis"
2. **Confidence levels**: Each classification includes confidence (HIGH/MEDIUM/LOW)
3. **Unknown category**: Admits when analysis is insufficient
4. **Recursion detection**: Flags functions needing recurrence relation analysis
5. **Context awareness**: Explains that true complexity depends on iteration bounds and data flow

**For True Complexity Analysis**: Consider formal methods tools that use abstract interpretation or require manual annotation of loop invariants (e.g., Frama-C, WCET analyzers).

**SATD Detection with NLP Enhancement**:
```rust
/// Self-Admitted Technical Debt (SATD) Detection
/// References: Potdar & Shihab (2014) "An exploratory study on self-admitted technical debt"
///            Maldonado et al. (2017) "Detecting and quantifying different types of self-admitted technical debt"
fn detect_satd(source_code: &str) -> Vec<SATDComment> {
    // Phase 1: Regex-based detection (baseline)
    let regex_matches = detect_satd_regex(source_code);

    // Phase 2: NLP-based classification (enhanced)
    let nlp_enhanced = classify_satd_with_nlp(regex_matches);

    nlp_enhanced
}

/// Phase 1: Traditional regex-based SATD detection
fn detect_satd_regex(source_code: &str) -> Vec<SATDComment> {
    let patterns = [
        (SATDType::TODO, r"//\s*TODO:?\s*(.+)"),
        (SATDType::FIXME, r"//\s*FIXME:?\s*(.+)"),
        (SATDType::HACK, r"//\s*HACK:?\s*(.+)"),
        (SATDType::XXX, r"//\s*XXX:?\s*(.+)"),
        (SATDType::BUG, r"//\s*BUG:?\s*(.+)"),
    ];

    let mut comments = vec![];

    for (satd_type, pattern) in patterns {
        let regex = Regex::new(pattern).unwrap();
        for (line_num, line) in source_code.lines().enumerate() {
            if let Some(captures) = regex.captures(line) {
                comments.push(SATDComment {
                    satd_type,
                    line_number: line_num + 1,
                    message: captures[1].to_string(),
                    file: current_file.clone(),
                });
            }
        }
    }

    comments
}

/// Phase 2: NLP-based classification and categorization
/// Improves precision over regex-only approach
fn classify_satd_with_nlp(candidates: Vec<SATDComment>) -> Vec<SATDComment> {
    candidates.into_iter()
        .map(|mut comment| {
            // Extract linguistic features
            let features = extract_linguistic_features(&comment.message);

            // Classify SATD type based on NLP features
            comment.satd_type = classify_satd_type(&features, comment.satd_type);

            // Classify SATD category (Design, Defect, Documentation, Test, etc.)
            comment.category = classify_satd_category(&features);

            // Calculate severity based on linguistic cues
            comment.severity = calculate_satd_severity(&features);

            comment
        })
        .collect()
}

#[derive(Debug)]
struct LinguisticFeatures {
    has_temporal_words: bool,        // "temporary", "quick", "for now"
    has_uncertainty: bool,            // "maybe", "probably", "not sure"
    has_urgency: bool,                // "urgent", "critical", "ASAP"
    has_complexity_words: bool,       // "complex", "complicated", "messy"
    has_negative_sentiment: bool,     // "bad", "ugly", "wrong", "broken"
    has_design_words: bool,           // "refactor", "redesign", "architecture"
    has_performance_words: bool,      // "slow", "optimize", "performance"
    word_count: usize,
    has_numbers: bool,                // Specific issue numbers, dates
}

fn extract_linguistic_features(text: &str) -> LinguisticFeatures {
    let lower = text.to_lowercase();

    LinguisticFeatures {
        has_temporal_words: lower.contains("temporary") || lower.contains("for now") ||
                           lower.contains("quick") || lower.contains("later"),
        has_uncertainty: lower.contains("maybe") || lower.contains("probably") ||
                        lower.contains("not sure") || lower.contains("might"),
        has_urgency: lower.contains("urgent") || lower.contains("critical") ||
                    lower.contains("asap") || lower.contains("immediately"),
        has_complexity_words: lower.contains("complex") || lower.contains("messy") ||
                             lower.contains("complicated"),
        has_negative_sentiment: lower.contains("bad") || lower.contains("ugly") ||
                               lower.contains("wrong") || lower.contains("broken"),
        has_design_words: lower.contains("refactor") || lower.contains("redesign") ||
                         lower.contains("architecture") || lower.contains("pattern"),
        has_performance_words: lower.contains("slow") || lower.contains("optimize") ||
                              lower.contains("performance") || lower.contains("inefficient"),
        word_count: text.split_whitespace().count(),
        has_numbers: text.chars().any(|c| c.is_ascii_digit()),
    }
}

/// Refined SATD type classification based on linguistic features
fn classify_satd_type(features: &LinguisticFeatures, original: SATDType) -> SATDType {
    // Upgrade TODO to FIXME if urgency or negative sentiment
    if original == SATDType::TODO && (features.has_urgency || features.has_negative_sentiment) {
        return SATDType::FIXME;
    }

    // Classify based on content (may override regex-based label)
    if features.has_design_words {
        return SATDType::REFACTOR;
    }

    if features.has_performance_words {
        return SATDType::OPTIMIZE;
    }

    original  // Keep original if no strong signals
}

/// SATD category classification (research-based taxonomy)
/// Categories from Maldonado et al. (2017)
#[derive(Debug, PartialEq)]
enum SATDCategory {
    Design,          // Architecture, modularity issues
    Defect,          // Known bugs, error handling
    Documentation,   // Missing/incomplete docs
    Test,            // Missing/incomplete tests
    Performance,     // Optimization needed
    Requirement,     // Missing/incomplete features
    Unknown,
}

fn classify_satd_category(features: &LinguisticFeatures) -> SATDCategory {
    if features.has_design_words {
        return SATDCategory::Design;
    }

    if features.has_performance_words {
        return SATDCategory::Performance;
    }

    if features.has_negative_sentiment {
        return SATDCategory::Defect;
    }

    SATDCategory::Unknown
}

/// Calculate SATD severity based on linguistic cues
fn calculate_satd_severity(features: &LinguisticFeatures) -> SATDSeverity {
    let mut score = 0;

    if features.has_urgency { score += 3; }
    if features.has_negative_sentiment { score += 2; }
    if features.has_complexity_words { score += 1; }
    if features.has_temporal_words { score += 1; }  // Temporary hacks = risky

    match score {
        0..=1 => SATDSeverity::LOW,
        2..=3 => SATDSeverity::MEDIUM,
        4..=5 => SATDSeverity::HIGH,
        _ => SATDSeverity::CRITICAL,
    }
}

#[derive(Debug)]
enum SATDSeverity {
    LOW,      // Minor issue, low priority
    MEDIUM,   // Should address soon
    HIGH,     // Address in current sprint
    CRITICAL, // Urgent, blocking issue
}
```

**Key NLP Improvements**:
1. **Context-aware classification**: Uses linguistic features beyond keywords
2. **Category taxonomy**: Classifies into research-based categories (Design, Defect, etc.)
3. **Severity scoring**: Quantifies urgency based on linguistic cues
4. **Type refinement**: Upgrades TODO to FIXME based on urgency/sentiment
5. **Feature extraction**: Detects temporal words, uncertainty, complexity signals

**Comparison**:
```rust
// Input comment: "// TODO: temporary hack until we refactor - this is broken"

// Regex-only:
SATDComment { type: TODO, severity: None, category: None }

// NLP-enhanced:
SATDComment {
    type: FIXME,                    // Upgraded due to "broken"
    severity: HIGH,                 // "temporary", "broken" = risky
    category: Design,               // "refactor" detected
    features: {
        has_temporal_words: true,   // "temporary"
        has_negative_sentiment: true, // "broken"
        has_design_words: true,     // "refactor"
    }
}
```

**Code Churn** (git-based):
```bash
#!/bin/bash
# Calculate churn for a file

FILE="$1"
COMMIT_LIMIT="${2:-100}"

# Get commit count
COMMITS=$(git log --oneline --follow --since="$COMMIT_LIMIT commits ago" -- "$FILE" | wc -l)

# Get lines added/deleted
STATS=$(git log --numstat --follow --since="$COMMIT_LIMIT commits ago" -- "$FILE" | \
        awk '{added+=$1; deleted+=$2} END {print added, deleted}')

ADDED=$(echo "$STATS" | awk '{print $1}')
DELETED=$(echo "$STATS" | awk '{print $2}')

# Get current file size
LINES=$(wc -l < "$FILE")

# Calculate churn rate
CHURN_RATE=$(echo "scale=2; ($ADDED + $DELETED) / $LINES" | bc)

echo "File: $FILE"
echo "Commits: $COMMITS"
echo "Lines added: $ADDED"
echo "Lines deleted: $DELETED"
echo "Current size: $LINES"
echo "Churn rate: $CHURN_RATE"
```

### 8.2 Metric Thresholds

**Complexity Thresholds**:
```yaml
cyclomatic_complexity:
  green: 1-10
  yellow: 11-20
  orange: 21-50
  red: 51+

cognitive_complexity:
  green: 1-15
  yellow: 16-30
  orange: 31-60
  red: 61+

nesting_depth:
  green: 1-3
  yellow: 4-5
  orange: 6-7
  red: 8+

function_length:
  green: 1-50
  yellow: 51-100
  orange: 101-200
  red: 201+
```

**Churn Thresholds**:
```yaml
commits_per_file:
  low: 1-5
  medium: 6-10
  high: 11-15
  critical: 16+

churn_rate:
  stable: 0.0-0.3
  moderate: 0.4-0.5
  high: 0.6-0.8
  volatile: 0.9+

bugs_per_commit:
  low: 0.0-0.1
  medium: 0.2-0.3
  high: 0.4-0.5
  critical: 0.6+
```

**Coverage Thresholds**:
```yaml
line_coverage:
  poor: 0-60%
  fair: 61-75%
  good: 76-85%
  excellent: 86-100%

mutation_score:
  poor: 0-60%
  fair: 61-75%
  good: 76-90%
  excellent: 91-100%

property_test_count:
  minimal: 0-3
  adequate: 4-7
  good: 8-12
  excellent: 13+
```

---

## 9. Report Template & GitHub Integration

### 9.1 Complete Markdown Template

```markdown
# BUG REPORT: [Short Title]

**Bug ID**: BUG-[CATEGORY]-[NUMBER]
**Discovery Date**: YYYY-MM-DD
**Reporter**: Bug Discovery System v1.0.0
**Severity**: [CRITICAL | HIGH | MEDIUM | LOW]
**Status**: Open
**Affects Versions**: [v3.147.0, v3.147.1]
**Working Versions**: [v3.146.0]

---

## 📋 Executive Summary

**TL;DR**: [One sentence description]

**Impact**: [Number] files affected, [Number] tests blocked, [Percentage]% of functionality broken

**Root Cause**: [One sentence root cause from Five-Whys]

**Fix Complexity**: [SIMPLE | MODERATE | COMPLEX]

**Estimated Time to Fix**: [Hours/Days]

---

## 🐛 Bug Details

### Reproduction Steps

1. [Step 1]
2. [Step 2]
3. [Step 3]

### Expected Behavior

\`\`\`
[What should happen]
\`\`\`

### Actual Behavior

\`\`\`
[What actually happens]
\`\`\`

### Minimal Reproduction Code

\`\`\`ruchy
// File: test_bug_[ID].ruchy
// Size: [N] lines (minimized from [M] lines)

[Minimal code that triggers bug]
\`\`\`

### Environment

- **Ruchy Version**: v3.147.0
- **OS**: Linux/macOS/Windows
- **Architecture**: x86_64/ARM64
- **Rust Version**: 1.70.0
- **Compilation Target**: debug/release/wasm

---

## 📊 Quantitative Analysis

<details open>
<summary><strong>Complexity Metrics</strong> (click to collapse)</summary>

| Metric | Value | Threshold | Status |
|--------|-------|-----------|--------|
| Cyclomatic Complexity | 47 | ≤20 | 🔴 CRITICAL |
| Cognitive Complexity | 62 | ≤30 | 🔴 CRITICAL |
| Max Nesting Depth | 8 | ≤5 | 🟠 HIGH |
| Function Length | 450 LOC | ≤200 | 🔴 CRITICAL |
| Big-O Time | O(n²) | O(n) | 🟠 HIGH |
| Big-O Space | O(n) | O(n) | 🟢 OK |

**Analysis**: Function exceeds complexity thresholds - refactoring required

</details>

<details>
<summary><strong>SATD Analysis</strong> (click to expand)</summary>

### SATD Analysis

| Type | Count | Oldest | Average Age |
|------|-------|--------|-------------|
| TODO | 12 | 347 days | 156 days |
| FIXME | 7 | 89 days | 45 days |
| HACK | 4 | 23 days | 12 days |
| **TOTAL** | **23** | **347 days** | **104 days** |

**Critical SATD**:
\`\`\`rust
// Line 234: TODO: Handle edge case for nested expressions (347 days old)
// Line 456: FIXME: This breaks with Vec types (89 days old)
// Line 678: HACK: Workaround for parser bug (23 days old)
\`\`\`

**Correlation**: Files with >15 SATD have 3x higher bug rate

</details>

<details>
<summary><strong>Code Churn Analysis</strong> (click to expand)</summary>

### Code Churn Analysis

| Metric | Value | Threshold | Status |
|--------|-------|-----------|--------|
| Commits (last 100) | 18 | ≤10 | 🔴 HIGH |
| Lines Added | 3,450 | - | - |
| Lines Deleted | 2,100 | - | - |
| Churn Rate | 0.85 | ≤0.5 | 🔴 CRITICAL |
| Bugs Found | 8 | - | - |
| Bugs/Commit | 0.44 | ≤0.2 | 🔴 CRITICAL |

**Analysis**: **HIGHEST RISK FILE** - High churn + high bug rate

**Churn Timeline**:
\`\`\`
v3.145: 2 commits, 0 bugs
v3.146: 4 commits, 1 bug
v3.147: 12 commits, 7 bugs ← CRITICAL SPIKE
\`\`\`

</details>

<details>
<summary><strong>Formalization Hints</strong> (click to expand)</summary>

### Formalization Hints

| Metric | Score | Target | Status |
|--------|-------|--------|--------|
| Provability Score | 0.23 | ≥0.7 | 🔴 LOW |
| Provable Functions | 3 | - | - |
| Unprovable Functions | 15 | - | - |

**Issues**:
- Multiple side effects (mutable state)
- No clear invariants
- Complex control flow (47 branches)

**Proof Hints**:
1. Add precondition: "input is valid token stream"
2. Add invariant: "token position always advances"
3. Add postcondition: "returns valid AST or error"
4. Simplify logic: Extract helper functions
5. Remove side effects: Use immutable data structures

</details>

<details>
<summary><strong>Dependency Analysis</strong> (click to expand)</summary>

### Dependency Analysis

| Metric | Value | Status |
|--------|-------|--------|
| Fan-in (dependencies on this file) | 23 | 🟠 HIGH |
| Fan-out (this file's dependencies) | 7 | 🟢 OK |
| Instability | 0.23 | Stable |
| Abstractness | 0.1 | Concrete |
| Distance from Main Sequence | 0.67 | 🔴 "Zone of Pain" |

**Risk**: Changes affect 23 files - high blast radius

**Dependency Graph**:
\`\`\`
parser.rs
├── lexer.rs (tokenization)
├── ast.rs (AST construction)
├── error.rs (error handling)
├── type_checker.rs (type validation)
├── formatter.rs (code formatting)
├── runtime.rs (runtime support)
└── validator.rs (validation)

Dependents (23 files):
├── main.rs
├── compiler.rs
├── interpreter.rs
├── [... 20 more files]
\`\`\`

</details>

---

## 🔍 Root Cause Analysis: Five-Whys

**Problem**: Vec::new() causes infinite hang in v3.147.0

**Why 1**: Why does Vec::new() hang?
**Answer**: Implementation contains infinite loop in initialization

**Why 2**: Why does the implementation contain an infinite loop?
**Answer**: Recent refactoring (commit abc123) introduced while loop without break

**Why 3**: Why was the refactoring merged without catching the bug?
**Answer**: Test coverage for Vec::new() was only 45% (mutation score)

**Why 4**: Why was test coverage so low?
**Answer**: Vec implementation added without TDD (no RED test written first)

**Why 5**: Why was Vec added without TDD workflow?
**Answer**: No enforcement of TDD workflow in CI/CD pipeline

**ROOT CAUSE**: Missing TDD enforcement + Low mutation test coverage

**Contributing Factors**:
1. High code churn (18 commits in parser.rs)
2. High complexity (47 cyclomatic, 62 cognitive)
3. Many SATD comments (23 total, oldest 347 days)
4. Low provability (0.23 score)

---

## 🔧 Fix Strategy

### Phase 1: RED (Write Failing Test)

**Objective**: Create test that demonstrates the bug

\`\`\`ruchy
// File: validation/tests/test_vec_new_no_hang.ruchy

#[test]
fun test_vec_new_completes_quickly() {
    // ARRANGE
    let max_time_ms = 100

    // ACT
    let start = now()
    let v = Vec::new()
    let elapsed_ms = now() - start

    // ASSERT
    assert!(
        elapsed_ms < max_time_ms,
        f"Vec::new() should complete in <{max_time_ms}ms, took {elapsed_ms}ms"
    )
}

// Run with: ruchy test test_vec_new_no_hang.ruchy
// Expected: ❌ FAIL (timeout or >100ms)
\`\`\`

**Validation**:
\`\`\`bash
$ ruchy test test_vec_new_no_hang.ruchy
Running test: test_vec_new_completes_quickly
[... 5 second timeout ...]
❌ FAIL: Test timed out (exceeded 5000ms)
\`\`\`

### Phase 2: GREEN (Minimal Fix)

**Objective**: Make test pass with minimal changes

\`\`\`rust
// File: runtime/vec.rs
// Lines changed: 3

impl Vec {
    pub fn new() -> Self {
        // OLD (buggy):
        // loop { /* infinite loop */ }

        // NEW (fixed):
        Vec {
            data: std::vec::Vec::new(),  // Use Rust's Vec
            len: 0,
        }
    }
}
\`\`\`

**Validation**:
\`\`\`bash
$ ruchy test test_vec_new_no_hang.ruchy
Running test: test_vec_new_completes_quickly
✅ PASS (completed in 0.3ms)
\`\`\`

### Phase 3: REFACTOR (Improve)

**Objective**: Improve code quality while keeping tests green

\`\`\`rust
// File: runtime/vec.rs
// Improvement: Delegate to with_capacity for DRY

impl Vec {
    pub fn new() -> Self {
        Self::with_capacity(0)  // Reuse existing method
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vec {
            data: std::vec::Vec::with_capacity(capacity),
            len: 0,
        }
    }
}
\`\`\`

**Validation**:
\`\`\`bash
$ ruchy test validation/tests/
Running 12 tests...
✅ All tests passing (12/12)

$ ruchy test --mutation validation/tests/test_vec_new_no_hang.ruchy
Mutation score: 95% (18/19 mutations killed)
✅ PASS (>90% required)
\`\`\`

### Phase 4: COMPREHENSIVE VALIDATION

**Test Suite Results**:
\`\`\`
Unit Tests:       ✅ 12/12 passing (100%)
Mutation Tests:   ✅ 95% score (18/19 mutations killed)
Property Tests:   ✅ 8/8 properties verified
Fuzz Tests:       ✅ 10,000 inputs, 0 crashes, 0 hangs
Regression Tests: ✅ All 8 previous Vec bugs still fixed
\`\`\`

**Complexity Improvement**:
\`\`\`
Before:
  Cyclomatic: 47 → After: 8 (83% reduction)
  Cognitive: 62 → After: 12 (81% reduction)
  Nesting: 8 → After: 2 (75% reduction)
  Length: 450 LOC → After: 89 LOC (80% reduction)
\`\`\`

---

## 🛡️ Prevention Strategy

### Immediate Actions (Today)

1. **Add Regression Test**
   \`\`\`bash
   cp test_vec_new_no_hang.ruchy validation/tests/regression/
   git add validation/tests/regression/test_vec_new_no_hang.ruchy
   git commit -m \"test: Add regression test for Vec::new() hang (Issue #76)\"
   \`\`\`

2. **Update Documentation**
   \`\`\`markdown
   <!-- In docs/api/vec.md -->
   ## Vec::new()

   Creates a new empty vector.

   **Time Complexity**: O(1)
   **Space Complexity**: O(1)
   **Guarantees**: Completes in <1ms

   **Example**:
   \`\`\`ruchy
   let v = Vec::new()
   assert_eq!(v.len(), 0)
   \`\`\`
   \`\`\`

3. **Code Review Checklist**
   \`\`\`markdown
   <!-- In .github/PULL_REQUEST_TEMPLATE.md -->
   ## For runtime/* changes:
   - [ ] Added timeout test (<1s per operation)
   - [ ] Mutation score >90%
   - [ ] No infinite loops (manual review)
   - [ ] Updated docs/api/
   \`\`\`

### Short-Term (Week 1-2)

1. **Increase Test Coverage**
   \`\`\`bash
   # Target: >80% line coverage, >90% mutation score
   make test-vec-coverage
   make test-vec-mutations
   \`\`\`

2. **Add Property Tests**
   \`\`\`ruchy
   #[property_test(iterations=1000)]
   fun vec_new_is_empty(seed: u64) {
       let v = Vec::new()
       assert_eq!(v.len(), 0)
       assert!(v.is_empty())
   }

   #[property_test(iterations=1000)]
   fun vec_operations_dont_hang(ops: Vec<VecOp>) {
       let v = Vec::new()
       for op in ops {
           apply_with_timeout(op, v, timeout=1000ms)
       }
   }
   \`\`\`

3. **Add Fuzz Tests**
   \`\`\`ruchy
   #[fuzz_test(iterations=10000)]
   fun fuzz_vec_operations(inputs: Vec<u8>) {
       let v = Vec::new()
       for byte in inputs {
           match byte % 4 {
               0 => v.push(byte as i32),
               1 => { if !v.is_empty() { v.pop() } },
               2 => { let _ = v.len() },
               3 => { v.clear() },
           }
       }
   }
   \`\`\`

### Medium-Term (Month 1)

1. **Enforce TDD Workflow**
   \`\`\`bash
   # Pre-commit hook: Require RED test first
   cat >> .git/hooks/pre-commit <<'EOF'
   #!/bin/bash
   # Check for RED test in commit
   if git diff --cached --name-only | grep -q \"^src/\"; then
       if ! git diff --cached --name-only | grep -q \"test.*\\.ruchy$\"; then
           echo \"❌ ERROR: Code changes require test changes\"
           echo \"TDD Workflow: Write RED test first\"
           exit 1
       fi
   fi
   EOF
   chmod +x .git/hooks/pre-commit
   \`\`\`

2. **CI/CD Integration**
   \`\`\`yaml
   # .github/workflows/quality.yml
   - name: Mutation Testing
     run: |
       cargo test --package ruchy --test mutation_tests
       if [ $MUTATION_SCORE -lt 80 ]; then
           echo \"❌ Mutation score <80%: $MUTATION_SCORE%\"
           exit 1
       fi
   \`\`\`

3. **Code Churn Monitoring**
   \`\`\`yaml
   # .github/workflows/churn-alert.yml
   - name: Check Code Churn
     run: |
       ./scripts/analyze-churn.sh runtime/
       if [ $BUGS_PER_COMMIT -gt 0.3 ]; then
           echo \"⚠️  WARNING: High bug rate in runtime/\"
           echo \"Bugs/commit: $BUGS_PER_COMMIT (threshold: 0.3)\"
           # Create issue
           gh issue create --title \"High churn in runtime/\" --body \"...\"
       fi
   \`\`\`

### Long-Term (Month 2+)

1. **Refactor High-Risk Files**
   \`\`\`
   Priority 1: parser.rs (18 commits, 0.44 bugs/commit) 🔴
   Priority 2: runtime.rs (14 commits, 0.36 bugs/commit) 🔴
   Priority 3: formatter.rs (15 commits, 0.27 bugs/commit) 🟠

   Strategy:
   - Extract helper functions (reduce complexity)
   - Add invariants (improve provability)
   - Increase test coverage (>90% mutation score)
   \`\`\`

2. **Implement Formal Verification**
   \`\`\`rust
   // Use ruchy prove for critical paths
   #[proved]
   fun vec_new() -> Vec {
       ensures!(result.len() == 0)
       ensures!(result.capacity() >= 0)

       Vec {
           data: std::vec::Vec::new(),
           len: 0,
       }
   }
   \`\`\`

### Similar Bug Prevention

**Pattern**: Runtime hangs on operations

**Detection Strategy**:
\`\`\`yaml
automated_checks:
  - name: Code Churn
    trigger: commit
    action: Flag files with >10 commits in last 100

  - name: Complexity
    trigger: PR
    action: Block if function CC >20

  - name: Mutation
    trigger: PR
    action: Require >90% score for runtime/*

  - name: Timeout
    trigger: test
    action: Fail tests exceeding 1s timeout
\`\`\`

**Prevention Checklist**:
- [ ] All runtime operations have <1s timeout tests
- [ ] Mutation score >90% for runtime/*
- [ ] No functions with CC >20
- [ ] All loops have provable termination
- [ ] Code review required for runtime/* changes

---

## 🧪 Testing Recommendations

### Unit Tests (Minimum Required)

\`\`\`ruchy
#[test]
fun test_vec_new() {
    let v = Vec::new()
    assert_eq!(v.len(), 0)
}

#[test]
fun test_vec_new_no_hang() {
    let start = now()
    let v = Vec::new()
    assert!(now() - start < 100)  // <100ms
}

#[test]
fun test_vec_push() {
    let v = Vec::new()
    v.push(42)
    assert_eq!(v.len(), 1)
}

#[test]
fun test_vec_operations_sequence() {
    let v = Vec::new()
    v.push(1)
    v.push(2)
    assert_eq!(v.pop(), 2)
    assert_eq!(v.len(), 1)
}
\`\`\`

### Property Tests (8+ Properties)

\`\`\`ruchy
#[property_test]
fun vec_len_matches_pushes(values: Vec<i32>) {
    let v = Vec::new()
    for val in values {
        v.push(val)
    }
    assert_eq!(v.len(), values.len())
}

#[property_test]
fun vec_push_pop_roundtrip(value: i32) {
    let v = Vec::new()
    v.push(value)
    assert_eq!(v.pop(), value)
}

#[property_test]
fun vec_operations_dont_crash(ops: Vec<VecOp>) {
    let v = Vec::new()
    for op in ops {
        apply_op(op, v)  // Should never crash
    }
}
\`\`\`

### Fuzz Tests (10K+ Iterations)

\`\`\`ruchy
#[fuzz_test(iterations=10000, timeout=1000ms)]
fun fuzz_vec_random_operations(seed: u64) {
    let rng = Random::new(seed)
    let v = Vec::new()

    for _ in 0..100 {
        match rng.gen_range(0, 5) {
            0 => v.push(rng.gen()),
            1 => { if !v.is_empty() { v.pop() } },
            2 => v.clear(),
            3 => { let _ = v.len() },
            4 => { let _ = v.capacity() },
        }
    }
}
\`\`\`

### Mutation Tests (>90% Score)

\`\`\`rust
// Run with: ruchy test --mutation
//
// Expected mutations:
// 1. new() → panic!() - Should be killed by test_vec_new
// 2. len == 0 → len == 1 - Should be killed by test_vec_new
// 3. push() → {} - Should be killed by test_vec_push
// 4. pop() → None - Should be killed by test_vec_push_pop
// ... 14 more mutations
//
// Target: 18/19 mutations killed (95% score)
\`\`\`

### Regression Tests (All Previous Bugs)

\`\`\`ruchy
// Test that all previous Vec bugs remain fixed
#[test]
fun test_regression_vec_issue_76() {
    // Issue #76: Vec::new() hang
    let v = Vec::new()
    assert!(true)  // Should reach here
}

#[test]
fun test_regression_vec_issue_74() {
    // Issue #74: vec! macro broken
    let v = vec![1, 2, 3]
    assert_eq!(v.len(), 3)
}

#[test]
fun test_regression_vec_issue_62() {
    // Issue #62: vec! not implemented
    let v = vec![]
    assert_eq!(v.len(), 0)
}
\`\`\`

---

## 🔗 Related Issues

### Same Root Cause
- #74: vec! macro completely broken - Same infinite loop pattern
- #62: vec! not implemented in interpreter - Related to Vec implementation

### Similar Symptoms
- #75: Command.output() runtime hang - Different code path, same hang symptom
- #54: Boolean negation `!` causes hang - Pattern: runtime hangs
- #40: Pattern in loops causes hang - Pattern: loop-related hangs

### Dependent Issues
- #77: Comprehensive bug report - Documented this bug + others
- #73: Command parameter parsing - Related to Command implementation

### Prevention Tools
- QUALITY-005: Code Churn Analysis - Would detect 18 commits → high risk
- QUALITY-003: ML Defect Prediction - Would predict 95% bug probability
- QUALITY-006: Mutation Testing - Would show 45% coverage gap

---

## 📈 Success Criteria

### Fix Validation
- [ ] Minimal reproduction case passes
- [ ] All unit tests pass (12/12)
- [ ] Mutation score >90% (18/19)
- [ ] Property tests pass (8/8)
- [ ] Fuzz tests pass (10,000/10,000)
- [ ] Regression tests pass (all previous bugs)
- [ ] Complexity reduced (CC <20, Cog <30)
- [ ] No new SATD comments added

### Prevention Validation
- [ ] Pre-commit hook enforces TDD
- [ ] CI/CD blocks <80% mutation score
- [ ] Code churn monitoring active
- [ ] Documentation updated
- [ ] Code review checklist updated

### Long-Term Tracking
- [ ] Zero regressions in next 3 releases
- [ ] Churn rate <0.3 for runtime/*
- [ ] Bugs/commit <0.2 for runtime/*
- [ ] No new runtime hang bugs

---

## 📝 Additional Notes

### Discovery Method
- **Technique**: Differential testing (v3.146 vs v3.147)
- **Test Case**: ubuntu-config-scripts RUCHY-001 (Logger module)
- **Minimization**: Reduced from 450 LOC to 3 LOC reproduction

### Impact Assessment
- **Severity**: 🔴 CRITICAL (blocks all work)
- **Scope**: 3 files blocked (RUCHY-001, 002, 003)
- **Tests**: 30 tests blocked (50% of test suite)
- **Time Lost**: 3+ days debugging + waiting for fix
- **Users Affected**: All users of Vec::new() in v3.147.x

### Fix Confidence
- **Complexity**: SIMPLE (3-line fix)
- **Risk**: LOW (delegation to Rust std::vec::Vec)
- **Test Coverage**: HIGH (95% mutation score)
- **Regression Risk**: LOW (comprehensive regression suite)

---

## 🏷️ Labels

- `bug` - Confirmed bug
- `critical` - Blocks all work
- `runtime` - Runtime behavior
- `regression` - v3.147.0 regression
- `needs-tdd` - Requires TDD workflow
- `high-churn` - File has high churn rate
- `complexity` - High complexity metrics

---

## 👥 Assignees

- [ ] @ruchy-maintainer - Review and prioritize
- [ ] @runtime-team - Implement fix
- [ ] @qa-team - Validate fix

---

## 🔔 Notifications

- [ ] Notify affected users (ubuntu-config-scripts, etc.)
- [ ] Update release notes for v3.147.2 (if fixed)
- [ ] Update documentation with lessons learned

---

**Generated by**: RuchyRuchy Bug Discovery System v1.0.0
**Report Date**: 2025-10-29
**Analysis Time**: 2.3 seconds
**Confidence**: 95%
```

---

### 9.2 GitHub API Integration

**Filing Workflow**:
```rust
use octocrab::Octocrab;

struct GitHubReporter {
    client: Octocrab,
    repo_owner: String,
    repo_name: String,
}

impl GitHubReporter {
    async fn file_bug_report(&self, report: BugReport) -> Result<IssueUrl, Error> {
        let markdown = self.generate_markdown(&report);

        let issue = self.client
            .issues(&self.repo_owner, &self.repo_name)
            .create(report.title)
            .body(markdown)
            .labels(report.labels)
            .send()
            .await?;

        Ok(issue.html_url)
    }

    fn generate_markdown(&self, report: &BugReport) -> String {
        // Use template from Section 9.1
        render_template(MARKDOWN_TEMPLATE, report)
    }
}
```

**Auto-Linking Related Issues**:
```rust
fn find_related_issues(&self, bug: &Bug) -> Vec<IssueLink> {
    let mut related = vec![];

    // Search by file path
    let file_issues = self.search_issues(
        format!("is:issue {} in:body", bug.file_path)
    );

    // Search by error message
    let error_issues = self.search_issues(
        format!("is:issue \"{}\" in:body", bug.error_message)
    );

    // Search by symptoms (hang, crash, etc.)
    let symptom_issues = self.search_issues(
        format!("is:issue label:{}", bug.symptom_label)
    );

    related.extend(file_issues);
    related.extend(error_issues);
    related.extend(symptom_issues);
    related.dedup();

    related
}
```

---

## 10. Implementation Plan

### 10.1 Phased Rollout

**Phase 1: Discovery Module** (Week 1-2)
- Implement differential testing
- Implement grammar-based fuzzing
- Implement property-based testing
- Implement code churn analysis
- Deliverable: Bug discovery CLI tool

**Phase 2: Replicator Module** (Week 3)
- Implement delta debugging (minimization)
- Implement standalone test generation
- Implement TDD workflow scaffolding
- Deliverable: Bug replicator CLI tool

**Phase 3: Reporter Module** (Week 4-5)
- Implement quantitative analysis framework
- Implement Five-Whys automation
- Implement markdown report generation
- Deliverable: Bug reporter CLI tool

**Phase 4: GitHub Integration** (Week 6)
- Implement GitHub API integration
- Implement auto-filing workflow
- Implement issue linking
- Deliverable: End-to-end automated system

**Phase 5: Validation & Documentation** (Week 7)
- Test against historical bugs
- Measure detection rates
- Create comprehensive documentation
- Deliverable: Production-ready system

### 10.2 Success Metrics

**Discovery**:
- Detect 100% of known regression bugs (differential testing)
- Generate 10,000+ fuzz test cases per run
- Find 8+ property violations per compiler version
- Identify 100% of high-churn files (>0.3 bugs/commit)

**Replication**:
- Minimize bugs to <10 LOC (vs. original 100+ LOC)
- Generate standalone test in <5 seconds
- 100% of minimized cases still trigger bug

**Reporting**:
- Generate comprehensive report in <10 seconds
- Include all 10 quantitative metrics
- Provide actionable fix strategy (TDD workflow)
- 95% confidence in root cause analysis

**Integration**:
- Auto-file to GitHub in <30 seconds
- Link to 3+ related issues per bug
- Track bug lifecycle to resolution

---

## 11. Testing & Validation

### 11.1 System Testing

**Test Against Historical Bugs**:
```bash
# Test discovery on known bugs
./bug-discovery --test-mode \
    --bugs ruchy-issues-62-to-79.json \
    --expected-detection-rate 0.95

# Expected output:
# Detected: 17/18 bugs (94.4%)
# Missed: Issue #69 (forward reference)
# False positives: 2 (acceptable <5%)
```

**Validation Checklist**:
- [ ] Detects 100% of runtime hang bugs (#76, #75, #74, #54, #40)
- [ ] Detects 90%+ of parser regression bugs
- [ ] Detects 100% of formatter bugs (#72, #64, #60, #31)
- [ ] Minimizes all bugs to <10 LOC
- [ ] Generates actionable TDD workflows
- [ ] Files valid GitHub issues (no API errors)
- [ ] Links to correct related issues (>80% accuracy)

### 11.2 Quality Gates

**Pre-Release Checklist**:
- [ ] All unit tests passing (100%)
- [ ] Mutation score >90%
- [ ] Property tests passing (20+ properties)
- [ ] Fuzz tests passing (50,000+ cases)
- [ ] Historical bug detection >95%
- [ ] Documentation complete
- [ ] Integration tests passing

---

## 12. Academic Foundations and Research Citations

This specification is grounded in peer-reviewed computer science research. The following citations support the methodologies and claims throughout this document.

### 12.1 Core Methodologies

**Delta Debugging (Minimization)**:
1. **Zeller, A., & Hildebrandt, R. (2002)**. "Simplifying and isolating failure-inducing input." *IEEE Transactions on Software Engineering*, 28(2), 183-200.
   - **Relevance**: Foundational algorithm for Bug Replicator Module (Section 6.1)
   - **Key Contribution**: O(n log n) minimization algorithm

2. **Misherghi, G., & Su, Z. (2006)**. "HDD: Hierarchical Delta Debugging." *Proceedings of the 28th International Conference on Software Engineering*.
   - **Relevance**: Tree-based AST minimization (Section 6.1)

**Differential Testing**:
3. **McKeeman, W. M. (1998)**. "Differential testing for software." *Digital Technical Journal*, 10(1), 100-107.
   - **Relevance**: Bug Discovery Module (Section 5.1)

4. **Kalibera, T., & Jones, R. (2013)**. "Quantifying Performance Changes with Effect Size Confidence Intervals." *Proceedings of the ACM SIGPLAN Workshop on Virtual Machines and Intermediate Languages*.
   - **Relevance**: Statistical analysis for performance regressions (Section 5.1)

### 12.2 Code Quality and Defect Prediction

5. **Nagappan, N., & Bell, T. (2005)**. "Use of relative code churn measures to predict system defect density." *ICSE*, 284-292.
   - **Relevance**: Code Churn Analysis (Section 7.2)
   - **Key Finding**: Strong correlation (r=0.72) between churn and defects

6. **Kamei, Y., et al. (2013)**. "A large-scale empirical study of just-in-time quality assurance." *TSE*, 39(6), 757-773.
   - **Relevance**: Just-in-Time defect prediction

7. **Chidamber, S. R., & Kemerer, C. F. (1994)**. "A metrics suite for object-oriented design." *TSE*, 20(6), 476-493.
   - **Relevance**: Complexity metrics (Section 7.2)

8. **Campbell, G. (2018)**. "Cognitive Complexity, a new way of measuring understandability." *SonarSource White Paper*.
   - **Relevance**: Cognitive complexity metric (Section 7.2)

### 12.3 Technical Debt and SATD

9. **Potdar, A., & Shihab, E. (2014)**. "An exploratory study on self-admitted technical debt." *ICSME*.
   - **Relevance**: SATD Detection (Section 8)

10. **Maldonado, E., et al. (2017)**. "Detecting and quantifying different types of self-admitted technical debt." *MTD Workshop*.
    - **Relevance**: SATD Category Taxonomy (Section 8)

### 12.4 Formal Verification

11. **Leino, K. R. M. (2010)**. "Dafny: An automatic program verifier for functional correctness." *LPAR*.
    - **Relevance**: Formalization Hints (Section 8)

12. **Filliâtre, J.-C., & Paskevich, A. (2013)**. "Why3 — Where Programs Meet Provers." *ESOP*.
    - **Relevance**: Tool recommendations for verification

### 12.5 Fuzzing

13. **Miller, B. P., Fredriksen, L., & So, B. (1990)**. "An empirical study of the reliability of UNIX utilities." *CACM*, 33(12), 32-44.
    - **Relevance**: Foundational fuzzing study (Section 5.1)

### 12.6 Complexity Analysis

14. **Wegbreit, B. (1975)**. "Mechanical program analysis." *CACM*, 18(9), 528-539.
    - **Relevance**: Algorithmic complexity undecidability
    - **Application**: Justification for "heuristic" vs. "Big-O" labeling

### 12.7 Root Cause Analysis

15. **Ohno, T. (1988)**. "Toyota Production System: Beyond Large-Scale Production." *Productivity Press*.
    - **Relevance**: Five-Whys methodology (Section 7.3)

16. **Card, A. J. (2017)**. "The problem with '5 whys'." *BMJ Quality & Safety*, 26(8), 671-677.
    - **Relevance**: Limitations of automated Five-Whys
    - **Application**: Jidoka approach - system provides data, human validates

### 12.8 Summary

**Research-Driven Design Decisions**:
- Confidence Scoring: Addresses alert fatigue
- Statistical Testing: Rigorous performance regression detection
- Tree-Based Minimization: Preserves syntax correctness
- NLP for SATD: Improves precision over regex
- Formalization Hints: Honest framing of limitations
- Assisted Five-Whys: Acknowledges human judgment requirement

---

## 13. Appendices

### Appendix A: Bug Categories Reference

See Section 3.1 for complete categorization of 79 Ruchy bugs

### Appendix B: Quantitative Metrics Reference

See Section 8 for all metric calculation methods and thresholds

### Appendix C: Related Tools

**Existing Tools (Integration Points)**:
- QUALITY-003: ML Defect Prediction (feeds into discovery)
- QUALITY-005: Code Churn Analysis (quantitative metric)
- QUALITY-006: Mutation Testing (test quality validation)
- PMAT TDG: Overall quality scoring (context for reports)

**New Tools (This Spec)**:
- Bug Discovery CLI
- Bug Replicator CLI
- Bug Reporter CLI
- GitHub Auto-Filer

---

**Specification Version**: 1.0.0
**Status**: Ready for Implementation
**Next Step**: Create OPTION-7 ticket series in roadmap.yaml

---

**End of Specification**
