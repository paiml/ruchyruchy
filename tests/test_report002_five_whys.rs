// REPORT-002: Assisted Five-Whys Analysis (INTEGRATION TESTS)
//
// Tests for data-driven, human-assisted Five-Whys root cause analysis.
//
// Requirements (from roadmap):
// - Data point collection (commits, complexity, churn, SATD)
// - Hypothesis generation (data-driven with confidence)
// - Confidence scoring (HIGH/MEDIUM/LOW)
// - Human validation markers (CRITICAL: assisted, not automated)
// - Markdown output generation
//
// Expected behavior:
// - System provides objective data points
// - Suggests hypotheses with confidence levels
// - Always marks human validation required
// - Generates well-formatted markdown
//
// Research grounding: Ohno (1988) Toyota Production System, Card (2017) Agile Development

use ruchyruchy::bug_reporting::five_whys::{
    ConfidenceLevel, DataPoint, FiveWhysAnalysis, Hypothesis, WhyLayer,
};

/// Test: Data Point Collection
///
/// This test verifies that data points can be collected from various sources
/// (git commits, complexity metrics, churn analysis, SATD detection) and
/// properly stored with relevance scores.
///
/// Following Ohno's emphasis on "go and see" (Genchi Genbutsu), the system
/// must collect objective data before suggesting hypotheses.
#[test]
fn test_data_point_collection() {
    // Collect data points from different sources
    let commit_data = DataPoint::new(
        "Recent commits".to_string(),
        "15 commits in last week, 10 to parser.rs".to_string(),
        "git log".to_string(),
        0.9, // High relevance
    );

    let complexity_data = DataPoint::new(
        "Cyclomatic complexity".to_string(),
        "25 (HIGH - threshold 10)".to_string(),
        "complexity analyzer".to_string(),
        0.85,
    );

    let churn_data = DataPoint::new(
        "Code churn".to_string(),
        "50 changes, 15 bugs found".to_string(),
        "git history + bug tracker".to_string(),
        0.8,
    );

    let satd_data = DataPoint::new(
        "Technical debt".to_string(),
        "3 FIXME comments in affected function".to_string(),
        "SATD detector".to_string(),
        0.75,
    );

    // Verify data point properties
    assert_eq!(commit_data.name, "Recent commits");
    assert_eq!(commit_data.relevance, 0.9);
    assert!(commit_data.is_highly_relevant(), "Should be highly relevant (>0.7)");

    assert_eq!(complexity_data.source, "complexity analyzer");
    assert!(complexity_data.is_highly_relevant());

    assert_eq!(churn_data.relevance, 0.8);
    assert!(churn_data.is_highly_relevant());

    assert_eq!(satd_data.relevance, 0.75);
    assert!(satd_data.is_highly_relevant());

    // Verify relevance bounds [0.0, 1.0]
    let invalid_high = DataPoint::new(
        "test".to_string(),
        "value".to_string(),
        "source".to_string(),
        1.5, // Should be clamped to 1.0
    );
    assert_eq!(invalid_high.relevance, 1.0, "Relevance should be clamped to 1.0");

    let invalid_low = DataPoint::new(
        "test".to_string(),
        "value".to_string(),
        "source".to_string(),
        -0.5, // Should be clamped to 0.0
    );
    assert_eq!(invalid_low.relevance, 0.0, "Relevance should be clamped to 0.0");
}

/// Test: Hypothesis Generation with Confidence
///
/// This test verifies that hypotheses can be generated with appropriate
/// confidence levels based on supporting data.
///
/// CRITICAL: These are SUGGESTIONS, not conclusions. Human must validate.
#[test]
fn test_hypothesis_generation_with_confidence() {
    // High confidence hypothesis (strong data correlation)
    let high_confidence_hypothesis = Hypothesis::new(
        "Parser complexity increased due to recent feature additions".to_string(),
        ConfidenceLevel::High,
    )
    .add_data(DataPoint::new(
        "Complexity spike".to_string(),
        "Cyclomatic increased from 10 to 25 in 2 weeks".to_string(),
        "complexity history".to_string(),
        0.95,
    ))
    .add_data(DataPoint::new(
        "Feature commits".to_string(),
        "10 commits adding new parsing rules".to_string(),
        "git log".to_string(),
        0.9,
    ));

    // Verify hypothesis properties
    assert_eq!(high_confidence_hypothesis.confidence, ConfidenceLevel::High);
    assert_eq!(high_confidence_hypothesis.confidence.score(), 0.9);
    assert_eq!(high_confidence_hypothesis.confidence.as_str(), "HIGH");
    assert!(
        high_confidence_hypothesis.needs_validation,
        "CRITICAL: Must always require human validation"
    );
    assert_eq!(high_confidence_hypothesis.supporting_data.len(), 2);

    // Medium confidence hypothesis (moderate data correlation)
    let medium_confidence_hypothesis = Hypothesis::new(
        "Error handling may be insufficient".to_string(),
        ConfidenceLevel::Medium,
    )
    .add_data(DataPoint::new(
        "Error paths".to_string(),
        "3 FIXME comments about error handling".to_string(),
        "SATD detector".to_string(),
        0.6,
    ));

    assert_eq!(medium_confidence_hypothesis.confidence, ConfidenceLevel::Medium);
    assert_eq!(medium_confidence_hypothesis.confidence.score(), 0.65);
    assert!(medium_confidence_hypothesis.needs_validation);

    // Low confidence hypothesis (weak data correlation)
    let low_confidence_hypothesis = Hypothesis::new(
        "Memory allocation pattern might be inefficient".to_string(),
        ConfidenceLevel::Low,
    )
    .add_data(DataPoint::new(
        "Memory usage".to_string(),
        "Slightly elevated but within normal range".to_string(),
        "profiler".to_string(),
        0.4,
    ));

    assert_eq!(low_confidence_hypothesis.confidence, ConfidenceLevel::Low);
    assert_eq!(low_confidence_hypothesis.confidence.score(), 0.35);
    assert!(low_confidence_hypothesis.needs_validation);
}

/// Test: Five-Whys Layer Construction
///
/// This test verifies that complete Five-Whys analysis can be constructed
/// with multiple layers (Why #1 through Why #5), each containing:
/// - Question/hypothesis
/// - Supporting data points
/// - Confidence level
#[test]
fn test_five_whys_layer_construction() {
    // Create a Five-Whys analysis for a parser bug
    let mut analysis = FiveWhysAnalysis::new(
        "Parser crashes on nested expressions with depth >10".to_string(),
    );

    // Why #1: What triggered the bug?
    let mut why1 = WhyLayer::new(
        1,
        "Stack overflow in recursive descent parser".to_string(),
    );
    why1.add_data_point(DataPoint::new(
        "Stack trace".to_string(),
        "Recursive call depth exceeded at parse_expression()".to_string(),
        "crash dump".to_string(),
        0.95,
    ));
    why1.add_hypothesis(
        Hypothesis::new(
            "Parser uses unbounded recursion for nested expressions".to_string(),
            ConfidenceLevel::High,
        )
        .add_data(DataPoint::new(
            "Code review".to_string(),
            "No depth limit check in parse_expression()".to_string(),
            "source code analysis".to_string(),
            0.9,
        )),
    );

    // Why #2: Why does parser use unbounded recursion?
    let mut why2 = WhyLayer::new(
        2,
        "No recursion depth limit implemented".to_string(),
    );
    why2.add_data_point(DataPoint::new(
        "Code history".to_string(),
        "Depth limit was removed in commit abc123".to_string(),
        "git log".to_string(),
        0.85,
    ));
    why2.add_hypothesis(
        Hypothesis::new(
            "Depth limit removed to support deeply nested macros".to_string(),
            ConfidenceLevel::Medium,
        )
        .add_data(DataPoint::new(
            "Commit message".to_string(),
            "'Removed artificial nesting limit for macro expansion'".to_string(),
            "git show abc123".to_string(),
            0.7,
        )),
    );

    // Why #3: Why was depth limit removed without bounds check?
    let mut why3 = WhyLayer::new(
        3,
        "Trade-off between flexibility and safety not validated".to_string(),
    );
    why3.add_hypothesis(
        Hypothesis::new(
            "Assumed macro nesting would remain reasonable in practice".to_string(),
            ConfidenceLevel::Low,
        )
        .add_data(DataPoint::new(
            "Test coverage".to_string(),
            "No tests for deeply nested expressions (>5 levels)".to_string(),
            "test suite analysis".to_string(),
            0.6,
        )),
    );

    // Add layers to analysis
    analysis.add_layer(why1);
    analysis.add_layer(why2);
    analysis.add_layer(why3);

    // Verify analysis structure
    assert_eq!(analysis.layers.len(), 3);
    assert_eq!(analysis.layers[0].layer, 1);
    assert_eq!(analysis.layers[1].layer, 2);
    assert_eq!(analysis.layers[2].layer, 3);

    // Verify overall confidence decreases as we go deeper
    assert_eq!(analysis.layers[0].hypotheses[0].confidence, ConfidenceLevel::High);
    assert_eq!(analysis.layers[1].hypotheses[0].confidence, ConfidenceLevel::Medium);
    assert_eq!(analysis.layers[2].hypotheses[0].confidence, ConfidenceLevel::Low);

    // CRITICAL: Verify human validation always required
    assert!(
        analysis.needs_validation,
        "Five-Whys analysis MUST always require human validation"
    );
}

/// Test: Human Validation Markers
///
/// This test verifies that the system ALWAYS marks analysis as requiring
/// human validation. This is CRITICAL to prevent false confidence in
/// automated root cause analysis.
///
/// Following Ohno's principle: humans understand causality, systems provide data.
#[test]
fn test_human_validation_markers() {
    // Create analysis with high-confidence hypotheses
    let mut analysis = FiveWhysAnalysis::new("Test bug".to_string());

    let mut layer = WhyLayer::new(1, "Highly confident hypothesis".to_string());
    layer.add_hypothesis(
        Hypothesis::new(
            "Root cause identified with 95% data correlation".to_string(),
            ConfidenceLevel::High,
        )
        .add_data(DataPoint::new(
            "Strong evidence".to_string(),
            "Multiple data sources confirm".to_string(),
            "various".to_string(),
            0.95,
        )),
    );

    analysis.add_layer(layer);

    // Calculate overall confidence based on layers
    analysis.calculate_confidence();

    // CRITICAL: Even with high confidence, validation MUST be required
    assert!(
        analysis.needs_validation,
        "System must NEVER claim automated root cause analysis"
    );

    // Every hypothesis must require validation
    for layer in &analysis.layers {
        for hypothesis in &layer.hypotheses {
            assert!(
                hypothesis.needs_validation,
                "Every hypothesis MUST require human validation"
            );
        }
    }

    // Overall confidence should be set based on evidence
    assert_eq!(
        analysis.overall_confidence,
        ConfidenceLevel::High,
        "Overall confidence reflects data strength, not certainty"
    );
}

/// Test: Markdown Output Generation
///
/// This test verifies that Five-Whys analysis can be formatted as
/// well-structured markdown for inclusion in bug reports.
///
/// Output must be human-readable and clearly indicate validation required.
#[test]
fn test_markdown_output_generation() {
    // Create a complete Five-Whys analysis
    let mut analysis = FiveWhysAnalysis::new(
        "Performance regression in sort function".to_string(),
    );

    let mut why1 = WhyLayer::new(
        1,
        "Algorithm changed from O(n log n) to O(n^2)".to_string(),
    );
    why1.add_data_point(DataPoint::new(
        "Performance profiling".to_string(),
        "Execution time 100x slower on large inputs".to_string(),
        "profiler".to_string(),
        0.95,
    ));
    why1.add_hypothesis(
        Hypothesis::new(
            "Quicksort replaced with bubble sort in recent refactor".to_string(),
            ConfidenceLevel::High,
        )
        .add_data(DataPoint::new(
            "Git diff".to_string(),
            "Commit def456: 'Simplified sorting algorithm'".to_string(),
            "git log".to_string(),
            0.9,
        )),
    );

    analysis.add_layer(why1);

    // Generate markdown
    let markdown = analysis.to_markdown();

    // Verify markdown structure
    assert!(markdown.contains("# Five-Whys Analysis"), "Should have title");
    assert!(
        markdown.contains("IMPORTANT"),
        "Should warn this is ASSISTED analysis"
    );
    assert!(
        markdown.contains("human validation"),
        "Should explicitly require human validation"
    );
    assert!(
        markdown.contains("## Why #1"),
        "Should format layers as sections"
    );
    assert!(
        markdown.contains("Performance regression in sort function"),
        "Should include bug description"
    );
    assert!(
        markdown.contains("HIGH") || markdown.contains("High"),
        "Should show confidence level"
    );
    assert!(
        markdown.contains("Algorithm changed from O(n log n) to O(n^2)"),
        "Should include layer question"
    );

    // Verify data points are included
    assert!(
        markdown.contains("Performance profiling"),
        "Should include data point names"
    );
    assert!(
        markdown.contains("profiler"),
        "Should cite data sources"
    );

    // Verify hypotheses are included
    assert!(
        markdown.contains("Quicksort replaced with bubble sort"),
        "Should include hypothesis statements"
    );
}

/// Test: Comprehensive Integration
///
/// This test verifies the complete workflow:
/// 1. Collect data from multiple sources
/// 2. Generate hypotheses with confidence
/// 3. Build Five-Whys layers
/// 4. Mark human validation required
/// 5. Generate markdown output
#[test]
fn test_comprehensive_five_whys_integration() {
    // Step 1: Collect data
    let complexity_data = DataPoint::new(
        "Function complexity".to_string(),
        "Cyclomatic: 25, Cognitive: 35".to_string(),
        "static analyzer".to_string(),
        0.9,
    );

    let satd_data = DataPoint::new(
        "Technical debt".to_string(),
        "5 FIXME comments, 3 HACK markers".to_string(),
        "SATD detector".to_string(),
        0.8,
    );

    let churn_data = DataPoint::new(
        "Change frequency".to_string(),
        "Modified 30 times in 3 months, 8 bugs".to_string(),
        "git history".to_string(),
        0.85,
    );

    // Step 2: Create analysis
    let mut analysis = FiveWhysAnalysis::new(
        "Memory leak in data processing pipeline".to_string(),
    );

    // Step 3: Build layers with hypotheses
    let mut why1 = WhyLayer::new(1, "Objects not being freed after processing".to_string());
    why1.add_data_point(complexity_data.clone());
    why1.add_data_point(satd_data.clone());
    why1.add_hypothesis(
        Hypothesis::new(
            "Complex control flow prevents cleanup in error paths".to_string(),
            ConfidenceLevel::High,
        )
        .add_data(complexity_data)
        .add_data(satd_data),
    );

    let mut why2 = WhyLayer::new(2, "Error handling incomplete".to_string());
    why2.add_data_point(churn_data.clone());
    why2.add_hypothesis(
        Hypothesis::new(
            "Frequent changes introduced edge cases without cleanup".to_string(),
            ConfidenceLevel::Medium,
        )
        .add_data(churn_data),
    );

    analysis.add_layer(why1);
    analysis.add_layer(why2);

    // Step 4: Verify validation markers
    assert!(analysis.needs_validation, "Must require validation");
    assert_eq!(analysis.layers.len(), 2);

    // Step 5: Generate and verify output
    let markdown = analysis.to_markdown();

    assert!(markdown.len() > 100, "Should generate substantial output");
    assert!(markdown.contains("Memory leak"), "Should include bug description");
    assert!(markdown.contains("Why #1"), "Should have first layer");
    assert!(markdown.contains("Why #2"), "Should have second layer");
    assert!(
        markdown.contains("IMPORTANT") || markdown.contains("validation"),
        "Should emphasize human validation required"
    );

    println!("✅ Comprehensive Five-Whys integration test passed");
    println!("   - {} layers analyzed", analysis.layers.len());
    println!("   - {} data points collected",
        analysis.layers.iter().map(|l| l.data_points.len()).sum::<usize>());
    println!("   - Validation required: {}", analysis.needs_validation);
    println!("   - Markdown length: {} characters", markdown.len());
}
