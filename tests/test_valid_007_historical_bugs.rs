// VALID-007: Historical Bug Validation (INTEGRATION TESTS)
//
// Tests for validating bug discovery system against historical bugs.
//
// Requirements (from roadmap):
// - Historical bug corpus (79 bugs)
// - Detection rate measurement (target: 95%+)
// - False positive analysis (<5%)
// - Critical bug tracking
// - Markdown report generation
//
// Expected behavior:
// - Load historical bugs from GitHub issues
// - Run detection algorithm on each bug
// - Calculate detection rate: detected / total
// - Calculate FP rate: false_positives / detected
// - Track critical bugs separately
// - Generate comprehensive markdown report
// - Verify targets: 95%+ detection, <5% FP
//
// Validation Strategy:
// - Create bug corpus with known properties
// - Test with perfect, good, and poor detectors
// - Verify metrics calculation accuracy
// - Test report generation formatting
// - Validate target checking logic

use ruchyruchy::bug_reporting::validation::{
    BugCategory, BugCorpusValidator, DetectionResult, HistoricalBug, ValidationMetrics,
    ValidationReport,
};

/// Test: Historical Bug Corpus Creation
///
/// This test verifies that a bug corpus can be created and populated:
/// - Add historical bugs with metadata
/// - Query by category
/// - Filter by critical flag
/// - Count bugs
#[test]
fn test_historical_bug_corpus_creation() {
    let mut validator = BugCorpusValidator::new();

    // Start with empty corpus
    assert_eq!(validator.bug_count(), 0);

    // Add various bug types
    let mut crash_bug = HistoricalBug::new(
        101,
        "Parser crashes on recursive types".to_string(),
        "The parser encounters stack overflow on deeply nested types".to_string(),
        BugCategory::Crash,
    );
    crash_bug.add_file("src/parser/type_parser.rs".to_string());
    crash_bug.set_error_message("thread 'main' panicked at 'stack overflow'".to_string());
    crash_bug.add_label("bug".to_string());
    crash_bug.add_label("severity:critical".to_string());
    crash_bug.set_critical(true);

    let mut hang_bug = HistoricalBug::new(
        102,
        "Lexer hangs on unterminated string".to_string(),
        "Lexer enters infinite loop when string literal is not terminated".to_string(),
        BugCategory::Hang,
    );
    hang_bug.add_file("src/lexer/string_lexer.rs".to_string());
    hang_bug.set_critical(false);

    let type_error_bug = HistoricalBug::new(
        103,
        "Type inference fails on generic functions".to_string(),
        "Type checker cannot infer types for generic function calls".to_string(),
        BugCategory::TypeError,
    );

    validator.add_bug(crash_bug);
    validator.add_bug(hang_bug);
    validator.add_bug(type_error_bug);

    // Verify corpus populated
    assert_eq!(validator.bug_count(), 3);

    // Query by category
    let crashes = validator.bugs_by_category(&BugCategory::Crash);
    assert_eq!(crashes.len(), 1);
    assert_eq!(crashes[0].issue_number, 101);

    let hangs = validator.bugs_by_category(&BugCategory::Hang);
    assert_eq!(hangs.len(), 1);
    assert_eq!(hangs[0].issue_number, 102);

    // Query critical bugs
    let critical = validator.critical_bugs();
    assert_eq!(critical.len(), 1);
    assert_eq!(critical[0].issue_number, 101);
}

/// Test: Complete Validation Workflow
///
/// This test verifies the full validation workflow:
/// - Create bug corpus
/// - Run detector function on each bug
/// - Collect detection results
/// - Generate validation report with metrics
#[test]
fn test_validation_workflow_complete() {
    let mut validator = BugCorpusValidator::new();

    // Add 10 bugs (5 crashes, 3 hangs, 2 type errors)
    for i in 1..=5 {
        validator.add_bug(HistoricalBug::new(
            i,
            format!("Crash bug {}", i),
            "Description".to_string(),
            BugCategory::Crash,
        ));
    }

    for i in 6..=8 {
        validator.add_bug(HistoricalBug::new(
            i,
            format!("Hang bug {}", i),
            "Description".to_string(),
            BugCategory::Hang,
        ));
    }

    for i in 9..=10 {
        validator.add_bug(HistoricalBug::new(
            i,
            format!("Type error {}", i),
            "Description".to_string(),
            BugCategory::TypeError,
        ));
    }

    // Detector that catches crashes and hangs, but not type errors
    let detector = |bug: &HistoricalBug| match bug.category {
        BugCategory::Crash => {
            DetectionResult::detected("Differential Testing".to_string(), 0.95)
        }
        BugCategory::Hang => DetectionResult::detected("Timeout Detection".to_string(), 0.90),
        _ => DetectionResult::missed("No type checking validation".to_string()),
    };

    let report = validator.validate(detector);

    // Verify metrics
    assert_eq!(report.metrics.total_bugs, 10);
    assert_eq!(report.metrics.detected, 8); // 5 crashes + 3 hangs
    assert_eq!(report.metrics.missed, 2); // 2 type errors
    assert!((report.metrics.detection_rate - 0.8).abs() < 0.01); // 80%

    // Verify detection results recorded
    assert_eq!(report.results.len(), 10);

    // Verify missed bugs tracked
    assert_eq!(report.missed_bugs.len(), 2);
    assert!(report.missed_bugs.contains(&9));
    assert!(report.missed_bugs.contains(&10));
}

/// Test: Detection Rate Calculation
///
/// This test verifies detection rate calculation accuracy:
/// - Perfect detection (100%)
/// - Good detection (95%+)
/// - Poor detection (<90%)
/// - Edge cases (0%, empty corpus)
#[test]
fn test_detection_rate_calculation() {
    // Test 1: Perfect detection (100%)
    let mut validator = BugCorpusValidator::new();
    for i in 1..=100 {
        validator.add_bug(HistoricalBug::new(
            i,
            format!("Bug {}", i),
            "Description".to_string(),
            BugCategory::Crash,
        ));
    }

    let perfect_detector = |_: &HistoricalBug| {
        DetectionResult::detected("Perfect detector".to_string(), 1.0)
    };

    let perfect_report = validator.validate(perfect_detector);
    assert_eq!(perfect_report.metrics.detection_rate, 1.0);
    assert_eq!(perfect_report.metrics.detected, 100);
    assert_eq!(perfect_report.metrics.missed, 0);
    assert!(perfect_report.metrics.meets_target());

    // Test 2: Good detection (96%)
    let good_detector = |bug: &HistoricalBug| {
        if bug.issue_number <= 96 {
            DetectionResult::detected("Good detector".to_string(), 0.9)
        } else {
            DetectionResult::missed("Edge case".to_string())
        }
    };

    let good_report = validator.validate(good_detector);
    assert_eq!(good_report.metrics.detection_rate, 0.96);
    assert_eq!(good_report.metrics.detected, 96);
    assert_eq!(good_report.metrics.missed, 4);
    assert!(good_report.metrics.meets_target());

    // Test 3: Poor detection (80%)
    let poor_detector = |bug: &HistoricalBug| {
        if bug.issue_number <= 80 {
            DetectionResult::detected("Poor detector".to_string(), 0.7)
        } else {
            DetectionResult::missed("Many misses".to_string())
        }
    };

    let poor_report = validator.validate(poor_detector);
    assert_eq!(poor_report.metrics.detection_rate, 0.80);
    assert_eq!(poor_report.metrics.detected, 80);
    assert_eq!(poor_report.metrics.missed, 20);
    assert!(!poor_report.metrics.meets_target()); // Below 95% target

    // Test 4: Zero detection (0%)
    let zero_detector = |_: &HistoricalBug| DetectionResult::missed("Never detects".to_string());

    let zero_report = validator.validate(zero_detector);
    assert_eq!(zero_report.metrics.detection_rate, 0.0);
    assert_eq!(zero_report.metrics.detected, 0);
    assert_eq!(zero_report.metrics.missed, 100);
    assert!(!zero_report.metrics.meets_target());
}

/// Test: False Positive Tracking
///
/// This test verifies false positive rate calculation:
/// - FP rate = false_positives / detected
/// - Target: <5% false positive rate
/// - Track FP details in report
#[test]
fn test_false_positive_tracking() {
    // Test 1: Low FP rate (good)
    let metrics_good = ValidationMetrics::new(
        100, // total bugs
        96,  // detected
        2,   // false positives
        48,  // critical detected
        50,  // critical total
    );

    assert!((metrics_good.false_positive_rate - 0.0208).abs() < 0.01); // 2/96 = ~2.08%
    assert!(metrics_good.meets_target()); // <5% FP, 96% detection

    // Test 2: High FP rate (bad)
    let metrics_bad = ValidationMetrics::new(
        100, // total bugs
        96,  // detected
        10,  // false positives
        48,  // critical detected
        50,  // critical total
    );

    assert!((metrics_bad.false_positive_rate - 0.1042).abs() < 0.01); // 10/96 = ~10.42%
    assert!(!metrics_bad.meets_target()); // >5% FP fails target

    // Test 3: Track FP details in report
    let mut report = ValidationReport::new(metrics_good);
    report.add_false_positive(
        "Reported crash in test code (not production)".to_string(),
    );
    report.add_false_positive(
        "Duplicate of existing issue #42".to_string(),
    );

    assert_eq!(report.false_positive_details.len(), 2);
    assert!(report
        .false_positive_details[0]
        .contains("test code"));
    assert!(report.false_positive_details[1].contains("Duplicate"));

    // Markdown report should include FP details
    let markdown = report.to_markdown();
    assert!(markdown.contains("False Positive Details"));
    assert!(markdown.contains("test code"));
    assert!(markdown.contains("Duplicate"));
}

/// Test: Critical Bug Detection
///
/// This test verifies critical bug tracking:
/// - Separate counter for critical bugs
/// - Critical detection rate calculation
/// - Priority given to critical bugs in report
#[test]
fn test_critical_bug_detection() {
    let mut validator = BugCorpusValidator::new();

    // Add 10 bugs: 5 critical, 5 normal
    for i in 1..=10 {
        let mut bug = HistoricalBug::new(
            i,
            format!("Bug {}", i),
            "Description".to_string(),
            BugCategory::Crash,
        );

        if i <= 5 {
            bug.set_critical(true);
        }

        validator.add_bug(bug);
    }

    assert_eq!(validator.critical_bugs().len(), 5);

    // Detector that catches 4/5 critical bugs and 3/5 normal bugs
    let detector = |bug: &HistoricalBug| {
        // Miss bug #5 (critical) and bugs #8, #9 (normal)
        if bug.issue_number == 5 || bug.issue_number == 8 || bug.issue_number == 9 {
            DetectionResult::missed("Missed".to_string())
        } else {
            DetectionResult::detected("Found".to_string(), 0.9)
        }
    };

    let report = validator.validate(detector);

    // Verify overall detection
    assert_eq!(report.metrics.detected, 7); // 4 critical + 3 normal
    assert_eq!(report.metrics.missed, 3); // 1 critical + 2 normal

    // Verify critical bug tracking
    assert_eq!(report.metrics.critical_total, 5);
    assert_eq!(report.metrics.critical_detected, 4);

    // Critical detection rate: 4/5 = 80%
    let critical_rate = report.metrics.critical_detected as f64
        / report.metrics.critical_total as f64;
    assert!((critical_rate - 0.8).abs() < 0.01);
}

/// Test: Markdown Report Generation
///
/// This test verifies markdown report formatting:
/// - Report structure (headers, sections)
/// - Metrics summary
/// - Missed bugs list
/// - False positive details
/// - Target achievement status
#[test]
fn test_markdown_report_generation() {
    let mut validator = BugCorpusValidator::new();

    // Add 20 bugs
    for i in 1..=20 {
        let mut bug = HistoricalBug::new(
            i,
            format!("Bug #{}: Test issue", i),
            "Description".to_string(),
            BugCategory::Crash,
        );

        if i <= 10 {
            bug.set_critical(true);
        }

        validator.add_bug(bug);
    }

    // Detector that misses bugs 15, 18, 20
    let detector = |bug: &HistoricalBug| {
        if bug.issue_number == 15 || bug.issue_number == 18 || bug.issue_number == 20 {
            DetectionResult::missed(format!(
                "Out of scope for {} detection",
                bug.category.as_str()
            ))
        } else {
            DetectionResult::detected("Found via fuzzing".to_string(), 0.92)
        }
    };

    let mut report = validator.validate(detector);

    // Add some false positives
    report.add_false_positive("FP: Issue in test code".to_string());

    let markdown = report.to_markdown();

    // Verify report structure
    assert!(markdown.contains("# Bug Discovery Validation Report"));

    // Verify summary section
    assert!(markdown.contains("## Summary"));
    assert!(markdown.contains("Detection Rate: 85.0%")); // 17/20
    assert!(markdown.contains("False Positive Rate:"));

    // Verify detection rate section
    assert!(markdown.contains("## Detection Rate Analysis"));
    assert!(markdown.contains("**Total Historical Bugs**: 20"));
    assert!(markdown.contains("**Bugs Detected**: 17"));
    assert!(markdown.contains("**Bugs Missed**: 3"));
    assert!(markdown.contains("**Target**: 95%+ detection rate"));

    // Verify FP section
    assert!(markdown.contains("## False Positive Analysis"));
    assert!(markdown.contains("**False Positives**:"));
    assert!(markdown.contains("FP: Issue in test code"));

    // Verify missed bugs section
    assert!(markdown.contains("## Missed Bugs Analysis"));
    assert!(markdown.contains("❌ **3 bugs missed:**"));
    assert!(markdown.contains("Issue #15:"));
    assert!(markdown.contains("Issue #18:"));
    assert!(markdown.contains("Issue #20:"));
    assert!(markdown.contains("Out of scope"));

    // Verify critical bugs section
    assert!(markdown.contains("## Critical Bugs"));
    assert!(markdown.contains("**Critical Bugs Detected**: 10 / 10"));
    assert!(markdown.contains("**Critical Detection Rate**: 100.0%"));

    // Verify footer
    assert!(markdown.contains("Generated by RuchyRuchy"));
}

/// Test: Target Achievement Validation
///
/// This test verifies the 95%+ detection, <5% FP target logic:
/// - Meets target: detection >= 95%, FP < 5%
/// - Fails target: detection < 95% OR FP >= 5%
/// - Report indicates pass/fail status
#[test]
fn test_meets_target_validation() {
    // Test 1: Meets target (96% detection, 2% FP)
    let pass = ValidationMetrics::new(
        100, // total
        96,  // detected
        2,   // false positives (2/96 = 2.08% < 5%)
        48,  // critical detected
        50,  // critical total
    );

    assert!(pass.meets_target());
    assert!(pass.detection_rate >= 0.95);
    assert!(pass.false_positive_rate < 0.05);

    let pass_summary = pass.summary();
    assert!(pass_summary.contains("YES ✅"));

    // Test 2: Fails target (low detection: 90%)
    let fail_detection = ValidationMetrics::new(
        100, // total
        90,  // detected (90% < 95%)
        2,   // false positives
        45,  // critical detected
        50,  // critical total
    );

    assert!(!fail_detection.meets_target());
    assert!(fail_detection.detection_rate < 0.95);

    let fail_det_summary = fail_detection.summary();
    assert!(fail_det_summary.contains("NO ❌"));

    // Test 3: Fails target (high FP: 10%)
    let fail_fp = ValidationMetrics::new(
        100, // total
        96,  // detected
        10,  // false positives (10/96 = 10.4% > 5%)
        48,  // critical detected
        50,  // critical total
    );

    assert!(!fail_fp.meets_target());
    assert!(fail_fp.false_positive_rate >= 0.05);

    let fail_fp_summary = fail_fp.summary();
    assert!(fail_fp_summary.contains("NO ❌"));

    // Test 4: Edge case - exactly 95% detection, exactly 5% FP
    let edge = ValidationMetrics::new(
        100, // total
        95,  // detected (95% exactly)
        5,   // false positives (5/95 = 5.26% > 5%)
        47,  // critical detected
        50,  // critical total
    );

    assert!(!edge.meets_target()); // FP rate is 5.26%, which fails (<5% required)
}

/// Test: Empty Corpus Edge Case
///
/// This test verifies handling of empty bug corpus:
/// - Detection rate = 0.0 (no division by zero)
/// - No missed bugs
/// - Report generates correctly
#[test]
fn test_empty_corpus_edge_case() {
    let validator = BugCorpusValidator::new();

    assert_eq!(validator.bug_count(), 0);

    let detector = |_: &HistoricalBug| {
        DetectionResult::detected("Never called".to_string(), 1.0)
    };

    let report = validator.validate(detector);

    assert_eq!(report.metrics.total_bugs, 0);
    assert_eq!(report.metrics.detected, 0);
    assert_eq!(report.metrics.missed, 0);
    assert_eq!(report.metrics.detection_rate, 0.0); // No division by zero
    assert!(!report.metrics.meets_target()); // Can't meet target with 0 bugs

    // Report should generate without panic
    let markdown = report.to_markdown();
    assert!(markdown.contains("# Bug Discovery Validation Report"));
    assert!(markdown.contains("**Total Historical Bugs**: 0"));
}
