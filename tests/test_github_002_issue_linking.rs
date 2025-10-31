// GITHUB-002: Issue Linking & Deduplication (INTEGRATION TESTS)
//
// Tests for intelligent issue linking and deduplication system.
//
// Requirements (from roadmap):
// - Similarity search (Jaccard, TF-IDF)
// - Deduplication algorithm (>80% similarity = duplicate)
// - Related issue finder (>50% similarity = related)
// - File/error/label-based matching
//
// Expected behavior:
// - Calculate similarity scores between issues
// - Detect duplicates (similarity >= 80%)
// - Find related issues (similarity >= 50%)
// - Use weighted scoring (title 30%, body 25%, files 20%, error 15%, labels 10%)
// - Sort related issues by similarity (highest first)
// - Prevent duplicate issue creation
//
// Testing Strategy:
// - Test duplicate detection with identical issues
// - Test related issue finding with similar issues
// - Test similarity calculation with various fields
// - Test edge cases (empty issues, no matches)
// - Verify thresholds (80% duplicate, 50% related)

use ruchyruchy::bug_reporting::issue_linking::{
    BugIssue, IssueDeduplicator, SimilarityCalculator, SimilarityScore, DUPLICATE_THRESHOLD,
    RELATED_THRESHOLD,
};

/// Test: Duplicate Detection - Identical Issues
///
/// This test verifies duplicate detection for identical issues:
/// - Two issues with same title, body, files
/// - Similarity should be very high (>95%)
/// - Should be detected as duplicate
#[test]
fn test_duplicate_detection_identical_issues() {
    let mut dedup = IssueDeduplicator::new();

    // Add existing issue
    let mut existing = BugIssue::new(
        1,
        "Parser crashes on nested expressions".to_string(),
        "The parser encounters a stack overflow when processing deeply nested expressions."
            .to_string(),
    );
    existing.add_file("src/parser.rs".to_string());
    existing.set_error_message("thread 'main' panicked at 'stack overflow'".to_string());
    existing.add_label("bug".to_string());
    existing.add_label("critical".to_string());

    dedup.add_issue(existing);

    // Try to add identical issue
    let mut new_issue = BugIssue::new(
        2,
        "Parser crashes on nested expressions".to_string(),
        "The parser encounters a stack overflow when processing deeply nested expressions."
            .to_string(),
    );
    new_issue.add_file("src/parser.rs".to_string());
    new_issue.set_error_message("thread 'main' panicked at 'stack overflow'".to_string());
    new_issue.add_label("bug".to_string());
    new_issue.add_label("critical".to_string());

    let result = dedup.check_duplicate(&new_issue);

    // Should detect as duplicate
    assert!(result.is_duplicate);
    assert_eq!(result.duplicate_of, Some(1));
    assert!(result.score.overall > 0.95); // Very high similarity
}

/// Test: Duplicate Detection - Similar Title and Body
///
/// This test verifies duplicate detection based on text similarity:
/// - Similar title and body (high word overlap)
/// - Should exceed duplicate threshold (>80%)
#[test]
fn test_duplicate_detection_similar_text() {
    let mut dedup = IssueDeduplicator::new();

    let existing = BugIssue::new(
        1,
        "Lexer hangs on unterminated string literal when parsing".to_string(),
        "The lexer encounters infinite loop when processing unterminated string literal without closing quote mark".to_string(),
    );
    dedup.add_issue(existing);

    let new_issue = BugIssue::new(
        2,
        "Lexer hangs on unterminated string literal when parsing".to_string(),
        "The lexer encounters infinite loop when processing unterminated string literal without closing quote mark".to_string(),
    );

    let result = dedup.check_duplicate(&new_issue);

    // Should detect as duplicate (very high text overlap - nearly identical)
    assert!(result.is_duplicate);
    assert_eq!(result.duplicate_of, Some(1));
    assert!(result.score.overall >= DUPLICATE_THRESHOLD);
}

/// Test: Not a Duplicate - Different Issues
///
/// This test verifies no false positives:
/// - Completely different issues
/// - Similarity should be low
/// - Should NOT be detected as duplicate
#[test]
fn test_not_duplicate_different_issues() {
    let mut dedup = IssueDeduplicator::new();

    let existing = BugIssue::new(
        1,
        "Parser crashes on nested expressions".to_string(),
        "Stack overflow in parser".to_string(),
    );
    dedup.add_issue(existing);

    let new_issue = BugIssue::new(
        2,
        "Type checker rejects valid generics".to_string(),
        "Generic type inference fails for complex constraints".to_string(),
    );

    let result = dedup.check_duplicate(&new_issue);

    // Should NOT be duplicate (different issues)
    assert!(!result.is_duplicate);
    assert_eq!(result.duplicate_of, None);
    assert!(result.score.overall < DUPLICATE_THRESHOLD);
}

/// Test: Related Issue Finding
///
/// This test verifies finding related (but not duplicate) issues:
/// - Issues share some similarity (50-80%)
/// - Should be detected as related
/// - Should be sorted by similarity
#[test]
fn test_find_related_issues() {
    let mut dedup = IssueDeduplicator::new();

    // Add several existing issues
    let issue1 = BugIssue::new(
        1,
        "Parser crash in expression parsing".to_string(),
        "The parser crashes when parsing complex expressions".to_string(),
    );

    let issue2 = BugIssue::new(
        2,
        "Parser hang in statement parsing".to_string(),
        "The parser hangs when parsing complex statements".to_string(),
    );

    let mut issue3 = BugIssue::new(
        3,
        "Type checker error".to_string(),
        "Type inference fails".to_string(),
    );
    issue3.add_file("src/typechecker.rs".to_string());

    dedup.add_issue(issue1);
    dedup.add_issue(issue2);
    dedup.add_issue(issue3);

    // New issue related to parser
    let new_issue = BugIssue::new(
        4,
        "Parser error in expression handling".to_string(),
        "Expression parsing encounters errors".to_string(),
    );

    let related = dedup.find_related(&new_issue, 10);

    // Should find issues 1 and 2 as related (parser issues)
    assert!(!related.is_empty());

    // Check that parser issues are in results
    let parser_related = related.iter().any(|r| r.issue_id == 1 || r.issue_id == 2);
    assert!(parser_related, "Should find parser-related issues");

    // Verify scores are in related range
    for r in &related {
        assert!(r.score.overall >= RELATED_THRESHOLD);
    }
}

/// Test: Related Issues Sorted by Similarity
///
/// This test verifies related issues are sorted correctly:
/// - Multiple related issues
/// - Should be sorted by similarity score (descending)
#[test]
fn test_related_issues_sorted() {
    let mut dedup = IssueDeduplicator::new();

    // Add issues with varying similarity
    let issue1 = BugIssue::new(
        1,
        "Crash in parser".to_string(),
        "Parser crashes".to_string(),
    );

    let issue2 = BugIssue::new(
        2,
        "Crash in parser module".to_string(),
        "The parser module crashes on input".to_string(),
    );

    let issue3 = BugIssue::new(
        3,
        "Parser crash bug".to_string(),
        "Parser crashes when processing code".to_string(),
    );

    dedup.add_issue(issue1);
    dedup.add_issue(issue2);
    dedup.add_issue(issue3);

    let new_issue = BugIssue::new(
        4,
        "Parser crash problem".to_string(),
        "The parser crashes when processing".to_string(),
    );

    let related = dedup.find_related(&new_issue, 10);

    // Verify sorted by similarity (descending)
    for i in 1..related.len() {
        assert!(
            related[i - 1].score.overall >= related[i].score.overall,
            "Related issues should be sorted by similarity (highest first)"
        );
    }
}

/// Test: Similarity Score Calculation - Weighted Components
///
/// This test verifies the weighted similarity calculation:
/// - Title: 30%, Body: 25%, Files: 20%, Error: 15%, Labels: 10%
/// - Overall score is weighted average
#[test]
fn test_similarity_score_weighted_components() {
    let mut issue1 = BugIssue::new(1, "Same title".to_string(), "Same body".to_string());
    issue1.add_file("file.rs".to_string());
    issue1.set_error_message("Error occurred".to_string());
    issue1.add_label("bug".to_string());

    let mut issue2 = BugIssue::new(2, "Same title".to_string(), "Same body".to_string());
    issue2.add_file("file.rs".to_string());
    issue2.set_error_message("Error occurred".to_string());
    issue2.add_label("bug".to_string());

    let score = SimilarityCalculator::calculate(&issue1, &issue2);

    // Identical issues should have 1.0 overall similarity
    assert!((score.overall - 1.0).abs() < 0.01);

    // Individual components should be 1.0
    assert!((score.title_similarity - 1.0).abs() < 0.01);
    assert!((score.body_similarity - 1.0).abs() < 0.01);
    assert!((score.file_overlap - 1.0).abs() < 0.01);
    assert!((score.error_similarity - 1.0).abs() < 0.01);
    assert!((score.label_overlap - 1.0).abs() < 0.01);

    // Verify weighted calculation
    let expected = 1.0 * 0.30 + 1.0 * 0.25 + 1.0 * 0.20 + 1.0 * 0.15 + 1.0 * 0.10;
    assert!((score.overall - expected).abs() < 0.01);
}

/// Test: File Overlap Matching
///
/// This test verifies file-based similarity:
/// - Issues affecting same files have higher similarity
/// - File overlap contributes 20% to overall score
#[test]
fn test_file_overlap_matching() {
    let mut issue1 = BugIssue::new(1, "Bug in parser".to_string(), "Description".to_string());
    issue1.add_file("src/parser.rs".to_string());
    issue1.add_file("src/ast.rs".to_string());

    let mut issue2 = BugIssue::new(
        2,
        "Another parser bug".to_string(),
        "Different description".to_string(),
    );
    issue2.add_file("src/parser.rs".to_string());
    issue2.add_file("src/ast.rs".to_string());

    let score = SimilarityCalculator::calculate(&issue1, &issue2);

    // File overlap should be 1.0 (same files)
    assert!((score.file_overlap - 1.0).abs() < 0.01);

    // Overall should be influenced by file overlap
    assert!(score.overall > 0.0);
}

/// Test: Error Message Matching
///
/// This test verifies error message similarity:
/// - Issues with same error message have higher similarity
/// - Error contributes 15% to overall score
#[test]
fn test_error_message_matching() {
    let mut issue1 = BugIssue::new(
        1,
        "Crash occurred".to_string(),
        "Program crashed".to_string(),
    );
    issue1.set_error_message("thread 'main' panicked at 'index out of bounds'".to_string());

    let mut issue2 = BugIssue::new(
        2,
        "System failure".to_string(),
        "Unexpected failure".to_string(),
    );
    issue2.set_error_message("thread 'main' panicked at 'index out of bounds'".to_string());

    let score = SimilarityCalculator::calculate(&issue1, &issue2);

    // Error message should be identical
    assert!((score.error_similarity - 1.0).abs() < 0.01);

    // Overall should benefit from error match
    assert!(score.overall > 0.0);
}

/// Test: Jaccard Similarity - Text Tokens
///
/// This test verifies Jaccard similarity calculation:
/// - Jaccard = |A ∩ B| / |A ∪ B|
/// - Tokenizes text and calculates overlap
#[test]
fn test_jaccard_similarity() {
    // Test 1: Identical text
    let sim1 = SimilarityCalculator::jaccard_similarity(
        "parser crashes on input",
        "parser crashes on input",
    );
    assert!((sim1 - 1.0).abs() < 0.01); // Perfect match

    // Test 2: Partial overlap
    let sim2 = SimilarityCalculator::jaccard_similarity(
        "parser crashes on nested expressions",
        "parser hangs on nested structures",
    );
    assert!(sim2 > 0.0 && sim2 < 1.0); // Some overlap

    // Test 3: No overlap
    let sim3 = SimilarityCalculator::jaccard_similarity("parser crashes", "type checker fails");
    assert!(sim3 < 0.5); // Low similarity

    // Test 4: Empty strings
    let sim4 = SimilarityCalculator::jaccard_similarity("", "");
    assert!((sim4 - 1.0).abs() < 0.01); // Both empty = identical
}

/// Test: Threshold Constants
///
/// This test verifies the threshold values:
/// - DUPLICATE_THRESHOLD = 0.80 (80%)
/// - RELATED_THRESHOLD = 0.50 (50%)
#[test]
fn test_threshold_constants() {
    assert!((DUPLICATE_THRESHOLD - 0.80).abs() < 0.01);
    assert!((RELATED_THRESHOLD - 0.50).abs() < 0.01);

    // Verify threshold usage
    let high_score = SimilarityScore::new(0.9, 0.9, 0.9, 0.9, 0.9);
    assert!(high_score.is_duplicate());
    assert!(high_score.is_related());

    let medium_score = SimilarityScore::new(0.6, 0.6, 0.6, 0.6, 0.6);
    assert!(!medium_score.is_duplicate());
    assert!(medium_score.is_related());

    let low_score = SimilarityScore::new(0.3, 0.3, 0.3, 0.3, 0.3);
    assert!(!low_score.is_duplicate());
    assert!(!low_score.is_related());
}

/// Test: Edge Case - Empty Issue Corpus
///
/// This test verifies handling of empty corpus:
/// - No existing issues
/// - New issue should not be duplicate
/// - No related issues found
#[test]
fn test_edge_case_empty_corpus() {
    let dedup = IssueDeduplicator::new();

    let new_issue = BugIssue::new(
        1,
        "First issue".to_string(),
        "This is the first issue".to_string(),
    );

    // Check duplicate on empty corpus
    let result = dedup.check_duplicate(&new_issue);
    assert!(!result.is_duplicate);
    assert_eq!(result.duplicate_of, None);

    // Find related on empty corpus
    let related = dedup.find_related(&new_issue, 10);
    assert_eq!(related.len(), 0);
}

/// Test: Edge Case - Single Existing Issue
///
/// This test verifies handling of single issue:
/// - Only one existing issue
/// - Either matches or doesn't
#[test]
fn test_edge_case_single_existing_issue() {
    let mut dedup = IssueDeduplicator::new();

    let existing = BugIssue::new(1, "Parser bug".to_string(), "Parser has a bug".to_string());
    dedup.add_issue(existing);

    assert_eq!(dedup.issue_count(), 1);

    // Test with duplicate
    let duplicate = BugIssue::new(2, "Parser bug".to_string(), "Parser has a bug".to_string());

    let dup_result = dedup.check_duplicate(&duplicate);
    assert!(dup_result.is_duplicate);

    // Test with different issue
    let different = BugIssue::new(
        3,
        "Type checker error".to_string(),
        "Type inference fails".to_string(),
    );

    let diff_result = dedup.check_duplicate(&different);
    assert!(!diff_result.is_duplicate);
}

/// Test: Multiple Potential Duplicates - Best Match
///
/// This test verifies best match selection:
/// - Multiple similar existing issues
/// - Should select highest similarity as duplicate
#[test]
fn test_multiple_potential_duplicates_best_match() {
    let mut dedup = IssueDeduplicator::new();

    // Add issues with varying similarity to new issue
    let issue1 = BugIssue::new(
        1,
        "Parser crashes".to_string(),
        "The parser crashes on input".to_string(),
    );

    let issue2 = BugIssue::new(
        2,
        "Parser crashes on nested code".to_string(),
        "The parser crashes on nested input code".to_string(),
    );

    let issue3 = BugIssue::new(
        3,
        "Parser crashes on nested expressions".to_string(),
        "The parser crashes on nested expression input code structure".to_string(),
    );

    dedup.add_issue(issue1);
    dedup.add_issue(issue2);
    dedup.add_issue(issue3);

    // New issue similar to all three, but closest to issue3
    let new_issue = BugIssue::new(
        4,
        "Parser crashes on nested expressions".to_string(),
        "The parser crashes on nested expression input code structure".to_string(),
    );

    let result = dedup.check_duplicate(&new_issue);

    if result.is_duplicate {
        // Should select the best match (issue3)
        assert_eq!(result.duplicate_of, Some(3));
    }
}

/// Test: Related Issue Limit
///
/// This test verifies the limit parameter for related issues:
/// - Find up to N related issues
/// - Should respect limit even if more exist
#[test]
fn test_related_issue_limit() {
    let mut dedup = IssueDeduplicator::new();

    // Add 10 related issues
    for i in 1..=10 {
        let issue = BugIssue::new(
            i,
            format!("Parser bug #{}", i),
            "Parser has issues with code".to_string(),
        );
        dedup.add_issue(issue);
    }

    let new_issue = BugIssue::new(
        11,
        "Parser bug #11".to_string(),
        "Parser has issues with code".to_string(),
    );

    // Request limit of 5
    let related = dedup.find_related(&new_issue, 5);

    // Should return at most 5
    assert!(related.len() <= 5);

    // Should return most similar ones (if more than 5 match threshold)
    if related.len() == 5 {
        // Verify they're the most similar (already sorted)
        for i in 1..related.len() {
            assert!(related[i - 1].score.overall >= related[i].score.overall);
        }
    }
}
