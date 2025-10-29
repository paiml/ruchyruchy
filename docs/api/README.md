# Bug Discovery System - API Reference

**Version**: 1.0.0
**Date**: 2025-10-29

---

## Table of Contents

### Bug Discovery Module
1. [Differential Testing API](#1-differential-testing-api)
2. [Property Testing API](#2-property-testing-api)
3. [Fuzz Testing API](#3-fuzz-testing-api)
4. [Mutation Testing API](#4-mutation-testing-api)
5. [Confidence Scoring API](#5-confidence-scoring-api)

### Bug Replication Module
6. [Delta Debugging API](#6-delta-debugging-api)
7. [Test Minimization API](#7-test-minimization-api)

### Bug Reporting Module
8. [Complexity Metrics API](#8-complexity-metrics-api)
9. [Churn Analysis API](#9-churn-analysis-api)
10. [SATD Detection API](#10-satd-detection-api)
11. [Five-Whys Analysis API](#11-five-whys-analysis-api)
12. [TDD Integration API](#12-tdd-integration-api)
13. [Report Generator API](#13-report-generator-api)

### GitHub Integration Module
14. [GitHub Client API](#14-github-client-api)
15. [Issue Deduplication API](#15-issue-deduplication-api)

### Validation Module
16. [Historical Bug Validation API](#16-historical-bug-validation-api)

---

## 1. Differential Testing API

### Module: `ruchyruchy::bug_discovery::differential`

#### Types

##### `DifferentialConfig`

```rust
pub struct DifferentialConfig {
    pub old_version: String,
    pub new_version: String,
    pub timeout_ms: u64,
}
```

**Fields**:
- `old_version`: Version identifier for baseline (e.g., "1.28.0")
- `new_version`: Version identifier for comparison (e.g., "1.29.0")
- `timeout_ms`: Maximum execution time per test case (milliseconds)

##### `DifferentialResult`

```rust
pub enum DifferentialResult {
    Match,
    Mismatch { old_output: String, new_output: String },
    Timeout { version: String },
}
```

**Variants**:
- `Match`: Outputs match (no regression)
- `Mismatch`: Outputs differ (potential regression)
- `Timeout`: Execution exceeded timeout (potential hang)

#### Functions

##### `VersionRegressionTester::new`

```rust
pub fn new(config: DifferentialConfig) -> Self
```

**Description**: Create new version regression tester

**Parameters**:
- `config`: Configuration specifying old/new versions and timeout

**Returns**: `VersionRegressionTester` instance

**Example**:
```rust
let tester = VersionRegressionTester::new(DifferentialConfig {
    old_version: "1.28.0".to_string(),
    new_version: "1.29.0".to_string(),
    timeout_ms: 5000,
});
```

##### `VersionRegressionTester::test`

```rust
pub fn test(&self, input: &str) -> DifferentialResult
```

**Description**: Test single input for regression

**Parameters**:
- `input`: Test case source code

**Returns**: `DifferentialResult` indicating match, mismatch, or timeout

**Example**:
```rust
let result = tester.test("fun main() { println(\"Hello\"); }");
match result {
    DifferentialResult::Match => println!("No regression"),
    DifferentialResult::Mismatch { old_output, new_output } => {
        println!("Regression found!");
        println!("Old: {}", old_output);
        println!("New: {}", new_output);
    },
    DifferentialResult::Timeout { version } => {
        println!("Timeout in version: {}", version);
    },
}
```

---

## 2. Property Testing API

### Module: `ruchyruchy::bug_discovery::property`

#### Types

##### `PropertyConfig`

```rust
pub struct PropertyConfig {
    pub num_cases: usize,
    pub max_depth: usize,
    pub timeout_ms: u64,
    pub shrink_on_failure: bool,
}
```

**Fields**:
- `num_cases`: Number of test cases to generate (recommended: 10,000)
- `max_depth`: Maximum recursion depth for generators (recommended: 10)
- `timeout_ms`: Timeout per test case (milliseconds)
- `shrink_on_failure`: Enable shrinking to find minimal counterexample

##### `PropertyResult`

```rust
pub enum PropertyResult {
    Success { cases_tested: usize },
    Failure { counterexample: Value, shrunk: Option<Value> },
}
```

**Variants**:
- `Success`: Property holds for all test cases
- `Failure`: Property violated; includes counterexample and (optionally) shrunk minimal example

#### Functions

##### `PropertyTester::new`

```rust
pub fn new(config: PropertyConfig) -> Self
```

**Description**: Create new property tester

**Parameters**:
- `config`: Property testing configuration

**Returns**: `PropertyTester` instance

##### `PropertyTester::test_property`

```rust
pub fn test_property<F, G>(
    &self,
    property_name: &str,
    property: F,
    generator: G
) -> PropertyResult
where
    F: Fn(&Value) -> bool,
    G: Generator<Value>
```

**Description**: Test a property on generated inputs

**Parameters**:
- `property_name`: Human-readable property name (e.g., "roundtrip")
- `property`: Function that returns `true` if property holds for input
- `generator`: Input generator implementing `Generator` trait

**Returns**: `PropertyResult` indicating success or failure with counterexample

**Example**:
```rust
let tester = PropertyTester::new(PropertyConfig {
    num_cases: 10_000,
    max_depth: 10,
    timeout_ms: 5000,
    shrink_on_failure: true,
});

let result = tester.test_property(
    "parse_roundtrip",
    |input: &String| {
        let ast = parse(input);
        let emitted = ast.emit();
        parse(&emitted) == ast
    },
    StringGenerator::new()
);
```

---

## 3. Fuzz Testing API

### Module: `ruchyruchy::bug_discovery::fuzz`

#### Types

##### `FuzzConfig`

```rust
pub struct FuzzConfig {
    pub max_depth: usize,
    pub timeout_ms: u64,
    pub num_iterations: usize,
}
```

**Fields**:
- `max_depth`: Maximum nesting depth for grammar-based generation
- `timeout_ms`: Timeout per fuzz iteration
- `num_iterations`: Total number of fuzz iterations to run

##### `FuzzResult`

```rust
pub enum FuzzResult {
    Pass,
    Crash,
    Timeout,
    UnexpectedError(String),
}
```

**Variants**:
- `Pass`: Input processed successfully
- `Crash`: Crash detected (e.g., panic, segfault)
- `Timeout`: Execution exceeded timeout (potential hang)
- `UnexpectedError`: Unexpected error type

##### `FuzzStatistics`

```rust
pub struct FuzzStatistics {
    pub total: usize,
    pub passes: usize,
    pub crashes: usize,
    pub timeouts: usize,
    pub errors: usize,
    pub coverage: f64,
    pub crash_inputs: Vec<String>,
}
```

**Fields**:
- `total`: Total inputs tested
- `passes`: Successful executions
- `crashes`: Crash count
- `timeouts`: Timeout count
- `errors`: Error count
- `coverage`: Code coverage achieved (0.0-1.0)
- `crash_inputs`: List of inputs that caused crashes

#### Functions

##### `GrammarFuzzer::new`

```rust
pub fn new(config: FuzzConfig) -> Self
```

**Description**: Create grammar-based fuzzer

**Parameters**:
- `config`: Fuzzing configuration

**Returns**: `GrammarFuzzer` instance

##### `GrammarFuzzer::fuzz`

```rust
pub fn fuzz<F>(&self, grammar: &Grammar, test_fn: F) -> FuzzStatistics
where
    F: Fn(&str) -> FuzzResult
```

**Description**: Run grammar-based fuzzing

**Parameters**:
- `grammar`: Language grammar for generation
- `test_fn`: Function that tests each input and returns `FuzzResult`

**Returns**: `FuzzStatistics` with results

**Example**:
```rust
let fuzzer = GrammarFuzzer::new(FuzzConfig {
    max_depth: 15,
    timeout_ms: 1000,
    num_iterations: 100_000,
});

let grammar = Grammar::from_file("ruchy.grammar")?;

let stats = fuzzer.fuzz(&grammar, |input| {
    match run_compiler(input) {
        Ok(_) => FuzzResult::Pass,
        Err(e) if e.is_crash() => FuzzResult::Crash,
        Err(e) if e.is_timeout() => FuzzResult::Timeout,
        Err(e) => FuzzResult::UnexpectedError(e.to_string()),
    }
});

println!("Crashes: {} out of {}", stats.crashes, stats.total);
```

---

## 4. Mutation Testing API

### Module: `ruchyruchy::bug_discovery::mutation`

#### Types

##### `MutationConfig`

```rust
pub struct MutationConfig {
    pub operators: Vec<MutationOperator>,
    pub timeout_ms: u64,
}
```

**Fields**:
- `operators`: List of mutation operators to apply
- `timeout_ms`: Timeout per mutant execution

##### `MutationOperator`

```rust
pub enum MutationOperator {
    ArithmeticReplacement,  // + → -, * → /
    BoundaryShift,          // < → <=, > → >=
    LogicalNegation,        // && → ||, !x → x
    TypeMismatch,           // i32 → i64, String → &str
}
```

##### `MutationResult`

```rust
pub struct MutationResult {
    pub mutation_score: f64,  // Percentage of killed mutants (0.0-1.0)
    pub total: usize,
    pub killed: usize,        // Test suite caught mutation
    pub survived: usize,      // Test suite missed mutation
    pub timeout: usize,
    pub survived_mutants: Vec<Mutant>,
}
```

**Fields**:
- `mutation_score`: Quality score (killed / total)
- `total`: Total mutants generated
- `killed`: Mutants caught by tests
- `survived`: Mutants not caught (test gaps)
- `timeout`: Mutants that timed out
- `survived_mutants`: Details of survived mutants

##### `Mutant`

```rust
pub struct Mutant {
    pub id: usize,
    pub file: String,
    pub line: usize,
    pub operator: MutationOperator,
    pub original: String,
    pub mutated: String,
    pub diff: String,
}
```

#### Functions

##### `MutationEngine::new`

```rust
pub fn new(config: MutationConfig) -> Self
```

**Description**: Create mutation testing engine

**Parameters**:
- `config`: Mutation configuration

**Returns**: `MutationEngine` instance

##### `MutationEngine::test`

```rust
pub fn test(&self, source_code: &str, test_suite: &TestSuite) -> MutationResult
```

**Description**: Run mutation testing on source code

**Parameters**:
- `source_code`: Code to mutate
- `test_suite`: Test suite to run against mutants

**Returns**: `MutationResult` with mutation score and details

**Example**:
```rust
let engine = MutationEngine::new(MutationConfig {
    operators: vec![
        MutationOperator::ArithmeticReplacement,
        MutationOperator::BoundaryShift,
        MutationOperator::LogicalNegation,
    ],
    timeout_ms: 5000,
});

let source = read_file("src/parser.rs")?;
let tests = TestSuite::discover()?;

let result = engine.test(&source, &tests);

println!("Mutation Score: {:.1}%", result.mutation_score * 100.0);
println!("Killed: {}/{}", result.killed, result.total);

for mutant in result.survived_mutants {
    println!("Undetected: {}:{} - {:?}",
        mutant.file, mutant.line, mutant.operator);
}
```

---

## 5. Confidence Scoring API

### Module: `ruchyruchy::bug_discovery::confidence`

#### Types

##### `ConfidenceScore`

```rust
pub struct ConfidenceScore {
    pub overall: f64,                    // 0.0-1.0
    pub discovery_method_weight: f64,    // Weight: 0.35
    pub reproducibility_score: f64,      // Weight: 0.30
    pub quantitative_evidence: f64,      // Weight: 0.20
    pub root_cause_clarity: f64,         // Weight: 0.15
}
```

**Fields**:
- `overall`: Weighted average of all factors (0.0-1.0)
- `discovery_method_weight`: Confidence in discovery method (see weights table)
- `reproducibility_score`: Determinism and minimization quality
- `quantitative_evidence`: Quality of metrics data
- `root_cause_clarity`: Quality of root cause hypothesis

**Discovery Method Weights**:
- Differential (Version Regression): 1.0
- Property Test Violation: 0.95
- Differential (Target Mismatch): 0.9
- Grammar Fuzz (Crash/Hang): 0.85
- Mutation (Undetected): 0.80
- Code Churn Hotspot: 0.70
- SATD Detection: 0.60
- Complexity Spike: 0.50

**Confidence Levels**:
- 0.85-1.0: Very High (review immediately)
- 0.70-0.84: High (review within 24h)
- 0.50-0.69: Medium (review within week)
- 0.25-0.49: Low (backlog)
- 0.0-0.24: Very Low (likely false positive)

#### Functions

##### `ConfidenceScore::new`

```rust
pub fn new(
    discovery_method_weight: f64,
    reproducibility_score: f64,
    quantitative_evidence: f64,
    root_cause_clarity: f64,
) -> Self
```

**Description**: Create new confidence score with automatic overall calculation

**Parameters**:
- `discovery_method_weight`: Confidence in discovery method (0.0-1.0)
- `reproducibility_score`: Reproducibility quality (0.0-1.0)
- `quantitative_evidence`: Metrics quality (0.0-1.0)
- `root_cause_clarity`: Root cause quality (0.0-1.0)

**Returns**: `ConfidenceScore` with calculated `overall` field

**Formula**:
```rust
overall = 0.35 * discovery_method_weight
        + 0.30 * reproducibility_score
        + 0.20 * quantitative_evidence
        + 0.15 * root_cause_clarity
```

**Example**:
```rust
let confidence = ConfidenceScore::new(
    0.95,  // Property test violation
    0.90,  // 100% deterministic, <10 LOC
    0.85,  // High complexity + high churn
    0.80,  // Clear Five-Whys hypothesis
);
// confidence.overall = 0.8925 (Very High)
```

##### `ConfidenceScore::level`

```rust
pub fn level(&self) -> ConfidenceLevel
```

**Description**: Get confidence level enum from overall score

**Returns**: `ConfidenceLevel` (VeryHigh, High, Medium, Low, VeryLow)

**Example**:
```rust
match confidence.level() {
    ConfidenceLevel::VeryHigh => println!("Review immediately!"),
    ConfidenceLevel::High => println!("Review within 24 hours"),
    ConfidenceLevel::Medium => println!("Review within week"),
    ConfidenceLevel::Low => println!("Add to backlog"),
    ConfidenceLevel::VeryLow => println!("Likely false positive"),
}
```

---

## 6. Delta Debugging API

### Module: `ruchyruchy::bug_replication::delta_debug`

#### Types

##### `MinimizationConfig`

```rust
pub struct MinimizationConfig {
    pub target_size: usize,        // Prefer <10 lines
    pub timeout_per_attempt: u64,  // Milliseconds
    pub max_attempts: usize,       // Give up after N attempts
}
```

##### `MinimizationResult`

```rust
pub struct MinimizationResult {
    pub original_size: usize,      // Lines in original input
    pub minimized_size: usize,     // Lines in minimized input
    pub reduction_ratio: f64,      // minimized_size / original_size
    pub attempts: usize,           // Total attempts made
    pub minimized_input: String,   // Final minimal input
}
```

#### Functions

##### `DeltaDebugger::new`

```rust
pub fn new(config: MinimizationConfig) -> Self
```

**Description**: Create delta debugger for test case minimization

**Parameters**:
- `config`: Minimization configuration

**Returns**: `DeltaDebugger` instance

##### `DeltaDebugger::minimize`

```rust
pub fn minimize<F>(&self, input: &str, test_fn: F) -> MinimizationResult
where
    F: Fn(&str) -> bool
```

**Description**: Minimize failing input to smallest reproducing case

**Parameters**:
- `input`: Original failing input
- `test_fn`: Function that returns `true` if input still fails

**Returns**: `MinimizationResult` with minimal input

**Algorithm**: Binary search through input, removing chunks and testing if failure persists

**Example**:
```rust
let debugger = DeltaDebugger::new(MinimizationConfig {
    target_size: 10,
    timeout_per_attempt: 1000,
    max_attempts: 1000,
});

let original = read_file("failing_test_500_lines.ruchy")?;

let result = debugger.minimize(&original, |input| {
    // Returns true if input still triggers bug
    let ast = parse(input);
    let emitted = ast.emit();
    parse(&emitted) != ast // Roundtrip fails
});

println!("Minimized from {} to {} lines ({:.1}% reduction)",
    result.original_size,
    result.minimized_size,
    (1.0 - result.reduction_ratio) * 100.0
);
println!("Minimal failing input:\n{}", result.minimized_input);
```

---

## 8. Complexity Metrics API

### Module: `ruchyruchy::bug_reporting::metrics`

#### Types

##### `ComplexityMetrics`

```rust
pub struct ComplexityMetrics {
    pub cyclomatic: usize,       // Linearly independent paths
    pub cognitive: usize,        // Human perception of difficulty
    pub loop_nesting_depth: usize,
    pub function_length: usize,  // Lines of code
}
```

**Thresholds**:
- Cyclomatic >15: High risk
- Cognitive >20: Very high risk
- Loop depth >3: Refactor recommended
- Function length >50: Consider splitting

#### Functions

##### `ComplexityMetrics::analyze`

```rust
pub fn analyze(source_code: &str) -> Self
```

**Description**: Analyze source code complexity

**Parameters**:
- `source_code`: Source code to analyze

**Returns**: `ComplexityMetrics` with all metrics calculated

**Example**:
```rust
let source = read_file("src/parser.rs")?;
let metrics = ComplexityMetrics::analyze(&source);

println!("Cyclomatic: {} {}", metrics.cyclomatic,
    if metrics.cyclomatic > 15 { "⚠️  HIGH RISK" } else { "✅" });
println!("Cognitive: {} {}", metrics.cognitive,
    if metrics.cognitive > 20 { "⚠️  VERY HIGH RISK" } else { "✅" });
println!("Loop depth: {} {}", metrics.loop_nesting_depth,
    if metrics.loop_nesting_depth > 3 { "⚠️  REFACTOR" } else { "✅" });
println!("Function length: {} {}", metrics.function_length,
    if metrics.function_length > 50 { "⚠️  SPLIT" } else { "✅" });
```

---

## 9. Churn Analysis API

### Module: `ruchyruchy::bug_reporting::metrics`

#### Types

##### `ChurnCorrelation`

```rust
pub struct ChurnCorrelation {
    pub file_path: String,
    pub commit_count: usize,
    pub change_count: usize,
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub churn_rate: f64,       // (added + deleted) / days
    pub last_change: SystemTime,
}
```

**Interpretation**:
- Churn rate >10: High instability
- Recent changes (<30 days): Higher risk
- Research: 80%+ of post-release bugs in high-churn files

#### Functions

##### `ChurnCorrelation::analyze_file`

```rust
pub fn analyze_file(file_path: &str, days: usize) -> Result<Self>
```

**Description**: Analyze git churn for file over time period

**Parameters**:
- `file_path`: Path to file relative to git root
- `days`: Time period to analyze (e.g., 90 days)

**Returns**: `ChurnCorrelation` with metrics

**Example**:
```rust
let churn = ChurnCorrelation::analyze_file("src/parser.rs", 90)?;

println!("Churn Analysis (last {} days):", 90);
println!("  Commits: {}", churn.commit_count);
println!("  Changes: {}", churn.change_count);
println!("  Added: {} lines", churn.lines_added);
println!("  Deleted: {} lines", churn.lines_deleted);
println!("  Churn rate: {:.2} {}", churn.churn_rate,
    if churn.churn_rate > 10.0 { "⚠️  HIGH INSTABILITY" } else { "✅" });
```

---

## 10. SATD Detection API

### Module: `ruchyruchy::bug_reporting::metrics`

#### Types

##### `SatdType`

```rust
pub enum SatdType {
    Todo,    // TODO: Planned work
    Fixme,   // FIXME: Known bug
    Hack,    // HACK: Temporary workaround
    Xxx,     // XXX: Problem area
    Note,    // NOTE: Important note
}
```

##### `SatdItem`

```rust
pub struct SatdItem {
    pub satd_type: SatdType,
    pub line_number: usize,
    pub text: String,
    pub priority: SatdPriority,
}
```

##### `SatdPriority`

```rust
pub enum SatdPriority {
    High,    // FIXME, HACK in high-churn files
    Medium,  // TODO in recent commits
    Low,     // NOTE, old TODOs
}
```

##### `SatdDetector`

```rust
pub struct SatdDetector {
    pub total_count: usize,
    pub debts: Vec<SatdItem>,
}
```

#### Functions

##### `SatdDetector::analyze`

```rust
pub fn analyze(source_code: &str) -> Self
```

**Description**: Detect self-admitted technical debt in source code

**Parameters**:
- `source_code`: Source code to analyze

**Returns**: `SatdDetector` with all SATD items found

**Example**:
```rust
let source = read_file("src/parser.rs")?;
let satd = SatdDetector::analyze(&source);

println!("SATD Analysis:");
println!("  Total: {}", satd.total_count);

for debt in satd.debts {
    let emoji = match debt.satd_type {
        SatdType::Fixme | SatdType::Hack => "🔴",
        SatdType::Todo => "🟡",
        SatdType::Note | SatdType::Xxx => "🔵",
    };
    println!("\n  {} {:?} (line {})", emoji, debt.satd_type, debt.line_number);
    println!("     {}", debt.text);
    println!("     Priority: {:?}", debt.priority);
}
```

---

## 11. Five-Whys Analysis API

### Module: `ruchyruchy::bug_reporting::five_whys`

#### Types

##### `ConfidenceLevel`

```rust
pub enum ConfidenceLevel {
    VeryHigh,   // 0.90-1.00: Direct evidence
    High,       // 0.75-0.89: Strong correlation
    Medium,     // 0.50-0.74: Reasonable hypothesis
    Low,        // 0.25-0.49: Speculative
    VeryLow,    // 0.00-0.24: Weak evidence
}
```

##### `DataPoint`

```rust
pub struct DataPoint {
    pub value: String,
    pub source: DataSource,
}
```

##### `DataSource`

```rust
pub enum DataSource {
    StaticAnalysis,  // 0.30 confidence weight
    ChurnAnalysis,   // 0.25
    SatdDetector,    // 0.20
    TestCoverage,    // 0.15
    IssueTracker,    // 0.10
}
```

##### `WhyLayer`

```rust
pub struct WhyLayer {
    pub question: String,
    pub hypothesis: String,
    pub confidence: ConfidenceLevel,
    pub data_points: Vec<DataPoint>,
}
```

##### `FiveWhysAnalysis`

```rust
pub struct FiveWhysAnalysis {
    pub layers: Vec<WhyLayer>,    // Up to 5 layers
    pub conclusion: String,
}
```

#### Functions

##### `FiveWhysAnalyzer::analyze`

```rust
pub fn analyze(
    bug_description: &str,
    metrics: &ComplexityMetrics,
    churn: &ChurnCorrelation,
    satd: &SatdDetector,
) -> FiveWhysAnalysis
```

**Description**: Perform Five-Whys root cause analysis with data-driven hypotheses

**Parameters**:
- `bug_description`: Description of the bug
- `metrics`: Complexity metrics
- `churn`: Code churn analysis
- `satd`: SATD detection results

**Returns**: `FiveWhysAnalysis` with up to 5 "why" layers and conclusion

**Example**:
```rust
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
    println!("Confidence: {:?}", layer.confidence);
    println!("Data points:");
    for dp in &layer.data_points {
        println!("  - {} (source: {:?})", dp.value, dp.source);
    }
}

println!("\nConclusion:");
println!("{}", analysis.conclusion);
```

---

## 12. TDD Integration API

### Module: `ruchyruchy::bug_reporting::tdd`

#### Types

##### `TddPhase`

```rust
pub enum TddPhase {
    Red,       // Write failing test
    Green,     // Minimal implementation
    Refactor,  // Improve code quality
}
```

##### `TestResult`

```rust
pub enum TestResult {
    Pass,
    Fail,
    NotRun,
}
```

##### `TddCycle`

```rust
pub struct TddCycle {
    pub cycle: usize,
    pub phase: TddPhase,
    pub test_result: TestResult,
    pub test_count: usize,
    pub passing: usize,
    pub failing: usize,
    pub coverage: f64,          // 0.0-1.0
    pub timestamp: SystemTime,
    pub duration: Duration,
    pub description: String,
}
```

##### `TddHistory`

```rust
pub struct TddHistory {
    cycles: Vec<TddCycle>,
    current_cycle: usize,
    start_time: SystemTime,
}
```

#### Functions

##### `TddHistory::new`

```rust
pub fn new() -> Self
```

**Description**: Create new TDD history tracker

**Returns**: Empty `TddHistory`

##### `TddHistory::add_cycle`

```rust
pub fn add_cycle(&mut self, cycle: TddCycle)
```

**Description**: Add TDD cycle to history

**Parameters**:
- `cycle`: TDD cycle to add

**Example**:
```rust
let mut history = TddHistory::new();

// RED: Write failing test
history.add_cycle(TddCycle {
    phase: TddPhase::Red,
    test_result: TestResult::Fail,
    test_count: 1,
    passing: 0,
    failing: 1,
    coverage: 0.0,
    description: "Add test for recursion limit".to_string(),
    /* ... */
});

// GREEN: Minimal implementation
history.add_cycle(TddCycle {
    phase: TddPhase::Green,
    test_result: TestResult::Pass,
    test_count: 1,
    passing: 1,
    failing: 0,
    coverage: 0.42,
    description: "Add max_depth check".to_string(),
    /* ... */
});

// REFACTOR: Improve
history.add_cycle(TddCycle {
    phase: TddPhase::Refactor,
    test_result: TestResult::Pass,
    test_count: 3,
    passing: 3,
    failing: 0,
    coverage: 0.68,
    description: "Extract helper function".to_string(),
    /* ... */
});
```

##### `TddHistory::to_markdown`

```rust
pub fn to_markdown(&self) -> String
```

**Description**: Generate markdown report of TDD cycles

**Returns**: Markdown string

---

## 13. Report Generator API

### Module: `ruchyruchy::bug_reporting::report_generator`

#### Types

##### `Severity`

```rust
pub enum Severity {
    Critical,  // System crash, data loss, security
    High,      // Major functionality broken
    Medium,    // Minor functionality affected
    Low,       // Cosmetic or documentation
}
```

##### `BugCategory`

```rust
pub enum BugCategory {
    Crash,
    Hang,
    WrongOutput,
    PerformanceRegression,
    MemoryLeak,
    TypeError,
    ParserError,
    Other(String),
}
```

##### `BugReport`

```rust
pub struct BugReport {
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub category: BugCategory,
    pub reproduction_code: String,
    pub expected: String,
    pub actual: String,
    pub confidence: ConfidenceScore,
    pub quantitative_analysis: Option<QuantitativeAnalysis>,
    pub five_whys: Option<FiveWhysAnalysis>,
    pub tdd_history: Option<TddHistory>,
    pub related_files: Vec<String>,
    pub fix_recommendations: Vec<String>,
    pub prevention: Vec<String>,
}
```

#### Functions

##### `BugReport::new`

```rust
pub fn new(
    title: String,
    description: String,
    severity: Severity,
    category: BugCategory,
    reproduction_code: String,
    expected: String,
    actual: String,
    confidence: ConfidenceScore,
) -> Self
```

**Description**: Create new bug report

**Parameters**:
- `title`: Bug title (concise, descriptive)
- `description`: Detailed description
- `severity`: Severity level
- `category`: Bug category
- `reproduction_code`: Minimal reproduction code
- `expected`: Expected behavior
- `actual`: Actual behavior
- `confidence`: Confidence score

**Returns**: `BugReport` with optional fields empty

##### Builder Methods

```rust
pub fn with_quantitative_analysis(mut self, analysis: QuantitativeAnalysis) -> Self
pub fn with_five_whys(mut self, five_whys: FiveWhysAnalysis) -> Self
pub fn with_tdd_history(mut self, history: TddHistory) -> Self
pub fn with_related_files(mut self, files: Vec<String>) -> Self
pub fn with_fix_recommendations(mut self, recommendations: Vec<String>) -> Self
pub fn with_prevention(mut self, prevention: Vec<String>) -> Self
```

**Description**: Add optional sections to bug report

**Returns**: Self for chaining

##### `BugReport::to_markdown`

```rust
pub fn to_markdown(&self) -> String
```

**Description**: Generate comprehensive markdown report

**Returns**: Markdown string with all sections

**Example**:
```rust
let report = BugReport::new(
    "Parser crashes on nested expressions".to_string(),
    "Stack overflow on deep nesting".to_string(),
    Severity::Critical,
    BugCategory::Crash,
    "fun test() { ((((1)))) }".to_string(),
    "Parse successfully".to_string(),
    "Stack overflow panic".to_string(),
    confidence,
)
.with_quantitative_analysis(metrics)
.with_five_whys(five_whys)
.with_tdd_history(tdd_history)
.with_related_files(vec!["src/parser.rs".to_string()])
.with_fix_recommendations(vec![
    "Add max_depth parameter".to_string(),
]);

let markdown = report.to_markdown();
write_file("bug_report.md", &markdown)?;
```

---

## 14. GitHub Client API

### Module: `ruchyruchy::bug_reporting::github_integration`

#### Types

##### `GitHubClient`

```rust
pub struct GitHubClient {
    owner: String,
    repo: String,
    token: String,
    base_url: String,
}
```

##### `IssueRequest`

```rust
pub struct IssueRequest {
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
}
```

##### `IssueResponse`

```rust
pub struct IssueResponse {
    pub number: u64,
    pub html_url: String,
    pub url: String,
    pub state: String,
}
```

#### Functions

##### `GitHubClient::new`

```rust
pub fn new(owner: String, repo: String, token: String) -> Self
```

**Description**: Create GitHub API client

**Parameters**:
- `owner`: Repository owner (e.g., "paiml")
- `repo`: Repository name (e.g., "ruchy")
- `token`: Personal Access Token

**Returns**: `GitHubClient` instance

**Example**:
```rust
let client = GitHubClient::new(
    std::env::var("GITHUB_OWNER").unwrap(),
    std::env::var("GITHUB_REPO").unwrap(),
    std::env::var("GITHUB_TOKEN").unwrap(),
);
```

##### `BugReportConverter::to_issue_request`

```rust
pub fn to_issue_request(report: &BugReport) -> IssueRequest
```

**Description**: Convert BugReport to GitHub IssueRequest with automatic labels

**Parameters**:
- `report`: Bug report to convert

**Returns**: `IssueRequest` with title, body, and labels

**Automatic Labels**:
- Severity: `severity: critical`, `severity: high`, etc.
- Category: `type: crash`, `type: hang`, etc.
- Confidence: `high-confidence`, `medium-confidence`, `low-confidence`
- Standard: `bug`

**Example**:
```rust
let issue_request = BugReportConverter::to_issue_request(&report);

// Issue created with:
// - Title: "[CRITICAL] Parser crashes on nested expressions"
// - Body: Full markdown report
// - Labels: ["severity: critical", "type: crash", "high-confidence", "bug"]
```

---

## 15. Issue Deduplication API

### Module: `ruchyruchy::bug_reporting::issue_linking`

#### Types

##### `BugIssue`

```rust
pub struct BugIssue {
    pub number: u64,
    pub title: String,
    pub body: String,
    pub files: Vec<String>,
    pub error_message: Option<String>,
    pub labels: Vec<String>,
}
```

##### `SimilarityScore`

```rust
pub struct SimilarityScore {
    pub overall: f64,             // Weighted average
    pub title_similarity: f64,    // Weight: 0.30
    pub body_similarity: f64,     // Weight: 0.25
    pub file_overlap: f64,        // Weight: 0.20
    pub error_similarity: f64,    // Weight: 0.15
    pub label_overlap: f64,       // Weight: 0.10
}
```

**Thresholds**:
- `DUPLICATE_THRESHOLD`: 0.80 (overall ≥0.80 is duplicate)
- `RELATED_THRESHOLD`: 0.50 (overall ≥0.50 is related)

##### `DuplicateResult`

```rust
pub struct DuplicateResult {
    pub is_duplicate: bool,
    pub duplicate_of: Option<u64>,
    pub similarity: SimilarityScore,
}
```

##### `RelatedIssue`

```rust
pub struct RelatedIssue {
    pub issue: BugIssue,
    pub similarity: SimilarityScore,
}
```

#### Functions

##### `IssueDeduplicator::new`

```rust
pub fn new() -> Self
```

**Description**: Create issue deduplicator

**Returns**: Empty `IssueDeduplicator`

##### `IssueDeduplicator::add_issue`

```rust
pub fn add_issue(&mut self, issue: BugIssue)
```

**Description**: Add existing issue to corpus

**Parameters**:
- `issue`: Issue to add

##### `IssueDeduplicator::check_duplicate`

```rust
pub fn check_duplicate(&self, new_issue: &BugIssue) -> DuplicateResult
```

**Description**: Check if new issue is duplicate of existing issues

**Parameters**:
- `new_issue`: Issue to check

**Returns**: `DuplicateResult` with duplicate status and similarity

**Example**:
```rust
let mut deduplicator = IssueDeduplicator::new();

// Load existing issues
for issue in existing_issues {
    deduplicator.add_issue(issue);
}

// Check new issue
let result = deduplicator.check_duplicate(&new_issue);

if result.is_duplicate {
    println!("Duplicate of issue #{}", result.duplicate_of.unwrap());
    println!("Similarity: {:.1}%", result.similarity.overall * 100.0);
} else {
    println!("Not a duplicate, file new issue");
}
```

##### `IssueDeduplicator::find_related`

```rust
pub fn find_related(&self, new_issue: &BugIssue, limit: usize) -> Vec<RelatedIssue>
```

**Description**: Find related issues (not duplicates, but similar)

**Parameters**:
- `new_issue`: Issue to find related issues for
- `limit`: Maximum number of related issues to return

**Returns**: Vector of related issues, sorted by similarity (descending)

**Example**:
```rust
let related = deduplicator.find_related(&new_issue, 5);

println!("Related issues:");
for r in related {
    println!("  #{}: {} (similarity: {:.1}%)",
        r.issue.number,
        r.issue.title,
        r.similarity.overall * 100.0
    );
}
```

---

## 16. Historical Bug Validation API

### Module: `ruchyruchy::bug_reporting::validation`

#### Types

##### `HistoricalBug`

```rust
pub struct HistoricalBug {
    pub issue_number: u64,
    pub title: String,
    pub body: String,
    pub category: BugCategory,
    pub files: Vec<String>,
    pub error_message: Option<String>,
    pub labels: Vec<String>,
    pub critical: bool,
}
```

##### `DetectionResult`

```rust
pub struct DetectionResult {
    pub detected: bool,
    pub method: Option<String>,
    pub confidence: f64,
    pub miss_reason: Option<String>,
}
```

##### `ValidationMetrics`

```rust
pub struct ValidationMetrics {
    pub total_bugs: usize,
    pub detected: usize,
    pub missed: usize,
    pub false_positives: usize,
    pub detection_rate: f64,           // detected / total
    pub false_positive_rate: f64,      // false_positives / (detected + false_positives)
    pub critical_detected: usize,
    pub critical_total: usize,
}
```

**Targets**:
- Detection rate: ≥0.95 (95%)
- False positive rate: <0.05 (5%)
- Critical detection: 1.0 (100%)

##### `ValidationReport`

```rust
pub struct ValidationReport {
    pub metrics: ValidationMetrics,
    pub detected_bugs: Vec<(HistoricalBug, DetectionResult)>,
    pub missed_bugs: Vec<(HistoricalBug, DetectionResult)>,
}
```

#### Functions

##### `BugCorpusValidator::new`

```rust
pub fn new(bugs: Vec<HistoricalBug>) -> Self
```

**Description**: Create validator with historical bug corpus

**Parameters**:
- `bugs`: Vector of historical bugs to validate against

**Returns**: `BugCorpusValidator` instance

##### `BugCorpusValidator::validate`

```rust
pub fn validate<F>(&self, mut detector: F) -> ValidationReport
where
    F: FnMut(&HistoricalBug) -> DetectionResult
```

**Description**: Validate detection system against historical bugs

**Parameters**:
- `detector`: Callback function that attempts to detect each bug

**Returns**: `ValidationReport` with metrics and details

**Example**:
```rust
let bugs = load_historical_bugs_from_github("paiml", "ruchy")?;
let validator = BugCorpusValidator::new(bugs);

let report = validator.validate(|bug| {
    // Run discovery system on bug
    let result = run_discovery_system(bug);

    DetectionResult {
        detected: result.is_some(),
        method: result.as_ref().map(|r| r.method.clone()),
        confidence: result.as_ref().map(|r| r.confidence).unwrap_or(0.0),
        miss_reason: if result.is_none() {
            Some("Not detected".to_string())
        } else {
            None
        },
    }
});

println!("Detection Rate: {:.1}%", report.metrics.detection_rate * 100.0);
println!("False Positive Rate: {:.1}%", report.metrics.false_positive_rate * 100.0);

if report.metrics.meets_target() {
    println!("✅ Validation PASSED (≥95% detection, <5% FP)");
} else {
    println!("❌ Validation FAILED");
}
```

##### `ValidationReport::to_markdown`

```rust
pub fn to_markdown(&self) -> String
```

**Description**: Generate markdown validation report

**Returns**: Markdown string with metrics, detected bugs, missed bugs

---

## Error Handling

All API functions that can fail return `Result<T, BugDiscoveryError>` where:

```rust
pub enum BugDiscoveryError {
    IoError(std::io::Error),
    ParseError(String),
    GitError(String),
    GitHubApiError(String),
    Timeout,
    InvalidInput(String),
}
```

**Example Error Handling**:

```rust
match run_discovery_system(&input) {
    Ok(findings) => {
        for finding in findings {
            println!("Bug found: {}", finding.title);
        }
    },
    Err(BugDiscoveryError::Timeout) => {
        eprintln!("Discovery timed out");
    },
    Err(BugDiscoveryError::GitHubApiError(msg)) => {
        eprintln!("GitHub API error: {}", msg);
    },
    Err(e) => {
        eprintln!("Error: {:?}", e);
    },
}
```

---

## Thread Safety

All types are `Send + Sync` unless otherwise noted. Safe for concurrent use:

```rust
use rayon::prelude::*;

let results: Vec<_> = test_cases
    .par_iter()
    .map(|test| run_discovery(test))
    .collect();
```

**Not thread-safe**:
- `TddHistory` (use `Mutex<TddHistory>` for concurrent updates)

---

## Performance Characteristics

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| Differential Testing | O(n) per test | O(1) |
| Property Testing | O(n * m) (n=cases, m=test time) | O(1) |
| Fuzz Testing | O(n) | O(1) |
| Mutation Testing | O(n * m) (n=mutants, m=test time) | O(n) |
| Complexity Analysis | O(n) (n=lines) | O(1) |
| Churn Analysis | O(m) (m=commits) | O(m) |
| SATD Detection | O(n) (n=lines) | O(k) (k=SATD count) |
| Five-Whys Analysis | O(1) | O(1) |
| Delta Debugging | O(n * log(n)) (n=input size) | O(n) |
| Jaccard Similarity | O(n + m) | O(n + m) |
| Issue Deduplication | O(n * m) (n=existing issues, m=input size) | O(n * m) |

---

## Version Compatibility

- **Rust**: 1.70.0 or later
- **Cargo**: Latest stable
- **GitHub API**: v3 (REST)

---

## See Also

- [User Guide](../user_guide/README.md) - Comprehensive usage guide
- [Examples](../examples/README.md) - Example workflows
- [Troubleshooting](../troubleshooting/README.md) - Common issues and solutions
- [Specification](../specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md) - System specification

---

**Last Updated**: 2025-10-29
**API Version**: 1.0.0
