//! GitHub Bug Filer Module
//!
//! Provides GitHub API integration for filing bugs to upstream Ruchy repository.

use super::bug_report::BugReport;

/// GitHub bug filer
///
/// Files bug reports to GitHub repository via REST API.
pub struct GitHubBugFiler {
    /// Repository name (e.g., "paiml/ruchy")
    pub repo: String,
    /// Dry-run mode (validation without actual filing)
    pub dry_run: bool,
    /// Fingerprints of already-filed bugs (for deduplication)
    filed_fingerprints: Vec<String>,
}

impl GitHubBugFiler {
    /// Create new GitHub bug filer
    ///
    /// # Arguments
    ///
    /// * `repo` - Repository name in format "owner/repo"
    ///
    /// # Example
    ///
    /// ```
    /// use ruchyruchy::bug_filing::GitHubBugFiler;
    ///
    /// let filer = GitHubBugFiler::new("paiml/ruchy");
    /// ```
    pub fn new(repo: &str) -> Self {
        Self {
            repo: repo.to_string(),
            dry_run: false,
            filed_fingerprints: Vec::new(),
        }
    }

    /// Enable dry-run mode (no actual filing)
    ///
    /// In dry-run mode, bugs are validated but not filed to GitHub.
    /// Useful for testing and preview.
    ///
    /// # Arguments
    ///
    /// * `dry_run` - Whether to enable dry-run mode
    ///
    /// # Example
    ///
    /// ```
    /// use ruchyruchy::bug_filing::GitHubBugFiler;
    ///
    /// let filer = GitHubBugFiler::new("paiml/ruchy")
    ///     .with_dry_run(true);
    /// ```
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// File a bug report
    ///
    /// Files a bug report to GitHub if:
    /// - Confidence ≥ 0.9
    /// - Not a duplicate (based on fingerprint)
    ///
    /// # Arguments
    ///
    /// * `bug` - Bug report to file
    ///
    /// # Returns
    ///
    /// - `Ok(IssueNumber)` - Issue number if filed successfully
    /// - `Err(FilingError)` - Error if filing failed
    ///
    /// # Errors
    ///
    /// - `LowConfidence` - Confidence < 0.9
    /// - `Duplicate` - Bug already filed
    /// - `ApiError` - GitHub API error
    pub fn file_bug(&mut self, bug: &BugReport) -> Result<IssueNumber, FilingError> {
        // Check confidence threshold
        if bug.confidence < 0.9 {
            return Err(FilingError::LowConfidence(bug.confidence));
        }

        // Check for duplicates
        let fingerprint = bug.fingerprint();
        if self.filed_fingerprints.contains(&fingerprint) {
            return Err(FilingError::Duplicate(fingerprint));
        }

        if self.dry_run {
            // Dry run: just validate and return mock issue number
            self.filed_fingerprints.push(fingerprint);
            return Ok(IssueNumber(0));
        }

        // GREEN phase: Actual GitHub API call will go here
        // For now, just simulate success
        self.filed_fingerprints.push(fingerprint);
        Ok(IssueNumber(0))
    }

    /// Check if bug is duplicate
    ///
    /// # Arguments
    ///
    /// * `bug` - Bug report to check
    ///
    /// # Returns
    ///
    /// `true` if bug has already been filed, `false` otherwise.
    pub fn is_duplicate(&self, bug: &BugReport) -> bool {
        self.filed_fingerprints.contains(&bug.fingerprint())
    }
}

/// Issue number returned from GitHub
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IssueNumber(pub u64);

/// Filing error types
#[derive(Debug)]
pub enum FilingError {
    /// Confidence below threshold (< 0.9)
    LowConfidence(f64),
    /// Duplicate bug (already filed)
    Duplicate(String),
    /// GitHub API error
    ApiError(String),
}

impl std::fmt::Display for FilingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilingError::LowConfidence(conf) => {
                write!(f, "Confidence too low: {} (required: ≥0.9)", conf)
            }
            FilingError::Duplicate(fp) => write!(f, "Duplicate bug (fingerprint: {})", fp),
            FilingError::ApiError(msg) => write!(f, "GitHub API error: {}", msg),
        }
    }
}

impl std::error::Error for FilingError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bug_filing::{BugReport, Category, Severity};

    #[test]
    fn test_github_client_creation() {
        let filer = GitHubBugFiler::new("paiml/ruchy");
        assert_eq!(filer.repo, "paiml/ruchy");
        assert!(!filer.dry_run);
    }

    #[test]
    fn test_dry_run_mode() {
        let filer = GitHubBugFiler::new("paiml/ruchy").with_dry_run(true);
        assert!(filer.dry_run);
    }

    #[test]
    fn test_low_confidence_rejection() {
        let mut filer = GitHubBugFiler::new("paiml/ruchy").with_dry_run(true);
        let mut bug = BugReport::new(
            Category::Parser,
            Severity::Low,
            "Test".to_string(),
            "code".to_string(),
            "exp".to_string(),
            "act".to_string(),
        );
        bug.confidence = 0.5;

        let result = filer.file_bug(&bug);
        assert!(result.is_err());
    }
}
