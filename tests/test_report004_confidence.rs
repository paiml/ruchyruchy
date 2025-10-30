// REPORT-004: Confidence Scoring System (INTEGRATION TESTS)
//
// Tests for Jidoka-based confidence scoring to prevent alert fatigue.
//
// Requirements (from roadmap):
// - 4-factor confidence formula
// - Discovery method weights
// - Reproducibility scoring
// - Quantitative evidence scoring
// - Root cause clarity scoring
// - Prioritization thresholds
//
// Expected behavior:
// - Confidence score 0.0-1.0 from 4 weighted factors
// - Discovery methods weighted: property testing (0.9), fuzz testing (0.85),
//   manual testing (0.7), user report (0.5)
// - Reproducibility: always (1.0), often (0.8), sometimes (0.5), rarely (0.2)
// - Quantitative evidence: strong metrics (0.9), moderate (0.6), weak (0.3)
// - Root cause clarity: confirmed (1.0), likely (0.7), unclear (0.3)
// - Prioritization: CRITICAL (>0.8), HIGH (>0.6), MEDIUM (>0.4), LOW (<=0.4)
//
// Research grounding: Ohno (1988) Jidoka principle - intelligent automation

use ruchyruchy::bug_reporting::confidence::{
    ConfidenceScore, ConfidenceScorer, DiscoveryMethod, Priority, QuantitativeEvidence,
    Reproducibility, RootCauseClarity,
};

/// Test: 4-Factor Confidence Formula
///
/// This test verifies that confidence scores are calculated correctly
/// using the weighted 4-factor formula:
///
/// confidence = 0.30 * discovery_weight
///            + 0.30 * reproducibility_weight
///            + 0.25 * quantitative_evidence_weight
///            + 0.15 * root_cause_clarity_weight
///
/// Weights add to 1.0, ensuring score stays in [0.0, 1.0] range.
#[test]
fn test_four_factor_confidence_formula() {
    // High confidence bug: property testing, always reproducible, strong evidence, confirmed cause
    let high_confidence = ConfidenceScore::new(
        DiscoveryMethod::PropertyTesting, // 0.9 weight
        Reproducibility::Always,           // 1.0 weight
        QuantitativeEvidence::Strong,      // 0.9 weight
        RootCauseClarity::Confirmed,       // 1.0 weight
    );

    // Expected: 0.30*0.9 + 0.30*1.0 + 0.25*0.9 + 0.15*1.0
    //         = 0.27 + 0.30 + 0.225 + 0.15 = 0.945
    let score = high_confidence.calculate();
    assert!(
        score > 0.9 && score <= 1.0,
        "High confidence should score >0.9 (got {})",
        score
    );
    assert!(
        score >= 0.0 && score <= 1.0,
        "Score must be in [0, 1] range (got {})",
        score
    );

    // Low confidence bug: user report, rarely reproducible, weak evidence, unclear cause
    let low_confidence = ConfidenceScore::new(
        DiscoveryMethod::UserReport, // 0.5 weight
        Reproducibility::Rarely,     // 0.2 weight
        QuantitativeEvidence::Weak,  // 0.3 weight
        RootCauseClarity::Unclear,   // 0.3 weight
    );

    // Expected: 0.30*0.5 + 0.30*0.2 + 0.25*0.3 + 0.15*0.3
    //         = 0.15 + 0.06 + 0.075 + 0.045 = 0.33
    let score = low_confidence.calculate();
    assert!(
        score < 0.5,
        "Low confidence should score <0.5 (got {})",
        score
    );

    // Medium confidence bug: fuzz testing, often reproducible, moderate evidence, likely cause
    let medium_confidence = ConfidenceScore::new(
        DiscoveryMethod::FuzzTesting,      // 0.85 weight
        Reproducibility::Often,            // 0.8 weight
        QuantitativeEvidence::Moderate,    // 0.6 weight
        RootCauseClarity::Likely,          // 0.7 weight
    );

    // Expected: 0.30*0.85 + 0.30*0.8 + 0.25*0.6 + 0.15*0.7
    //         = 0.255 + 0.24 + 0.15 + 0.105 = 0.75
    let score = medium_confidence.calculate();
    assert!(
        score > 0.6 && score < 0.9,
        "Medium confidence should score between 0.6 and 0.9 (got {})",
        score
    );
}

/// Test: Discovery Method Weights
///
/// This test verifies that different discovery methods have appropriate weights
/// based on their reliability and thoroughness.
///
/// Ranking (highest to lowest):
/// 1. PropertyTesting (0.9) - Mathematical properties verified
/// 2. FuzzTesting (0.85) - Large input space coverage
/// 3. ManualTesting (0.7) - Human-guided testing
/// 4. UserReport (0.5) - Real-world but potentially unclear
#[test]
fn test_discovery_method_weights() {
    // Property testing: highest confidence
    assert_eq!(DiscoveryMethod::PropertyTesting.weight(), 0.9);
    assert_eq!(
        DiscoveryMethod::PropertyTesting.as_str(),
        "Property Testing"
    );

    // Fuzz testing: very high confidence
    assert_eq!(DiscoveryMethod::FuzzTesting.weight(), 0.85);
    assert_eq!(DiscoveryMethod::FuzzTesting.as_str(), "Fuzz Testing");

    // Manual testing: moderate confidence
    assert_eq!(DiscoveryMethod::ManualTesting.weight(), 0.7);
    assert_eq!(DiscoveryMethod::ManualTesting.as_str(), "Manual Testing");

    // User report: lower confidence (may be imprecise)
    assert_eq!(DiscoveryMethod::UserReport.weight(), 0.5);
    assert_eq!(DiscoveryMethod::UserReport.as_str(), "User Report");

    // Verify ordering
    assert!(DiscoveryMethod::PropertyTesting.weight() > DiscoveryMethod::FuzzTesting.weight());
    assert!(DiscoveryMethod::FuzzTesting.weight() > DiscoveryMethod::ManualTesting.weight());
    assert!(DiscoveryMethod::ManualTesting.weight() > DiscoveryMethod::UserReport.weight());
}

/// Test: Reproducibility Scoring
///
/// This test verifies that reproducibility levels are weighted appropriately.
/// Reproducibility is critical for debugging and validation.
///
/// Levels:
/// - Always (1.0): 100% reproducible
/// - Often (0.8): >75% reproducible
/// - Sometimes (0.5): 25-75% reproducible
/// - Rarely (0.2): <25% reproducible
#[test]
fn test_reproducibility_scoring() {
    // Always reproducible: highest weight
    assert_eq!(Reproducibility::Always.weight(), 1.0);
    assert_eq!(Reproducibility::Always.as_str(), "Always");

    // Often reproducible: high weight
    assert_eq!(Reproducibility::Often.weight(), 0.8);
    assert_eq!(Reproducibility::Often.as_str(), "Often (>75%)");

    // Sometimes reproducible: medium weight
    assert_eq!(Reproducibility::Sometimes.weight(), 0.5);
    assert_eq!(Reproducibility::Sometimes.as_str(), "Sometimes (25-75%)");

    // Rarely reproducible: low weight (heisenbugs)
    assert_eq!(Reproducibility::Rarely.weight(), 0.2);
    assert_eq!(Reproducibility::Rarely.as_str(), "Rarely (<25%)");

    // Verify ordering
    assert!(Reproducibility::Always.weight() > Reproducibility::Often.weight());
    assert!(Reproducibility::Often.weight() > Reproducibility::Sometimes.weight());
    assert!(Reproducibility::Sometimes.weight() > Reproducibility::Rarely.weight());
}

/// Test: Quantitative Evidence Scoring
///
/// This test verifies that quantitative evidence strength is weighted correctly.
/// Evidence includes metrics like crash dumps, performance data, memory leaks, etc.
///
/// Levels:
/// - Strong (0.9): Multiple metrics, clear patterns
/// - Moderate (0.6): Some metrics, suggestive patterns
/// - Weak (0.3): Minimal metrics, unclear patterns
#[test]
fn test_quantitative_evidence_scoring() {
    // Strong evidence: multiple metrics confirm bug
    assert_eq!(QuantitativeEvidence::Strong.weight(), 0.9);
    assert_eq!(QuantitativeEvidence::Strong.as_str(), "Strong");

    // Moderate evidence: some metrics present
    assert_eq!(QuantitativeEvidence::Moderate.weight(), 0.6);
    assert_eq!(QuantitativeEvidence::Moderate.as_str(), "Moderate");

    // Weak evidence: minimal supporting data
    assert_eq!(QuantitativeEvidence::Weak.weight(), 0.3);
    assert_eq!(QuantitativeEvidence::Weak.as_str(), "Weak");

    // Verify ordering
    assert!(QuantitativeEvidence::Strong.weight() > QuantitativeEvidence::Moderate.weight());
    assert!(QuantitativeEvidence::Moderate.weight() > QuantitativeEvidence::Weak.weight());
}

/// Test: Root Cause Clarity Scoring
///
/// This test verifies that root cause understanding is weighted appropriately.
/// Clear root causes enable faster fixes and better prevention.
///
/// Levels:
/// - Confirmed (1.0): Root cause proven
/// - Likely (0.7): Root cause hypothesis with evidence
/// - Unclear (0.3): Root cause unknown or speculative
#[test]
fn test_root_cause_clarity_scoring() {
    // Confirmed: root cause proven
    assert_eq!(RootCauseClarity::Confirmed.weight(), 1.0);
    assert_eq!(RootCauseClarity::Confirmed.as_str(), "Confirmed");

    // Likely: strong hypothesis
    assert_eq!(RootCauseClarity::Likely.weight(), 0.7);
    assert_eq!(RootCauseClarity::Likely.as_str(), "Likely");

    // Unclear: needs investigation
    assert_eq!(RootCauseClarity::Unclear.weight(), 0.3);
    assert_eq!(RootCauseClarity::Unclear.as_str(), "Unclear");

    // Verify ordering
    assert!(RootCauseClarity::Confirmed.weight() > RootCauseClarity::Likely.weight());
    assert!(RootCauseClarity::Likely.weight() > RootCauseClarity::Unclear.weight());
}

/// Test: Prioritization Thresholds
///
/// This test verifies that bugs are correctly prioritized based on confidence score.
/// This prevents alert fatigue by focusing attention on high-confidence bugs.
///
/// Thresholds:
/// - CRITICAL: score > 0.8 (immediate action required)
/// - HIGH: score > 0.6 (prioritize soon)
/// - MEDIUM: score > 0.4 (address in normal flow)
/// - LOW: score <= 0.4 (needs more investigation)
#[test]
fn test_prioritization_thresholds() {
    // Critical priority: very high confidence (>0.8)
    let critical = ConfidenceScore::new(
        DiscoveryMethod::PropertyTesting,
        Reproducibility::Always,
        QuantitativeEvidence::Strong,
        RootCauseClarity::Confirmed,
    );
    assert_eq!(critical.priority(), Priority::Critical);
    assert_eq!(Priority::Critical.as_str(), "CRITICAL");
    assert!(critical.calculate() > 0.8);

    // High priority: high confidence (>0.6)
    let high = ConfidenceScore::new(
        DiscoveryMethod::FuzzTesting,
        Reproducibility::Often,
        QuantitativeEvidence::Moderate,
        RootCauseClarity::Likely,
    );
    assert_eq!(high.priority(), Priority::High);
    assert_eq!(Priority::High.as_str(), "HIGH");
    let score = high.calculate();
    assert!(score > 0.6 && score <= 0.8);

    // Medium priority: moderate confidence (>0.4)
    let medium = ConfidenceScore::new(
        DiscoveryMethod::ManualTesting,
        Reproducibility::Sometimes,
        QuantitativeEvidence::Weak,
        RootCauseClarity::Likely,
    );
    assert_eq!(medium.priority(), Priority::Medium);
    assert_eq!(Priority::Medium.as_str(), "MEDIUM");
    let score = medium.calculate();
    assert!(score > 0.4 && score <= 0.6);

    // Low priority: low confidence (<=0.4)
    let low = ConfidenceScore::new(
        DiscoveryMethod::UserReport,
        Reproducibility::Rarely,
        QuantitativeEvidence::Weak,
        RootCauseClarity::Unclear,
    );
    assert_eq!(low.priority(), Priority::Low);
    assert_eq!(Priority::Low.as_str(), "LOW");
    assert!(low.calculate() <= 0.4);
}

/// Test: Confidence Scorer Integration
///
/// This test verifies the complete confidence scoring workflow:
/// 1. Create bug report with all 4 factors
/// 2. Calculate confidence score
/// 3. Determine priority
/// 4. Generate explanation
#[test]
fn test_confidence_scorer_integration() {
    let scorer = ConfidenceScorer::new();

    // Test realistic bug scenarios

    // Scenario 1: Parser crash found by property testing
    let parser_bug = scorer.score(
        DiscoveryMethod::PropertyTesting, // Found by QuickCheck
        Reproducibility::Always,          // Crashes every time
        QuantitativeEvidence::Strong,     // Stack trace + AST dump
        RootCauseClarity::Confirmed,      // Off-by-one in recursion
    );

    assert!(parser_bug.score > 0.9, "Parser crash should be critical");
    assert_eq!(parser_bug.priority(), Priority::Critical);
    let explanation = parser_bug.explain();
    assert!(explanation.contains("Property Testing"));
    assert!(explanation.contains("Always"));
    assert!(explanation.contains("Strong"));
    assert!(explanation.contains("Confirmed"));

    // Scenario 2: Performance regression detected by benchmarks
    let perf_bug = scorer.score(
        DiscoveryMethod::FuzzTesting,    // Found by fuzz benchmarks
        Reproducibility::Often,          // 80% reproducible
        QuantitativeEvidence::Moderate,  // Timing data shows slowdown
        RootCauseClarity::Likely,        // Hypothesis: algorithm changed
    );

    let score = perf_bug.score;
    assert!(
        score > 0.6 && score <= 0.8,
        "Performance bug should be high priority"
    );
    assert_eq!(perf_bug.priority(), Priority::High);

    // Scenario 3: User-reported intermittent issue
    let user_bug = scorer.score(
        DiscoveryMethod::UserReport,   // User complained
        Reproducibility::Rarely,       // Can't reproduce locally
        QuantitativeEvidence::Weak,    // No crash logs
        RootCauseClarity::Unclear,     // Need more investigation
    );

    assert!(
        user_bug.score <= 0.4,
        "Unclear user report should be low priority"
    );
    assert_eq!(user_bug.priority(), Priority::Low);

    // Verify all scores are in valid range
    for bug in [parser_bug, perf_bug, user_bug] {
        assert!(
            bug.score >= 0.0 && bug.score <= 1.0,
            "Score must be in [0, 1] range"
        );
    }
}

/// Test: Confidence Score Properties
///
/// This test verifies mathematical properties of the confidence scoring system:
/// - Monotonicity: Better factors → higher score
/// - Symmetry: Factor order doesn't matter (uses commutative weighted sum)
/// - Bounds: Score always in [0.0, 1.0]
#[test]
fn test_confidence_score_properties() {
    let scorer = ConfidenceScorer::new();

    // Monotonicity: Improving any factor increases score
    let base = scorer.score(
        DiscoveryMethod::ManualTesting,
        Reproducibility::Sometimes,
        QuantitativeEvidence::Weak,
        RootCauseClarity::Unclear,
    );

    let better_discovery = scorer.score(
        DiscoveryMethod::PropertyTesting, // Improved
        Reproducibility::Sometimes,
        QuantitativeEvidence::Weak,
        RootCauseClarity::Unclear,
    );
    assert!(better_discovery.score > base.score);

    let better_repro = scorer.score(
        DiscoveryMethod::ManualTesting,
        Reproducibility::Always, // Improved
        QuantitativeEvidence::Weak,
        RootCauseClarity::Unclear,
    );
    assert!(better_repro.score > base.score);

    let better_evidence = scorer.score(
        DiscoveryMethod::ManualTesting,
        Reproducibility::Sometimes,
        QuantitativeEvidence::Strong, // Improved
        RootCauseClarity::Unclear,
    );
    assert!(better_evidence.score > base.score);

    let better_clarity = scorer.score(
        DiscoveryMethod::ManualTesting,
        Reproducibility::Sometimes,
        QuantitativeEvidence::Weak,
        RootCauseClarity::Confirmed, // Improved
    );
    assert!(better_clarity.score > base.score);

    // Bounds: All scores in [0, 1]
    let worst = scorer.score(
        DiscoveryMethod::UserReport,
        Reproducibility::Rarely,
        QuantitativeEvidence::Weak,
        RootCauseClarity::Unclear,
    );
    assert!(worst.score >= 0.0 && worst.score <= 1.0);

    let best = scorer.score(
        DiscoveryMethod::PropertyTesting,
        Reproducibility::Always,
        QuantitativeEvidence::Strong,
        RootCauseClarity::Confirmed,
    );
    assert!(best.score >= 0.0 && best.score <= 1.0);
    assert!(best.score > worst.score);
}
