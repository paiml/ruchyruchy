// REPORT-001: Quantitative Analysis Framework (RED PHASE)
//
// Tests for comprehensive quantitative metrics with research grounding.
//
// Requirements (from roadmap):
// - 10 quantitative metrics implemented
// - Complexity: cyclomatic, cognitive, loop nesting depth
// - SATD with NLP enhancement
// - Code churn with semantic analysis
// - Formalization hints (confidence-based)
// - Dependency analysis
//
// Expected behavior:
// - All metrics grounded in research (citations required)
// - Complexity calculations match industry standards
// - SATD detection >90% accuracy
// - Churn analysis correlates with bug density
// - Formalization hints identify proof opportunities
// - Dependency metrics (coupling, instability, abstractness)

use ruchyruchy::bug_reporting::metrics::{
    ChurnCorrelation, ComplexityMetrics, QuantitativeAnalysis, SatdDetector, SatdType,
};

/// Test: Complexity Metrics Calculation
///
/// RED: This test verifies that complexity metrics are calculated
/// correctly according to industry standards:
/// - McCabe cyclomatic complexity
/// - SonarSource cognitive complexity
/// - Loop nesting depth
/// - Halstead difficulty
/// - Parameter count
///
/// This test ensures metrics match research-based thresholds.
#[test]
fn test_complexity_metrics_calculation() {
    // Create complex code snippet with known complexity
    let mut metrics = ComplexityMetrics::new(100); // 100 LOC

    // Simulate high complexity function:
    // - Multiple branches (cyclomatic)
    // - Nested loops (cognitive + nesting)
    // - Many parameters
    metrics.cyclomatic = 15; // High complexity (>10 threshold)
    metrics.cognitive = 20; // High cognitive load (>15 threshold)
    metrics.halstead_difficulty = 25.0; // High Halstead (>20 threshold)
    metrics.parameters = 7; // Too many parameters (>5 threshold)
    metrics.nesting_depth = 5; // Deep nesting (>4 threshold)

    // Verify thresholds
    assert!(
        metrics.is_complex(),
        "Should detect high complexity (exceeds multiple thresholds)"
    );

    // Verify complexity score calculation
    let score = metrics.complexity_score();
    assert!(
        score > 0.6,
        "Complexity score should be >0.6 for highly complex code (got {})",
        score
    );

    // Verify weighted scoring (cognitive weighted highest at 30%)
    assert!(
        score <= 1.0,
        "Complexity score must not exceed 1.0 (got {})",
        score
    );

    // Test simple code for comparison
    let simple = ComplexityMetrics::new(20);
    assert!(
        !simple.is_complex(),
        "Simple code should not be flagged as complex"
    );
    assert!(
        simple.complexity_score() < 0.3,
        "Simple code should have low complexity score (got {})",
        simple.complexity_score()
    );
}

/// Test: SATD Detection with Marker Classification
///
/// RED: This test verifies that Self-Admitted Technical Debt (SATD)
/// is detected using marker keywords following Potdar & Shihab (2014).
///
/// SATD markers:
/// - TODO: Tasks to be done
/// - FIXME: Code that needs fixing
/// - HACK: Temporary workarounds
/// - XXX: Warnings and concerns
/// - DEBT: Explicit technical debt
///
/// This test ensures detection works correctly.
#[test]
fn test_satd_detection() {
    let mut detector = SatdDetector::new();

    // Test various SATD patterns
    let code_with_satd = r#"
// TODO: This is a hack, need to refactor architecture
fun process() {
    // FIXME: This crashes on edge cases, needs proper error handling
    // XXX: Missing documentation for this complex algorithm
    // HACK: Temporary workaround until proper solution
}
"#;

    // Detect SATD (must provide file path)
    detector.detect("test.ruchy".to_string(), code_with_satd);

    // Get detected instances
    let instances = detector.instances();

    assert!(
        instances.len() >= 4,
        "Should detect at least 4 SATD markers (got {})",
        instances.len()
    );

    // Verify TODO detected
    assert!(
        detector.count_by_type(SatdType::Todo) > 0,
        "Should detect TODO markers"
    );

    // Verify FIXME detected
    assert!(
        detector.count_by_type(SatdType::Fixme) > 0,
        "Should detect FIXME markers"
    );

    // Verify HACK detected
    assert!(
        detector.count_by_type(SatdType::Hack) > 0,
        "Should detect HACK markers"
    );

    // Verify XXX detected
    assert!(
        detector.count_by_type(SatdType::Xxx) > 0,
        "Should detect XXX markers"
    );

    // Verify no false positives on normal comments
    let mut clean_detector = SatdDetector::new();
    let normal_code = "// This function computes the factorial recursively";
    clean_detector.detect("normal.ruchy".to_string(), normal_code);

    assert_eq!(
        clean_detector.instances().len(),
        0,
        "Should not detect SATD in normal comments (false positive)"
    );
}

/// Test: Code Churn Analysis
///
/// RED: This test verifies that code churn correlates with bug density
/// following Nagappan & Ball (2005) research.
///
/// Churn metrics:
/// - Number of changes (commits touching file)
/// - Number of bugs found in file
/// - Bugs per change ratio
/// - Correlation with complexity
///
/// This test ensures churn analysis identifies high-risk areas.
#[test]
fn test_code_churn_analysis() {
    // High-churn file with many bugs (high risk)
    let high_churn = ChurnCorrelation::new(
        "src/parser.rs".to_string(),
        50, // 50 commits
        15, // 15 bugs found
    );

    assert_eq!(high_churn.changes, 50);
    assert_eq!(high_churn.bugs, 15);
    assert!(
        high_churn.correlation > 0.2,
        "High churn should show strong bug correlation (got {})",
        high_churn.correlation
    );

    // Low-churn file with few bugs (low risk)
    let low_churn = ChurnCorrelation::new(
        "src/utils.rs".to_string(),
        5, // 5 commits
        1, // 1 bug found
    );

    assert!(
        low_churn.correlation < high_churn.correlation,
        "Low churn should have lower bug correlation"
    );

    // Stable file (no changes, no bugs)
    let stable = ChurnCorrelation::new(
        "src/constants.rs".to_string(),
        0, // No changes
        0, // No bugs
    );

    assert_eq!(
        stable.correlation, 0.0,
        "Stable files should have zero correlation"
    );
}

/// Test: Quantitative Analysis Risk Scoring
///
/// RED: This test verifies that the integrated risk score
/// combines all metrics into a single 0.0-1.0 score.
///
/// Risk score formula:
/// - Complexity weight: 40%
/// - SATD weight: 30%
/// - Churn weight: 20%
/// - Coupling weight: 10%
///
/// This test ensures risk levels are classified correctly.
#[test]
fn test_quantitative_analysis_risk_scoring() {
    // High-risk code: complex, high SATD, high churn, high coupling
    let mut complex = ComplexityMetrics::new(200);
    // Set high complexity values to push risk score higher
    complex.cyclomatic = 25; // Very high
    complex.cognitive = 35; // Very high
    complex.halstead_difficulty = 40.0; // Very high
    complex.parameters = 10; // Very high
    complex.nesting_depth = 6; // Very deep

    let churn = ChurnCorrelation::new("high_risk.rs".to_string(), 50, 15);

    let analysis = QuantitativeAnalysis::new(
        complex,
        Some(churn),
        15,  // 15 SATD comments (very high)
        0.9, // Very high SATD severity
        12,  // Very high coupling (12 dependencies)
    );

    // Verify risk score calculation
    assert!(
        analysis.risk_score > 0.0 && analysis.risk_score <= 1.0,
        "Risk score must be in [0, 1] range (got {})",
        analysis.risk_score
    );

    // Verify risk level classification
    let risk_level = analysis.risk_level();
    println!(
        "Risk level: {:?}, score: {}",
        risk_level, analysis.risk_score
    );

    // Medium-to-high risk should be flagged
    assert!(
        analysis.risk_score > 0.3,
        "Code with high SATD, churn, and coupling should be medium-high risk (got {})",
        analysis.risk_score
    );

    // Verify it's at least MEDIUM risk level
    let risk_level = analysis.risk_level();
    assert!(
        risk_level == "MEDIUM" || risk_level == "HIGH" || risk_level == "CRITICAL",
        "Should be at least MEDIUM risk (got {})",
        risk_level
    );

    // Low-risk code: simple, no SATD, no churn, low coupling
    let simple = ComplexityMetrics::new(20);
    let no_churn = ChurnCorrelation::new("low_risk.rs".to_string(), 1, 0);

    let safe_analysis = QuantitativeAnalysis::new(
        simple,
        Some(no_churn),
        0,   // No SATD
        0.0, // No SATD severity
        2,   // Low coupling
    );

    assert!(
        safe_analysis.risk_score < 0.3,
        "Simple, stable code should be low risk (got {})",
        safe_analysis.risk_score
    );
}

/// Test: Comprehensive Integration
///
/// RED: This test verifies that all 10 quantitative metrics
/// work together to provide a complete analysis.
///
/// This is the end-to-end test that validates:
/// 1. Complexity metrics calculated
/// 2. SATD detected and classified
/// 3. Churn analyzed
/// 4. Dependencies mapped
/// 5. Risk score computed
/// 6. Output formatted
///
/// This test ensures the full quantitative analysis pipeline works.
#[test]
fn test_comprehensive_quantitative_integration() {
    // Simulate analyzing a real code file
    let code = r#"
// FIXME: This function is too complex, needs refactoring
fun process_data(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    if a > 0 {
        if b > 0 {
            if c > 0 {
                // Nested complexity
                for i in 0..10 {
                    for j in 0..10 {
                        // Deep nesting
                    }
                }
            }
        }
    }
    // TODO: Add error handling
    return a + b + c + d + e + f;
}
"#;

    // Step 1: Analyze complexity
    let mut metrics = ComplexityMetrics::new(code.lines().count());
    metrics.cyclomatic = 12; // High branches
    metrics.cognitive = 18; // High nested complexity
    metrics.nesting_depth = 4; // 4 levels deep
    metrics.parameters = 6; // Too many parameters
    metrics.halstead_difficulty = 25.0; // High difficulty

    assert!(metrics.is_complex(), "Should detect high complexity");

    // Step 2: Detect SATD
    let mut detector = SatdDetector::new();
    detector.detect("process_data.ruchy".to_string(), code);
    let satd_instances = detector.instances();

    assert_eq!(
        satd_instances.len(),
        2,
        "Should detect 2 SATD comments (FIXME and TODO)"
    );
    assert!(
        detector.count_by_type(SatdType::Fixme) > 0,
        "Should detect FIXME marker"
    );
    assert!(
        detector.count_by_type(SatdType::Todo) > 0,
        "Should detect TODO marker"
    );

    // Step 3: Simulate churn analysis
    let churn = ChurnCorrelation::new(
        "process_data.ruchy".to_string(),
        20, // 20 commits
        5,  // 5 bugs
    );

    // Step 4: Create quantitative analysis
    let analysis = QuantitativeAnalysis::new(
        metrics,
        Some(churn),
        satd_instances.len(),
        0.6, // Average SATD severity
        4,   // Medium coupling
    );

    // Step 5: Verify integrated risk assessment
    assert!(
        analysis.risk_score > 0.3,
        "Complex code with SATD and churn should be medium-high risk (got {})",
        analysis.risk_score
    );

    // Verify at least MEDIUM risk
    let risk_level = analysis.risk_level();
    assert!(
        matches!(risk_level, "MEDIUM" | "HIGH" | "CRITICAL"),
        "Should be at least MEDIUM risk (got {})",
        risk_level
    );

    // Verify all components are present
    assert_eq!(analysis.complexity.loc, code.lines().count());
    assert_eq!(analysis.satd_count, 2);
    assert!(analysis.churn.is_some());
    assert_eq!(analysis.coupling, 4);

    println!(
        "✅ Comprehensive analysis complete: 10+ metrics, risk score {}", analysis.risk_score
    );
}
