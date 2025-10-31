# INTERP-034: Ruchy Compiler Bug Filing (Upstream) - Specification

**Version**: 1.0.0
**Status**: Draft
**Priority**: Critical
**Phase**: RED-GREEN-REFACTOR-TOOL-BUG

## Executive Summary

This specification defines the architecture and implementation requirements for systematically filing discovered bugs from the RuchyRuchy interpreter development to the upstream Ruchy compiler repository at https://github.com/paiml/ruchy/issues.

## Motivation

### Problem Statement

During RuchyRuchy interpreter development (INTERP-001 through INTERP-033), we have systematically executed 212 examples from the Ruchy book and discovered numerous bugs, edge cases, and undefined behaviors in the Ruchy language specification and compiler implementation. However:

1. **Bugs Not Reported**: Discovered bugs remain undocumented in upstream Ruchy repository
2. **No Upstream Feedback**: Ruchy compiler development lacks systematic bug reports
3. **Lost Knowledge**: Bug discoveries isolated to RuchyRuchy project
4. **Quality Gap**: No upstream validation of interpreter findings

### Value Proposition

Systematic bug filing provides:

1. **Improved Ruchy Compiler**: Upstream benefits from systematic testing
2. **Language Specification Clarity**: Edge cases documented and resolved
3. **Community Contribution**: Educational project contributes to production compiler
4. **Validation**: RuchyRuchy findings validated by upstream maintainers
5. **Documentation**: Living record of language evolution

### Success Metrics

- ✅ 50+ bugs filed with GitHub API
- ✅ All bugs have confidence score >0.9
- ✅ 100% include minimal reproduction steps
- ✅ All bugs properly categorized (parser, runtime, type system, etc.)
- ✅ Upstream acknowledgment of reports

## Requirements

### Functional Requirements

#### FR-1: Bug Discovery Analysis

Analyze all interpreter test results to identify:

- **Parser Bugs**: Syntax errors, unexpected parse failures
- **Runtime Bugs**: Incorrect evaluation, crashes, panics
- **Type System Bugs**: Type inference failures, unsound types
- **Standard Library Bugs**: Missing functions, incorrect behavior
- **Documentation Bugs**: Examples that don't work, unclear specs

**Source Data**:
```
tests/test_interp_011_ch01_examples.rs  (Chapter 1: 12 tests)
tests/test_interp_012_ch02_examples.rs  (Chapter 2: 19 tests)
tests/test_interp_013_ch03_examples.rs  (Chapter 3: 15 tests)
tests/test_interp_014_ch04_examples.rs  (Chapter 4: 24 tests)
tests/test_interp_015_ch05_examples.rs  (Chapter 5: 22 tests)
tests/test_interp_016_ch06_examples.rs  (Chapter 6: 28 tests)
tests/test_interp_017_ch10_examples.rs  (Chapter 10: 18 tests)
```

####FR-2: GitHub API Integration

Implement GitHub API client for automated bug filing:

```rust
pub struct GitHubBugFiler {
    client: GitHubClient,
    repo: String,          // "paiml/ruchy"
    dry_run: bool,         // Test mode without actual filing
}

impl GitHubBugFiler {
    /// File a bug report to GitHub
    pub fn file_bug(&self, bug: &BugReport) -> Result<IssueNumber, FilingError> {
        // POST to /repos/paiml/ruchy/issues
        // Returns issue number on success
    }

    /// Check if bug already filed (avoid duplicates)
    pub fn is_duplicate(&self, bug: &BugReport) -> Result<bool, FilingError> {
        // Search existing issues for similar reports
    }
}
```

**GitHub API Requirements**:
- Authentication via `GITHUB_TOKEN` environment variable
- REST API v3 (https://api.github.com)
- Issue creation endpoint: `POST /repos/{owner}/{repo}/issues`
- Issue search endpoint: `GET /repos/{owner}/{repo}/issues`
- Rate limiting: Respect GitHub API rate limits (5000 requests/hour)

#### FR-3: Bug Report Quality

Each bug report must include:

```markdown
## Bug Report: [Short Description]

**Category**: [Parser|Runtime|Type System|Stdlib|Documentation]
**Confidence**: [0.0-1.0, must be >0.9]
**Severity**: [Critical|High|Medium|Low]
**Discovered By**: RuchyRuchy Interpreter v1.10.0

### Minimal Reproduction

\`\`\`ruchy
// Exact code that triggers the bug
[minimal reproduction code]
\`\`\`

### Expected Behavior

[What should happen according to specification]

### Actual Behavior

[What actually happens]

### Environment

- **Ruchy Version**: [output of `ruchy --version`]
- **OS**: Linux/Mac/Windows
- **RuchyRuchy Version**: v1.10.0
- **Test File**: tests/test_interp_XXX_chYY_examples.rs

### Additional Context

[Any relevant context, related bugs, or workarounds]

---
**Filed by**: RuchyRuchy Automated Bug Reporter
**Ticket**: INTERP-034
```

#### FR-4: Confidence Scoring

Calculate confidence score (0.0-1.0) based on:

1. **Reproducibility** (40%):
   - 1.0 = Always reproducible
   - 0.5 = Sometimes reproducible
   - 0.0 = Not reproducible

2. **Minimality** (30%):
   - 1.0 = Minimal reproduction (< 10 lines)
   - 0.5 = Medium reproduction (10-50 lines)
   - 0.0 = Complex reproduction (> 50 lines)

3. **Spec Violation** (20%):
   - 1.0 = Clear spec violation
   - 0.5 = Undefined behavior
   - 0.0 = Opinion/enhancement

4. **Impact** (10%):
   - 1.0 = Compiler crashes or data loss
   - 0.5 = Incorrect behavior
   - 0.0 = Minor inconvenience

**Threshold**: Only file bugs with confidence ≥0.9

#### FR-5: Categorization

Categorize each bug:

- **Parser**: Syntax parsing, tokenization, AST construction
- **Runtime**: Execution, evaluation, interpreter behavior
- **Type System**: Type inference, type checking, generics
- **Standard Library**: Built-in functions, modules
- **Documentation**: Examples, tutorials, specifications
- **Other**: Uncategorized or cross-cutting issues

### Non-Functional Requirements

#### NFR-1: Rate Limiting

- Respect GitHub API rate limits
- Batch filing with delays (1 bug per 5 seconds)
- Monitor rate limit headers
- Implement exponential backoff on failures

#### NFR-2: Idempotence

- Check for duplicate issues before filing
- Use bug fingerprint (hash of reproduction code + error message)
- Skip filing if duplicate found
- Log skipped duplicates

#### NFR-3: Dry Run Mode

- Support `--dry-run` flag for testing
- Print bug reports without filing
- Validate all reports before actual filing
- Generate summary statistics

#### NFR-4: Logging and Tracking

- Log all filed bugs to `docs/interpreter/BUGS_FILED.md`
- Include issue number, URL, timestamp
- Track filing status (success, skipped, failed)
- Generate summary report

## Architecture

### Component Design

```
┌─────────────────────────────────────────────────────────┐
│         RuchyRuchy Interpreter Test Results              │
│         (212 examples, discovered bugs)                  │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
           ┌────────────────────────┐
           │  BugDiscoveryAnalyzer  │
           │  - analyze_test_file() │
           │  - extract_bugs()      │
           │  - calculate_confidence()│
           └────────┬───────────────┘
                    │
                    ▼
     ┌──────────────────────────────────────┐
     │  BugReport (internal representation) │
     │  - category: Category                │
     │  - confidence: f64                   │
     │  - reproduction: String              │
     │  - expected: String                  │
     │  - actual: String                    │
     └──────────┬───────────────────────────┘
                │
                ▼
   ┌────────────────────────────────────────────┐
   │  GitHubBugFiler                            │
   │  - file_bug(bug: &BugReport)               │
   │  - is_duplicate(bug: &BugReport)           │
   │  - generate_issue_body(bug: &BugReport)    │
   └────────────┬───────────────────────────────┘
                │
                ▼
   ┌────────────────────────────────────────────┐
   │  GitHub Issues (paiml/ruchy)               │
   │  - 50+ bugs filed                          │
   │  - Proper categorization                   │
   │  - Minimal reproductions                   │
   └────────────────────────────────────────────┘
```

### Data Flow

1. **Input**: Interpreter test files and execution results
2. **Analysis**: Extract bugs, failures, unexpected behaviors
3. **Scoring**: Calculate confidence scores
4. **Filtering**: Select bugs with confidence ≥0.9
5. **Formatting**: Generate GitHub issue markdown
6. **Deduplication**: Check for existing issues
7. **Filing**: Submit via GitHub API
8. **Tracking**: Log to BUGS_FILED.md

## Implementation Plan

### Phase 1: RED (Failing Tests)

Create `tests/test_bug_filing_001.rs` with failing tests:

```rust
#[test]
fn test_github_api_client_creation() {
    let client = GitHubBugFiler::new("paiml/ruchy");
    assert!(client.is_ok());
}

#[test]
fn test_bug_report_generation() {
    let bug = BugReport {
        category: Category::Parser,
        confidence: 0.95,
        title: "Parser fails on valid if-else syntax".to_string(),
        reproduction: r#"if (true) { 42 } else { 0 }"#.to_string(),
        expected: "Should parse successfully".to_string(),
        actual: "Parse error: unexpected token".to_string(),
    };

    let markdown = bug.to_github_markdown();
    assert!(markdown.contains("## Bug Report"));
    assert!(markdown.contains("**Confidence**: 0.95"));
}

#[test]
fn test_confidence_calculation() {
    let analyzer = BugDiscoveryAnalyzer::new();
    let bug_data = BugData {
        reproducibility: Reproducibility::Always,
        lines_of_code: 5,
        spec_violation: true,
        impact: Impact::Critical,
    };

    let confidence = analyzer.calculate_confidence(&bug_data);
    assert!(confidence >= 0.9);
}

#[test]
fn test_dry_run_mode() {
    let filer = GitHubBugFiler::new("paiml/ruchy")
        .with_dry_run(true);

    let bug = create_test_bug();
    let result = filer.file_bug(&bug);

    assert!(result.is_ok());
    // Should not actually file, just validate
}

#[test]
fn test_duplicate_detection() {
    let filer = GitHubBugFiler::new("paiml/ruchy");
    let bug = create_test_bug();

    // First check should be false (no duplicate)
    assert!(!filer.is_duplicate(&bug).unwrap());

    // After filing, should detect duplicate
    filer.file_bug(&bug).unwrap();
    assert!(filer.is_duplicate(&bug).unwrap());
}
```

### Phase 2: GREEN (Minimal Implementation)

Implement `src/bug_filing/mod.rs`:

```rust
/// Bug filing module for upstream Ruchy compiler
pub mod github_client;
pub mod bug_report;
pub mod discovery;

pub use github_client::GitHubBugFiler;
pub use bug_report::{BugReport, Category};
pub use discovery::BugDiscoveryAnalyzer;
```

Implement `src/bug_filing/bug_report.rs`:

```rust
/// Bug report structure
#[derive(Debug, Clone)]
pub struct BugReport {
    pub category: Category,
    pub confidence: f64,
    pub title: String,
    pub reproduction: String,
    pub expected: String,
    pub actual: String,
    pub test_file: Option<String>,
}

impl BugReport {
    /// Convert to GitHub issue markdown
    pub fn to_github_markdown(&self) -> String {
        format!(
            "## Bug Report: {}\n\n\
             **Category**: {:?}\n\
             **Confidence**: {:.2}\n\n\
             ### Minimal Reproduction\n\n\
             ```ruchy\n{}\n```\n\n\
             ### Expected Behavior\n\n{}\n\n\
             ### Actual Behavior\n\n{}\n",
            self.title,
            self.category,
            self.confidence,
            self.reproduction,
            self.expected,
            self.actual
        )
    }
}
```

### Phase 3: REFACTOR (Optimization)

- Add bug fingerprinting for deduplication
- Implement batch filing with rate limiting
- Add progress reporting
- Optimize API calls

### Phase 4: TOOL (Quality Validation)

- Run all tests
- Run clippy
- Run fmt
- Validate against paiml/ruchy (dry-run mode)

### Phase 5: BUG (Actual Filing)

- **Manual Review**: Review all 50+ bugs before filing
- **Dry Run**: Test with `--dry-run` flag
- **Staged Filing**: File 10 bugs, wait for feedback, continue
- **Tracking**: Update BUGS_FILED.md with all issue numbers

## Test Coverage

### Unit Tests

1. `test_github_client_creation` - Client initialization
2. `test_bug_report_markdown` - Markdown formatting
3. `test_confidence_calculation` - Scoring algorithm
4. `test_category_classification` - Bug categorization

### Integration Tests

1. `test_file_single_bug_dry_run` - Dry run filing
2. `test_duplicate_detection` - Deduplication
3. `test_batch_filing` - Multiple bugs
4. `test_rate_limiting` - API rate limits

### Acceptance Tests

1. Dry run files 50+ bugs without errors
2. All bugs have confidence >0.9
3. All bugs include reproduction steps
4. GitHub API responds successfully (in dry-run mode)

## Example Bug Reports

### Example 1: Parser Bug

```markdown
## Bug Report: Parser fails on if-else without braces

**Category**: Parser
**Confidence**: 0.95
**Severity**: High
**Discovered By**: RuchyRuchy Interpreter v1.10.0

### Minimal Reproduction

\`\`\`ruchy
if (true) 42 else 0
\`\`\`

### Expected Behavior

Should parse successfully (braces optional for single expressions).

### Actual Behavior

Parse error: unexpected token 'else'

### Environment

- **Ruchy Version**: 0.1.0
- **OS**: Linux
- **RuchyRuchy Version**: v1.10.0
- **Test File**: tests/test_interp_014_ch04_examples.rs

### Additional Context

Discovered during Chapter 4 example execution. The Ruchy book shows this syntax
as valid, but the compiler rejects it.

---
**Filed by**: RuchyRuchy Automated Bug Reporter
**Ticket**: INTERP-034
```

### Example 2: Runtime Bug

```markdown
## Bug Report: println crashes on empty string

**Category**: Runtime
**Confidence**: 1.0
**Severity**: Critical
**Discovered By**: RuchyRuchy Interpreter v1.10.0

### Minimal Reproduction

\`\`\`ruchy
println("")
\`\`\`

### Expected Behavior

Should print empty line and return successfully.

### Actual Behavior

Runtime panic: thread 'main' panicked at 'index out of bounds'

### Environment

- **Ruchy Version**: 0.1.0
- **OS**: Linux
- **RuchyRuchy Version**: v1.10.0
- **Test File**: tests/test_interp_011_ch01_examples.rs

### Additional Context

This is a critical bug that crashes the compiler on trivial input.

---
**Filed by**: RuchyRuchy Automated Bug Reporter
**Ticket**: INTERP-034
```

## Success Criteria

1. ✅ 50+ bugs filed to https://github.com/paiml/ruchy/issues
2. ✅ All bugs have confidence score ≥0.9
3. ✅ 100% include minimal reproduction code
4. ✅ All bugs properly categorized
5. ✅ docs/interpreter/BUGS_FILED.md tracking document created
6. ✅ Zero duplicate issues filed
7. ✅ All filed via GitHub API (not manual)
8. ✅ PMAT TDG ≥85.0 for all code

## References

1. **Zimmermann, T. et al.** (2012). "What Makes a Good Bug Report?" *IEEE Transactions on Software Engineering*.
2. **Bettenburg, N. et al.** (2008). "What Makes a Good Bug Report?" *SIGSOFT FSE*.
3. **GitHub API Documentation**: https://docs.github.com/en/rest
4. **Ruchy Repository**: https://github.com/paiml/ruchy

---

**Document Version**: 1.0.0
**Last Updated**: 2025-10-31
**Status**: Draft → Review → Approved
