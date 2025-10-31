// GitHub Integration Module
// Implements GITHUB-001 from specification v1.0.0
//
// Purpose: Integrate with GitHub API for automatic bug filing
// - Create issues with comprehensive bug reports
// - Manage labels (severity, category)
// - Post comments for updates
// - Track issue lifecycle
//
// References:
// - GitHub REST API v3: https://docs.github.com/en/rest
// - Issue creation: POST /repos/{owner}/{repo}/issues
// - Label management: POST /repos/{owner}/{repo}/issues/{issue_number}/labels
// - Comments: POST /repos/{owner}/{repo}/issues/{issue_number}/comments
//
// Authentication: Personal Access Token (PAT) via Authorization header
// Rate limits: 5000 requests/hour for authenticated requests

use crate::bug_reporting::report_generator::{BugReport, Severity};

/// GitHub API client
#[derive(Debug, Clone)]
pub struct GitHubClient {
    /// Repository owner (e.g., "paiml")
    owner: String,

    /// Repository name (e.g., "ruchy")
    repo: String,

    /// Personal Access Token
    token: String,

    /// Base API URL (default: https://api.github.com)
    base_url: String,
}

impl GitHubClient {
    /// Create new GitHub client
    pub fn new(owner: String, repo: String, token: String) -> Self {
        Self {
            owner,
            repo,
            token,
            base_url: "https://api.github.com".to_string(),
        }
    }

    /// Set custom base URL (for testing or GitHub Enterprise)
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    /// Get repository owner
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Get repository name
    pub fn repo(&self) -> &str {
        &self.repo
    }

    /// Get authentication header value
    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    /// Build API endpoint URL
    pub fn endpoint(&self, path: &str) -> String {
        format!(
            "{}/repos/{}/{}{}",
            self.base_url, self.owner, self.repo, path
        )
    }
}

/// GitHub issue creation request
#[derive(Debug, Clone)]
pub struct IssueRequest {
    /// Issue title
    pub title: String,

    /// Issue body (markdown)
    pub body: String,

    /// Labels to apply
    pub labels: Vec<String>,

    /// Assignees (GitHub usernames)
    pub assignees: Vec<String>,
}

impl IssueRequest {
    /// Create new issue request
    pub fn new(title: String, body: String) -> Self {
        Self {
            title,
            body,
            labels: Vec::new(),
            assignees: Vec::new(),
        }
    }

    /// Add label
    pub fn add_label(&mut self, label: String) {
        self.labels.push(label);
    }

    /// Add assignee
    pub fn add_assignee(&mut self, assignee: String) {
        self.assignees.push(assignee);
    }

    /// Convert to JSON payload
    pub fn to_json(&self) -> String {
        let mut json = String::from("{");

        // Title
        json.push_str(&format!("\"title\":{:?}", self.title));

        // Body
        json.push_str(&format!(",\"body\":{:?}", self.body));

        // Labels
        if !self.labels.is_empty() {
            json.push_str(",\"labels\":[");
            for (i, label) in self.labels.iter().enumerate() {
                if i > 0 {
                    json.push(',');
                }
                json.push_str(&format!("{:?}", label));
            }
            json.push(']');
        }

        // Assignees
        if !self.assignees.is_empty() {
            json.push_str(",\"assignees\":[");
            for (i, assignee) in self.assignees.iter().enumerate() {
                if i > 0 {
                    json.push(',');
                }
                json.push_str(&format!("{:?}", assignee));
            }
            json.push(']');
        }

        json.push('}');
        json
    }
}

/// GitHub issue response
#[derive(Debug, Clone)]
pub struct IssueResponse {
    /// Issue number
    pub number: u64,

    /// Issue URL
    pub html_url: String,

    /// API URL
    pub url: String,

    /// Issue state (open/closed)
    pub state: String,
}

impl IssueResponse {
    /// Create new issue response
    pub fn new(number: u64, html_url: String, url: String, state: String) -> Self {
        Self {
            number,
            html_url,
            url,
            state,
        }
    }

    /// Parse from JSON (simplified - would use serde in production)
    pub fn from_json(json: &str) -> Option<Self> {
        // Simplified parsing for demonstration
        // In production, use serde_json
        let number = Self::extract_number(json)?;
        let html_url = Self::extract_string(json, "html_url")?;
        let url = Self::extract_string(json, "url")?;
        let state = Self::extract_string(json, "state")?;

        Some(Self::new(number, html_url, url, state))
    }

    fn extract_number(json: &str) -> Option<u64> {
        // Find "number":123
        let start = json.find("\"number\":")?;
        let after_colon = &json[start + 9..];
        let end = after_colon.find(|c: char| !c.is_numeric())?;
        after_colon[..end].parse().ok()
    }

    fn extract_string(json: &str, key: &str) -> Option<String> {
        // Find "key":"value"
        let pattern = format!("\"{}\":\"", key);
        let start = json.find(&pattern)?;
        let after_quote = &json[start + pattern.len()..];
        let end = after_quote.find('"')?;
        Some(after_quote[..end].to_string())
    }
}

/// Bug report to GitHub issue converter
pub struct BugReportConverter;

impl BugReportConverter {
    /// Convert BugReport to IssueRequest
    pub fn to_issue_request(report: &BugReport) -> IssueRequest {
        let title = Self::format_title(report);
        let body = report.to_markdown();

        let mut request = IssueRequest::new(title, body);

        // Add severity label
        request.add_label(Self::severity_label(report.severity));

        // Add category label
        request.add_label(Self::category_label(&report.category));

        // Add confidence label
        if report.confidence.overall >= 0.85 {
            request.add_label("high-confidence".to_string());
        } else if report.confidence.overall >= 0.70 {
            request.add_label("medium-confidence".to_string());
        } else {
            request.add_label("low-confidence".to_string());
        }

        // Add bug label
        request.add_label("bug".to_string());

        request
    }

    fn format_title(report: &BugReport) -> String {
        format!("[{}] {}", report.severity.as_str(), report.title)
    }

    fn severity_label(severity: Severity) -> String {
        match severity {
            Severity::Critical => "severity: critical".to_string(),
            Severity::High => "severity: high".to_string(),
            Severity::Medium => "severity: medium".to_string(),
            Severity::Low => "severity: low".to_string(),
        }
    }

    fn category_label(category: &crate::bug_reporting::report_generator::BugCategory) -> String {
        use crate::bug_reporting::report_generator::BugCategory;

        match category {
            BugCategory::Crash => "type: crash".to_string(),
            BugCategory::Hang => "type: hang".to_string(),
            BugCategory::WrongOutput => "type: wrong-output".to_string(),
            BugCategory::PerformanceRegression => "type: performance".to_string(),
            BugCategory::MemoryLeak => "type: memory-leak".to_string(),
            BugCategory::TypeError => "type: type-error".to_string(),
            BugCategory::ParserError => "type: parser-error".to_string(),
            BugCategory::Other(s) => format!("type: {}", s.to_lowercase().replace(' ', "-")),
        }
    }
}

/// Comment request
#[derive(Debug, Clone)]
pub struct CommentRequest {
    /// Comment body (markdown)
    pub body: String,
}

impl CommentRequest {
    /// Create new comment request
    pub fn new(body: String) -> Self {
        Self { body }
    }

    /// Convert to JSON payload
    pub fn to_json(&self) -> String {
        format!("{{\"body\":{:?}}}", self.body)
    }
}

/// GitHub integration result
#[derive(Debug, Clone)]
pub enum GitHubResult<T> {
    /// Operation succeeded
    Success(T),
    /// Operation failed with error message
    Error(String),
}

impl<T> GitHubResult<T> {
    /// Check if successful
    pub fn is_ok(&self) -> bool {
        matches!(self, GitHubResult::Success(_))
    }

    /// Check if error
    pub fn is_err(&self) -> bool {
        matches!(self, GitHubResult::Error(_))
    }

    /// Get success value
    pub fn ok(self) -> Option<T> {
        match self {
            GitHubResult::Success(v) => Some(v),
            GitHubResult::Error(_) => None,
        }
    }

    /// Get error message
    pub fn err(self) -> Option<String> {
        match self {
            GitHubResult::Success(_) => None,
            GitHubResult::Error(e) => Some(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bug_discovery::confidence::ConfidenceScore;
    use crate::bug_reporting::report_generator::{BugCategory, Severity};

    #[test]
    fn test_github_client_creation() {
        let client = GitHubClient::new(
            "paiml".to_string(),
            "ruchy".to_string(),
            "ghp_test123".to_string(),
        );

        assert_eq!(client.owner(), "paiml");
        assert_eq!(client.repo(), "ruchy");
        assert_eq!(client.auth_header(), "Bearer ghp_test123");
        assert_eq!(client.base_url, "https://api.github.com");
    }

    #[test]
    fn test_github_client_custom_base_url() {
        let client =
            GitHubClient::new("owner".to_string(), "repo".to_string(), "token".to_string())
                .with_base_url("https://custom.github.com".to_string());

        assert_eq!(client.base_url, "https://custom.github.com");
    }

    #[test]
    fn test_endpoint_building() {
        let client = GitHubClient::new(
            "paiml".to_string(),
            "ruchy".to_string(),
            "token".to_string(),
        );

        assert_eq!(
            client.endpoint("/issues"),
            "https://api.github.com/repos/paiml/ruchy/issues"
        );
        assert_eq!(
            client.endpoint("/issues/123"),
            "https://api.github.com/repos/paiml/ruchy/issues/123"
        );
    }

    #[test]
    fn test_issue_request_creation() {
        let request = IssueRequest::new("Test bug".to_string(), "Bug description".to_string());

        assert_eq!(request.title, "Test bug");
        assert_eq!(request.body, "Bug description");
        assert_eq!(request.labels.len(), 0);
        assert_eq!(request.assignees.len(), 0);
    }

    #[test]
    fn test_issue_request_add_label() {
        let mut request = IssueRequest::new("Test".to_string(), "Body".to_string());
        request.add_label("bug".to_string());
        request.add_label("critical".to_string());

        assert_eq!(request.labels.len(), 2);
        assert_eq!(request.labels[0], "bug");
        assert_eq!(request.labels[1], "critical");
    }

    #[test]
    fn test_issue_request_add_assignee() {
        let mut request = IssueRequest::new("Test".to_string(), "Body".to_string());
        request.add_assignee("noahgift".to_string());

        assert_eq!(request.assignees.len(), 1);
        assert_eq!(request.assignees[0], "noahgift");
    }

    #[test]
    fn test_issue_request_to_json() {
        let mut request = IssueRequest::new("Test title".to_string(), "Test body".to_string());
        request.add_label("bug".to_string());
        request.add_assignee("user".to_string());

        let json = request.to_json();

        assert!(json.contains("\"title\":\"Test title\""));
        assert!(json.contains("\"body\":\"Test body\""));
        assert!(json.contains("\"labels\":[\"bug\"]"));
        assert!(json.contains("\"assignees\":[\"user\"]"));
    }

    #[test]
    fn test_issue_response_creation() {
        let response = IssueResponse::new(
            123,
            "https://github.com/paiml/ruchy/issues/123".to_string(),
            "https://api.github.com/repos/paiml/ruchy/issues/123".to_string(),
            "open".to_string(),
        );

        assert_eq!(response.number, 123);
        assert_eq!(
            response.html_url,
            "https://github.com/paiml/ruchy/issues/123"
        );
        assert_eq!(response.state, "open");
    }

    #[test]
    fn test_issue_response_from_json() {
        let json = r#"{"number":456,"html_url":"https://github.com/test/test/issues/456","url":"https://api.github.com/repos/test/test/issues/456","state":"open"}"#;

        let response = IssueResponse::from_json(json).unwrap();

        assert_eq!(response.number, 456);
        assert_eq!(response.html_url, "https://github.com/test/test/issues/456");
        assert_eq!(response.state, "open");
    }

    #[test]
    fn test_bug_report_to_issue_request() {
        let confidence = ConfidenceScore {
            overall: 0.9,
            discovery_method_weight: 0.95,
            reproducibility_score: 0.90,
            quantitative_evidence: 0.90,
            root_cause_clarity: 0.85,
        };

        let report = BugReport::new(
            "Parser crashes on nested expressions".to_string(),
            "Stack overflow on deep nesting".to_string(),
            Severity::Critical,
            BugCategory::Crash,
            "fun test() { ((1)) }".to_string(),
            "Should parse".to_string(),
            "Stack overflow".to_string(),
            confidence,
        );

        let request = BugReportConverter::to_issue_request(&report);

        assert_eq!(
            request.title,
            "[CRITICAL] Parser crashes on nested expressions"
        );
        assert!(request.body.contains("# 🔴 CRITICAL"));
        assert!(request.labels.contains(&"severity: critical".to_string()));
        assert!(request.labels.contains(&"type: crash".to_string()));
        assert!(request.labels.contains(&"high-confidence".to_string()));
        assert!(request.labels.contains(&"bug".to_string()));
    }

    #[test]
    fn test_severity_labels() {
        assert_eq!(
            BugReportConverter::severity_label(Severity::Critical),
            "severity: critical"
        );
        assert_eq!(
            BugReportConverter::severity_label(Severity::High),
            "severity: high"
        );
        assert_eq!(
            BugReportConverter::severity_label(Severity::Medium),
            "severity: medium"
        );
        assert_eq!(
            BugReportConverter::severity_label(Severity::Low),
            "severity: low"
        );
    }

    #[test]
    fn test_category_labels() {
        assert_eq!(
            BugReportConverter::category_label(&BugCategory::Crash),
            "type: crash"
        );
        assert_eq!(
            BugReportConverter::category_label(&BugCategory::Hang),
            "type: hang"
        );
        assert_eq!(
            BugReportConverter::category_label(&BugCategory::WrongOutput),
            "type: wrong-output"
        );
        assert_eq!(
            BugReportConverter::category_label(&BugCategory::PerformanceRegression),
            "type: performance"
        );
    }

    #[test]
    fn test_confidence_labels() {
        let high_conf = ConfidenceScore {
            overall: 0.9,
            discovery_method_weight: 0.95,
            reproducibility_score: 0.9,
            quantitative_evidence: 0.9,
            root_cause_clarity: 0.85,
        };

        let report_high = BugReport::new(
            "Test".to_string(),
            "Desc".to_string(),
            Severity::High,
            BugCategory::Crash,
            "code".to_string(),
            "exp".to_string(),
            "act".to_string(),
            high_conf,
        );

        let request = BugReportConverter::to_issue_request(&report_high);
        assert!(request.labels.contains(&"high-confidence".to_string()));
    }

    #[test]
    fn test_comment_request() {
        let comment = CommentRequest::new("This is a test comment".to_string());

        assert_eq!(comment.body, "This is a test comment");

        let json = comment.to_json();
        assert!(json.contains("\"body\":\"This is a test comment\""));
    }

    #[test]
    fn test_github_result_success() {
        let result: GitHubResult<String> = GitHubResult::Success("test".to_string());

        assert!(result.is_ok());
        assert!(!result.is_err());
        assert_eq!(result.ok(), Some("test".to_string()));
    }

    #[test]
    fn test_github_result_error() {
        let result: GitHubResult<String> = GitHubResult::Error("error message".to_string());

        assert!(!result.is_ok());
        assert!(result.is_err());
        assert_eq!(result.err(), Some("error message".to_string()));
    }
}
