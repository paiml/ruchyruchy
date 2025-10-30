// GITHUB-001: GitHub API Integration (INTEGRATION TESTS)
//
// Tests for automatic bug filing via GitHub API.
//
// Requirements (from roadmap):
// - GitHub API authentication
// - Issue creation
// - Issue linking (related bugs)
// - Label management
// - Comment posting
//
// Expected behavior:
// - Authenticate with Personal Access Token (PAT)
// - Create issues with title, body, labels, assignees
// - Convert BugReport to IssueRequest with proper formatting
// - Link related issues via markdown references (#123)
// - Assign labels based on severity (critical, high, medium, low)
// - Post comments for updates and lifecycle tracking
//
// API Reference: GitHub REST API v3
// - POST /repos/{owner}/{repo}/issues
// - POST /repos/{owner}/{repo}/issues/{issue_number}/labels
// - POST /repos/{owner}/{repo}/issues/{issue_number}/comments

use ruchyruchy::bug_reporting::github_integration::{
    BugReportConverter, CommentRequest, GitHubClient, GitHubResult, IssueRequest, IssueResponse,
};
use ruchyruchy::bug_reporting::report_generator::{BugCategory, BugReport, Severity};
use ruchyruchy::bug_discovery::confidence::ConfidenceScore;

/// Test: GitHub Authentication
///
/// This test verifies that GitHubClient correctly handles authentication:
/// - Stores Personal Access Token (PAT)
/// - Generates proper Authorization header (Bearer token)
/// - Builds API endpoint URLs with owner/repo
#[test]
fn test_github_authentication() {
    // Create client with credentials
    let client = GitHubClient::new(
        "paiml".to_string(),
        "ruchy".to_string(),
        "ghp_test1234567890abcdefghijklmnopqrstuvwxyz".to_string(),
    );

    // Verify owner/repo stored correctly
    assert_eq!(client.owner(), "paiml");
    assert_eq!(client.repo(), "ruchy");

    // Verify auth header format: "Bearer {token}"
    let auth = client.auth_header();
    assert!(auth.starts_with("Bearer "), "Auth header should start with 'Bearer '");
    assert!(
        auth.contains("ghp_test"),
        "Auth header should contain token"
    );
    assert_eq!(
        auth,
        "Bearer ghp_test1234567890abcdefghijklmnopqrstuvwxyz"
    );

    // Verify endpoint URL construction
    let issues_endpoint = client.endpoint("/issues");
    assert_eq!(
        issues_endpoint,
        "https://api.github.com/repos/paiml/ruchy/issues"
    );

    let issue_123_endpoint = client.endpoint("/issues/123");
    assert_eq!(
        issue_123_endpoint,
        "https://api.github.com/repos/paiml/ruchy/issues/123"
    );

    // Verify custom base URL (for testing/GitHub Enterprise)
    let test_client = GitHubClient::new(
        "testowner".to_string(),
        "testrepo".to_string(),
        "test_token".to_string(),
    )
    .with_base_url("https://github.example.com/api/v3".to_string());

    let test_endpoint = test_client.endpoint("/issues");
    assert_eq!(
        test_endpoint,
        "https://github.example.com/api/v3/repos/testowner/testrepo/issues"
    );
}

/// Test: Issue Creation
///
/// This test verifies that issues can be created with proper structure:
/// - Title and body (markdown)
/// - Labels (severity, category)
/// - Assignees (GitHub usernames)
/// - JSON serialization for API request
#[test]
fn test_issue_creation() {
    // Create issue request with title and body
    let mut issue = IssueRequest::new(
        "Parser crashes on nested expressions".to_string(),
        "## Bug Description\n\nThe parser crashes when...\n\n## Steps to Reproduce\n1. Create file\n2. Run parser".to_string(),
    );

    // Verify basic fields
    assert_eq!(issue.title, "Parser crashes on nested expressions");
    assert!(issue.body.contains("## Bug Description"));
    assert_eq!(issue.labels.len(), 0);
    assert_eq!(issue.assignees.len(), 0);

    // Add labels
    issue.add_label("bug".to_string());
    issue.add_label("severity:critical".to_string());
    issue.add_label("component:parser".to_string());

    assert_eq!(issue.labels.len(), 3);
    assert!(issue.labels.contains(&"bug".to_string()));
    assert!(issue.labels.contains(&"severity:critical".to_string()));

    // Add assignees
    issue.add_assignee("noahgift".to_string());
    issue.add_assignee("alfredodeza".to_string());

    assert_eq!(issue.assignees.len(), 2);
    assert!(issue.assignees.contains(&"noahgift".to_string()));

    // Verify JSON serialization
    let json = issue.to_json();

    assert!(json.contains("\"title\""), "JSON should contain title field");
    assert!(
        json.contains("Parser crashes on nested expressions"),
        "JSON should contain title value"
    );
    assert!(json.contains("\"body\""), "JSON should contain body field");
    assert!(json.contains("\"labels\""), "JSON should contain labels field");
    assert!(json.contains("\"assignees\""), "JSON should contain assignees field");
    assert!(json.contains("bug"), "JSON should contain bug label");
    assert!(
        json.contains("noahgift"),
        "JSON should contain assignee"
    );

    // Verify JSON is valid (basic check)
    assert!(json.starts_with("{"));
    assert!(json.ends_with("}"));
}

/// Test: Issue Response Parsing
///
/// This test verifies that GitHub API responses can be parsed:
/// - Issue number
/// - HTML URL (for humans)
/// - API URL (for further requests)
/// - State (open/closed)
#[test]
fn test_issue_response_parsing() {
    // Simulate GitHub API response (no spaces after colons to match parser)
    let json = r#"{"number":42,"html_url":"https://github.com/paiml/ruchy/issues/42","url":"https://api.github.com/repos/paiml/ruchy/issues/42","state":"open","title":"Parser crash"}"#;

    // Parse response
    let response = IssueResponse::from_json(json);

    assert!(
        response.is_some(),
        "Should successfully parse valid GitHub response"
    );

    let response = response.unwrap();

    // Verify fields
    assert_eq!(response.number, 42);
    assert_eq!(
        response.html_url,
        "https://github.com/paiml/ruchy/issues/42"
    );
    assert_eq!(
        response.url,
        "https://api.github.com/repos/paiml/ruchy/issues/42"
    );
    assert_eq!(response.state, "open");

    // Test invalid JSON
    let invalid_json = r#"{"number": "not_a_number"}"#;
    let invalid_response = IssueResponse::from_json(invalid_json);
    assert!(
        invalid_response.is_none(),
        "Should return None for invalid JSON"
    );

    // Test with closed issue
    let closed_json = r#"{"number":100,"html_url":"https://github.com/paiml/ruchy/issues/100","url":"https://api.github.com/repos/paiml/ruchy/issues/100","state":"closed"}"#;

    let closed = IssueResponse::from_json(closed_json).unwrap();
    assert_eq!(closed.state, "closed");
    assert_eq!(closed.number, 100);
}

/// Test: BugReport to IssueRequest Conversion
///
/// This test verifies that BugReport can be converted to IssueRequest:
/// - Title includes severity prefix
/// - Body contains full markdown report
/// - Labels derived from severity and category
/// - Proper formatting for GitHub
#[test]
fn test_bug_report_conversion() {
    // Create confidence score for the bug (f64 values: 0.0-1.0)
    // discovery_method, reproducibility, evidence, root_cause
    let confidence = ConfidenceScore::new(
        0.9,  // High discovery method confidence (property testing)
        1.0,  // Always reproducible
        0.9,  // Strong quantitative evidence
        1.0,  // Root cause confirmed
    );

    // Create a BugReport with all required fields
    let report = BugReport::new(
        "Parser crashes on deeply nested expressions".to_string(),
        "The parser crashes with a stack overflow when processing deeply nested expressions.".to_string(),
        Severity::Critical,
        BugCategory::Crash,
        "let x = ((((((((((1))))))))));  // 100 nested parens".to_string(),
        "Parser should handle arbitrary nesting depth gracefully".to_string(),
        "Parser crashes with stack overflow error".to_string(),
        confidence,
    );

    // Convert to IssueRequest
    let issue = BugReportConverter::to_issue_request(&report);

    // Verify title includes severity
    assert!(
        issue.title.contains("[CRITICAL]") || issue.title.contains("Critical"),
        "Title should indicate critical severity"
    );
    assert!(
        issue.title.contains("Parser crashes"),
        "Title should contain bug description"
    );

    // Verify body contains full report
    assert!(
        issue.body.contains("Parser crashes") || issue.body.contains("stack overflow"),
        "Body should contain description"
    );
    assert!(
        issue.body.contains("nested") || issue.body.contains("parens"),
        "Body should contain reproduction code"
    );
    assert!(
        issue.body.contains("Expected") || issue.body.contains("expected") || issue.body.contains("should handle"),
        "Body should contain expected behavior"
    );
    assert!(
        issue.body.contains("Actual") || issue.body.contains("actual") || issue.body.contains("crashes"),
        "Body should contain actual behavior"
    );

    // Verify labels derived from severity
    assert!(
        issue.labels.iter().any(|l| l.contains("critical") || l.contains("severity")),
        "Labels should include severity"
    );

    // Verify labels derived from category
    assert!(
        issue.labels.iter().any(|l| l.contains("crash") || l.contains("bug")),
        "Labels should include category"
    );

    // Test with different severity
    let low_confidence = ConfidenceScore::new(
        0.5,  // Manual testing
        0.5,  // Sometimes reproducible
        0.3,  // Weak evidence
        0.3,  // Unclear root cause
    );

    let low_severity_report = BugReport::new(
        "Minor formatting issue".to_string(),
        "Output formatting is slightly inconsistent".to_string(),
        Severity::Low,
        BugCategory::Other("formatting".to_string()),
        "println(\"Hello\")".to_string(),
        "Consistent formatting".to_string(),
        "Slight inconsistency in whitespace".to_string(),
        low_confidence,
    );

    let low_issue = BugReportConverter::to_issue_request(&low_severity_report);

    assert!(
        low_issue.title.contains("[LOW]") || low_issue.title.contains("Low"),
        "Low severity should be indicated"
    );
}

/// Test: Label Assignment
///
/// This test verifies that labels are assigned based on:
/// - Severity level (critical, high, medium, low)
/// - Bug category (crash, hang, wrong-output, etc.)
/// - Component affected (parser, lexer, typechecker, etc.)
#[test]
fn test_label_assignment() {
    // Helper confidence score
    let high_conf = ConfidenceScore::new(
        0.85,  // Fuzz testing
        1.0,   // Always reproducible
        0.9,   // Strong evidence
        1.0,   // Confirmed
    );

    // Test 1: Critical crash in parser
    let critical_report = BugReport::new(
        "Parser crash".to_string(),
        "Parser crashes on input".to_string(),
        Severity::Critical,
        BugCategory::Crash,
        "test code".to_string(),
        "Should parse".to_string(),
        "Crashes".to_string(),
        high_conf.clone(),
    );

    let issue = BugReportConverter::to_issue_request(&critical_report);

    // Should have severity label
    assert!(
        issue.labels.iter().any(|l| l.to_lowercase().contains("critical")),
        "Should have critical label"
    );

    // Should have bug/crash label
    assert!(
        issue.labels.iter().any(|l| l.to_lowercase().contains("bug") || l.to_lowercase().contains("crash")),
        "Should have bug or crash label"
    );

    // Test 2: High priority wrong output
    let high_report = BugReport::new(
        "Wrong output".to_string(),
        "Output is incorrect".to_string(),
        Severity::High,
        BugCategory::WrongOutput,
        "test".to_string(),
        "correct output".to_string(),
        "wrong output".to_string(),
        high_conf.clone(),
    );

    let high_issue = BugReportConverter::to_issue_request(&high_report);

    assert!(
        high_issue.labels.iter().any(|l| l.to_lowercase().contains("high")),
        "Should have high severity label"
    );

    // Test 3: Medium priority hang
    let medium_report = BugReport::new(
        "Hang issue".to_string(),
        "Program hangs".to_string(),
        Severity::Medium,
        BugCategory::Hang,
        "test".to_string(),
        "completes".to_string(),
        "hangs".to_string(),
        high_conf.clone(),
    );

    let medium_issue = BugReportConverter::to_issue_request(&medium_report);

    assert!(
        medium_issue.labels.iter().any(|l| l.to_lowercase().contains("medium")),
        "Should have medium severity label"
    );

    // Test 4: Low priority other
    let low_report = BugReport::new(
        "Minor issue".to_string(),
        "Minor problem".to_string(),
        Severity::Low,
        BugCategory::Other("minor".to_string()),
        "test".to_string(),
        "perfect".to_string(),
        "minor issue".to_string(),
        high_conf.clone(),
    );

    let low_issue = BugReportConverter::to_issue_request(&low_report);

    assert!(
        low_issue.labels.iter().any(|l| l.to_lowercase().contains("low")),
        "Should have low severity label"
    );

    // Verify no duplicate labels
    for issue_req in [issue, high_issue, medium_issue, low_issue] {
        let unique_labels: std::collections::HashSet<_> = issue_req.labels.iter().collect();
        assert_eq!(
            unique_labels.len(),
            issue_req.labels.len(),
            "Labels should be unique (no duplicates)"
        );
    }
}

/// Test: Comment Posting
///
/// This test verifies that comments can be posted to issues:
/// - Comment body (markdown)
/// - JSON serialization
/// - Updates and lifecycle tracking
#[test]
fn test_comment_posting() {
    // Create a comment
    let comment = CommentRequest::new(
        "## Update\n\nRoot cause identified: off-by-one error in line 123.\n\n**Fix**: PR #456".to_string(),
    );

    // Verify body stored correctly
    assert!(comment.body.contains("Root cause identified"));
    assert!(comment.body.contains("PR #456"));

    // Verify JSON serialization
    let json = comment.to_json();

    assert!(json.contains("\"body\""), "JSON should contain body field");
    assert!(
        json.contains("Root cause identified"),
        "JSON should contain comment text"
    );
    assert!(json.starts_with("{"));
    assert!(json.ends_with("}"));

    // Test lifecycle comment
    let lifecycle_comment = CommentRequest::new(
        "This issue has been resolved in v1.2.3 and will be included in the next release.".to_string(),
    );

    assert!(lifecycle_comment.body.contains("resolved"));
    assert!(lifecycle_comment.body.contains("v1.2.3"));

    // Test update comment with references
    let update_comment = CommentRequest::new(
        "Related to #123 and #456. Possible duplicate of #789.".to_string(),
    );

    assert!(update_comment.body.contains("#123"));
    assert!(update_comment.body.contains("#456"));
    assert!(update_comment.body.contains("#789"));
}

/// Test: GitHubResult Success/Error Handling
///
/// This test verifies that GitHubResult properly wraps success/error:
/// - Success variant with value
/// - Error variant with message
/// - is_ok(), is_err() checks
/// - ok(), err() conversions
#[test]
fn test_github_result_handling() {
    // Test success case
    let success: GitHubResult<u64> = GitHubResult::Success(42);

    assert!(success.is_ok(), "Success should return true for is_ok()");
    assert!(!success.is_err(), "Success should return false for is_err()");

    let value = success.clone().ok();
    assert_eq!(value, Some(42));

    let error = success.err();
    assert_eq!(error, None);

    // Test error case
    let failure: GitHubResult<u64> = GitHubResult::Error("API rate limit exceeded".to_string());

    assert!(!failure.is_ok(), "Error should return false for is_ok()");
    assert!(failure.is_err(), "Error should return true for is_err()");

    let value = failure.clone().ok();
    assert_eq!(value, None);

    let error = failure.err();
    assert_eq!(error, Some("API rate limit exceeded".to_string()));

    // Test with IssueResponse
    let response = IssueResponse::new(
        100,
        "https://github.com/paiml/ruchy/issues/100".to_string(),
        "https://api.github.com/repos/paiml/ruchy/issues/100".to_string(),
        "open".to_string(),
    );

    let result: GitHubResult<IssueResponse> = GitHubResult::Success(response.clone());

    assert!(result.is_ok());
    let unwrapped = result.ok().unwrap();
    assert_eq!(unwrapped.number, 100);
    assert_eq!(unwrapped.state, "open");

    // Test error with IssueResponse
    let error_result: GitHubResult<IssueResponse> =
        GitHubResult::Error("404 Not Found".to_string());

    assert!(error_result.is_err());
    let error_msg = error_result.err().unwrap();
    assert_eq!(error_msg, "404 Not Found");
}
