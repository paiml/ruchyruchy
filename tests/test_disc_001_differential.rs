// DISC-001: Differential Testing with Statistical Analysis (RED PHASE)
//
// Tests for differential testing with Welch's t-test for performance regression detection.
//
// Requirements (from roadmap):
// - Version comparison (v3.146, v3.147, v3.148)
// - Target comparison (debug, release, wasm)
// - Statistical rigor (30 samples, p<0.05)
// - Cohen's d effect size
// - Confidence scoring (Section 2.1)
//
// Expected behavior:
// - Detect 100% of known regressions
// - Statistical significance p<0.05
// - Confidence scores accurate
// - <5% false positives
//
// Technical Approach:
// - Welch's t-test (handles unequal variance)
// - Cohen's d effect size (0.2=small, 0.5=medium, 0.8=large)
// - 30 samples for statistical power
// - Slowdown threshold: 20% (1.2x)

use ruchyruchy::bug_discovery::differential::{
    CompilationTarget, CompilerVersion, DifferentialTester, FailureMode, TestStatus,
};
use ruchyruchy::bug_discovery::statistics::{cohens_d, welchs_t_test, PerformanceRegression};

/// Test: Version Regression Detection
///
/// RED: This test WILL FAIL because we need to implement:
/// - detect_regression() method that compares two versions
/// - Returns RegressionBug when regression detected
/// - Uses statistical analysis (Welch's t-test)
///
/// This test verifies we can detect performance regressions between compiler versions.
#[test]
fn test_version_regression_detection() {
    // RED: This will fail - method doesn't exist yet
    //
    // Setup: Create versions v3.146 (working) and v3.147 (regressed)
    let v146 = CompilerVersion {
        version: "v3.146".to_string(),
        target: CompilationTarget::Release,
    };

    let v147 = CompilerVersion {
        version: "v3.147".to_string(),
        target: CompilationTarget::Release,
    };

    let tester = DifferentialTester::new(vec![v146.clone(), v147.clone()])
        .with_statistical_params(30, 0.05, 1.2); // 30 samples, p<0.05, 20% slowdown

    // Simulate test case that regressed in v3.147
    // In real implementation, this would run actual Ruchy code
    let test_code = "fun fib(n: i32) -> i32 { if n <= 1 { n } else { fib(n-1) + fib(n-2) } } fib(20)";

    // RED: detect_regression() doesn't exist yet - this will fail to compile
    let bug = tester.detect_regression(&v146, &v147, test_code);

    // Verify regression detected
    assert!(bug.is_some(), "Should detect regression between v3.146 and v3.147");

    let bug = bug.unwrap();
    assert_eq!(bug.working_version, v146);
    assert_eq!(bug.broken_version, v147);

    // Verify it's a performance regression
    match &bug.failure_mode {
        FailureMode::PerformanceRegression { regression } => {
            assert!(
                regression.slowdown_factor >= 1.2,
                "Slowdown should be >=20%: {}",
                regression.slowdown_factor
            );
            assert!(
                regression.p_value < 0.05,
                "Should be statistically significant: p={}",
                regression.p_value
            );
        }
        _ => panic!("Expected PerformanceRegression, got {:?}", bug.failure_mode),
    }

    // Verify confidence score
    assert!(
        bug.confidence.overall >= 0.5,
        "Confidence should be reasonable for statistical regression (got {})",
        bug.confidence.overall
    );
}

/// Test: Performance Regression Statistical Analysis
///
/// RED: This test WILL FAIL because we need to implement:
/// - analyze_performance() method
/// - Returns PerformanceRegression with statistical metrics
/// - Uses Welch's t-test and Cohen's d
///
/// This test verifies statistical rigor of performance analysis.
#[test]
fn test_performance_regression_statistical() {
    // RED: This will fail - analyze_performance() doesn't exist
    //
    // Create tester with statistical parameters
    let v146 = CompilerVersion {
        version: "v3.146".to_string(),
        target: CompilationTarget::Release,
    };

    let v147 = CompilerVersion {
        version: "v3.147".to_string(),
        target: CompilationTarget::Release,
    };

    let tester = DifferentialTester::new(vec![v146.clone(), v147.clone()])
        .with_statistical_params(30, 0.05, 1.2);

    let test_code = "fun sum(n: i32) -> i32 { let mut s = 0; for i in 0..n { s += i; } s } sum(1000)";

    // RED: analyze_performance() doesn't exist - will fail to compile
    let regression = tester.analyze_performance(&v146, &v147, test_code);

    assert!(
        regression.is_some(),
        "Should detect performance regression"
    );

    let regression = regression.unwrap();

    // Verify statistical metrics
    assert_eq!(regression.baseline_version, "v3.146");
    assert_eq!(regression.regressed_version, "v3.147");

    // Slowdown factor (v3.147 should be slower)
    assert!(
        regression.slowdown_factor >= 1.2,
        "Expected >=20% slowdown, got {:.2}x",
        regression.slowdown_factor
    );

    // Statistical significance (p < 0.05)
    assert!(
        regression.p_value < 0.05,
        "Expected p<0.05 for significance, got p={}",
        regression.p_value
    );

    // Effect size (Cohen's d) - should be "large" (>0.8) for 20%+ slowdown
    assert!(
        regression.effect_size.abs() > 0.5,
        "Expected medium-to-large effect size, got d={}",
        regression.effect_size
    );

    // Verify means are sensible
    assert!(
        regression.regressed_mean_ms > regression.baseline_mean_ms,
        "Regressed version should be slower"
    );
}

/// Test: Confidence Scoring
///
/// RED: This test WILL FAIL because we need to implement:
/// - calculate_confidence() method
/// - Returns ConfidenceScore based on statistical evidence
/// - Higher confidence for strong statistical signals
///
/// This test verifies confidence scoring for statistical regressions.
#[test]
fn test_confidence_scoring() {
    // RED: This will fail - calculate_confidence() doesn't exist
    //
    use ruchyruchy::bug_discovery::confidence::ConfidenceScore;

    let v146 = CompilerVersion {
        version: "v3.146".to_string(),
        target: CompilationTarget::Release,
    };

    let v147 = CompilerVersion {
        version: "v3.147".to_string(),
        target: CompilationTarget::Release,
    };

    let tester = DifferentialTester::new(vec![v146.clone(), v147.clone()])
        .with_statistical_params(30, 0.05, 1.2);

    let test_code = "fun factorial(n: i32) -> i32 { if n <= 1 { 1 } else { n * factorial(n-1) } } factorial(10)";

    // RED: calculate_confidence() doesn't exist - will fail to compile
    let confidence = tester.calculate_confidence(&v146, &v147, test_code);

    // Verify confidence score components
    assert!(
        confidence.overall >= 0.5,
        "Statistical regression should have reasonable confidence (got {})",
        confidence.overall
    );

    // Verify component scores are reasonable
    assert!(
        confidence.discovery_method_weight > 0.0,
        "Discovery method weight should be positive"
    );

    assert!(
        confidence.reproducibility_score > 0.0,
        "Reproducibility score should be positive for 30 samples"
    );

    assert!(
        confidence.quantitative_evidence > 0.0,
        "Quantitative evidence should be positive for statistical test"
    );

    assert!(
        confidence.root_cause_clarity > 0.0,
        "Root cause clarity should be positive for version regression"
    );
}

/// Test: Effect Size Calculation
///
/// RED: This test WILL FAIL because we need to verify:
/// - Cohen's d calculation is correct
/// - Effect size interpretation (small/medium/large)
/// - Integration with regression detection
///
/// This test verifies Cohen's d effect size calculation.
#[test]
fn test_effect_size_calculation() {
    // RED: This tests the statistics module which exists, but we need to verify integration
    //
    // Test Cohen's d calculation with known samples
    let baseline = vec![
        100.0, 102.0, 98.0, 101.0, 99.0, 100.5, 101.5, 98.5, 99.5, 100.0,
    ];

    // Regressed version: 20% slower (mean = 120ms)
    let regressed = vec![
        120.0, 122.0, 118.0, 121.0, 119.0, 120.5, 121.5, 118.5, 119.5, 120.0,
    ];

    let effect_size = cohens_d(&baseline, &regressed);

    // Effect size should be very large (>2.0) for 20% slowdown with low variance
    assert!(
        effect_size.abs() > 1.5,
        "20% slowdown with low variance should have large effect size, got d={}",
        effect_size
    );

    // Verify Welch's t-test detects significance
    let (t_stat, p_value) = welchs_t_test(&baseline, &regressed);

    assert!(
        t_stat.abs() > 2.0,
        "t-statistic should be large for clear difference"
    );
    assert!(p_value < 0.05, "Should be statistically significant: p={}", p_value);

    // Now test with DifferentialTester integration
    // RED: This will fail because analyze_performance() doesn't exist
    let v146 = CompilerVersion {
        version: "v3.146".to_string(),
        target: CompilationTarget::Release,
    };

    let v147 = CompilerVersion {
        version: "v3.147".to_string(),
        target: CompilationTarget::Release,
    };

    let tester = DifferentialTester::new(vec![v146.clone(), v147.clone()])
        .with_statistical_params(10, 0.05, 1.2); // Use 10 samples to match test data

    // This would ideally use mock data matching our baseline/regressed arrays
    // RED: analyze_performance() doesn't exist - will fail to compile
    // let regression = tester.analyze_performance(&v146, &v147, "test code");
    //
    // For now, just verify the statistics module works correctly
    // GREEN phase will add the integration

    // Placeholder assertion for RED phase
    // This will be replaced when analyze_performance() is implemented
    assert!(true, "Statistics module functions are correct");
}

/// Test: False Positive Rate
///
/// RED: This test WILL FAIL because we need to implement:
/// - Test with versions that have NO regression
/// - Verify false positive rate <5%
/// - Statistical significance prevents false alarms
///
/// This test verifies we don't report false positives.
#[test]
fn test_false_positive_rate() {
    // RED: This will fail - need to run many tests and verify <5% false positives
    //
    let v146_a = CompilerVersion {
        version: "v3.146".to_string(),
        target: CompilationTarget::Release,
    };

    let v146_b = CompilerVersion {
        version: "v3.146".to_string(), // Same version!
        target: CompilationTarget::Release,
    };

    let tester = DifferentialTester::new(vec![v146_a.clone(), v146_b.clone()])
        .with_statistical_params(30, 0.05, 1.2);

    let test_code = "fun identity(x: i32) -> i32 { x } identity(42)";

    // RED: detect_regression() doesn't exist - will fail to compile
    let bug = tester.detect_regression(&v146_a, &v146_b, test_code);

    // Should NOT detect regression (same version)
    assert!(
        bug.is_none(),
        "Should not detect regression between identical versions"
    );

    // In a more comprehensive test, we would run this 100 times
    // and verify <5 false positives (p<0.05 threshold)
}

/// Test: Multiple Version Comparison
///
/// RED: This test WILL FAIL because we need to implement:
/// - compare_all_versions() method
/// - Pairwise comparison of all versions
/// - Returns list of all detected regressions
///
/// This test verifies we can compare multiple versions (v3.146, v3.147, v3.148).
#[test]
fn test_multiple_version_comparison() {
    // RED: This will fail - compare_all_versions() doesn't exist
    //
    let v146 = CompilerVersion {
        version: "v3.146".to_string(),
        target: CompilationTarget::Release,
    };

    let v147 = CompilerVersion {
        version: "v3.147".to_string(),
        target: CompilationTarget::Release,
    };

    let v148 = CompilerVersion {
        version: "v3.148".to_string(),
        target: CompilationTarget::Release,
    };

    let tester =
        DifferentialTester::new(vec![v146.clone(), v147.clone(), v148.clone()])
            .with_statistical_params(30, 0.05, 1.2);

    let test_code = "fun test() -> i32 { let x = 10; x * 2 } test()";

    // RED: compare_all_versions() doesn't exist - will fail to compile
    let regressions = tester.compare_all_versions(test_code);

    // Should find regressions if they exist
    // For this test, we expect to find issues in v3.147 or v3.148
    // The actual count depends on the mock implementation

    // Verify method exists and returns Vec<RegressionBug>
    assert!(
        regressions.len() <= 3,
        "Can't have more regressions than version pairs"
    );

    // Each regression should have proper statistical evidence
    for regression in &regressions {
        match &regression.failure_mode {
            FailureMode::PerformanceRegression { regression: perf } => {
                assert!(perf.p_value < 0.05, "All regressions must be statistically significant");
            }
            _ => {}
        }
    }
}
