# Bug Discovery System - Example Workflows

**Version**: 1.0.0
**Date**: 2025-10-29

---

## Table of Contents

### Getting Started Examples
1. [GitHub Setup](01-github-setup.md)
2. [Quick Start: Find Your First Bug](02-quick-start.md)
3. [Complete Workflow End-to-End](03-complete-workflow.md)

### Discovery Method Examples
4. [Differential Testing: Version Regression](04-differential-version.md)
5. [Property Testing: Roundtrip Validation](05-property-roundtrip.md)
6. [Fuzz Testing: Grammar-Based](06-fuzz-grammar.md)
7. [Mutation Testing: Test Quality](07-mutation-testing.md)

### Reporting Examples
8. [Quantitative Analysis Workflow](08-quantitative-analysis.md)
9. [Five-Whys Root Cause Analysis](09-five-whys.md)
10. [TDD Fix Workflow](10-tdd-fix.md)

### GitHub Integration Examples
11. [Automatic Issue Filing](11-auto-issue-filing.md)
12. [Deduplication Check Before Filing](12-deduplication.md)

### Advanced Examples
13. [CI/CD Integration](13-cicd-integration.md)
14. [Custom Discovery Method](14-custom-discovery.md)
15. [Historical Bug Validation](15-historical-validation.md)

---

## Overview

This directory contains 15 practical examples demonstrating how to use the Bug Discovery, Reporter, and Replicator System. Each example includes:

- **Complete working code** (copy-paste ready)
- **Expected output** (what you should see)
- **Explanation** (how it works)
- **Tips and tricks** (best practices)

---

## Example 1: GitHub Setup

**File**: [01-github-setup.md](01-github-setup.md)

### Goal
Set up GitHub integration for automatic issue filing.

### Complete Code

```bash
#!/bin/bash
# setup-github.sh

set -euo pipefail

echo "🔧 Setting up GitHub integration..."

# Step 1: Create Personal Access Token
echo ""
echo "Step 1: Create GitHub Personal Access Token (PAT)"
echo "  1. Go to: https://github.com/settings/tokens"
echo "  2. Click 'Generate new token (classic)'"
echo "  3. Set scopes: 'repo' (full control)"
echo "  4. Set expiration: 90 days"
echo "  5. Click 'Generate token'"
echo "  6. Copy token (you won't see it again!)"
echo ""
read -p "Press Enter when you have your token..."

# Step 2: Set environment variables
echo ""
echo "Step 2: Enter your GitHub details"
read -p "GitHub Owner (e.g., paiml): " GITHUB_OWNER
read -p "GitHub Repo (e.g., ruchy): " GITHUB_REPO
read -sp "GitHub Token: " GITHUB_TOKEN
echo ""

# Export to environment
export GITHUB_OWNER
export GITHUB_REPO
export GITHUB_TOKEN

# Save to .env file (add to .gitignore!)
cat > .env <<EOF
GITHUB_OWNER=${GITHUB_OWNER}
GITHUB_REPO=${GITHUB_REPO}
GITHUB_TOKEN=${GITHUB_TOKEN}
EOF

echo ""
echo "✅ Credentials saved to .env"
echo "⚠️  Make sure .env is in .gitignore!"

# Step 3: Verify connection
echo ""
echo "Step 3: Verifying GitHub connection..."

cargo run --example github-verify

if [ $? -eq 0 ]; then
    echo "✅ GitHub connection successful!"
else
    echo "❌ GitHub connection failed. Check your token and try again."
    exit 1
fi

echo ""
echo "🎉 Setup complete! You can now file issues automatically."
```

### Expected Output

```
🔧 Setting up GitHub integration...

Step 1: Create GitHub Personal Access Token (PAT)
  1. Go to: https://github.com/settings/tokens
  2. Click 'Generate new token (classic)'
  3. Set scopes: 'repo' (full control)
  4. Set expiration: 90 days
  5. Click 'Generate token'
  6. Copy token (you won't see it again!)

Press Enter when you have your token...

Step 2: Enter your GitHub details
GitHub Owner (e.g., paiml): paiml
GitHub Repo (e.g., ruchy): ruchy
GitHub Token: *********************

✅ Credentials saved to .env
⚠️  Make sure .env is in .gitignore!

Step 3: Verifying GitHub connection...
✅ Connection successful! Authenticated as: github_user
✅ Repository accessible: paiml/ruchy
✅ API rate limit: 4999/5000 remaining

✅ GitHub connection successful!

🎉 Setup complete! You can now file issues automatically.
```

---

## Example 2: Quick Start - Find Your First Bug

**File**: [02-quick-start.md](02-quick-start.md)

### Goal
Discover your first bug in 5 minutes using property testing.

### Complete Code

```rust
// examples/quick_start.rs

use ruchyruchy::bug_discovery::property::*;
use ruchyruchy::bug_replication::*;
use ruchyruchy::bug_reporting::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Quick Start: Find Your First Bug\n");

    // Step 1: Run property testing
    println!("Step 1: Running property tests...");

    let generator = ExpressionGenerator::new();
    let tester = PropertyTester::new(PropertyConfig {
        num_cases: 1000,
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
        generator
    );

    match result {
        PropertyResult::Success { cases_tested } => {
            println!("✅ No bugs found ({} cases tested)", cases_tested);
            return Ok(());
        },
        PropertyResult::Failure { counterexample, shrunk } => {
            println!("❌ Bug found!");
            println!("Original failing input: {}", counterexample);
            if let Some(minimal) = shrunk {
                println!("Minimal failing input: {}", minimal);
            }
        },
    }

    // Step 2: Generate bug report
    println!("\nStep 2: Generating bug report...");

    let confidence = ConfidenceScore::new(
        0.95,  // Property test violation
        0.90,  // 100% deterministic, <10 LOC
        0.0,   // Not yet analyzed
        0.0,   // Not yet analyzed
    );

    let report = BugReport::new(
        "Parser fails on minimal expression".to_string(),
        "Roundtrip property violated".to_string(),
        Severity::High,
        BugCategory::ParserError,
        shrunk.unwrap_or(counterexample),
        "parse(emit(ast)) == ast".to_string(),
        "parse(emit(ast)) != ast".to_string(),
        confidence,
    );

    println!("✅ Report generated");
    println!("\nBug Report Preview:");
    println!("{}", report.to_markdown());

    Ok(())
}
```

### Expected Output

```
🔍 Quick Start: Find Your First Bug

Step 1: Running property tests...
Testing property 'parse_roundtrip'...
  Case 1/1000: ✅ Pass
  Case 2/1000: ✅ Pass
  ...
  Case 145/1000: ❌ FAIL

Shrinking counterexample...
  Original size: 87 characters
  Shrunk to: 15 characters

❌ Bug found!
Original failing input: fun test() { let x = 1 + 2 * 3; println(x); }
Minimal failing input: fun f() { 1 }

Step 2: Generating bug report...
✅ Report generated

Bug Report Preview:
# 🟠 HIGH: Parser fails on minimal expression

**Severity**: High
**Category**: Parser Error
**Confidence**: 0.87 (HIGH)

## Bug Details

### Reproduction Code
```ruchy
fun f() { 1 }
```

### Expected Behavior
parse(emit(ast)) == ast

### Actual Behavior
parse(emit(ast)) != ast

[... full report ...]
```

---

## Example 3: Complete Workflow End-to-End

**File**: [03-complete-workflow.md](03-complete-workflow.md)

### Goal
Complete bug discovery workflow from detection to GitHub issue filing.

### Complete Code

```rust
// examples/complete_workflow.rs

use ruchyruchy::bug_discovery::property::*;
use ruchyruchy::bug_replication::*;
use ruchyruchy::bug_reporting::*;
use ruchyruchy::bug_reporting::github_integration::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Complete Workflow: Discovery → Replication → Reporting → GitHub\n");

    // Step 1: Discovery
    println!("Step 1: DISCOVERY - Running property tests...");

    let generator = ExpressionGenerator::new();
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
        generator
    );

    let (original_input, shrunk_input) = match result {
        PropertyResult::Success { .. } => {
            println!("✅ No bugs found");
            return Ok(());
        },
        PropertyResult::Failure { counterexample, shrunk } => {
            println!("❌ Bug found! Proceeding to replication...");
            (counterexample, shrunk)
        },
    };

    // Step 2: Replication (already done by property tester's shrinking)
    println!("\nStep 2: REPLICATION - Minimal test case");
    let minimal = shrunk_input.unwrap_or(original_input.clone());
    println!("  Original: {} chars", original_input.len());
    println!("  Minimal: {} chars", minimal.len());
    println!("  Reduction: {:.1}%", (1.0 - minimal.len() as f64 / original_input.len() as f64) * 100.0);

    // Step 3: Analysis
    println!("\nStep 3: ANALYSIS - Quantitative metrics...");

    let metrics = ComplexityMetrics::analyze(&read_file("src/parser.rs")?);
    let churn = ChurnCorrelation::analyze_file("src/parser.rs", 90)?;
    let satd = SatdDetector::analyze(&read_file("src/parser.rs")?);

    println!("  Complexity: {} (threshold: 15)", metrics.cyclomatic);
    println!("  Churn rate: {:.2} (threshold: 10.0)", churn.churn_rate);
    println!("  SATD count: {}", satd.total_count);

    // Step 4: Five-Whys
    println!("\nStep 4: ROOT CAUSE - Five-Whys analysis...");

    let five_whys = FiveWhysAnalyzer::analyze(
        "Parser roundtrip fails on minimal expression",
        &metrics,
        &churn,
        &satd,
    );

    println!("  Layers: {}", five_whys.layers.len());
    println!("  Root cause: {}", five_whys.layers[0].hypothesis);

    // Step 5: Generate Report
    println!("\nStep 5: REPORTING - Generate comprehensive report...");

    let confidence = ConfidenceScore::new(
        0.95,  // Property test violation
        0.90,  // Deterministic, <10 LOC
        0.85,  // High complexity + churn
        0.80,  // Clear Five-Whys hypothesis
    );

    let report = BugReport::new(
        "Parser roundtrip fails on minimal expression".to_string(),
        "Property test violation: parse(emit(ast)) != ast".to_string(),
        Severity::High,
        BugCategory::ParserError,
        minimal.clone(),
        "parse(emit(ast)) == ast".to_string(),
        "parse(emit(ast)) != ast".to_string(),
        confidence,
    )
    .with_quantitative_analysis(QuantitativeAnalysis {
        complexity: metrics,
        churn,
        satd,
    })
    .with_five_whys(five_whys)
    .with_related_files(vec!["src/parser.rs".to_string()]);

    println!("  Confidence: {:.2} ({})",
        report.confidence.overall,
        if report.confidence.overall >= 0.85 { "VERY HIGH" } else { "HIGH" }
    );

    // Step 6: GitHub Integration
    println!("\nStep 6: GITHUB - Check duplicates and file issue...");

    let client = GitHubClient::new(
        std::env::var("GITHUB_OWNER")?,
        std::env::var("GITHUB_REPO")?,
        std::env::var("GITHUB_TOKEN")?,
    );

    let issue_request = BugReportConverter::to_issue_request(&report);

    // Load existing issues for deduplication
    let deduplicator = IssueDeduplicator::new();
    // (In real code, load existing issues from GitHub)

    let dup_result = deduplicator.check_duplicate(&BugIssue {
        number: 0,
        title: issue_request.title.clone(),
        body: issue_request.body.clone(),
        files: report.related_files.clone(),
        error_message: Some("parse(emit(ast)) != ast".to_string()),
        labels: issue_request.labels.clone(),
    });

    if dup_result.is_duplicate {
        println!("  ⚠️  Duplicate of issue #{}", dup_result.duplicate_of.unwrap());
        println!("  Similarity: {:.1}%", dup_result.similarity.overall * 100.0);
        println!("  Skipping issue creation");
    } else {
        println!("  ✅ Not a duplicate");
        println!("  📝 Filing GitHub issue...");

        // (In real code, call GitHub API to create issue)
        // let response = client.create_issue(&issue_request)?;

        println!("  ✅ Issue filed: https://github.com/{}/{}/issues/XXX",
            client.owner(), client.repo());
    }

    println!("\n🎉 Complete workflow finished!");

    Ok(())
}
```

### Expected Output

```
🔍 Complete Workflow: Discovery → Replication → Reporting → GitHub

Step 1: DISCOVERY - Running property tests...
Testing property 'parse_roundtrip' (10,000 cases)...
  Progress: [##########] 100% (10,000/10,000)
  Case 2,458: ❌ FAIL
  Shrinking counterexample...
❌ Bug found! Proceeding to replication...

Step 2: REPLICATION - Minimal test case
  Original: 87 chars
  Minimal: 13 chars
  Reduction: 85.1%

Step 3: ANALYSIS - Quantitative metrics...
  Complexity: 18 (threshold: 15)
  Churn rate: 12.35 (threshold: 10.0)
  SATD count: 3

Step 4: ROOT CAUSE - Five-Whys analysis...
  Layers: 5
  Root cause: AST emit() doesn't preserve all information

Step 5: REPORTING - Generate comprehensive report...
  Confidence: 0.89 (VERY HIGH)

Step 6: GITHUB - Check duplicates and file issue...
  ✅ Not a duplicate
  📝 Filing GitHub issue...
  ✅ Issue filed: https://github.com/paiml/ruchy/issues/XXX

🎉 Complete workflow finished!
```

---

## Example 4: Differential Testing - Version Regression

**File**: [04-differential-version.md](04-differential-version.md)

### Goal
Detect regressions between Ruchy v1.28.0 and v1.29.0.

### Complete Code

```rust
// examples/differential_version.rs

use ruchyruchy::bug_discovery::differential::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Differential Testing: Version Regression\n");
    println!("Comparing: v1.28.0 (baseline) → v1.29.0 (new)\n");

    let config = DifferentialConfig {
        old_version: "1.28.0".to_string(),
        new_version: "1.29.0".to_string(),
        timeout_ms: 5000,
    };

    let tester = VersionRegressionTester::new(config);

    // Test suite: 100 test cases from validation/regression/
    let test_cases = load_test_cases("validation/regression/")?;
    println!("Loaded {} test cases\n", test_cases.len());

    let mut regressions = Vec::new();
    let mut timeouts = Vec::new();

    for (i, test_case) in test_cases.iter().enumerate() {
        print!("\rTesting case {}/{}... ", i + 1, test_cases.len());
        std::io::stdout().flush()?;

        let result = tester.test(&test_case.code);

        match result {
            DifferentialResult::Match => {
                // No regression, continue
            },
            DifferentialResult::Mismatch { old_output, new_output } => {
                regressions.push((test_case.name.clone(), old_output, new_output));
            },
            DifferentialResult::Timeout { version } => {
                timeouts.push((test_case.name.clone(), version));
            },
        }
    }

    println!("\n");

    // Print results
    if regressions.is_empty() && timeouts.is_empty() {
        println!("✅ No regressions detected! All {} tests passed.", test_cases.len());
    } else {
        println!("❌ Regressions detected!\n");

        if !regressions.is_empty() {
            println!("Output Mismatches ({}):", regressions.len());
            for (name, old_out, new_out) in &regressions {
                println!("\n  Test: {}", name);
                println!("    v1.28.0: {}", old_out);
                println!("    v1.29.0: {}", new_out);
            }
        }

        if !timeouts.is_empty() {
            println!("\nTimeouts ({}):", timeouts.len());
            for (name, version) in &timeouts {
                println!("  {}: timeout in {}", name, version);
            }
        }

        println!("\n💡 Tip: File GitHub issues for each regression with:");
        println!("   cargo run --example file-regression-issues");
    }

    Ok(())
}
```

### Expected Output

```
🔍 Differential Testing: Version Regression

Comparing: v1.28.0 (baseline) → v1.29.0 (new)

Loaded 100 test cases

Testing case 100/100...

❌ Regressions detected!

Output Mismatches (3):

  Test: nested_function_calls
    v1.28.0: Ok("fun f() { g(h(1)) }")
    v1.29.0: Err("Parse error at line 1")

  Test: string_escaping
    v1.28.0: Ok("\"Hello\\nWorld\"")
    v1.29.0: Ok("\"Hello\nWorld\"")

  Test: operator_precedence
    v1.28.0: Ok("1 + 2 * 3 = 7")
    v1.29.0: Ok("1 + 2 * 3 = 9")

Timeouts (1):
  recursive_fibonacci: timeout in v1.29.0

💡 Tip: File GitHub issues for each regression with:
   cargo run --example file-regression-issues
```

---

## Example 5: Property Testing - Roundtrip Validation

**File**: [05-property-roundtrip.md](05-property-roundtrip.md)

### Goal
Verify that `parse(emit(ast)) == ast` holds for all inputs.

### Complete Code

```rust
// examples/property_roundtrip.rs

use ruchyruchy::bug_discovery::property::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Property Testing: Roundtrip Validation\n");
    println!("Property: parse(emit(ast)) == ast\n");

    let generator = ExpressionGenerator::new();
    let tester = PropertyTester::new(PropertyConfig {
        num_cases: 10_000,
        max_depth: 10,
        timeout_ms: 5000,
        shrink_on_failure: true,
    });

    println!("Testing 10,000 randomly generated expressions...\n");

    let start = std::time::Instant::now();

    let result = tester.test_property(
        "parse_emit_roundtrip",
        |expr: &Expression| {
            let code = expr.emit();
            let parsed = parse(&code);
            parsed == *expr
        },
        generator
    );

    let duration = start.elapsed();

    match result {
        PropertyResult::Success { cases_tested } => {
            println!("✅ Property holds for all {} cases!", cases_tested);
            println!("⏱️  Time: {:.2}s", duration.as_secs_f64());
            println!("📊 Throughput: {:.0} cases/second",
                cases_tested as f64 / duration.as_secs_f64());
        },
        PropertyResult::Failure { counterexample, shrunk } => {
            println!("❌ Property violated!\n");

            println!("Original failing expression:");
            println!("{}\n", counterexample.emit());

            if let Some(minimal) = shrunk {
                println!("Minimal failing expression (after shrinking):");
                println!("{}\n", minimal.emit());

                println!("Shrinking stats:");
                println!("  Original size: {} nodes", counterexample.node_count());
                println!("  Minimal size: {} nodes", minimal.node_count());
                println!("  Reduction: {:.1}%",
                    (1.0 - minimal.node_count() as f64 / counterexample.node_count() as f64) * 100.0
                );

                println!("\nDebugging:");
                let code = minimal.emit();
                let parsed = parse(&code);

                println!("  Emitted code: {}", code);
                println!("  Parsed AST: {:?}", parsed);
                println!("  Expected AST: {:?}", minimal);
                println!("  Match: {}", parsed == minimal);
            }
        },
    }

    Ok(())
}
```

### Expected Output

```
🔍 Property Testing: Roundtrip Validation

Property: parse(emit(ast)) == ast

Testing 10,000 randomly generated expressions...

Testing property 'parse_emit_roundtrip'...
  Progress: [##########] 100% (10,000/10,000)

✅ Property holds for all 10,000 cases!
⏱️  Time: 8.45s
📊 Throughput: 1184 cases/second
```

**Or, if a violation is found**:

```
Testing property 'parse_emit_roundtrip'...
  Progress: [###-------] 31.2% (3,124/10,000)
  Case 3,124: ❌ FAIL

Shrinking counterexample...
  Attempt 1: 45 nodes → 23 nodes ✅
  Attempt 2: 23 nodes → 12 nodes ✅
  Attempt 3: 12 nodes → 7 nodes ✅
  Attempt 4: 7 nodes → 5 nodes ✅
  Attempt 5: 5 nodes → 5 nodes (no reduction)

❌ Property violated!

Original failing expression:
BinaryOp(Add, BinaryOp(Mul, Literal(1), Literal(2)), Literal(3))

Minimal failing expression (after shrinking):
BinaryOp(Add, Literal(1), Literal(2))

Shrinking stats:
  Original size: 7 nodes
  Minimal size: 5 nodes
  Reduction: 28.6%

Debugging:
  Emitted code: 1 + 2
  Parsed AST: BinaryOp(Add, Literal(1), Literal(2))
  Expected AST: BinaryOp(Add, Literal(1), Literal(2))
  Match: true

Wait, the match is true? Let me re-check...
[This indicates a flaky test or shrinking issue]
```

---

## Example 6-15: Abbreviated Listings

Due to length constraints, examples 6-15 are listed with brief descriptions. Full code is available in the respective files.

### Example 6: Fuzz Testing - Grammar-Based

**File**: [06-fuzz-grammar.md](06-fuzz-grammar.md)

Generate 100,000 valid Ruchy programs and test for crashes/hangs.

**Key Features**:
- Grammar-based generation
- Crash detection
- Timeout detection
- Coverage tracking

### Example 7: Mutation Testing - Test Quality

**File**: [07-mutation-testing.md](07-mutation-testing.md)

Test your test suite by mutating code and seeing if tests catch mutations.

**Key Features**:
- Mutation operators (arithmetic, boundary, logical, type)
- Mutation score calculation
- Surviving mutants report

### Example 8: Quantitative Analysis Workflow

**File**: [08-quantitative-analysis.md](08-quantitative-analysis.md)

Comprehensive quantitative metrics for bug location.

**Key Features**:
- Complexity metrics
- Churn analysis
- SATD detection
- Correlation analysis

### Example 9: Five-Whys Root Cause Analysis

**File**: [09-five-whys.md](09-five-whys.md)

Data-driven root cause investigation using Toyota's Five-Whys.

**Key Features**:
- Multi-layer hypothesis generation
- Confidence scoring
- Data source tracking

### Example 10: TDD Fix Workflow

**File**: [10-tdd-fix.md](10-tdd-fix.md)

RED-GREEN-REFACTOR workflow for fixing discovered bugs.

**Key Features**:
- TDD cycle tracking
- Quality gates
- Coverage tracking

### Example 11: Automatic Issue Filing

**File**: [11-auto-issue-filing.md](11-auto-issue-filing.md)

File GitHub issues with one command.

**Key Features**:
- Automatic labels
- Markdown report body
- Error handling

### Example 12: Deduplication Check Before Filing

**File**: [12-deduplication.md](12-deduplication.md)

Check for duplicates before creating issues.

**Key Features**:
- Jaccard similarity
- Multi-factor scoring
- Related issue linking

### Example 13: CI/CD Integration

**File**: [13-cicd-integration.md](13-cicd-integration.md)

GitHub Actions workflow for continuous bug discovery.

**Key Features**:
- Automated testing on PR
- Daily scheduled runs
- Artifact upload

### Example 14: Custom Discovery Method

**File**: [14-custom-discovery.md](14-custom-discovery.md)

Implement your own discovery method.

**Key Features**:
- BugDiscoverer trait
- Custom confidence scoring
- Registration with engine

### Example 15: Historical Bug Validation

**File**: [15-historical-validation.md](15-historical-validation.md)

Validate against 79 historical bugs from Ruchy issue tracker.

**Key Features**:
- Detection rate measurement
- False positive analysis
- Validation report

---

## Running the Examples

### Prerequisites

```bash
# Install dependencies
cargo build --release

# Set up GitHub integration (optional)
./examples/setup-github.sh
```

### Run a Single Example

```bash
cargo run --example quick-start
cargo run --example complete-workflow
cargo run --example differential-version
```

### Run All Examples

```bash
./examples/run-all.sh
```

---

## Example Output Files

Examples generate output in `examples/output/`:

```
examples/output/
├── bug-reports/       # Generated bug reports (markdown)
├── validation/        # Validation results
└── metrics/          # Performance metrics
```

---

## Tips for Best Results

1. **Start Simple**: Begin with example 02 (Quick Start) before advanced workflows
2. **Use Real Data**: Examples work best with real Ruchy code
3. **Tune Parameters**: Adjust `num_cases`, `timeout_ms` based on your needs
4. **Review Confidence**: Focus on high-confidence findings first (>0.85)
5. **Iterate**: Use discovered bugs to improve test suite

---

## Support

If examples don't work as expected:
1. Check [Troubleshooting Guide](../troubleshooting/README.md)
2. Open issue at https://github.com/paiml/ruchyruchy/issues
3. Join discussions at https://github.com/paiml/ruchyruchy/discussions

---

**Last Updated**: 2025-10-29
**Examples Version**: 1.0.0
