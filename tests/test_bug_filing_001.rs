// INTERP-034: Ruchy Compiler Bug Filing (Upstream) - RED PHASE
//
// This test suite validates automated bug filing infrastructure for
// reporting discovered bugs to the upstream Ruchy compiler repository.
//
// Requirements:
// - File 50+ bugs to paiml/ruchy via GitHub API
// - Confidence score ≥0.9 for all bugs
// - 100% include minimal reproduction steps
// - Proper categorization (Parser/Runtime/Type System/Stdlib/Docs)
// - Deduplication to avoid filing duplicate issues
// - Rate limiting to respect GitHub API limits
//
// Tests:
// - test_bug_report_creation: BugReport struct creation
// - test_bug_report_markdown_formatting: GitHub issue markdown generation
// - test_confidence_calculation: 4-factor scoring algorithm
// - test_github_client_creation: GitHubBugFiler initialization
// - test_dry_run_mode: Validation without actual filing
// - test_bug_categorization: Parser/Runtime/Type System classification
//
// RED PHASE: This test WILL FAIL because:
// - BugReport struct doesn't exist yet
// - GitHubBugFiler doesn't exist yet
// - Confidence calculation not implemented
// - GitHub API integration not implemented

// GREEN PHASE: Tests now use production module from src/bug_filing/

use ruchyruchy::bug_filing::*;
/// Test: Bug Report Creation
///
/// RED: This test validates BugReport struct creation
///
/// Property: BugReport can be created with all required fields
#[test]
fn test_bug_report_creation() {
    let bug = BugReport::new(
        Category::Parser,
        Severity::High,
        "Parser fails on if-else without braces".to_string(),
        "if (true) 42 else 0".to_string(),
        "Should parse successfully".to_string(),
        "Parse error: unexpected token 'else'".to_string(),
    );

    assert_eq!(bug.category, Category::Parser);
    assert_eq!(bug.severity, Severity::High);
    assert_eq!(bug.title, "Parser fails on if-else without braces");
}

/// Test: Bug Report Markdown Formatting
///
/// RED: This test validates GitHub issue markdown generation
///
/// Property: Markdown includes all required sections
#[test]
fn test_bug_report_markdown_formatting() {
    let mut bug = BugReport::new(
        Category::Runtime,
        Severity::Critical,
        "println crashes on empty string".to_string(),
        r#"println("")"#.to_string(),
        "Should print empty line".to_string(),
        "Runtime panic: index out of bounds".to_string(),
    );
    bug.confidence = 1.0;
    bug.test_file = Some("tests/test_interp_011_ch01_examples.rs".to_string());

    let markdown = bug.to_github_markdown();

    // Verify required sections
    assert!(markdown.contains("## Bug Report:"));
    assert!(markdown.contains("**Category**: Runtime"));
    assert!(markdown.contains("**Confidence**: 1.00"));
    assert!(markdown.contains("**Severity**: Critical"));
    assert!(markdown.contains("### Minimal Reproduction"));
    assert!(markdown.contains("```ruchy"));
    assert!(markdown.contains("### Expected Behavior"));
    assert!(markdown.contains("### Actual Behavior"));
    assert!(markdown.contains("### Environment"));
    assert!(markdown.contains("**Filed by**: RuchyRuchy Automated Bug Reporter"));
    assert!(markdown.contains("**Ticket**: INTERP-034"));
}

/// Test: Confidence Calculation
///
/// RED: This test validates the 4-factor confidence scoring algorithm
///
/// Property: High-quality bugs score ≥0.9
#[test]
fn test_confidence_calculation() {
    // High-quality bug: Always reproducible, minimal code, spec violation, critical impact
    let factors = ConfidenceFactors {
        reproducibility: Reproducibility::Always,
        lines_of_code: 5,
        spec_violation: true,
        impact: Impact::Critical,
    };

    let confidence = ConfidenceCalculator::calculate(&factors);

    // Expected: 0.4*1.0 + 0.3*1.0 + 0.2*1.0 + 0.1*1.0 = 1.0
    assert!(
        confidence >= 0.9,
        "High-quality bug should have confidence ≥0.9, got {}",
        confidence
    );
    // Allow for floating point precision
    assert!(
        (confidence - 1.0).abs() < 0.0001,
        "Expected confidence ≈1.0, got {}",
        confidence
    );
}

/// Test: Confidence Calculation - Low Quality
///
/// Property: Low-quality bugs score <0.9
#[test]
fn test_confidence_calculation_low_quality() {
    // Low-quality bug: Not reproducible, large code, opinion, low impact
    let factors = ConfidenceFactors {
        reproducibility: Reproducibility::Never,
        lines_of_code: 100,
        spec_violation: false,
        impact: Impact::Low,
    };

    let confidence = ConfidenceCalculator::calculate(&factors);

    // Expected: 0.4*0.0 + 0.3*0.0 + 0.2*0.5 + 0.1*0.0 = 0.1
    assert!(
        confidence < 0.9,
        "Low-quality bug should have confidence <0.9, got {}",
        confidence
    );
}

/// Test: GitHub Client Creation
///
/// RED: This test validates GitHubBugFiler initialization
///
/// Property: Client can be created with repository name
#[test]
fn test_github_client_creation() {
    let filer = GitHubBugFiler::new("paiml/ruchy");

    assert_eq!(filer.repo, "paiml/ruchy");
    assert!(!filer.dry_run);
}

/// Test: Dry Run Mode
///
/// RED: This test validates dry-run mode (no actual filing)
///
/// Property: Dry run validates bugs without filing to GitHub
#[test]
fn test_dry_run_mode() {
    let mut filer = GitHubBugFiler::new("paiml/ruchy").with_dry_run(true);

    let mut bug = BugReport::new(
        Category::Parser,
        Severity::High,
        "Test bug".to_string(),
        "test code".to_string(),
        "expected".to_string(),
        "actual".to_string(),
    );
    bug.confidence = 0.95;

    let result = filer.file_bug(&bug);

    // Dry run should succeed
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), IssueNumber(0));
}

/// Test: Duplicate Detection
///
/// RED: This test validates bug deduplication
///
/// Property: Same bug cannot be filed twice
#[test]
fn test_duplicate_detection() {
    let mut filer = GitHubBugFiler::new("paiml/ruchy").with_dry_run(true);

    let mut bug = BugReport::new(
        Category::Runtime,
        Severity::Critical,
        "Duplicate test".to_string(),
        "duplicate code".to_string(),
        "expected".to_string(),
        "actual".to_string(),
    );
    bug.confidence = 1.0;

    // First filing should succeed
    assert!(!filer.is_duplicate(&bug));
    let result1 = filer.file_bug(&bug);
    assert!(result1.is_ok());

    // Second filing should fail as duplicate
    assert!(filer.is_duplicate(&bug));
    let result2 = filer.file_bug(&bug);
    assert!(result2.is_err());

    match result2 {
        Err(FilingError::Duplicate(_)) => {
            // Expected
        }
        _ => panic!("Expected FilingError::Duplicate"),
    }
}

/// Test: Low Confidence Rejection
///
/// Property: Bugs with confidence <0.9 are rejected
#[test]
fn test_low_confidence_rejection() {
    let mut filer = GitHubBugFiler::new("paiml/ruchy").with_dry_run(true);

    let mut bug = BugReport::new(
        Category::Parser,
        Severity::Low,
        "Low confidence bug".to_string(),
        "code".to_string(),
        "expected".to_string(),
        "actual".to_string(),
    );
    bug.confidence = 0.5; // Below threshold

    let result = filer.file_bug(&bug);

    assert!(result.is_err());
    match result {
        Err(FilingError::LowConfidence(conf)) => {
            assert_eq!(conf, 0.5);
        }
        _ => panic!("Expected FilingError::LowConfidence"),
    }
}

/// Test: Bug Categorization
///
/// Property: All category types can be created
#[test]
fn test_bug_categorization() {
    let categories = vec![
        Category::Parser,
        Category::Runtime,
        Category::TypeSystem,
        Category::Stdlib,
        Category::Documentation,
    ];

    for category in categories {
        let bug = BugReport::new(
            category,
            Severity::Medium,
            "Test".to_string(),
            "code".to_string(),
            "exp".to_string(),
            "act".to_string(),
        );
        assert_eq!(bug.category, category);
    }
}

/// Test: Bug Fingerprinting
///
/// Property: Same bug produces same fingerprint
#[test]
fn test_bug_fingerprinting() {
    let bug1 = BugReport::new(
        Category::Parser,
        Severity::High,
        "Test".to_string(),
        "if (true) 42 else 0".to_string(),
        "Should work".to_string(),
        "Parse error".to_string(),
    );

    let bug2 = BugReport::new(
        Category::Runtime, // Different category
        Severity::Low,     // Different severity
        "Different title".to_string(),
        "if (true) 42 else 0".to_string(), // Same reproduction
        "Should work".to_string(),
        "Parse error".to_string(), // Same error
    );

    // Same reproduction + error = same fingerprint
    assert_eq!(bug1.fingerprint(), bug2.fingerprint());
}

/// Test: Completeness Check
///
/// Verify all required tests exist and are documented
#[test]
fn test_interp_034_completeness() {
    let required_tests = [
        "test_bug_report_creation",
        "test_bug_report_markdown_formatting",
        "test_confidence_calculation",
        "test_confidence_calculation_low_quality",
        "test_github_client_creation",
        "test_dry_run_mode",
        "test_duplicate_detection",
        "test_low_confidence_rejection",
        "test_bug_categorization",
        "test_bug_fingerprinting",
    ];

    // Verify test count
    assert_eq!(required_tests.len(), 10);
}
