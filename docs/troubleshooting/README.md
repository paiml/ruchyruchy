# Bug Discovery System - Troubleshooting Guide

**Version**: 1.0.0
**Date**: 2025-10-29

---

## Table of Contents

### Common Issues
1. [Installation Problems](#1-installation-problems)
2. [GitHub Integration Issues](#2-github-integration-issues)
3. [Discovery Method Failures](#3-discovery-method-failures)
4. [Performance Problems](#4-performance-problems)
5. [Report Generation Errors](#5-report-generation-errors)

### Discovery-Specific Issues
6. [Property Testing Issues](#6-property-testing-issues)
7. [Fuzz Testing Issues](#7-fuzz-testing-issues)
8. [Mutation Testing Issues](#8-mutation-testing-issues)
9. [Differential Testing Issues](#9-differential-testing-issues)

### Integration Issues
10. [CI/CD Integration Problems](#10-cicd-integration-problems)
11. [False Positives](#11-false-positives)
12. [False Negatives](#12-false-negatives)

---

## 1. Installation Problems

### Issue: `cargo build` fails with dependency errors

**Error**:
```
error: failed to resolve dependencies
  package 'ruchyruchy' depends on 'some-crate' but it is not found
```

**Solution**:
```bash
# Update Rust toolchain
rustup update stable

# Clean and rebuild
cargo clean
cargo build --release

# If still failing, update Cargo.lock
rm Cargo.lock
cargo build --release
```

---

### Issue: Rust version too old

**Error**:
```
error: package requires rustc 1.70.0 or later
```

**Solution**:
```bash
# Check current version
rustc --version

# Update to latest stable
rustup update stable
rustup default stable

# Verify update
rustc --version
# Should show: rustc 1.70.0 or later
```

---

### Issue: Tests fail with "command not found: ruchy"

**Error**:
```
error: command not found: ruchy
```

**Solution**:
This system tests Ruchy code but doesn't require the Ruchy compiler for the core functionality. If you're running validation tests that need Ruchy:

```bash
# Install Ruchy compiler
cargo install ruchy

# Verify installation
ruchy --version

# Add to PATH if needed
export PATH="$HOME/.cargo/bin:$PATH"
```

---

## 2. GitHub Integration Issues

### Issue: "401 Unauthorized" when filing issues

**Error**:
```
Error: GitHubApiError("401 Unauthorized")
```

**Cause**: Invalid or expired GitHub token

**Solution**:
```bash
# Step 1: Create new PAT at https://github.com/settings/tokens
# Required scopes: 'repo' (full control)

# Step 2: Update environment variable
export GITHUB_TOKEN="ghp_new_token_here"

# Step 3: Verify
cargo run --example github-verify
```

**Permanent Solution**:
Add to `~/.bashrc` or `~/.zshrc`:
```bash
export GITHUB_TOKEN="ghp_your_token"
export GITHUB_OWNER="paiml"
export GITHUB_REPO="ruchy"
```

---

### Issue: Rate limit exceeded

**Error**:
```
Error: GitHubApiError("403 Forbidden - Rate limit exceeded")
```

**Cause**: Exceeded 5000 requests/hour limit

**Solution**:
```bash
# Check rate limit status
curl -H "Authorization: Bearer $GITHUB_TOKEN" \
  https://api.github.com/rate_limit

# Output shows:
# {
#   "resources": {
#     "core": {
#       "limit": 5000,
#       "remaining": 0,
#       "reset": 1635724800  # Unix timestamp
#     }
#   }
# }

# Wait until reset time or:
# 1. Enable caching (reduces API calls)
# 2. Batch operations
# 3. Use multiple tokens (rotate)
```

**Prevention**:
```rust
// Enable caching in your code
let client = GitHubClient::new(/* ... */)
    .with_cache(true)
    .with_cache_ttl(3600); // 1 hour
```

---

### Issue: "Repository not found" or "404 Not Found"

**Error**:
```
Error: GitHubApiError("404 Not Found")
```

**Causes**:
1. Wrong repository name
2. Private repository without proper permissions
3. Token doesn't have 'repo' scope

**Solution**:
```bash
# Verify repository exists
curl https://api.github.com/repos/$GITHUB_OWNER/$GITHUB_REPO

# If private, verify token has access
curl -H "Authorization: Bearer $GITHUB_TOKEN" \
  https://api.github.com/repos/$GITHUB_OWNER/$GITHUB_REPO

# If 404, check:
echo "Owner: $GITHUB_OWNER"
echo "Repo: $GITHUB_REPO"
# Should be: paiml / ruchy (not: paiml/ruchy)
```

---

## 3. Discovery Method Failures

### Issue: Property tests timeout

**Error**:
```
Error: Timeout after 5000ms
Property: parse_roundtrip
Case: 1234/10000
```

**Cause**: Test case too complex or infinite loop

**Solution 1**: Increase timeout
```rust
let tester = PropertyTester::new(PropertyConfig {
    num_cases: 10_000,
    max_depth: 10,
    timeout_ms: 10_000, // Increased from 5000
    shrink_on_failure: true,
});
```

**Solution 2**: Reduce max_depth
```rust
let tester = PropertyTester::new(PropertyConfig {
    num_cases: 10_000,
    max_depth: 5, // Reduced from 10
    timeout_ms: 5000,
    shrink_on_failure: true,
});
```

**Solution 3**: Add timeout handling
```rust
let result = tester.test_property_with_timeout(
    "parse_roundtrip",
    property_fn,
    generator,
    Duration::from_secs(60), // Overall timeout
);
```

---

### Issue: Too many false positives

**Error**:
```
Discovery found 500 bugs, but 450 are false positives
False positive rate: 90%
```

**Cause**: Confidence threshold too low

**Solution**:
```rust
// Filter by confidence before reporting
let high_confidence_bugs: Vec<_> = findings
    .iter()
    .filter(|f| f.confidence.overall >= 0.85) // Only very high confidence
    .collect();

println!("High-confidence bugs: {}", high_confidence_bugs.len());
```

**Recommended Thresholds**:
- **Production**: ≥0.85 (very high confidence only)
- **Development**: ≥0.70 (high confidence)
- **Research**: ≥0.50 (medium confidence)

---

### Issue: No bugs found (false negatives)

**Error**:
```
Ran 10,000 test cases
Bugs found: 0
```

**Possible Causes**:
1. Test suite too small
2. Max depth too shallow
3. Generator doesn't cover bug-triggering inputs

**Solution 1**: Increase test cases
```rust
let config = PropertyConfig {
    num_cases: 100_000, // Increased from 10,000
    max_depth: 15,      // Increased from 10
    timeout_ms: 5000,
    shrink_on_failure: true,
};
```

**Solution 2**: Use multiple discovery methods
```rust
// Combine property testing + fuzz testing
let property_findings = run_property_tests()?;
let fuzz_findings = run_fuzz_tests()?;
let mutation_findings = run_mutation_tests()?;

let all_findings: Vec<_> = property_findings
    .into_iter()
    .chain(fuzz_findings)
    .chain(mutation_findings)
    .collect();

println!("Total findings: {}", all_findings.len());
```

**Solution 3**: Review historical bugs
```rust
// Validate against known bugs to check detection
let validator = BugCorpusValidator::new(historical_bugs);
let report = validator.validate(|bug| {
    run_discovery_on_bug(bug)
});

if report.metrics.detection_rate < 0.95 {
    println!("⚠️  Low detection rate: {:.1}%", report.metrics.detection_rate * 100.0);
    println!("   Review discovery method configuration");
}
```

---

## 4. Performance Problems

### Issue: Discovery is too slow

**Problem**:
```
Property testing: 2.5 hours for 10,000 cases
Expected: <10 minutes
```

**Diagnosis**:
```rust
// Add timing to each test case
let start = Instant::now();
for (i, case) in test_cases.iter().enumerate() {
    let case_start = Instant::now();
    let result = test(case);
    let case_duration = case_start.elapsed();

    if case_duration > Duration::from_millis(100) {
        println!("⚠️  Slow test case #{}: {:.2}s", i, case_duration.as_secs_f64());
    }
}
let total_duration = start.elapsed();
println!("Total: {:.2}s", total_duration.as_secs_f64());
println!("Average: {:.2}ms/case", total_duration.as_millis() as f64 / test_cases.len() as f64);
```

**Solution 1**: Parallel execution
```rust
use rayon::prelude::*;

let results: Vec<_> = test_cases
    .par_iter()
    .map(|case| test(case))
    .collect();
```

**Solution 2**: Reduce test cases but increase coverage
```rust
let config = PropertyConfig {
    num_cases: 1_000,    // Reduced from 10,000
    max_depth: 15,       // But increased depth
    timeout_ms: 1000,    // Tighter timeout
    shrink_on_failure: true,
};
```

**Solution 3**: Enable incremental testing
```bash
# Only test changed files
cargo run --bin discover -- \
  --incremental \
  --git-diff main \
  --confidence-threshold 0.85
```

---

### Issue: Out of memory during fuzzing

**Error**:
```
Error: OutOfMemory
Killed by OOM killer
```

**Cause**: Fuzzer generates too many large inputs

**Solution 1**: Limit input size
```rust
let fuzzer = GrammarFuzzer::new(FuzzConfig {
    max_depth: 10,         // Limit nesting
    timeout_ms: 1000,
    num_iterations: 100_000,
})
.with_max_input_size(10_000); // Max 10KB per input
```

**Solution 2**: Stream inputs instead of storing
```rust
// Don't do this:
let mut all_inputs = Vec::new();
for _ in 0..100_000 {
    let input = fuzzer.generate();
    all_inputs.push(input); // Memory grows!
}

// Do this:
for _ in 0..100_000 {
    let input = fuzzer.generate();
    test(&input);
    // Input dropped after use
}
```

**Solution 3**: Use memory limits
```bash
# Linux: limit memory to 4GB
ulimit -v 4194304
cargo run --bin fuzz
```

---

## 5. Report Generation Errors

### Issue: Markdown report has formatting errors

**Problem**:
```markdown
# Bug Report

## Expected
fun main() { println("Hello"); }

## Actual
fun main() { println("Hello"); }  # Missing closing quote!
```

**Cause**: Unescaped special characters in code

**Solution**:
```rust
// Use proper markdown escaping
fn escape_markdown(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('`', "\\`")
     .replace('*', "\\*")
     .replace('_', "\\_")
     .replace('[', "\\[")
     .replace(']', "\\]")
}

// Or use code fences with language
format!("```ruchy\n{}\n```", code) // Proper escaping
```

---

### Issue: Five-Whys analysis produces nonsense

**Problem**:
```
Why #1: Why does the parser crash?
Hypothesis: The moon is made of cheese
Confidence: VeryHigh
```

**Cause**: Insufficient data points for analysis

**Solution**:
```rust
// Check data quality before Five-Whys
let metrics = ComplexityMetrics::analyze(&source)?;
let churn = ChurnCorrelation::analyze_file(&path, 90)?;
let satd = SatdDetector::analyze(&source)?;

// Require minimum data
if metrics.cyclomatic == 0 || churn.commit_count == 0 {
    println!("⚠️  Insufficient data for Five-Whys analysis");
    println!("   Metrics: {}", metrics.cyclomatic);
    println!("   Commits: {}", churn.commit_count);
    return None;
}

let five_whys = FiveWhysAnalyzer::analyze(
    &bug_description,
    &metrics,
    &churn,
    &satd,
);
```

---

## 6. Property Testing Issues

### Issue: Shrinking fails to find minimal example

**Problem**:
```
Original: 500 lines
Shrunk to: 498 lines (0.4% reduction)
Expected: <10 lines
```

**Cause**: Poor shrinking strategy

**Solution 1**: Use better shrinking algorithm
```rust
let tester = PropertyTester::new(PropertyConfig {
    num_cases: 10_000,
    max_depth: 10,
    timeout_ms: 5000,
    shrink_on_failure: true,
})
.with_shrink_strategy(ShrinkStrategy::DeltaDebug); // Better than default
```

**Solution 2**: Manual delta debugging
```rust
use ruchyruchy::bug_replication::*;

let debugger = DeltaDebugger::new(MinimizationConfig {
    target_size: 10,
    timeout_per_attempt: 1000,
    max_attempts: 1000,
});

let minimal = debugger.minimize(&original_failing_input, |input| {
    // Returns true if input still fails
    test(input).is_failure()
});

println!("Minimized from {} to {} lines", original.lines().count(), minimal.lines().count());
```

---

### Issue: Property test is flaky (sometimes passes, sometimes fails)

**Problem**:
```
Run 1: ✅ Pass (10,000/10,000)
Run 2: ❌ Fail (case 3,456)
Run 3: ✅ Pass (10,000/10,000)
```

**Cause**: Non-deterministic behavior (timing, randomness, I/O)

**Solution 1**: Add deterministic seeds
```rust
let generator = ExpressionGenerator::new()
    .with_seed(12345); // Fixed seed for reproducibility

let result = tester.test_property(
    "parse_roundtrip",
    property_fn,
    generator,
);
```

**Solution 2**: Isolate non-determinism
```rust
// Instead of testing parse → execute → check output
// Test only: parse → emit → parse (no execution)

// Flaky (execution is non-deterministic):
|input| {
    let ast = parse(input);
    let output = execute(&ast); // Non-deterministic!
    output == expected
}

// Deterministic:
|input| {
    let ast = parse(input);
    let emitted = ast.emit();
    parse(&emitted) == ast // Pure comparison
}
```

---

## 7. Fuzz Testing Issues

### Issue: Fuzzer generates invalid inputs

**Problem**:
```
Generated input: "§¶∞§∞¶§∞¶"
Expected: Valid Ruchy code
```

**Cause**: Using mutation-based fuzzing without grammar

**Solution**: Switch to grammar-based fuzzing
```rust
// Don't do this (mutation-based on random bytes):
let fuzzer = MutationFuzzer::new(/* ... */);

// Do this (grammar-based):
let grammar = Grammar::from_file("ruchy.grammar")?;
let fuzzer = GrammarFuzzer::new(/* ... */);
let stats = fuzzer.fuzz(&grammar, test_fn);
```

---

### Issue: Fuzzer finds too many crashes (overwhelmed)

**Problem**:
```
Crashes found: 10,000
Time to triage: weeks
```

**Solution 1**: Deduplicate crashes
```rust
let mut unique_crashes: HashMap<String, Vec<String>> = HashMap::new();

for crash_input in stats.crash_inputs {
    let stack_trace = get_stack_trace(&crash_input);
    unique_crashes
        .entry(stack_trace)
        .or_insert(Vec::new())
        .push(crash_input);
}

println!("Unique crash types: {}", unique_crashes.len());

for (stack_trace, inputs) in &unique_crashes {
    println!("\nCrash type:");
    println!("{}", stack_trace);
    println!("Occurrences: {}", inputs.len());
    println!("Example: {}", inputs[0]);
}
```

**Solution 2**: Minimize each crash
```rust
for crash_input in unique_crash_examples {
    let minimal = minimize_crash(&crash_input);
    file_bug_report(&minimal);
}
```

---

## 8. Mutation Testing Issues

### Issue: Mutation score is 0% (no mutants killed)

**Problem**:
```
Mutation Score: 0.0%
Killed: 0/100
Survived: 100/100
```

**Cause**: Test suite doesn't actually test the code

**Solution**: Review test coverage
```bash
# Check test coverage
cargo tarpaulin --out Html

# Open coverage report
open tarpaulin-report.html

# Look for untested lines, add tests
```

**Example**: Add tests for surviving mutants
```rust
// Mutant survived: + → - on line 42
// Original: let sum = a + b;
// Mutated: let sum = a - b;
// Test suite didn't catch this!

// Add test:
#[test]
fn test_addition_not_subtraction() {
    assert_eq!(add(5, 3), 8);   // Not 2!
    assert_eq!(add(10, -5), 5); // Not 15!
}
```

---

### Issue: Mutation testing takes too long

**Problem**:
```
Mutation testing: 4 hours
Expected: <30 minutes
```

**Solution 1**: Parallel execution
```rust
let engine = MutationEngine::new(config)
    .with_parallelism(num_cpus::get()); // Use all cores
```

**Solution 2**: Selective mutation
```rust
// Only mutate hot spots (high complexity, high churn)
let engine = MutationEngine::new(config)
    .with_filter(|line| {
        let complexity = get_complexity_at_line(line);
        let churn = get_churn_at_line(line);
        complexity > 10 || churn > 5
    });
```

**Solution 3**: Incremental mutation testing
```bash
# Only test changed lines
git diff main --name-only | grep '\.rs$' | \
  cargo run --bin mutate --incremental
```

---

## 9. Differential Testing Issues

### Issue: Differential testing reports expected differences

**Problem**:
```
Mismatch found!
Old version: TypeScript output
New version: Rust output
```

**Cause**: Testing across different targets instead of versions

**Solution**: Clarify what you're testing
```rust
// If testing version regression:
let tester = VersionRegressionTester::new(DifferentialConfig {
    old_version: "1.28.0".to_string(),
    new_version: "1.29.0".to_string(),
    timeout_ms: 5000,
});

// If testing target equivalence:
let tester = TargetMismatchTester::new(/* ... */);
// Lower confidence (0.9 instead of 1.0)
// Some differences are expected
```

---

## 10. CI/CD Integration Problems

### Issue: GitHub Actions workflow fails

**Error**:
```
Error: GITHUB_TOKEN not found
```

**Solution**: Add secret to repository
```yaml
# In .github/workflows/bug-discovery.yml
jobs:
  discover:
    steps:
      - name: Run Discovery
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }} # Use GitHub's built-in token
          # Or use custom token:
          # GITHUB_TOKEN: ${{ secrets.CUSTOM_GITHUB_TOKEN }}
        run: |
          cargo run --release -- discover --auto-file-issues
```

Add custom token at: Repository Settings → Secrets and variables → Actions → New repository secret

---

### Issue: Pre-commit hook is too slow

**Problem**:
```
Running bug discovery...
(5 minutes pass)
Commit aborted (developer frustrated)
```

**Solution 1**: Run only fast checks in pre-commit
```bash
# .git/hooks/pre-commit

# Fast checks only (property tests on changed files)
CHANGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$')

if [ -n "$CHANGED_FILES" ]; then
    cargo run --release -- discover \
        --property \
        --files "$CHANGED_FILES" \
        --confidence-threshold 0.90 \
        --timeout 30s
fi
```

**Solution 2**: Run full discovery in CI instead
```yaml
# .github/workflows/bug-discovery.yml
on:
  push:
    branches: [main]
  pull_request:

jobs:
  discover:
    runs-on: ubuntu-latest
    steps:
      - name: Full Discovery (all methods)
        run: |
          cargo run --release -- discover \
            --differential \
            --property \
            --fuzz \
            --mutation \
            --confidence-threshold 0.85
```

---

## 11. False Positives

### Issue: Findings are not real bugs

**Problem**:
```
Filed 100 issues
Closed as "not a bug": 80
False positive rate: 80%
```

**Solution 1**: Increase confidence threshold
```rust
// Before (too permissive):
let findings: Vec<_> = all_findings
    .iter()
    .filter(|f| f.confidence.overall >= 0.50) // 50% threshold
    .collect();

// After (more conservative):
let findings: Vec<_> = all_findings
    .iter()
    .filter(|f| f.confidence.overall >= 0.85) // 85% threshold
    .collect();
```

**Solution 2**: Human review for medium confidence
```rust
for finding in all_findings {
    match finding.confidence.level() {
        ConfidenceLevel::VeryHigh => {
            // File immediately (high precision)
            file_github_issue(&finding)?;
        },
        ConfidenceLevel::High | ConfidenceLevel::Medium => {
            // Human review required
            println!("Review finding: {}", finding.title);
            println!("Confidence: {:.2}", finding.confidence.overall);
            print!("File issue? (y/n): ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim() == "y" {
                file_github_issue(&finding)?;
            }
        },
        _ => {
            // Ignore low confidence
        },
    }
}
```

**Solution 3**: Validate with historical bugs
```rust
// Measure false positive rate on known-good code
let validator = BugCorpusValidator::new(known_good_code);
let report = validator.validate(|code| run_discovery(code));

if report.metrics.false_positive_rate > 0.05 {
    println!("⚠️  High false positive rate: {:.1}%",
        report.metrics.false_positive_rate * 100.0);
    println!("   Consider increasing confidence threshold");
}
```

---

## 12. False Negatives

### Issue: Missing known bugs

**Problem**:
```
Historical bugs: 79
Detected: 60
Missed: 19
Detection rate: 75.9% (target: 95%+)
```

**Solution 1**: Use multiple discovery methods
```rust
// Combine all methods for maximum coverage
let differential_bugs = run_differential_tests()?;
let property_bugs = run_property_tests()?;
let fuzz_bugs = run_fuzz_tests()?;
let mutation_bugs = run_mutation_tests()?;

let all_bugs: Vec<_> = differential_bugs
    .into_iter()
    .chain(property_bugs)
    .chain(fuzz_bugs)
    .chain(mutation_bugs)
    .collect();

// Deduplicate
let unique_bugs = deduplicate(all_bugs);

println!("Total unique bugs: {}", unique_bugs.len());
```

**Solution 2**: Analyze missed bugs
```rust
for (bug, result) in report.missed_bugs {
    println!("\nMissed: {} (issue #{})", bug.title, bug.issue_number);
    println!("Category: {:?}", bug.category);
    println!("Reason: {}", result.miss_reason.unwrap_or("Unknown".to_string()));

    // Identify patterns
    // - What category are we missing? (e.g., all TypeErrors)
    // - What discovery methods did we use? (e.g., only property testing)
    // - What can we improve?
}
```

**Solution 3**: Improve generators
```rust
// If missing Type Errors, add type-focused generator
let type_error_generator = TypeErrorGenerator::new()
    .with_invalid_types()
    .with_type_mismatches()
    .with_missing_type_annotations();

let bugs = run_property_tests_with_generator(type_error_generator)?;
```

---

## Getting Help

If your issue isn't covered here:

1. **Check User Guide**: [docs/user_guide/README.md](../user_guide/README.md)
2. **Check API Docs**: [docs/api/README.md](../api/README.md)
3. **Check Examples**: [docs/examples/README.md](../examples/README.md)
4. **Search Issues**: https://github.com/paiml/ruchyruchy/issues
5. **Open New Issue**: https://github.com/paiml/ruchyruchy/issues/new
6. **Join Discussions**: https://github.com/paiml/ruchyruchy/discussions

---

## Debug Mode

Enable detailed logging for troubleshooting:

```bash
# Set log level
export RUST_LOG=ruchyruchy=debug

# Run with debug output
cargo run --release -- discover --property
```

**Output**:
```
[DEBUG] PropertyTester: Starting test_property 'parse_roundtrip'
[DEBUG] Generator: Generated input #1 (42 chars)
[DEBUG] Property: Testing input...
[DEBUG] Property: Result = true (passed)
[DEBUG] Generator: Generated input #2 (87 chars)
...
[DEBUG] PropertyTester: Completed 10,000/10,000 cases
[DEBUG] PropertyTester: No failures found
```

---

**Last Updated**: 2025-10-29
**Troubleshooting Guide Version**: 1.0.0
