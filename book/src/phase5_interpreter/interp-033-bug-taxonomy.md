# INTERP-033: Bug Taxonomy & Comprehensive Analysis

## Context

Bug taxonomy provides systematic categorization, prioritization, and analysis of discovered bugs. This ticket implements a comprehensive bug tracking database with severity levels, categories, and root cause analysis.

**Why this is needed**: Discovered bugs must be tracked, prioritized, and analyzed systematically. A taxonomy enables pattern recognition, root cause analysis, and prioritization of fixes.

## RED: Write Failing Test

Tests were written first to define taxonomy requirements:

```rust
// File: tests/test_interp_033_bug_taxonomy.rs
#[test]
fn test_bug_taxonomy_basic() {
    let mut taxonomy = BugTaxonomy::new();

    let bug = BugReport::new(
        "BUG-001".to_string(),
        "Block expressions not supported".to_string(),
        BugCategory::Parser,
        Severity::Medium,
        "Parser cannot handle block expressions with braces".to_string(),
        RootCause::MissingFeature,
        "Limits expressiveness".to_string(),
        "Try to parse { let x = 10; x }".to_string(),
    );

    taxonomy.add_bug(bug);
    assert_eq!(taxonomy.total_bugs(), 1);
}

#[test]
fn test_severity_distribution() {
    let mut taxonomy = BugTaxonomy::new();
    // ... add bugs with different severities ...

    let dist = taxonomy.severity_distribution();
    assert!(dist.len() > 0);
}
```

**Expected**: Tests fail because `BugTaxonomy` and `BugReport` don't exist.

**Actual**: Compilation error - bug taxonomy infrastructure not implemented.

**Validation**: `cargo test test_bug_taxonomy_basic` exits with status 1.

## GREEN: Minimal Implementation

Implemented comprehensive bug taxonomy system:

```rust
// File: tests/test_interp_033_bug_taxonomy.rs
#[derive(Debug, Clone, PartialEq)]
pub enum BugCategory {
    Parser,
    Evaluator,
    Performance,
    Safety,
    TypeSystem,
    Optimizer,
    Compatibility,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Critical,  // Crash, data loss, security
    High,      // Major functionality broken
    Medium,    // Minor functionality broken
    Low,       // Cosmetic, performance
}

#[derive(Debug, Clone, PartialEq)]
pub enum RootCause {
    MissingFeature,
    IncorrectLogic,
    EdgeCaseHandling,
    PerformanceBottleneck,
    DesignLimitation,
}

pub struct BugReport {
    pub id: String,
    pub title: String,
    pub category: BugCategory,
    pub severity: Severity,
    pub description: String,
    pub root_cause: RootCause,
    pub impact: String,
    pub reproduction: String,
}

pub struct BugTaxonomy {
    bugs: Vec<BugReport>,
}

impl BugTaxonomy {
    pub fn new() -> Self {
        Self { bugs: Vec::new() }
    }

    pub fn add_bug(&mut self, bug: BugReport) {
        self.bugs.push(bug);
    }

    pub fn total_bugs(&self) -> usize {
        self.bugs.len()
    }

    pub fn severity_distribution(&self) -> HashMap<String, usize> {
        let mut dist = HashMap::new();
        for bug in &self.bugs {
            let severity = format!("{:?}", bug.severity);
            *dist.entry(severity).or_insert(0) += 1;
        }
        dist
    }

    pub fn category_distribution(&self) -> HashMap<String, usize> {
        let mut dist = HashMap::new();
        for bug in &self.bugs {
            let category = format!("{:?}", bug.category);
            *dist.entry(category).or_insert(0) += 1;
        }
        dist
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Bug Taxonomy Report ===\n\n");
        report.push_str(&format!("Total Bugs: {}\n\n", self.total_bugs()));

        // Severity distribution
        report.push_str("Severity Distribution:\n");
        for (severity, count) in self.severity_distribution() {
            report.push_str(&format!("  {}: {}\n", severity, count));
        }

        // Category distribution
        report.push_str("\nCategory Distribution:\n");
        for (category, count) in self.category_distribution() {
            report.push_str(&format!("  {}: {}\n", category, count));
        }

        // Detailed bug list
        report.push_str("\nDetailed Bug List:\n");
        for bug in &self.bugs {
            report.push_str(&format!("\n{}. {} ({:?})\n", bug.id, bug.title, bug.severity));
            report.push_str(&format!("   Category: {:?}\n", bug.category));
            report.push_str(&format!("   Root Cause: {:?}\n", bug.root_cause));
            report.push_str(&format!("   Impact: {}\n", bug.impact));
        }

        report
    }
}
```

**Key Design Decisions**:
1. **7 Bug Categories**: Parser, Evaluator, Performance, Safety, TypeSystem, Optimizer, Compatibility
2. **4 Severity Levels**: Critical, High, Medium, Low
3. **5 Root Causes**: MissingFeature, IncorrectLogic, EdgeCaseHandling, PerformanceBottleneck, DesignLimitation
4. **Comprehensive Metadata**: ID, title, description, impact, reproduction steps

**Result**: ✅ All 7 tests passing

**Discovered Bugs Cataloged**:
- **BUG-001**: Block expressions not supported (Parser, Medium)
- **BUG-002**: Variable lookup overhead (Performance, Low)
- **BUG-003**: if-else as rvalue not supported (Parser, Medium)

**Validation**: `cargo test --test test_interp_033_bug_taxonomy` exits with status 0.

## REFACTOR: Improvements

After getting tests green, refactored for:

1. **Report Generation**: Comprehensive bug reports with distributions
2. **Query Methods**: Filter by severity, category, root cause
3. **Pattern Analysis**: Identify common root causes

**Clippy Fix**: BugReport::new() had 8 parameters (clippy limit is 7):
```rust
#[allow(clippy::too_many_arguments)]  // Bug reports need comprehensive metadata
pub fn new(
    id: String,
    title: String,
    category: BugCategory,
    severity: Severity,
    description: String,
    root_cause: RootCause,
    impact: String,
    reproduction: String,
) -> Self { ... }
```

**Alternative considered**: Builder pattern, but decided against it for simplicity.

## TOOL VALIDATION (7 Rust Tools)

```bash
cargo test --test test_interp_033_bug_taxonomy  # ✅ 7/7 tests passing
cargo clippy -- -D warnings                     # ✅ Zero warnings
cargo fmt -- --check                            # ✅ Properly formatted
```

**Results**:
1. `cargo test`: ✅ 7/7 tests passing
2. `cargo clippy`: ✅ Zero warnings (allow annotation for too_many_arguments)
3. `cargo fmt --check`: ✅ No formatting issues
4. Taxonomy: ✅ 3 bugs cataloged
5. Severity distribution: ✅ 0 Critical, 0 High, 2 Medium, 1 Low
6. Category distribution: ✅ 2 Parser, 1 Performance
7. Report generation: ✅ Comprehensive report produced

## REPRODUCIBILITY

**Script**: `tests/test_interp_033_bug_taxonomy.rs` (self-contained)

```bash
cargo test --test test_interp_033_bug_taxonomy
# Exit status: 0
# Output: 7/7 tests passing
# Bugs tracked: 3
```

**Idempotent**: Yes - bug database is deterministic.

## DEBUGGABILITY

**Debug Session**:
```bash
# Run taxonomy tests
cargo test test_bug_taxonomy_basic

# Check severity distribution
cargo test test_severity_distribution

# Generate full report
cargo test test_report_generation -- --nocapture
```

**Sample Report Output**:
```
=== Bug Taxonomy Report ===

Total Bugs: 3

Severity Distribution:
  Medium: 2
  Low: 1

Category Distribution:
  Parser: 2
  Performance: 1

Detailed Bug List:

BUG-001. Block expressions not supported (Medium)
   Category: Parser
   Root Cause: MissingFeature
   Impact: Limits expressiveness of generated test programs

BUG-002. Variable lookup overhead (Low)
   Category: Performance
   Root Cause: PerformanceBottleneck
   Impact: Variable-heavy programs have 60x overhead

BUG-003. if-else as rvalue not supported (Medium)
   Category: Parser
   Root Cause: MissingFeature
   Impact: Cannot use if-else in expression positions
```

## Discoveries

### Bug Patterns Identified
1. **Parser Limitations**: 2/3 bugs are parser-related (missing features)
2. **Performance Issues**: 1/3 bugs are performance bottlenecks
3. **Root Causes**: All bugs stem from MissingFeature (2) or PerformanceBottleneck (1)
4. **Severity**: No critical bugs (good!), focus on medium priority items

### BUG-001: Block Expressions Not Supported
- **Discovery Method**: Fuzzing (INTERP-029)
- **Category**: Parser
- **Severity**: Medium
- **Root Cause**: MissingFeature
- **Impact**: Limits expressiveness
- **Reproduction**: `{ let x = 10; x }`
- **Recommendation**: Add block expression support to parser

### BUG-002: Variable Lookup Performance Overhead
- **Discovery Method**: Benchmarking (INTERP-030)
- **Category**: Performance
- **Severity**: Low
- **Root Cause**: PerformanceBottleneck
- **Impact**: 60x overhead vs native (vs 28x for arithmetic)
- **Reproduction**: Run `test_benchmark_vector_ops`
- **Recommendation**: Consider array-based local variable storage

### BUG-003: if-else as rvalue Not Supported
- **Discovery Method**: Integration testing (INTERP-099)
- **Category**: Parser
- **Severity**: Medium
- **Root Cause**: MissingFeature
- **Impact**: Cannot use conditionals in expression positions
- **Reproduction**: `let x = if (cond) { 1 } else { 2 };`
- **Recommendation**: Extend parser to support if-else expressions

## Next Steps

INTERP-033 enables:
- **INTERP-034**: File bugs upstream at paiml/ruchy with comprehensive data
- **INTERP-035**: Export conformance test suite with known limitations documented
- **Prioritization**: Focus on Medium severity parser limitations first

## Validation Summary

- ✅ RED phase: Tests failed as expected (compilation error)
- ✅ GREEN phase: Tests passing with taxonomy infrastructure
- ✅ REFACTOR phase: Report generation and query methods
- ✅ TOOL VALIDATION: All Rust tooling passing
- ✅ REPRODUCIBILITY: Deterministic bug database
- ✅ DEBUGGABILITY: Comprehensive bug reports
- ✅ PATTERN ANALYSIS: 2 parser bugs, 1 performance bug identified

**Status**: 🟢 COMPLETE (7/7 phases validated)

**Taxonomy Statistics**:
- 7 tests implemented
- 7 tests passing
- 0 tests failing
- Bugs tracked: 3
- Severity: 0 Critical, 0 High, 2 Medium, 1 Low
- Categories: 2 Parser, 1 Performance
- Root causes: 2 MissingFeature, 1 PerformanceBottleneck
