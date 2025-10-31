// RuchyRuchy Integration Demo
//
// This example demonstrates how to integrate RuchyRuchy tools into your project:
// - Bug discovery (property testing, differential testing, code churn)
// - Bug replication (delta debugging, git bisection, test generation)
// - Bug reporting (GitHub integration, confidence scoring, Five-Whys)
//
// Usage:
//   cargo run --example integration_demo

use ruchyruchy::bug_discovery::confidence::{
    ConfidenceScorer, DiscoveryMethod, EvidenceLevel, Reproducibility,
    RootCauseClarity,
};
use ruchyruchy::bug_discovery::differential::{
    CompilerVersion, DifferentialTester, FailureMode, RegressionBug, TestResult,
};
use ruchyruchy::bug_discovery::schema_fuzzer::{
    Constructor, Operation, RuntimeSchema,
};
use ruchyruchy::bug_replication::bisect::{BisectionResult, Commit, CommitId};
use ruchyruchy::bug_replication::harness::{Environment, ExecutionResult, ReproducibleTest};
use ruchyruchy::bug_replication::minimizer::{
    DeltaDebugger, TestOutcome,
};
use ruchyruchy::bug_reporting::confidence::{QuantitativeEvidence, Reproducibility as ReportRepro};
use ruchyruchy::bug_reporting::five_whys::{ConfidenceLevel, DataPoint, Hypothesis, WhyLayer};
use ruchyruchy::bug_reporting::github_integration::{
    IssueRequest,
};

fn main() {
    println!("🎯 RuchyRuchy Integration Demo\n");
    println!("{}", "=".repeat(60));

    demo_bug_discovery();
    demo_bug_replication();
    demo_bug_reporting();

    println!("\n✅ All demos completed successfully!");
    println!("\nNext steps:");
    println!("  1. Review INTEGRATION_GUIDE.md for detailed integration steps");
    println!("  2. Add pre-commit hooks for continuous validation");
    println!("  3. Integrate property tests into your test suite");
    println!("  4. Set up GitHub integration for automated bug filing");
}

fn demo_bug_discovery() {
    println!("\n📊 Part 1: Bug Discovery Tools");
    println!("{}", "-".repeat(60));

    // 1. Confidence Scoring
    println!("\n1.1 Confidence Scoring");
    let scorer = ConfidenceScorer;
    let score = scorer.calculate_confidence(
        DiscoveryMethod::PropertyTesting,
        Reproducibility::Deterministic,
        EvidenceLevel::Metrics,
        RootCauseClarity::Identified,
    );

    println!("   Confidence Score: {:.2}", score.overall);
    println!("   Priority: {:?}", score.priority());
    println!(
        "   Breakdown: discovery={:.2}, repro={:.2}, evidence={:.2}, clarity={:.2}",
        score.discovery_method_weight,
        score.reproducibility_score,
        score.quantitative_evidence,
        score.root_cause_clarity
    );

    // 2. Differential Testing
    println!("\n1.2 Differential Testing");
    let tester = DifferentialTester::new();
    println!("   Differential Tester initialized");
    println!("   Use: Compare outputs between compiler versions");
    println!("   Example: Detect regressions from v3.145.0 to v3.146.0");

    let regression = RegressionBug {
        test_name: "parser_test_001".to_string(),
        old_version: CompilerVersion {
            version: "v3.145.0".to_string(),
        },
        new_version: CompilerVersion {
            version: "v3.146.0".to_string(),
        },
        failure_mode: FailureMode::Crash,
        error_message: "panic in parser".to_string(),
        input: "fun main() { }".to_string(),
        old_result: TestResult {
            status: TestStatus::Pass,
            output: "success".to_string(),
            exit_code: 0,
        },
        new_result: TestResult {
            status: TestStatus::Fail,
            output: "panic".to_string(),
            exit_code: 1,
        },
    };

    println!("   Regression detected: {}", regression.test_name);
    println!("   Failure mode: {:?}", regression.failure_mode);

    // 3. Schema-Based Fuzzing
    println!("\n1.3 Schema-Based Fuzzing");
    let schema = RuntimeSchema {
        name: "HashMap".to_string(),
        constructors: vec![Constructor {
            name: "new".to_string(),
            params: vec![],
            initializes: vec!["internal_map".to_string()],
        }],
        operations: vec![
            Operation {
                name: "insert".to_string(),
                params: vec!["key".to_string(), "value".to_string()],
                preconditions: vec![],
                postconditions: vec!["size += 1".to_string()],
            },
            Operation {
                name: "get".to_string(),
                params: vec!["key".to_string()],
                preconditions: vec![],
                postconditions: vec![],
            },
        ],
        invariants: vec!["size >= 0".to_string(), "no null keys".to_string()],
    };

    println!("   Schema: {}", schema.name);
    println!("   Constructors: {}", schema.constructors.len());
    println!("   Operations: {}", schema.operations.len());
    println!("   Invariants: {}", schema.invariants.len());
    println!("   Use: Generate test cases that validate invariants");
}

fn demo_bug_replication() {
    println!("\n🔬 Part 2: Bug Replication Tools");
    println!("{}", "-".repeat(60));

    // 1. Delta Debugging (Minimization)
    println!("\n2.1 Delta Debugging");

    let test_fn = |code: &str| {
        // Simplified test: fails if code contains "panic"
        if code.contains("panic") {
            TestOutcome::Fail
        } else {
            TestOutcome::Pass
        }
    };

    let minimizer = DeltaDebugger::new(test_fn);

    let large_code = r#"
fun main() {
    let x = 1;
    let y = 2;
    panic("bug here");
    let z = 3;
}
"#;

    println!("   Original code: {} lines", large_code.lines().count());

    // Simulate minimization
    let minimized = "panic(\"bug here\");";
    println!("   Minimized code: {} lines", minimized.lines().count());
    println!(
        "   Reduction: {}%",
        100 - (minimized.len() * 100 / large_code.len())
    );

    // 2. Git Bisection
    println!("\n2.2 Git Bisection");
    println!("   Bisection setup:");
    println!("   - Good commit: abc123");
    println!("   - Bad commit: def456");
    println!("   - Test: compiler crashes on test.ruchy");
    println!("   Use: Find the exact commit that introduced the bug");

    let bisect_result = BisectionResult {
        bad_commit: Commit {
            id: CommitId("def456".to_string()),
            message: "Refactor parser".to_string(),
            author: "developer".to_string(),
            date: "2025-10-28".to_string(),
        },
        steps: 5,
    };

    println!(
        "   Result: Bad commit found in {} steps",
        bisect_result.steps
    );
    println!("   Commit: {}", bisect_result.bad_commit.id.0);
    println!("   Message: {}", bisect_result.bad_commit.message);

    // 3. Test Harness Generation
    println!("\n2.3 Test Harness Generation");
    let test = ReproducibleTest {
        name: "bug_123_reproduction".to_string(),
        code: "fun main() { panic(\"bug\"); }".to_string(),
        environment: Environment {
            ruchy_version: "v3.146.0".to_string(),
            os: "Linux".to_string(),
            arch: "x86_64".to_string(),
        },
        expected_result: ExecutionResult::Crash,
        timeout_ms: 5000,
    };

    println!("   Test name: {}", test.name);
    println!("   Ruchy version: {}", test.environment.ruchy_version);
    println!("   Expected result: {:?}", test.expected_result);
    println!("   Timeout: {}ms", test.timeout_ms);
    println!("   Use: Generate standalone test files for bug reports");
}

fn demo_bug_reporting() {
    println!("\n📝 Part 3: Bug Reporting Tools");
    println!("{}", "-".repeat(60));

    // 1. Confidence Scoring (for reports)
    println!("\n3.1 Confidence Scoring for Reports");
    let scorer = ruchyruchy::bug_reporting::confidence::ConfidenceScorer;
    let score = scorer.calculate(
        ruchyruchy::bug_reporting::confidence::DiscoveryMethod::PropertyTesting,
        ReportRepro::Deterministic,
        QuantitativeEvidence::MetricsAvailable,
        ruchyruchy::bug_reporting::confidence::RootCauseClarity::Identified,
    );

    println!("   Overall confidence: {:.2}", score.overall);
    println!("   Priority: {:?}", score.priority());
    println!(
        "   Recommendation: {}",
        if score.overall >= 0.85 {
            "Auto-file to GitHub"
        } else {
            "Manual review needed"
        }
    );

    // 2. Five-Whys Analysis
    println!("\n3.2 Five-Whys Analysis");
    let why_layer_1 = WhyLayer {
        question: "Why did the parser crash?".to_string(),
        data_points: vec![
            DataPoint {
                key: "crash_location".to_string(),
                value: "parser.rs:142".to_string(),
                source: "stack trace".to_string(),
            },
            DataPoint {
                key: "panic_message".to_string(),
                value: "unexpected token".to_string(),
                source: "error log".to_string(),
            },
        ],
        hypothesis: Hypothesis {
            description: "Parser encountered malformed input".to_string(),
            confidence: ConfidenceLevel::High,
            supporting_evidence: vec!["Stack trace shows token validation".to_string()],
        },
    };

    println!("   Why #1: {}", why_layer_1.question);
    println!("   Data points: {}", why_layer_1.data_points.len());
    println!("   Hypothesis: {}", why_layer_1.hypothesis.description);
    println!("   Confidence: {:?}", why_layer_1.hypothesis.confidence);
    println!("   Use: Root cause analysis with objective data");

    // 3. GitHub Integration
    println!("\n3.3 GitHub Integration");
    println!("   GitHub client setup:");
    println!("   - Repository: paiml/ruchy");
    println!("   - Authentication: Bearer token");
    println!("   - Auto-linking: Enabled");

    let issue_request = IssueRequest {
        title: "Parser crashes on valid Ruchy code".to_string(),
        body: r#"
## Bug Report

**Discovery Method**: Property-based testing
**Confidence**: 0.92 (CRITICAL)
**Reproducibility**: Deterministic

### Reproduction Steps
1. Create file with: `fun main() { }`
2. Run: `ruchy compile test.ruchy`
3. Observe: Parser crash

### Expected Behavior
Should compile successfully

### Actual Behavior
Parser panics at parser.rs:142

### Minimized Test Case
```ruchy
fun main() { }
```

### Environment
- Ruchy version: v3.146.0
- OS: Linux x86_64

### Related Issues
- #123: Similar parser crash
- #124: Token validation bug
"#
        .to_string(),
        labels: vec![
            "bug".to_string(),
            "parser".to_string(),
            "critical".to_string(),
        ],
    };

    println!("   Issue title: {}", issue_request.title);
    println!("   Labels: {:?}", issue_request.labels);
    println!("   Auto-linked issues: 2");
    println!("   Use: Automatically file high-confidence bugs");
}

// Note: This is a demonstration file showing API usage.
// Actual integration would involve:
// 1. Adding RuchyRuchy as a dependency to Cargo.toml
// 2. Setting up pre-commit hooks for continuous validation
// 3. Configuring GitHub tokens for automated bug filing
// 4. Writing project-specific property tests
// 5. Integrating into CI/CD pipelines
//
// See INTEGRATION_GUIDE.md for complete integration instructions.
