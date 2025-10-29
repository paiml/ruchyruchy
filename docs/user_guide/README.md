# Bug Discovery, Reporter, and Replicator System - User Guide

**Version**: 1.0.0
**Date**: 2025-10-29
**Project**: RuchyRuchy Bug Discovery & Analysis System

---

## Table of Contents

### Part 1: Getting Started
1. [Introduction](#1-introduction)
2. [Installation & Setup](#2-installation--setup)
3. [Quick Start Guide](#3-quick-start-guide)
4. [System Architecture](#4-system-architecture)

### Part 2: Bug Discovery
5. [Discovery Methods](#5-discovery-methods)
6. [Differential Testing](#6-differential-testing)
7. [Property-Based Testing](#7-property-based-testing)
8. [Fuzz Testing](#8-fuzz-testing)
9. [Mutation Testing](#9-mutation-testing)

### Part 3: Bug Reporting
10. [Quantitative Analysis](#10-quantitative-analysis)
11. [Five-Whys Analysis](#11-five-whys-analysis)
12. [TDD Integration](#12-tdd-integration)
13. [Report Generation](#13-report-generation)
14. [Confidence Scoring](#14-confidence-scoring)

### Part 4: GitHub Integration
15. [GitHub API Setup](#15-github-api-setup)
16. [Automatic Issue Filing](#16-automatic-issue-filing)
17. [Issue Deduplication](#17-issue-deduplication)
18. [Related Issue Linking](#18-related-issue-linking)

### Part 5: Validation & Quality
19. [Historical Bug Validation](#19-historical-bug-validation)
20. [Quality Metrics](#20-quality-metrics)
21. [Performance Tuning](#21-performance-tuning)

### Part 6: Advanced Topics
22. [Custom Discovery Methods](#22-custom-discovery-methods)
23. [Extending the System](#23-extending-the-system)
24. [Integration with CI/CD](#24-integration-with-cicd)

### Appendices
A. [API Reference](../api/README.md)
B. [Example Workflows](../examples/README.md)
C. [Troubleshooting Guide](../troubleshooting/README.md)
D. [FAQ](#appendix-d-faq)
E. [Glossary](#appendix-e-glossary)

---

## Part 1: Getting Started

### 1. Introduction

#### 1.1 What is the Bug Discovery System?

The Bug Discovery, Reporter, and Replicator System is a comprehensive automated framework for:

1. **Discovering** bugs through systematic testing (differential, property, fuzz, mutation)
2. **Replicating** bugs with minimal reproducible examples
3. **Reporting** bugs with extreme quantitative detail
4. **Preventing** future bugs through deep root cause analysis

#### 1.2 Key Features

- **Confidence Scoring** (0.0-1.0): Prevents alert fatigue by ranking findings
- **Quantitative Analysis**: Complexity, churn, SATD, coupling metrics
- **Five-Whys Root Cause**: Data-driven hypothesis generation
- **TDD Integration**: RED-GREEN-REFACTOR workflow tracking
- **GitHub Integration**: Automatic issue filing with deduplication
- **Historical Validation**: 95%+ detection rate on past bugs

#### 1.3 Jidoka Principle: Automation with Human Touch

This system follows Toyota's **Jidoka principle**: automation provides high-quality data and hypotheses, but humans make final causal judgments.

**What the system does**:
- Generates confidence scores (0.0-1.0) for prioritization
- Provides quantitative evidence (complexity, churn, SATD)
- Suggests hypotheses via Five-Whys
- Identifies likely root causes

**What humans do**:
- Review high-confidence findings first (>0.85)
- Validate hypotheses with domain knowledge
- Make final causal judgments
- Decide on fix strategies

#### 1.4 Historical Context

**Analyzed Bugs**:
- **Ruchy compiler**: 79 issues (31 open, 48 closed)
- **ubuntu-config-scripts**: 5/9 conversions broken by bugs (56% failure rate)
- **Critical patterns**: Runtime hangs, parser errors, formatter bugs

**Impact**:
- **62.5%** of production work blocked by bugs
- **20 developer days** lost to debugging
- **100%** detection rate possible with proper tooling

---

### 2. Installation & Setup

#### 2.1 Prerequisites

- **Rust**: 1.70.0 or later
- **Cargo**: Latest stable version
- **Git**: For version control integration
- **GitHub Account**: For automatic issue filing (optional)

#### 2.2 Installation

```bash
# Clone the repository
git clone https://github.com/paiml/ruchyruchy.git
cd ruchyruchy

# Build the system
cargo build --release

# Run tests to verify installation
cargo test

# Expected: 263+ tests passing
```

#### 2.3 GitHub Integration Setup (Optional)

For automatic issue filing, you need a GitHub Personal Access Token (PAT):

```bash
# 1. Create PAT at https://github.com/settings/tokens
# Scopes required: repo (full control)

# 2. Set environment variable
export GITHUB_TOKEN="ghp_your_token_here"

# 3. Configure repository
export GITHUB_OWNER="paiml"
export GITHUB_REPO="ruchy"

# 4. Verify connection (see examples/01-github-setup.md)
```

#### 2.4 Directory Structure

```
ruchyruchy/
├── src/
│   ├── bug_discovery/        # Discovery methods (differential, property, fuzz, mutation)
│   ├── bug_replication/      # Minimization and replication
│   └── bug_reporting/        # Analysis, reporting, GitHub integration
├── docs/
│   ├── user_guide/          # This guide
│   ├── api/                 # API documentation
│   ├── examples/            # Example workflows
│   └── troubleshooting/     # Troubleshooting guide
├── validation/
│   └── historical_bugs/     # 79 historical bugs for validation
└── Cargo.toml
```

---

### 3. Quick Start Guide

#### 3.1 Five-Minute Tour

Let's discover, replicate, and report a bug in 5 minutes.

**Step 1: Run Property Testing**

```rust
use ruchyruchy::bug_discovery::property::*;

// Property: parsing should roundtrip
let generator = ExpressionGenerator::new();
let tester = PropertyTester::new(PropertyConfig {
    num_cases: 1000,
    max_depth: 10,
    timeout_ms: 5000,
});

let result = tester.test_property(
    "parse_roundtrip",
    |input: String| {
        let ast = parse(&input);
        let emitted = ast.emit();
        parse(&emitted) == ast
    },
    generator
);

// Failure found! Confidence: 0.95 (property violation)
```

**Step 2: Replicate with Minimal Example**

```rust
use ruchyruchy::bug_replication::*;

// Original failing input: 500 lines
let original_input = "fun test() { ... }"; // 500 lines

// Minimize to smallest failing input
let minimizer = DeltaDebugger::new(MinimizationConfig {
    target_size: 10, // Prefer <10 lines
    timeout_per_attempt: 1000,
    max_attempts: 1000,
});

let minimal = minimizer.minimize(&original_input, |input| {
    // Returns true if input still fails
    let ast = parse(input);
    let emitted = ast.emit();
    parse(&emitted) != ast
});

// Result: "fun f() { 1 }" // 1 line!
```

**Step 3: Generate Bug Report**

```rust
use ruchyruchy::bug_reporting::*;

// Quantitative analysis
let metrics = ComplexityMetrics::analyze(&minimal);
let churn = ChurnCorrelation::analyze("src/parser.rs");
let satd = SatdDetector::analyze("src/parser.rs");

// Five-Whys root cause analysis
let five_whys = FiveWhysAnalyzer::analyze(&minimal, &metrics);

// Generate report
let report = BugReport::new(
    "Parser fails on minimal function".to_string(),
    "Roundtrip property violated".to_string(),
    Severity::High,
    BugCategory::ParserError,
    minimal,
    "parse(emit(ast)) == ast".to_string(),
    "parse(emit(ast)) != ast".to_string(),
    confidence_score, // 0.95 (property violation)
)
.with_quantitative_analysis(metrics)
.with_five_whys(five_whys);

// Print markdown report
println!("{}", report.to_markdown());
```

**Step 4: File GitHub Issue (Optional)**

```rust
use ruchyruchy::bug_reporting::github_integration::*;

let client = GitHubClient::new(
    "paiml".to_string(),
    "ruchy".to_string(),
    std::env::var("GITHUB_TOKEN").unwrap(),
);

let issue_request = BugReportConverter::to_issue_request(&report);

// Automatic deduplication check
let deduplicator = IssueDeduplicator::new();
let duplicate_result = deduplicator.check_duplicate(&issue_request);

if !duplicate_result.is_duplicate {
    // File new issue
    // GitHub API call would go here
    println!("Issue filed: https://github.com/paiml/ruchy/issues/XXX");
} else {
    println!("Duplicate of #{}", duplicate_result.duplicate_of.unwrap());
}
```

#### 3.2 What Just Happened?

1. **Discovery**: Property testing found a roundtrip bug (confidence: 0.95)
2. **Replication**: Delta debugging minimized 500 lines → 1 line
3. **Reporting**: Generated comprehensive report with:
   - Quantitative metrics (complexity, churn, SATD)
   - Five-Whys root cause analysis
   - TDD fix workflow
   - Confidence score (0.95)
4. **GitHub**: Checked for duplicates, filed issue with auto-labels

---

### 4. System Architecture

#### 4.1 Component Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Bug Discovery                         │
│  - Differential Testing (version, target, oracle)        │
│  - Property Testing (roundtrip, invariants)              │
│  - Fuzz Testing (grammar-based, mutation-based)          │
│  - Mutation Testing (operator, boundary, type mutations) │
│                                                          │
│  Output: Bug + Confidence Score (0.0-1.0)                │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│                 Bug Replication                          │
│  - Delta Debugging (minimize input)                      │
│  - Test Case Reduction (simplify reproduction)           │
│  - Deterministic Replay (consistent behavior)            │
│                                                          │
│  Output: Minimal Reproducible Example (<10 LOC)          │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│                  Bug Reporting                           │
│  - Quantitative Analysis (complexity, churn, SATD)       │
│  - Five-Whys Analysis (root cause hypotheses)            │
│  - TDD Integration (RED-GREEN-REFACTOR workflow)         │
│  - Confidence Scoring (discovery + repro + quant + root) │
│  - Markdown Report Generation                            │
│                                                          │
│  Output: Comprehensive Bug Report (markdown)             │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│                GitHub Integration                        │
│  - Issue Creation (automatic labels, assignees)          │
│  - Deduplication (Jaccard similarity >0.80)              │
│  - Related Issue Linking (similarity >0.50)              │
│  - Comment Updates (progress tracking)                   │
│                                                          │
│  Output: GitHub Issue Filed or Duplicate Detected        │
└─────────────────────────────────────────────────────────┘
```

#### 4.2 Data Flow

```
Input Code → Discovery Method → Bug Found?
                                    ↓
                                  Yes: Confidence Score
                                    ↓
                           Minimize Reproduction
                                    ↓
                           Quantitative Analysis
                                    ↓
                           Five-Whys Root Cause
                                    ↓
                           Generate Report
                                    ↓
                           Check Duplicates
                                    ↓
                           File GitHub Issue (or link to duplicate)
```

#### 4.3 Confidence Score Calculation

```rust
struct ConfidenceScore {
    overall: f64,                    // Weighted average (see below)
    discovery_method_weight: f64,    // 0.35 weight (highest)
    reproducibility_score: f64,      // 0.30 weight
    quantitative_evidence: f64,      // 0.20 weight
    root_cause_clarity: f64,         // 0.15 weight
}

// Overall = 0.35*discovery + 0.30*repro + 0.20*quant + 0.15*root

// Example: Property test violation
// - discovery_method_weight: 0.95 (property violation)
// - reproducibility_score: 0.90 (100% deterministic, <10 LOC)
// - quantitative_evidence: 0.85 (high complexity, high churn)
// - root_cause_clarity: 0.80 (Five-Whys clear hypothesis)
// Overall = 0.35*0.95 + 0.30*0.90 + 0.20*0.85 + 0.15*0.80
//         = 0.3325 + 0.27 + 0.17 + 0.12
//         = 0.8925 (HIGH CONFIDENCE - review immediately)
```

---

## Part 2: Bug Discovery

### 5. Discovery Methods

The system provides four primary discovery methods:

1. **Differential Testing**: Compare behavior across versions/targets/oracles
2. **Property Testing**: Verify mathematical properties (e.g., roundtrip)
3. **Fuzz Testing**: Generate random inputs to find crashes/hangs
4. **Mutation Testing**: Mutate code to test fault detection

#### 5.1 Discovery Method Confidence Weights

| Discovery Method | Confidence | Rationale |
|------------------|-----------|-----------|
| Differential (Version Regression) | 1.0 | Clear before/after comparison |
| Property Test Violation | 0.95 | Mathematical invariant violated |
| Differential (Target Mismatch) | 0.9 | Clear discrepancy |
| Grammar Fuzz (Crash/Hang) | 0.85 | Clear failure mode |
| Mutation (Undetected) | 0.80 | Test gap found |
| Code Churn Hotspot | 0.70 | Statistical correlation |
| SATD Detection | 0.60 | Developer-admitted issue |
| Complexity Spike | 0.50 | Potential issue area |

#### 5.2 Choosing a Discovery Method

**Use Differential Testing when**:
- You have multiple versions (e.g., v1.28.0 vs v1.29.0)
- You have multiple targets (e.g., TypeScript vs Rust codegen)
- You have a reference oracle (e.g., production compiler vs bootstrap)

**Use Property Testing when**:
- You have mathematical properties (e.g., `parse(emit(ast)) == ast`)
- You want to verify invariants (e.g., type soundness)
- You want high confidence (0.95) bugs

**Use Fuzz Testing when**:
- You want to find crashes/hangs
- You have a grammar (for grammar-based fuzzing)
- You want to explore edge cases

**Use Mutation Testing when**:
- You want to test your test suite quality
- You want to find gaps in fault detection
- You want to improve test coverage

---

### 6. Differential Testing

#### 6.1 Version Regression Testing

**Purpose**: Detect bugs introduced in new versions.

**Example**:

```rust
use ruchyruchy::bug_discovery::differential::*;

let config = DifferentialConfig {
    old_version: "1.28.0".to_string(),
    new_version: "1.29.0".to_string(),
    timeout_ms: 5000,
};

let tester = VersionRegressionTester::new(config);

let test_cases = vec![
    "fun main() { println(\"Hello\"); }",
    "fun add(x: i32, y: i32) -> i32 { x + y }",
    // ... 1000+ test cases
];

for (i, test_case) in test_cases.iter().enumerate() {
    let result = tester.test(test_case);

    match result {
        DifferentialResult::Match => {
            // No regression
        },
        DifferentialResult::Mismatch { old_output, new_output } => {
            println!("Regression found in test case {}", i);
            println!("Old output: {}", old_output);
            println!("New output: {}", new_output);

            // Confidence: 1.0 (clear regression)
            // File bug report
        },
        DifferentialResult::Timeout { version } => {
            println!("Timeout in {} on test case {}", version, i);
            // Potential hang bug
        },
    }
}
```

**Confidence**: 1.0 (highest - clear before/after comparison)

#### 6.2 Target Mismatch Testing

**Purpose**: Detect discrepancies between compilation targets (TypeScript vs Rust).

**Example**:

```rust
use ruchyruchy::bug_discovery::differential::*;

let tester = TargetMismatchTester::new(DifferentialConfig {
    targets: vec!["typescript".to_string(), "rust".to_string()],
    timeout_ms: 5000,
});

let result = tester.test("fun fib(n: i32) -> i32 { ... }");

match result {
    DifferentialResult::Match => {
        // TypeScript and Rust outputs match
    },
    DifferentialResult::Mismatch { ts_output, rust_output } => {
        println!("Target mismatch found!");
        println!("TypeScript: {}", ts_output);
        println!("Rust: {}", rust_output);

        // Confidence: 0.9 (may be expected difference)
        // File bug report
    },
}
```

**Confidence**: 0.9 (may be expected differences)

#### 6.3 Oracle Comparison Testing

**Purpose**: Compare against a reference implementation (oracle).

**Example**:

```rust
use ruchyruchy::bug_discovery::differential::*;

let tester = OracleComparisonTester::new(OracleConfig {
    oracle: "production-compiler".to_string(),
    system_under_test: "bootstrap-compiler".to_string(),
    timeout_ms: 5000,
});

let result = tester.test("fun test() { 1 + 1 }");

match result {
    DifferentialResult::Match => {
        // Bootstrap matches production
    },
    DifferentialResult::Mismatch { oracle_output, sut_output } => {
        println!("Oracle mismatch found!");
        println!("Production: {}", oracle_output);
        println!("Bootstrap: {}", sut_output);

        // Confidence: 1.0 (clear discrepancy with oracle)
        // File bug report
    },
}
```

**Confidence**: 1.0 (oracle is trusted reference)

---

### 7. Property-Based Testing

#### 7.1 Roundtrip Properties

**Purpose**: Verify that serialization/deserialization roundtrips correctly.

**Example**:

```rust
use ruchyruchy::bug_discovery::property::*;

let generator = ExpressionGenerator::new();
let tester = PropertyTester::new(PropertyConfig {
    num_cases: 10_000,
    max_depth: 10,
    timeout_ms: 5000,
    shrink_on_failure: true,
});

let result = tester.test_property(
    "parse_emit_roundtrip",
    |expr: Expression| {
        let code = expr.emit();
        let parsed = parse(&code);
        parsed == expr
    },
    generator
);

match result {
    PropertyResult::Success { cases_tested } => {
        println!("✅ Property holds for {} cases", cases_tested);
    },
    PropertyResult::Failure { counterexample, shrunk } => {
        println!("❌ Property violated!");
        println!("Counterexample: {:?}", counterexample);
        println!("Shrunk to: {:?}", shrunk);

        // Confidence: 0.95 (property violation)
        // File bug report with shrunk counterexample
    },
}
```

**Confidence**: 0.95 (mathematical property violated)

#### 7.2 Custom Properties

**Example**: Type soundness property

```rust
let result = tester.test_property(
    "type_soundness",
    |program: Program| {
        // Well-typed programs don't crash
        if let Some(typed_ast) = typecheck(&program) {
            let result = execute(&typed_ast);
            !result.is_crash()
        } else {
            true // Ill-typed programs rejected
        }
    },
    ProgramGenerator::new()
);
```

**Confidence**: 0.95 (type soundness is critical)

---

### 8. Fuzz Testing

#### 8.1 Grammar-Based Fuzzing

**Purpose**: Generate valid inputs according to language grammar.

**Example**:

```rust
use ruchyruchy::bug_discovery::fuzz::*;

let grammar = Grammar::from_file("ruchy.grammar")?;
let fuzzer = GrammarFuzzer::new(FuzzConfig {
    max_depth: 15,
    timeout_ms: 1000,
    num_iterations: 100_000,
});

let stats = fuzzer.fuzz(&grammar, |input| {
    // Test input, return FuzzResult
    match run_compiler(&input) {
        Ok(_) => FuzzResult::Pass,
        Err(e) if e.is_expected_error() => FuzzResult::Pass,
        Err(e) if e.is_crash() => FuzzResult::Crash,
        Err(e) if e.is_timeout() => FuzzResult::Timeout,
    }
});

println!("Fuzz stats:");
println!("  Total: {}", stats.total);
println!("  Crashes: {}", stats.crashes);
println!("  Timeouts: {}", stats.timeouts);
println!("  Coverage: {:.2}%", stats.coverage * 100.0);

for crash in stats.crash_inputs {
    println!("Crash found: {}", crash);
    // Confidence: 0.85 (clear failure mode)
    // Minimize and file bug report
}
```

**Confidence**: 0.85 (crashes/hangs are clear bugs)

#### 8.2 Mutation-Based Fuzzing

**Purpose**: Mutate existing inputs to find variations that fail.

**Example**:

```rust
let fuzzer = MutationFuzzer::new(FuzzConfig {
    mutation_rate: 0.05,
    num_iterations: 50_000,
    timeout_ms: 1000,
});

let seed_inputs = vec![
    "fun main() { println(\"Hello\"); }",
    "fun add(x: i32, y: i32) -> i32 { x + y }",
    // ... more seeds
];

let stats = fuzzer.fuzz(&seed_inputs, |input| {
    match run_compiler(input) {
        Ok(_) => FuzzResult::Pass,
        Err(e) => FuzzResult::Crash,
    }
});

// Mutations applied:
// - Character substitution
// - Insertion/deletion
// - Token swapping
// - Bracket removal
// - Comment injection
```

**Confidence**: 0.85 (mutations find edge cases)

---

### 9. Mutation Testing

#### 9.1 Testing the Tests

**Purpose**: Verify test suite quality by mutating code and checking if tests catch mutations.

**Example**:

```rust
use ruchyruchy::bug_discovery::mutation::*;

let mutator = MutationEngine::new(MutationConfig {
    operators: vec![
        MutationOperator::ArithmeticReplacement,
        MutationOperator::BoundaryShift,
        MutationOperator::LogicalNegation,
        MutationOperator::TypeMismatch,
    ],
    timeout_ms: 5000,
});

let source_code = read_file("src/parser.rs")?;
let test_suite = TestSuite::discover()?;

let result = mutator.test(&source_code, &test_suite);

println!("Mutation Score: {:.2}%", result.mutation_score * 100.0);
println!("Killed: {}", result.killed);
println!("Survived: {}", result.survived);
println!("Timeout: {}", result.timeout);

for mutant in result.survived_mutants {
    println!("Undetected mutation:");
    println!("  Location: {}:{}", mutant.file, mutant.line);
    println!("  Type: {:?}", mutant.operator);
    println!("  Diff: {}", mutant.diff);

    // Confidence: 0.80 (test gap found)
    // Recommend adding test case
}
```

**Mutation Operators**:
- **Arithmetic Replacement**: `+` → `-`, `*` → `/`
- **Boundary Shift**: `<` → `<=`, `>` → `>=`
- **Logical Negation**: `&&` → `||`, `!x` → `x`
- **Type Mismatch**: `i32` → `i64`, `String` → `&str`

**Confidence**: 0.80 (survived mutants indicate test gaps)

---

## Part 3: Bug Reporting

### 10. Quantitative Analysis

#### 10.1 Complexity Metrics

**Purpose**: Measure code complexity to identify bug-prone areas.

**Metrics Calculated**:
- **Cyclomatic Complexity**: Number of linearly independent paths
- **Cognitive Complexity**: Human perception of difficulty
- **Loop Nesting Depth**: Maximum depth of nested loops
- **Function Length**: Lines of code per function

**Example**:

```rust
use ruchyruchy::bug_reporting::metrics::*;

let source = read_file("src/parser.rs")?;
let metrics = ComplexityMetrics::analyze(&source);

println!("Complexity Metrics:");
println!("  Cyclomatic: {}", metrics.cyclomatic);
println!("  Cognitive: {}", metrics.cognitive);
println!("  Loop depth: {}", metrics.loop_nesting_depth);
println!("  Function length: {}", metrics.function_length);

// Thresholds:
// - Cyclomatic >15: High risk
// - Cognitive >20: Very high risk
// - Loop depth >3: Refactor recommended
// - Function length >50: Consider splitting

if metrics.cyclomatic > 15 {
    println!("⚠️  High complexity detected (cyclomatic: {})", metrics.cyclomatic);
    println!("   Recommendation: Refactor to reduce complexity");
}
```

**Quantitative Evidence Score**:
```rust
fn calculate_quantitative_evidence(metrics: &ComplexityMetrics, churn: &ChurnMetrics) -> f64 {
    let complexity_score = match metrics.cyclomatic {
        0..=10 => 0.3,
        11..=15 => 0.6,
        16..=20 => 0.8,
        _ => 1.0,
    };

    let churn_score = match churn.change_count {
        0..=5 => 0.2,
        6..=10 => 0.5,
        11..=20 => 0.8,
        _ => 1.0,
    };

    (complexity_score + churn_score) / 2.0
}
```

#### 10.2 Code Churn Analysis

**Purpose**: Identify files with high change frequency (hot spots).

**Example**:

```rust
use ruchyruchy::bug_reporting::metrics::*;

let churn = ChurnCorrelation::analyze_file("src/parser.rs", 90 /* days */)?;

println!("Churn Analysis (last 90 days):");
println!("  Commits: {}", churn.commit_count);
println!("  Total changes: {}", churn.change_count);
println!("  Lines added: {}", churn.lines_added);
println!("  Lines deleted: {}", churn.lines_deleted);
println!("  Churn rate: {:.2}", churn.churn_rate);

// Churn rate = (lines_added + lines_deleted) / days
// High churn rate (>10) indicates instability

if churn.churn_rate > 10.0 {
    println!("⚠️  High churn detected (rate: {:.2})", churn.churn_rate);
    println!("   This file has high change frequency (instability)");
}
```

**Research**: Kim et al. (2013) showed **80%+ of post-release bugs** occur in files with:
- High code churn (>10 changes/90 days)
- High cyclomatic complexity (>15)
- Recent changes (<30 days)

#### 10.3 SATD (Self-Admitted Technical Debt) Detection

**Purpose**: Find developer-admitted issues (TODO, FIXME, HACK comments).

**Example**:

```rust
use ruchyruchy::bug_reporting::metrics::*;

let source = read_file("src/parser.rs")?;
let satd = SatdDetector::analyze(&source);

println!("SATD Analysis:");
println!("  Total SATD: {}", satd.total_count);

for debt in satd.debts {
    println!("\n  Type: {:?}", debt.satd_type);
    println!("  Line: {}", debt.line_number);
    println!("  Text: {}", debt.text);
    println!("  Priority: {:?}", debt.priority);
}

// SATD Types:
// - TODO: Planned work
// - FIXME: Known bug
// - HACK: Temporary workaround
// - XXX: Problem area
// - NOTE: Important note
```

**Priority Calculation**:
```rust
// High priority: FIXME, HACK in high-churn files
// Medium priority: TODO in recent commits
// Low priority: NOTE, old TODOs
```

---

### 11. Five-Whys Analysis

#### 11.1 Data-Driven Root Cause

**Purpose**: Apply Toyota's Five-Whys technique with data-driven hypotheses.

**Example**:

```rust
use ruchyruchy::bug_reporting::five_whys::*;

let analysis = FiveWhysAnalyzer::analyze(
    "Parser crashes on nested expressions",
    &metrics,
    &churn,
    &satd,
);

println!("Five-Whys Analysis:");
for (i, layer) in analysis.layers.iter().enumerate() {
    println!("\nWhy #{}: {}", i + 1, layer.question);
    println!("Hypothesis: {}", layer.hypothesis);
    println!("Confidence: {}", layer.confidence);
    println!("Data points:");
    for dp in &layer.data_points {
        println!("  - {} (source: {:?})", dp.value, dp.source);
    }
}

println!("\nRoot Cause Conclusion:");
println!("{}", analysis.conclusion);
```

**Example Output**:

```
Why #1: Why does the parser crash on nested expressions?
Hypothesis: Stack overflow due to unbounded recursion
Confidence: High
Data points:
  - Cyclomatic complexity: 25 (source: Static Analysis)
  - Loop nesting depth: 5 (source: Static Analysis)
  - No max depth check in code (source: SATD - "FIXME: Add recursion limit")

Why #2: Why is there unbounded recursion?
Hypothesis: No recursion depth limit enforced
Confidence: High
Data points:
  - No `max_depth` parameter in parse_expression() (source: Code Review)
  - SATD comment: "FIXME: Add recursion limit" (source: SATD Detector)

Why #3: Why was recursion limit not implemented?
Hypothesis: Technical debt accumulated over time
Confidence: Medium
Data points:
  - File changed 15 times in last 90 days (source: Churn Analysis)
  - 3 FIXME comments in function (source: SATD Detector)
  - No unit test for deep nesting (source: Test Coverage)

Why #4: Why was technical debt not addressed?
Hypothesis: Fast iteration prioritized over robustness
Confidence: Low
Data points:
  - High churn rate: 12.5 changes/90 days (source: Churn Analysis)
  - No quality gates enforcing complexity limits (source: CI/CD Config)

Why #5: Why were quality gates not enforced?
Hypothesis: Project in rapid development phase
Confidence: Low
Data points:
  - First stable release not yet achieved (source: Roadmap)
  - Focus on feature completeness over robustness (source: Issue Tracker)

Root Cause Conclusion:
Primary: No recursion depth limit in parse_expression() (HIGH CONFIDENCE)
Secondary: Technical debt from rapid development (MEDIUM CONFIDENCE)
Tertiary: Missing quality gates (LOW CONFIDENCE)

Recommendation:
1. Add max_depth parameter to parse_expression() [IMMEDIATE]
2. Implement recursion depth check with clear error [IMMEDIATE]
3. Add property test for deep nesting [SHORT-TERM]
4. Enforce complexity limits in CI/CD [LONG-TERM]
```

**Jidoka Principle**: System provides data-driven hypotheses, humans validate with domain knowledge.

#### 11.2 Hypothesis Confidence Levels

```rust
pub enum ConfidenceLevel {
    VeryHigh,   // 0.90-1.00: Direct evidence from code/data
    High,       // 0.75-0.89: Strong statistical correlation
    Medium,     // 0.50-0.74: Reasonable hypothesis
    Low,        // 0.25-0.49: Speculative, needs validation
    VeryLow,    // 0.00-0.24: Weak evidence
}
```

**Confidence Calculation**:
```rust
fn calculate_hypothesis_confidence(data_points: &[DataPoint]) -> f64 {
    let mut score = 0.0;

    for dp in data_points {
        score += match dp.source {
            DataSource::StaticAnalysis => 0.3,  // High confidence
            DataSource::ChurnAnalysis => 0.25,
            DataSource::SatdDetector => 0.2,
            DataSource::TestCoverage => 0.15,
            DataSource::IssueTracker => 0.1,    // Lower confidence
        };
    }

    score.min(1.0)
}
```

---

### 12. TDD Integration

#### 12.1 RED-GREEN-REFACTOR Workflow

**Purpose**: Track TDD cycles to document fix workflow.

**Example**:

```rust
use ruchyruchy::bug_reporting::tdd::*;

let mut history = TddHistory::new();

// RED: Write failing test
history.add_cycle(TddCycle {
    phase: TddPhase::Red,
    test_result: TestResult::Fail,
    test_count: 1,
    passing: 0,
    failing: 1,
    coverage: 0.0,
    description: "Add test for recursion depth limit".to_string(),
});

// GREEN: Minimal implementation
history.add_cycle(TddCycle {
    phase: TddPhase::Green,
    test_result: TestResult::Pass,
    test_count: 1,
    passing: 1,
    failing: 0,
    coverage: 0.42,
    description: "Add max_depth parameter and check".to_string(),
});

// REFACTOR: Improve code quality
history.add_cycle(TddCycle {
    phase: TddPhase::Refactor,
    test_result: TestResult::Pass,
    test_count: 3,
    passing: 3,
    failing: 0,
    coverage: 0.68,
    description: "Extract recursion check to helper function".to_string(),
});

// Generate TDD report
println!("{}", history.to_markdown());
```

**Output**:

```markdown
## TDD Fix Workflow

### Cycle 1: RED
- **Test Result**: ❌ FAIL
- **Tests**: 0/1 passing
- **Coverage**: 0.0%
- **Description**: Add test for recursion depth limit

### Cycle 2: GREEN
- **Test Result**: ✅ PASS
- **Tests**: 1/1 passing
- **Coverage**: 42.0%
- **Description**: Add max_depth parameter and check

### Cycle 3: REFACTOR
- **Test Result**: ✅ PASS
- **Tests**: 3/3 passing
- **Coverage**: 68.0%
- **Description**: Extract recursion check to helper function

**Summary**: 3 cycles, final coverage: 68.0%
```

#### 12.2 Quality Gates

**Purpose**: Enforce quality requirements at each TDD phase.

```rust
let gates = QualityGates {
    min_coverage: 0.80,             // 80%+ coverage required
    max_complexity: 15,             // Cyclomatic <15
    max_function_length: 50,        // <50 LOC per function
    require_property_tests: true,   // Property tests mandatory
    require_mutation_testing: true, // Mutation score >0.80
};

let result = gates.check(&tdd_cycle);

match result {
    GateResult::Pass => {
        println!("✅ Quality gates passed");
    },
    GateResult::Fail { violations } => {
        println!("❌ Quality gates failed:");
        for v in violations {
            println!("  - {}", v);
        }
    },
}
```

---

### 13. Report Generation

#### 13.1 Comprehensive Bug Report Structure

**Sections**:
1. **Executive Summary**: Title, severity, category, confidence
2. **Bug Details**: Reproduction code, expected vs actual behavior
3. **Confidence Analysis**: Discovery method, reproducibility, quantitative evidence, root cause clarity
4. **Quantitative Analysis**: Complexity, churn, SATD metrics
5. **Five-Whys Analysis**: Data-driven root cause hypotheses
6. **TDD Fix Workflow**: RED-GREEN-REFACTOR cycles
7. **Related Files**: Dependency graph, call chains
8. **Fix Recommendations**: Immediate, short-term, long-term actions
9. **Prevention Strategy**: How to prevent similar bugs

**Example**:

```rust
use ruchyruchy::bug_reporting::report_generator::*;

let report = BugReport::new(
    "Parser crashes on nested expressions".to_string(),
    "Stack overflow on deep nesting".to_string(),
    Severity::Critical,
    BugCategory::Crash,
    "fun test() { ((((1)))) }".to_string(),
    "Parse successfully".to_string(),
    "Stack overflow panic".to_string(),
    confidence_score,
)
.with_quantitative_analysis(metrics)
.with_five_whys(five_whys)
.with_tdd_history(tdd_history)
.with_related_files(vec!["src/parser.rs".to_string()])
.with_fix_recommendations(vec![
    "Add max_depth parameter to parse_expression()".to_string(),
    "Implement recursion depth check".to_string(),
    "Add property test for deep nesting".to_string(),
])
.with_prevention(vec![
    "Enforce complexity limits in CI/CD".to_string(),
    "Add property tests for all recursive functions".to_string(),
]);

// Generate markdown report
let markdown = report.to_markdown();
write_file("bug_report.md", &markdown)?;
```

#### 13.2 Markdown Report Example

```markdown
# 🔴 CRITICAL: Parser crashes on nested expressions

**Severity**: Critical
**Category**: Crash
**Confidence**: 0.89 (HIGH)
**Discovered**: 2025-10-29
**Status**: Unfixed

---

## Executive Summary

Stack overflow on deep nesting

**Confidence Score**: 0.89 / 1.0 (HIGH)
- Discovery Method: 0.95 (Property test violation)
- Reproducibility: 0.90 (100% deterministic, <10 LOC)
- Quantitative Evidence: 0.85 (High complexity, high churn)
- Root Cause Clarity: 0.80 (Clear hypothesis with data)

**Recommendation**: Review immediately (confidence >0.85)

---

## Bug Details

### Reproduction Code
```ruchy
fun test() { ((((1)))) }
```

### Expected Behavior
Parse successfully

### Actual Behavior
Stack overflow panic

### Steps to Reproduce
1. Create file with deeply nested parentheses (depth >100)
2. Run `ruchy parse test.ruchy`
3. Observe stack overflow panic

---

## Quantitative Analysis

### Complexity Metrics
- **Cyclomatic Complexity**: 25 (HIGH RISK - threshold: 15)
- **Cognitive Complexity**: 32 (VERY HIGH RISK - threshold: 20)
- **Loop Nesting Depth**: 5 (REFACTOR RECOMMENDED - threshold: 3)
- **Function Length**: 85 LOC (CONSIDER SPLITTING - threshold: 50)

**Risk Level**: VERY HIGH (multiple thresholds exceeded)

### Code Churn (Last 90 Days)
- **Commits**: 12
- **Total Changes**: 45
- **Lines Added**: 320
- **Lines Deleted**: 180
- **Churn Rate**: 12.5 (HIGH - threshold: 10.0)

**Interpretation**: File has high change frequency (instability)

### SATD (Self-Admitted Technical Debt)
- **Total**: 3 debts found
- **High Priority**: 1 (FIXME: Add recursion limit)
- **Medium Priority**: 2 (TODO: Improve error messages, TODO: Add tests)

---

## Five-Whys Root Cause Analysis

[See section 11.1 for full example]

**Root Cause Conclusion**:
- **Primary**: No recursion depth limit in parse_expression() (HIGH CONFIDENCE)
- **Secondary**: Technical debt from rapid development (MEDIUM CONFIDENCE)

---

## TDD Fix Workflow

[See section 12.1 for full example]

---

## Related Files
- `src/parser.rs` (primary bug location)
- `src/ast.rs` (dependency)
- `tests/parser_tests.rs` (test coverage)

---

## Fix Recommendations

### Immediate (Day 1)
1. ✅ Add `max_depth` parameter to `parse_expression()`
2. ✅ Implement recursion depth check with clear error message
3. ✅ Add property test for deep nesting (depth >100)

### Short-term (Week 1)
1. Refactor `parse_expression()` to reduce cyclomatic complexity (<15)
2. Extract nested logic to helper functions
3. Add mutation tests for recursion handling

### Long-term (Month 1)
1. Enforce complexity limits in CI/CD quality gates
2. Add property tests for all recursive functions
3. Review and resolve all SATD comments

---

## Prevention Strategy

### Code Review Checklist
- [ ] All recursive functions have depth limits
- [ ] Complexity <15 per function
- [ ] Property tests for recursion/loops
- [ ] No FIXME comments in critical paths

### Quality Gates (CI/CD)
- Cyclomatic complexity <15 (BLOCKING)
- Mutation score >0.80 (BLOCKING)
- Property test coverage >80% (BLOCKING)
- Zero high-priority SATD (BLOCKING)

### Process Improvements
1. Require property tests for all new recursive code
2. Add recursion depth parameter to coding standards
3. Monthly SATD review and cleanup sprint

---

**Report Generated**: 2025-10-29
**Tool**: RuchyRuchy Bug Discovery System v1.0.0
```

---

### 14. Confidence Scoring

#### 14.1 Overall Confidence Formula

```rust
struct ConfidenceScore {
    overall: f64,                    // 0.0-1.0 (weighted average)
    discovery_method_weight: f64,    // Weight: 0.35 (highest)
    reproducibility_score: f64,      // Weight: 0.30
    quantitative_evidence: f64,      // Weight: 0.20
    root_cause_clarity: f64,         // Weight: 0.15
}

impl ConfidenceScore {
    fn calculate_overall(&mut self) {
        self.overall = 0.35 * self.discovery_method_weight
                     + 0.30 * self.reproducibility_score
                     + 0.20 * self.quantitative_evidence
                     + 0.15 * self.root_cause_clarity;
    }
}
```

#### 14.2 Confidence Thresholds

| Range | Level | Action |
|-------|-------|--------|
| 0.85-1.0 | Very High | Review immediately, likely true positive |
| 0.70-0.84 | High | Review within 24 hours, probably valid |
| 0.50-0.69 | Medium | Review within week, needs validation |
| 0.25-0.49 | Low | Backlog, validate when time permits |
| 0.0-0.24 | Very Low | Likely false positive, deprioritize |

#### 14.3 Preventing Alert Fatigue

**Problem**: Traditional static analysis tools generate 1000s of warnings, most are false positives.

**Solution**: Confidence scoring filters noise and prioritizes real issues.

**Example**:

```rust
// Without confidence scoring: 1000 warnings
// - 950 low-confidence (false positives)
// - 50 medium-confidence (need validation)
// - 10 high-confidence (real bugs)
// Developer overwhelmed, ignores all warnings

// With confidence scoring: Filter to high-confidence only
let high_confidence_bugs: Vec<_> = all_findings
    .iter()
    .filter(|f| f.confidence.overall >= 0.85)
    .collect();

// Result: 10 high-confidence bugs (all real)
// Developer reviews 10 bugs instead of 1000
// Alert fatigue prevented
```

**Research**: Christakis & Bird (2016) showed developers ignore tools with >10% false positive rate.

**Target**: <5% false positive rate at confidence >0.85 threshold.

---

## Part 4: GitHub Integration

### 15. GitHub API Setup

#### 15.1 Creating a Personal Access Token (PAT)

1. Go to https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Set scopes:
   - `repo` (full control of private repositories)
   - `public_repo` (for public repos only)
4. Set expiration (recommend: 90 days)
5. Click "Generate token"
6. Copy token (you won't see it again!)

#### 15.2 Configuration

```bash
# Set environment variables
export GITHUB_TOKEN="ghp_your_token_here"
export GITHUB_OWNER="paiml"
export GITHUB_REPO="ruchy"

# Verify connection
cargo run --example github-test
```

#### 15.3 Rate Limits

**Authenticated**: 5000 requests/hour
**Unauthenticated**: 60 requests/hour

**Best Practices**:
- Always use authentication (PAT)
- Cache issue list locally (refresh hourly)
- Batch operations when possible
- Handle rate limit errors gracefully

---

### 16. Automatic Issue Filing

#### 16.1 Basic Issue Creation

```rust
use ruchyruchy::bug_reporting::github_integration::*;

let client = GitHubClient::new(
    std::env::var("GITHUB_OWNER").unwrap(),
    std::env::var("GITHUB_REPO").unwrap(),
    std::env::var("GITHUB_TOKEN").unwrap(),
);

let report = BugReport::new(/* ... */);
let issue_request = BugReportConverter::to_issue_request(&report);

// Issue created with automatic labels:
// - severity: critical
// - type: crash
// - high-confidence
// - bug
```

#### 16.2 Automatic Label Assignment

**Severity Labels**:
- `severity: critical` (Severity::Critical)
- `severity: high` (Severity::High)
- `severity: medium` (Severity::Medium)
- `severity: low` (Severity::Low)

**Category Labels**:
- `type: crash` (BugCategory::Crash)
- `type: hang` (BugCategory::Hang)
- `type: wrong-output` (BugCategory::WrongOutput)
- `type: performance` (BugCategory::PerformanceRegression)
- `type: memory-leak` (BugCategory::MemoryLeak)
- `type: type-error` (BugCategory::TypeError)
- `type: parser-error` (BugCategory::ParserError)

**Confidence Labels**:
- `high-confidence` (confidence ≥0.85)
- `medium-confidence` (0.70 ≤ confidence <0.85)
- `low-confidence` (confidence <0.70)

**Standard Label**:
- `bug` (always added)

---

### 17. Issue Deduplication

#### 17.1 Jaccard Similarity Algorithm

**Purpose**: Detect duplicate issues before filing.

**Formula**:
```
similarity(A, B) = |A ∩ B| / |A ∪ B|
```

**Example**:
```rust
use ruchyruchy::bug_reporting::issue_linking::*;

let deduplicator = IssueDeduplicator::new();

// Add existing issues
deduplicator.add_issue(BugIssue {
    number: 123,
    title: "Parser crashes on nested expressions".to_string(),
    body: "Stack overflow when parsing deeply nested parentheses".to_string(),
    files: vec!["src/parser.rs".to_string()],
    error_message: Some("Stack overflow".to_string()),
    labels: vec!["bug".to_string(), "severity: critical".to_string()],
});

// Check new issue for duplicates
let new_issue = BugIssue {
    number: 0,
    title: "Parser crash with deep nesting".to_string(),
    body: "Stack overflow on nested parens".to_string(),
    files: vec!["src/parser.rs".to_string()],
    error_message: Some("Stack overflow".to_string()),
    labels: vec!["bug".to_string(), "severity: high".to_string()],
};

let result = deduplicator.check_duplicate(&new_issue);

if result.is_duplicate {
    println!("Duplicate of issue #{}", result.duplicate_of.unwrap());
    println!("Similarity: {:.2}%", result.similarity.overall * 100.0);
} else {
    println!("Not a duplicate, file new issue");
}
```

#### 17.2 Multi-Factor Similarity Scoring

**Weighted Factors**:
- **Title**: 30% (most important)
- **Body**: 25%
- **Files**: 20%
- **Error**: 15%
- **Labels**: 10%

**Thresholds**:
- **Duplicate**: similarity ≥0.80 (very similar)
- **Related**: 0.50 ≤ similarity <0.80 (somewhat similar)

**Example**:

```rust
let similarity = SimilarityScore::new(
    0.85, // title_similarity
    0.80, // body_similarity
    0.75, // file_overlap
    0.90, // error_similarity
    0.70, // label_overlap
);

// Overall = 0.30*0.85 + 0.25*0.80 + 0.20*0.75 + 0.15*0.90 + 0.10*0.70
//         = 0.255 + 0.20 + 0.15 + 0.135 + 0.07
//         = 0.81 (DUPLICATE!)

if similarity.is_duplicate() {
    println!("This is a duplicate (similarity: {:.2})", similarity.overall);
} else if similarity.is_related() {
    println!("This is related (similarity: {:.2})", similarity.overall);
} else {
    println!("This is distinct (similarity: {:.2})", similarity.overall);
}
```

---

### 18. Related Issue Linking

#### 18.1 Finding Related Issues

**Purpose**: Link new issues to related existing issues (not duplicates, but similar).

**Example**:

```rust
let related = deduplicator.find_related(&new_issue, 5 /* max results */);

println!("Related issues:");
for r in related {
    println!("  #{}: {} (similarity: {:.2}%)",
        r.issue.number,
        r.issue.title,
        r.similarity.overall * 100.0
    );
}
```

**Output**:

```
Related issues:
  #120: Parser error on complex expressions (similarity: 72.0%)
  #115: Stack overflow in type checker (similarity: 65.0%)
  #98: Recursion limit needed (similarity: 58.0%)
```

#### 18.2 Automatic Linking in Issue Body

When filing a new issue, the system automatically adds a "Related Issues" section:

```markdown
## Related Issues
- #120: Parser error on complex expressions (similarity: 72%)
- #115: Stack overflow in type checker (similarity: 65%)
- #98: Recursion limit needed (similarity: 58%)
```

---

## Part 5: Validation & Quality

### 19. Historical Bug Validation

#### 19.1 Validation Against 79 Historical Bugs

**Purpose**: Measure detection rate on known bugs from Ruchy's issue tracker.

**Example**:

```rust
use ruchyruchy::bug_reporting::validation::*;

let corpus = BugCorpus::load_from_github(
    "paiml",
    "ruchy",
    std::env::var("GITHUB_TOKEN").unwrap(),
)?;

println!("Loaded {} historical bugs", corpus.bugs.len());

let validator = BugCorpusValidator::new(corpus);

let report = validator.validate(|bug| {
    // Run discovery system on each bug
    let result = run_discovery_system(bug);

    DetectionResult {
        detected: result.is_some(),
        method: result.as_ref().map(|r| r.discovery_method.clone()),
        confidence: result.as_ref().map(|r| r.confidence.overall).unwrap_or(0.0),
        miss_reason: if result.is_none() {
            Some("Not detected".to_string())
        } else {
            None
        },
    }
});

println!("\nValidation Report:");
println!("{}", report.to_markdown());
```

#### 19.2 Validation Metrics

**Targets**:
- **Detection Rate**: ≥95% (detect 95% of historical bugs)
- **False Positive Rate**: <5% (less than 5% false alarms)
- **Critical Detection**: 100% (all critical bugs must be detected)

**Example Report**:

```markdown
# Historical Bug Validation Report

**Corpus**: 79 bugs from Ruchy issue tracker
**Detection Rate**: 97.5% (77/79 detected)
**False Positive Rate**: 3.2%
**Critical Detection**: 100% (15/15)

## Detected Bugs by Method
- Differential Testing: 25 (32%)
- Property Testing: 30 (38%)
- Fuzz Testing: 18 (23%)
- Mutation Testing: 4 (5%)

## Missed Bugs (2)
1. **Issue #45**: Subtle type inference edge case
   - **Reason**: Property tests didn't cover this specific type combination
   - **Recommendation**: Extend property test generator

2. **Issue #67**: Performance regression on large inputs
   - **Reason**: Fuzz tests used small inputs (<1000 LOC)
   - **Recommendation**: Add large-input fuzz cases

## False Positives (3 out of 95 findings)
- Finding #12: Expected difference between TypeScript and Rust targets
- Finding #34: Intentional behavior change (not a bug)
- Finding #78: Duplicate of existing issue

**Conclusion**: System meets target (97.5% ≥ 95%, 3.2% < 5%)
```

---

### 20. Quality Metrics

#### 20.1 System Performance

**Throughput Targets**:
- **Differential Testing**: >100 test cases/second
- **Property Testing**: >1000 test cases/second
- **Fuzz Testing**: >10,000 inputs/second
- **Report Generation**: <10 seconds per report
- **GitHub Issue Filing**: <30 seconds per issue

**Memory Usage**:
- **Peak RSS**: <500 MB
- **Average RSS**: <200 MB

**Example Benchmark**:

```rust
use ruchyruchy::benchmarks::*;

let results = run_benchmarks()?;

println!("Performance Metrics:");
println!("  Differential: {:.0} tests/sec", results.differential_throughput);
println!("  Property: {:.0} tests/sec", results.property_throughput);
println!("  Fuzz: {:.0} inputs/sec", results.fuzz_throughput);
println!("  Report gen: {:.2}s", results.report_generation_time);
println!("  GitHub filing: {:.2}s", results.github_filing_time);
println!("  Peak memory: {:.1} MB", results.peak_memory_mb);
```

#### 20.2 Quality Dashboard

**Metrics Tracked**:
- Detection rate (historical bugs)
- False positive rate
- Confidence score distribution
- Time to detect (median, p95, p99)
- Report quality (manual review)

**Example Dashboard**:

```
┌─────────────────────────────────────────────────┐
│         Bug Discovery Quality Dashboard         │
└─────────────────────────────────────────────────┘

Detection Rate:        97.5% ✅ (target: 95%)
False Positive Rate:   3.2%  ✅ (target: <5%)
Critical Detection:    100%  ✅ (target: 100%)

Confidence Distribution:
  Very High (0.85+):   45% (42 bugs)
  High (0.70-0.84):    35% (33 bugs)
  Medium (0.50-0.69):  15% (14 bugs)
  Low (<0.50):         5%  (5 bugs)

Time to Detect (median):
  Differential: 0.8s
  Property:     2.1s
  Fuzz:         12.5s
  Mutation:     45.0s

Report Quality (manual review of 20 reports):
  Excellent: 75% (15/20)
  Good:      20% (4/20)
  Fair:      5%  (1/20)
  Poor:      0%  (0/20)

Overall: EXCELLENT ✅
```

---

### 21. Performance Tuning

#### 21.1 Optimization Strategies

**Parallel Testing**:
```rust
use rayon::prelude::*;

let results: Vec<_> = test_cases
    .par_iter()
    .map(|test| run_test(test))
    .collect();
```

**Caching**:
```rust
use std::collections::HashMap;

let mut cache: HashMap<String, TestResult> = HashMap::new();

for test in tests {
    if let Some(cached) = cache.get(&test.hash()) {
        // Use cached result
    } else {
        let result = run_test(test);
        cache.insert(test.hash(), result);
    }
}
```

**Incremental Analysis**:
```rust
// Only re-analyze changed files
let changed_files = git_diff()?;
let analysis = ComplexityMetrics::analyze_incremental(&changed_files);
```

#### 21.2 Memory Optimization

**Streaming Large Inputs**:
```rust
use std::io::BufReader;

let file = File::open("large_test_suite.txt")?;
let reader = BufReader::new(file);

for line in reader.lines() {
    let test = line?;
    run_test(&test);
    // Process one at a time, don't load all into memory
}
```

**Lazy Evaluation**:
```rust
let findings = discover_bugs_lazy(&test_suite); // Iterator, not Vec

for finding in findings.take(10) { // Stop after 10 high-confidence bugs
    if finding.confidence.overall >= 0.85 {
        file_github_issue(&finding);
    }
}
```

---

## Part 6: Advanced Topics

### 22. Custom Discovery Methods

#### 22.1 Implementing a Custom Discoverer

**Example**: Oracle-based discovery using production compiler as oracle

```rust
use ruchyruchy::bug_discovery::*;

pub struct OracleDiscoverer {
    oracle_path: String,
    timeout_ms: u64,
}

impl BugDiscoverer for OracleDiscoverer {
    fn discover(&self, input: &str) -> Option<BugFinding> {
        // Run oracle (production compiler)
        let oracle_result = run_command(&self.oracle_path, input, self.timeout_ms)?;

        // Run system under test
        let sut_result = run_system_under_test(input)?;

        // Compare results
        if oracle_result != sut_result {
            Some(BugFinding {
                discovery_method: "Oracle Comparison".to_string(),
                confidence: ConfidenceScore {
                    overall: 1.0,
                    discovery_method_weight: 1.0, // Oracle is trusted
                    reproducibility_score: 1.0,   // Deterministic
                    quantitative_evidence: 0.0,   // Not yet analyzed
                    root_cause_clarity: 0.0,      // Not yet analyzed
                },
                input: input.to_string(),
                expected: oracle_result,
                actual: sut_result,
            })
        } else {
            None
        }
    }
}
```

#### 22.2 Registering Custom Discoverers

```rust
let mut discovery_engine = DiscoveryEngine::new();

discovery_engine.register(Box::new(DifferentialTester::new(/* ... */)));
discovery_engine.register(Box::new(PropertyTester::new(/* ... */)));
discovery_engine.register(Box::new(OracleDiscoverer::new(/* ... */)));

let findings = discovery_engine.discover_all(&test_suite);
```

---

### 23. Extending the System

#### 23.1 Adding New Metrics

**Example**: Add Halstead complexity metric

```rust
pub struct HalsteadMetrics {
    pub n1: usize, // Unique operators
    pub n2: usize, // Unique operands
    pub N1: usize, // Total operators
    pub N2: usize, // Total operands
    pub vocabulary: usize,
    pub length: usize,
    pub volume: f64,
    pub difficulty: f64,
    pub effort: f64,
}

impl HalsteadMetrics {
    pub fn analyze(source: &str) -> Self {
        // Implementation
    }
}

// Integrate with QuantitativeAnalysis
impl QuantitativeAnalysis {
    pub fn with_halstead(mut self, halstead: HalsteadMetrics) -> Self {
        self.halstead = Some(halstead);
        self
    }
}
```

#### 23.2 Custom Report Sections

```rust
pub trait ReportSection {
    fn title(&self) -> String;
    fn to_markdown(&self) -> String;
}

pub struct CustomSection {
    title: String,
    content: String,
}

impl ReportSection for CustomSection {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn to_markdown(&self) -> String {
        format!("## {}\n\n{}\n", self.title, self.content)
    }
}

// Add to report
let report = BugReport::new(/* ... */)
    .with_custom_section(Box::new(CustomSection {
        title: "Historical Context".to_string(),
        content: "This bug is similar to issue #45...".to_string(),
    }));
```

---

### 24. Integration with CI/CD

#### 24.1 GitHub Actions Workflow

```yaml
name: Bug Discovery

on:
  pull_request:
  schedule:
    - cron: '0 0 * * *'  # Daily at midnight

jobs:
  discover:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build Bug Discovery System
        run: cargo build --release

      - name: Run Discovery
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          cargo run --release -- discover \
            --differential \
            --property \
            --fuzz \
            --auto-file-issues

      - name: Upload Report
        uses: actions/upload-artifact@v2
        with:
          name: bug-discovery-report
          path: bug_report.md
```

#### 24.2 Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

set -euo pipefail

echo "Running bug discovery on changed files..."

# Get changed files
CHANGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)

if [ -z "$CHANGED_FILES" ]; then
    echo "No Rust files changed, skipping discovery"
    exit 0
fi

# Run property tests on changed code
cargo run --release -- discover \
    --property \
    --files "$CHANGED_FILES" \
    --confidence-threshold 0.85

if [ $? -ne 0 ]; then
    echo "❌ High-confidence bugs detected! Fix before committing."
    exit 1
fi

echo "✅ No high-confidence bugs detected"
exit 0
```

---

## Appendices

### Appendix D: FAQ

**Q: How long does full discovery take?**
A: Depends on test suite size. Typical times:
- Differential: ~1 minute (100 tests)
- Property: ~5 minutes (10,000 cases)
- Fuzz: ~30 minutes (100,000 inputs)
- Mutation: ~2 hours (full test suite)

**Q: Can I run discovery incrementally?**
A: Yes! Use `--incremental` flag to only test changed code:
```bash
cargo run -- discover --incremental --git-diff main
```

**Q: How do I reduce false positives?**
A: Increase confidence threshold:
```bash
cargo run -- discover --confidence-threshold 0.90
```
Or enable human review for medium-confidence findings.

**Q: Can I customize the Five-Whys analysis?**
A: Yes! Provide custom data sources:
```rust
let analyzer = FiveWhysAnalyzer::new()
    .with_data_source(Box::new(MyCustomDataSource))
    .with_max_depth(7); // Go deeper than 5 whys
```

**Q: How do I integrate with my existing bug tracker?**
A: Implement the `BugTracker` trait:
```rust
pub trait BugTracker {
    fn file_issue(&self, report: &BugReport) -> Result<IssueId>;
    fn check_duplicate(&self, report: &BugReport) -> Result<Option<IssueId>>;
}
```

**Q: What if I don't have a GitHub token?**
A: You can still use all discovery/reporting features. GitHub integration is optional.

**Q: How do I contribute new discovery methods?**
A: See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines. Implement the `BugDiscoverer` trait and submit a PR.

---

### Appendix E: Glossary

**Confidence Score**: 0.0-1.0 value indicating automation certainty that a finding is a real bug.

**Differential Testing**: Comparing behavior across versions, targets, or oracles to find discrepancies.

**Property Testing**: Verifying mathematical properties (e.g., roundtrip) on randomly generated inputs.

**Fuzz Testing**: Generating random inputs (valid or invalid) to find crashes, hangs, or unexpected behavior.

**Mutation Testing**: Testing the test suite by introducing bugs (mutations) and checking if tests catch them.

**Jidoka**: Toyota principle - automation with human touch. System provides data, humans make final judgments.

**Five-Whys**: Toyota root cause analysis technique - ask "why" five times to reach fundamental cause.

**TDD (Test-Driven Development)**: RED-GREEN-REFACTOR cycle - write failing test, make it pass, refactor.

**SATD (Self-Admitted Technical Debt)**: Developer comments admitting issues (TODO, FIXME, HACK).

**Cyclomatic Complexity**: Number of linearly independent paths through code (McCabe metric).

**Code Churn**: Rate of change in a file (lines added/deleted per unit time).

**Jaccard Similarity**: |A ∩ B| / |A ∪ B| - overlap divided by union.

**Delta Debugging**: Algorithm for minimizing failing test cases to smallest reproducing input.

---

## Conclusion

This user guide provides comprehensive coverage of the Bug Discovery, Reporter, and Replicator System. For additional resources:

- **API Reference**: See [docs/api/README.md](../api/README.md)
- **Examples**: See [docs/examples/README.md](../examples/README.md)
- **Troubleshooting**: See [docs/troubleshooting/README.md](../troubleshooting/README.md)
- **Specification**: See [docs/specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md](../specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md)

**Support**:
- GitHub Issues: https://github.com/paiml/ruchyruchy/issues
- Discussions: https://github.com/paiml/ruchyruchy/discussions

**Version**: 1.0.0
**Last Updated**: 2025-10-29
