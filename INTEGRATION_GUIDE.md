# RuchyRuchy Integration Guide

**Version**: 1.9.0
**Last Updated**: October 30, 2025
**Status**: Production Ready

---

## Table of Contents

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Part 1: Integrating with Ruchy Compiler](#part-1-integrating-with-ruchy-compiler)
4. [Part 2: Integrating with Any Ruchy Project](#part-2-integrating-with-any-ruchy-project)
5. [Available Tools](#available-tools)
6. [Workflows](#workflows)
7. [Best Practices](#best-practices)
8. [Troubleshooting](#troubleshooting)

---

## Overview

RuchyRuchy provides comprehensive debugging, bug discovery, and quality analysis tools for Ruchy projects. This guide covers:

- **Debugging Tools**: Source maps, time-travel debugging, execution tracing, performance profiling
- **Bug Discovery**: Differential testing, property-based testing, fuzz testing, code churn analysis
- **Bug Replication**: Delta debugging, git bisection, test harness generation
- **Bug Reporting**: GitHub integration, Five-Whys analysis, confidence scoring

### Key Features

- **95%+ Bug Detection Rate**: Validated against 79 historical Ruchy bugs
- **Zero-Cost Instrumentation**: Compiler-based tracing with no runtime overhead when disabled
- **Fast Feedback**: <6 second validation cycles in pre-commit hooks
- **Production Ready**: 387+ tests passing, A+ lint quality, zero technical debt

---

## Quick Start

### Installation

```bash
# Install from crates.io
cargo install ruchyruchy

# Or build from source
git clone https://github.com/paiml/ruchyruchy.git
cd ruchyruchy
cargo build --release
```

### Basic Usage

```bash
# Run Ruchy code with timeout detection
ruchydbg run test.ruchy --timeout 5000

# Validate debugging tools
ruchydbg validate

# Check version
ruchydbg --version
```

### Library Usage

```rust
use ruchyruchy::bug_discovery::property_testing::PropertyTest;
use ruchyruchy::bug_reporting::github_integration::GitHubClient;
use ruchyruchy::bug_replication::minimizer::DeltaDebugger;

// Property testing
let test = PropertyTest::new("roundtrip_property");
test.run(10000)?;

// GitHub integration
let client = GitHubClient::new("token", "owner", "repo");
client.create_issue(bug_report)?;

// Delta debugging
let minimizer = DeltaDebugger::new(test_fn);
let minimized = minimizer.minimize(code)?;
```

---

## Part 1: Integrating with Ruchy Compiler

This section focuses on integrating RuchyRuchy tools into the Ruchy compiler itself (`../ruchy`).

### Why Integrate with Ruchy Compiler?

- **Dogfooding**: Validate tools on production compiler code (50K+ LOC)
- **Continuous Validation**: Every Ruchy commit validates debugging tools
- **Real-World Testing**: Discover edge cases in actual production code
- **Fast Feedback**: <6 second validation cycle in pre-commit hooks
- **Bug Prevention**: Catch regressions before they reach production

### Step 1: Pre-Commit Hook Integration

The fastest way to integrate is via pre-commit hooks for continuous validation.

#### 1.1: Symlink Validation Script

From the Ruchy repository:

```bash
cd ../ruchy

# Create symlink to RuchyRuchy validation script
ln -s ../ruchyruchy/scripts/validate-debugging-tools.sh scripts/validate-debugging-tools.sh

# Make executable (if needed)
chmod +x scripts/validate-debugging-tools.sh
```

#### 1.2: Update Pre-Commit Hook

Edit `../ruchy/.git/hooks/pre-commit` and add after existing checks:

```bash
# RuchyRuchy debugging tools validation
echo -n "  RuchyRuchy debugging tools... "
if [ -f "../ruchyruchy/scripts/validate-debugging-tools.sh" ]; then
    if ../ruchyruchy/scripts/validate-debugging-tools.sh > /dev/null 2>&1; then
        echo "✅"
    else
        echo "❌"
        echo ""
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "RuchyRuchy debugging tools validation failed."
        echo "Run manually to see details:"
        echo "  cd ../ruchyruchy && ./scripts/validate-debugging-tools.sh"
        echo ""
        echo "This validates source maps and time-travel debugging."
        echo "To bypass (NOT RECOMMENDED): git commit --no-verify"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        exit 1
    fi
else
    echo "⚠️  (RuchyRuchy not found)"
fi
```

#### 1.3: Test Integration

```bash
cd ../ruchy

# Test validation script directly
../ruchyruchy/scripts/validate-debugging-tools.sh

# Expected output:
# 🔍 RuchyRuchy Debugging Tools Validation
# =========================================
#
# 🗺️  Validating source maps (fast mode)...
#   ✅ Source maps validated (3 lines, 1:1 mapping)
# ⏮️  Testing time-travel debugging (smoke test)...
#   ✅ Time-travel working (3 steps, backward replay)
# ⚡ Performance regression check...
#   ✅ Performance OK (100 mappings < 1s threshold)
#
# ✅ All debugging tools validated!

# Test pre-commit hook
git commit --allow-empty -m "TEST: Pre-commit hook integration"
```

### Step 2: Bug Discovery Integration

Integrate bug discovery tools into Ruchy's CI/CD pipeline.

#### 2.1: Property-Based Testing

Create `../ruchy/tests/ruchyruchy_property_tests.rs`:

```rust
use ruchyruchy::bug_discovery::property_testing::{PropertyTest, PropertyTestConfig};

#[test]
fn test_ruchy_parser_roundtrip() {
    let config = PropertyTestConfig {
        num_cases: 10000,
        max_size: 1000,
        shrink_attempts: 100,
    };

    let test = PropertyTest::new_with_config("parser_roundtrip", config);

    // Property: parse(emit(ast)) == ast
    test.run(|input| {
        let ast = ruchy_parser::parse(input)?;
        let emitted = ruchy_emitter::emit(&ast)?;
        let reparsed = ruchy_parser::parse(&emitted)?;

        assert_eq!(ast, reparsed, "Roundtrip property violated");
        Ok(())
    }).expect("Property test failed");
}
```

#### 2.2: Differential Testing

Create `../ruchy/tests/ruchyruchy_differential_tests.rs`:

```rust
use ruchyruchy::bug_discovery::differential::{DifferentialTester, CompilerVersion};

#[test]
fn test_ruchy_regression_detection() {
    let tester = DifferentialTester::new();

    let v1 = CompilerVersion::new("v3.145.0");
    let v2 = CompilerVersion::new("v3.146.0");

    // Test for regressions between versions
    let regressions = tester.test_versions(v1, v2, &test_suite)?;

    assert!(
        regressions.is_empty(),
        "Found {} regressions: {:#?}",
        regressions.len(),
        regressions
    );
}
```

#### 2.3: Code Churn Analysis

Create `../ruchy/scripts/analyze-churn.sh`:

```bash
#!/bin/bash
# Analyze code churn for bug-prone areas

set -euo pipefail

cd ../ruchyruchy

cargo run --release --example analyze_churn -- \
    --repo ../ruchy \
    --since "1 month ago" \
    --threshold 10 \
    --output ../ruchy/churn-report.json

echo "📊 Code churn analysis complete"
echo "   Report: churn-report.json"
echo "   Review high-churn areas for potential bugs"
```

### Step 3: CI/CD Integration

Add RuchyRuchy validation to Ruchy's GitHub Actions workflow.

#### 3.1: Update `.github/workflows/ci.yml`

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install Ruchy
        run: cargo install ruchy

      - name: Clone RuchyRuchy
        run: |
          cd ..
          git clone https://github.com/paiml/ruchyruchy.git
          cd ruchyruchy
          cargo build --release

      - name: Run RuchyRuchy Validation
        run: |
          cd ../ruchyruchy
          ./scripts/validate-debugging-tools.sh

      - name: Run Property Tests
        run: cargo test --test ruchyruchy_property_tests

      - name: Run Differential Tests
        run: cargo test --test ruchyruchy_differential_tests
```

### Step 4: Bug Reporting Integration

Automate bug filing to Ruchy's GitHub issues.

#### 4.1: Create Bug Filing Script

Create `../ruchy/scripts/file-bug.sh`:

```bash
#!/bin/bash
# Automatically file bugs discovered by RuchyRuchy

set -euo pipefail

if [ -z "${GITHUB_TOKEN:-}" ]; then
    echo "❌ GITHUB_TOKEN not set"
    exit 1
fi

cd ../ruchyruchy

cargo run --release --example file_github_issue -- \
    --token "$GITHUB_TOKEN" \
    --repo "paiml/ruchy" \
    --bug-report "$1" \
    --labels "bug,automated" \
    --auto-link

echo "✅ Bug filed to GitHub"
```

#### 4.2: Usage Example

```bash
# After discovering a bug with property testing
cd ../ruchy
./scripts/file-bug.sh bug-report.json

# Output:
# ✅ Bug filed to GitHub
#    Issue: https://github.com/paiml/ruchy/issues/123
#    Linked to 3 related issues
#    Confidence score: 0.92 (HIGH)
```

### Step 5: Performance Profiling Integration

Use RuchyRuchy's profiling tools to optimize Ruchy compiler performance.

#### 5.1: Profile Ruchy Compilation

```bash
cd ../ruchy

# Profile a compilation (requires perf_event_open feature)
cargo build --release --features profiling
RUCHY_PROFILE=1 ./target/release/ruchy compile large_file.ruchy

# Analyze profile data
cd ../ruchyruchy
cargo run --release --example analyze_profile -- \
    --profile ../ruchy/profile.json \
    --flame-graph ../ruchy/flame.svg \
    --hotspots 10

echo "🔥 Flame graph: flame.svg"
echo "📊 Top 10 hotspots identified"
```

---

## Part 2: Integrating with Any Ruchy Project

This section covers integrating RuchyRuchy tools into any Ruchy project.

### Prerequisites

- Ruchy compiler installed (`cargo install ruchy`)
- RuchyRuchy installed (`cargo install ruchyruchy`)
- Git repository for your project

### Step 1: Project Setup

#### 1.1: Add RuchyRuchy Dependency

For Rust projects with Ruchy code:

```toml
# Cargo.toml
[dependencies]
ruchyruchy = "1.9"

[dev-dependencies]
ruchyruchy = { version = "1.9", features = ["testing"] }
```

For pure Ruchy projects, use the CLI tools:

```bash
# Create ruchyruchy config
mkdir -p .ruchyruchy
cat > .ruchyruchy/config.toml <<EOF
[project]
name = "my-ruchy-project"
version = "0.1.0"

[debugging]
source_maps = true
time_travel = true
profiling = false

[bug_discovery]
property_tests = true
differential_tests = false
fuzz_tests = true

[quality]
min_test_coverage = 0.80
max_complexity = 20
zero_satd = true
EOF
```

#### 1.2: Create Pre-Commit Hook

```bash
# Install pre-commit hook
cat > .git/hooks/pre-commit <<'EOF'
#!/bin/bash
set -euo pipefail

echo "🔍 Running RuchyRuchy quality checks..."

# 1. Source map validation
ruchydbg validate || exit 1

# 2. Run property tests (if configured)
if [ -f "tests/property_tests.ruchy" ]; then
    ruchy test tests/property_tests.ruchy || exit 1
fi

# 3. Code churn analysis (non-blocking)
if command -v ruchyruchy &> /dev/null; then
    ruchyruchy analyze-churn --threshold 10 || echo "⚠️  High code churn detected"
fi

echo "✅ All checks passed"
EOF

chmod +x .git/hooks/pre-commit
```

### Step 2: Add Property-Based Testing

Create property tests for your Ruchy code.

#### 2.1: Create Property Test File

Create `tests/property_tests.ruchy`:

```ruchy
// Property-based tests for my Ruchy project

fun test_your_function_properties() {
    // Property 1: Idempotence
    // f(f(x)) == f(x)
    property("idempotence", |x| {
        let result1 = your_function(x);
        let result2 = your_function(result1);
        assert(result1 == result2);
    }, 10000);

    // Property 2: Commutativity
    // f(a, b) == f(b, a)
    property("commutativity", |a, b| {
        let result1 = your_function(a, b);
        let result2 = your_function(b, a);
        assert(result1 == result2);
    }, 10000);

    // Property 3: Associativity
    // f(f(a, b), c) == f(a, f(b, c))
    property("associativity", |a, b, c| {
        let result1 = your_function(your_function(a, b), c);
        let result2 = your_function(a, your_function(b, c));
        assert(result1 == result2);
    }, 10000);
}

fun main() {
    test_your_function_properties();
    println("✅ All property tests passed");
}
```

#### 2.2: Run Property Tests

```bash
# Run property tests
ruchy test tests/property_tests.ruchy

# Expected output:
# 🔍 Running property tests...
#    Property: idempotence (10000 cases)... ✅
#    Property: commutativity (10000 cases)... ✅
#    Property: associativity (10000 cases)... ✅
# ✅ All property tests passed (30000 total cases)
```

### Step 3: Add Debugging Support

Enable source maps and time-travel debugging for your project.

#### 3.1: Enable Source Maps

```bash
# Compile with source maps
ruchy compile --source-maps your_code.ruchy -o output.js

# Verify source maps
ruchydbg validate
```

#### 3.2: Use Time-Travel Debugging

```bash
# Run with time-travel recording
ruchydbg run your_code.ruchy --timeout 5000

# If it crashes/hangs, debug with time-travel:
# (Future feature - record-replay debugging)
```

### Step 4: Add Bug Discovery

Integrate bug discovery tools into your development workflow.

#### 4.1: Code Churn Analysis

```bash
# Analyze code churn to find bug-prone areas
ruchyruchy analyze-churn \
    --repo . \
    --since "1 month ago" \
    --threshold 10 \
    --output churn-report.json

# Review report
cat churn-report.json | jq '.hotspots[] | select(.churn > 15)'
```

#### 4.2: Historical Bug Validation

If you have historical bugs:

```bash
# Create bug corpus
mkdir -p bugs/corpus
# Add historical bug test cases...

# Validate bug detection rate
ruchyruchy validate-detection-rate \
    --corpus bugs/corpus \
    --tools "property,differential,fuzz" \
    --target-rate 0.95

# Expected output:
# 📊 Detection Rate Analysis
#    Total bugs: 50
#    Detected: 48
#    Detection rate: 96.0%
#    Target: 95.0%
#    Status: ✅ PASS
```

### Step 5: CI/CD Integration

Add RuchyRuchy to your CI/CD pipeline.

#### 5.1: GitHub Actions Example

Create `.github/workflows/ruchyruchy.yml`:

```yaml
name: RuchyRuchy Quality Checks

on: [push, pull_request]

jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install Ruchy
        run: cargo install ruchy

      - name: Install RuchyRuchy
        run: cargo install ruchyruchy

      - name: Run Source Map Validation
        run: ruchydbg validate

      - name: Run Property Tests
        run: |
          if [ -f "tests/property_tests.ruchy" ]; then
            ruchy test tests/property_tests.ruchy
          fi

      - name: Analyze Code Churn
        run: |
          ruchyruchy analyze-churn \
            --repo . \
            --since "1 week ago" \
            --threshold 10

      - name: Check Test Coverage
        run: |
          ruchy test --coverage
          # Validate >80% coverage
```

---

## Available Tools

### Debugging Tools

#### ruchydbg CLI

```bash
# Execute Ruchy code with timeout detection
ruchydbg run test.ruchy --timeout 5000

# Validate debugging tools (source maps, time-travel)
ruchydbg validate

# Show version
ruchydbg --version
```

#### Source Maps

```rust
use ruchyruchy::debugging::source_maps::SourceMap;

let source_map = SourceMap::new("test.ruchy");
source_map.add_mapping(1, 1)?;  // source line 1 -> target line 1
source_map.validate()?;
```

#### Time-Travel Debugging (Record-Replay)

```rust
use ruchyruchy::debugging::record_replay::{Recorder, Replay};

// Record execution
let recorder = Recorder::new();
recorder.record_step("initial_state", state1);
recorder.record_step("after_operation", state2);

// Replay to specific step
let replay = Replay::from_recording(recorder.finalize());
let state = replay.step_to(1)?;
```

### Bug Discovery Tools

#### Property-Based Testing

```rust
use ruchyruchy::bug_discovery::property_testing::PropertyTest;

let test = PropertyTest::new("roundtrip");
test.run_with_shrinking(|input| {
    let output = transform(input);
    let roundtrip = transform_back(output);
    assert_eq!(input, roundtrip);
    Ok(())
}, 10000)?;
```

#### Differential Testing

```rust
use ruchyruchy::bug_discovery::differential::DifferentialTester;

let tester = DifferentialTester::new();
let regressions = tester.test_versions(v1, v2, test_suite)?;

for bug in regressions {
    println!("Regression: {:?}", bug);
}
```

#### Code Churn Analysis

```rust
use ruchyruchy::bug_discovery::churn_analyzer::ChurnAnalyzer;

let analyzer = ChurnAnalyzer::new(".");
let hotspots = analyzer.analyze_since("1 month ago")?;

for hotspot in hotspots.iter().filter(|h| h.churn > 10) {
    println!("High churn: {} ({})", hotspot.file, hotspot.churn);
}
```

### Bug Replication Tools

#### Delta Debugging (Minimization)

```rust
use ruchyruchy::bug_replication::minimizer::{DeltaDebugger, MinimizationStrategy};

let minimizer = DeltaDebugger::new(|code| {
    // Test function: returns Pass/Fail/Timeout
    test_code(code)
});

let minimized = minimizer.minimize_with_strategy(
    large_code,
    MinimizationStrategy::LineLevel
)?;

println!("Minimized from {} to {} lines",
         large_code.lines().count(),
         minimized.lines().count());
```

#### Git Bisection

```rust
use ruchyruchy::bug_replication::bisect::GitBisector;

let bisector = GitBisector::new(".", |commit| {
    // Test function at specific commit
    checkout_and_test(commit)
});

let bad_commit = bisector.bisect(good_commit, bad_commit)?;
println!("Bug introduced in: {}", bad_commit.id);
```

#### Test Harness Generation

```rust
use ruchyruchy::bug_replication::harness::ReplicationHarness;

let harness = ReplicationHarness::new();
let test = harness.generate_standalone_test(bug_report)?;

// Write standalone test file
std::fs::write("tests/bug_123.ruchy", test)?;
```

### Bug Reporting Tools

#### GitHub Integration

```rust
use ruchyruchy::bug_reporting::github_integration::GitHubClient;

let client = GitHubClient::new("token", "owner", "repo");

// Create issue
let issue = client.create_issue(bug_report)?;
println!("Filed issue: {}", issue.url);

// Link related issues
client.link_related_issues(issue.number, related_bugs)?;
```

#### Confidence Scoring

```rust
use ruchyruchy::bug_reporting::confidence::{ConfidenceScorer, DiscoveryMethod};

let scorer = ConfidenceScorer::new();
let score = scorer.calculate(
    DiscoveryMethod::PropertyTesting,
    Reproducibility::Deterministic,
    QuantitativeEvidence::MetricsAvailable,
    RootCauseClarity::Identified
);

println!("Confidence: {} ({})", score.overall, score.priority());
// Output: Confidence: 0.92 (CRITICAL)
```

#### Five-Whys Analysis

```rust
use ruchyruchy::bug_reporting::five_whys::FiveWhysAnalyzer;

let analyzer = FiveWhysAnalyzer::new();
let analysis = analyzer.analyze(bug_report)?;

for (i, layer) in analysis.layers.iter().enumerate() {
    println!("Why #{}: {}", i+1, layer.question);
    println!("  Data: {:?}", layer.data_points);
    println!("  Hypothesis: {}", layer.hypothesis.description);
    println!("  Confidence: {:?}", layer.hypothesis.confidence);
}
```

---

## Workflows

### Workflow 1: Bug Discovery & Reporting

Complete workflow from bug discovery to GitHub issue filing.

```bash
#!/bin/bash
# discover-and-file-bug.sh

set -euo pipefail

echo "🔍 Step 1: Run property-based testing..."
ruchy test tests/property_tests.ruchy > property_results.txt || {
    echo "   Found potential bug in property tests"
}

echo "🔬 Step 2: Minimize failing test case..."
ruchyruchy minimize \
    --test property_results.txt \
    --strategy ast \
    --output minimized.ruchy

echo "📊 Step 3: Calculate confidence score..."
ruchyruchy confidence \
    --bug-report minimized.ruchy \
    --discovery-method property \
    --output confidence.json

CONFIDENCE=$(jq '.overall' confidence.json)
echo "   Confidence: $CONFIDENCE"

if (( $(echo "$CONFIDENCE > 0.85" | bc -l) )); then
    echo "📝 Step 4: File GitHub issue (high confidence)..."
    ruchyruchy file-issue \
        --token "$GITHUB_TOKEN" \
        --repo "owner/repo" \
        --bug-report minimized.ruchy \
        --confidence confidence.json \
        --auto-link

    echo "✅ Bug filed successfully"
else
    echo "⚠️  Step 4: Low confidence, manual review needed"
fi
```

### Workflow 2: Regression Testing

Detect regressions between compiler versions.

```bash
#!/bin/bash
# detect-regressions.sh

set -euo pipefail

OLD_VERSION="v3.145.0"
NEW_VERSION="v3.146.0"

echo "🔍 Testing for regressions: $OLD_VERSION -> $NEW_VERSION"

ruchyruchy differential-test \
    --old-version "$OLD_VERSION" \
    --new-version "$NEW_VERSION" \
    --test-suite tests/ \
    --output regressions.json

REGRESSION_COUNT=$(jq 'length' regressions.json)

if [ "$REGRESSION_COUNT" -gt 0 ]; then
    echo "❌ Found $REGRESSION_COUNT regressions"
    jq '.[] | {file: .file, error: .error}' regressions.json
    exit 1
else
    echo "✅ No regressions detected"
fi
```

### Workflow 3: Performance Profiling

Profile and optimize hot paths.

```bash
#!/bin/bash
# profile-and-optimize.sh

set -euo pipefail

echo "📊 Step 1: Profile execution..."
ruchydbg run --profile benchmark.ruchy > profile.json

echo "🔥 Step 2: Generate flame graph..."
ruchyruchy flame-graph \
    --profile profile.json \
    --output flame.svg

echo "🎯 Step 3: Identify hotspots..."
ruchyruchy analyze-hotspots \
    --profile profile.json \
    --top 10 \
    --output hotspots.txt

echo "📈 Top 10 hotspots:"
cat hotspots.txt

echo "💡 Step 4: Suggest optimizations..."
ruchyruchy suggest-optimizations \
    --hotspots hotspots.txt \
    --code src/ \
    --output suggestions.md

echo "✅ Optimization suggestions: suggestions.md"
```

---

## Best Practices

### 1. Pre-Commit Hook Integration

✅ **DO**: Add RuchyRuchy validation to pre-commit hooks for fast feedback
✅ **DO**: Keep validation fast (<6 seconds)
✅ **DO**: Run source map validation on every commit
❌ **DON'T**: Run expensive analyses (fuzz testing) in pre-commit hooks

### 2. Property-Based Testing

✅ **DO**: Write properties for core algorithms (roundtrip, idempotence, etc.)
✅ **DO**: Run 10,000+ test cases per property
✅ **DO**: Enable shrinking to find minimal failing cases
❌ **DON'T**: Write properties without clear mathematical meaning

### 3. Bug Filing

✅ **DO**: Calculate confidence scores before filing
✅ **DO**: Only auto-file bugs with confidence > 0.85
✅ **DO**: Include minimized test cases in bug reports
❌ **DON'T**: File bugs without reproduction steps

### 4. Code Churn Analysis

✅ **DO**: Track code churn monthly to identify hotspots
✅ **DO**: Focus testing on high-churn areas
✅ **DO**: Review high-churn areas in code reviews
❌ **DON'T**: Ignore high-churn files (they're bug-prone)

### 5. Performance Profiling

✅ **DO**: Profile before optimizing (measure first)
✅ **DO**: Use flame graphs to visualize hotspots
✅ **DO**: Benchmark before/after optimizations
❌ **DON'T**: Optimize without profiling data

---

## Troubleshooting

### Issue: "ruchydbg: command not found"

**Solution**: Install RuchyRuchy

```bash
cargo install ruchyruchy
# or
cd ruchyruchy && cargo build --release
export PATH="$PWD/target/release:$PATH"
```

### Issue: "ruchy: command not found"

**Solution**: Install Ruchy compiler

```bash
cargo install ruchy
```

### Issue: Pre-commit hook fails

**Solution**: Run validation manually to see details

```bash
cd ../ruchyruchy
./scripts/validate-debugging-tools.sh

# If source maps fail:
ruchy compile --source-maps test.ruchy
ruchydbg validate
```

### Issue: Property tests time out

**Solution**: Reduce test case count or increase timeout

```ruchy
// Before (slow)
property("test", test_fn, 100000);  // 100K cases

// After (faster)
property("test", test_fn, 10000);   // 10K cases
```

### Issue: GitHub API rate limiting

**Solution**: Use authenticated token for higher rate limits

```bash
export GITHUB_TOKEN="your_token_here"
ruchyruchy file-issue --token "$GITHUB_TOKEN" ...
```

### Issue: Delta debugging takes too long

**Solution**: Use line-level strategy instead of AST-level

```bash
# Fast (line-level)
ruchyruchy minimize --strategy line ...

# Slower but better (AST-level)
ruchyruchy minimize --strategy ast ...
```

---

## Next Steps

1. **Follow the Quick Start** to install and test basic functionality
2. **For Ruchy Compiler**: Follow Part 1 for pre-commit hook integration
3. **For Your Project**: Follow Part 2 for comprehensive integration
4. **Explore Tools**: Review Available Tools section for detailed APIs
5. **Learn Workflows**: Study the workflow examples for common tasks

## Resources

- **GitHub**: https://github.com/paiml/ruchyruchy
- **crates.io**: https://crates.io/crates/ruchyruchy
- **Documentation**: https://docs.rs/ruchyruchy
- **CHANGELOG**: See CHANGELOG.md for release history
- **Bug Reports**: https://github.com/paiml/ruchyruchy/issues

---

**Version**: 1.9.0
**Last Updated**: October 30, 2025
**Status**: ✅ Production Ready (387+ tests passing, 95%+ bug detection rate)
