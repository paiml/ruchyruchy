//! Bug Report Module
//!
//! Provides bug report representation and GitHub issue markdown generation
//! for filing bugs to the upstream Ruchy compiler repository.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Bug category classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// Parser bugs (syntax, tokenization, AST)
    Parser,
    /// Runtime bugs (execution, evaluation)
    Runtime,
    /// Type system bugs (inference, checking)
    TypeSystem,
    /// Standard library bugs (built-in functions)
    Stdlib,
    /// Documentation bugs (examples, specs)
    Documentation,
}

/// Severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Critical bugs (crashes, data loss)
    Critical,
    /// High severity bugs (incorrect behavior)
    High,
    /// Medium severity bugs (minor issues)
    Medium,
    /// Low severity bugs (cosmetic)
    Low,
}

/// Bug report structure
///
/// Represents a bug report for filing to upstream Ruchy compiler.
#[derive(Debug, Clone)]
pub struct BugReport {
    /// Bug category
    pub category: Category,
    /// Severity level
    pub severity: Severity,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Bug title
    pub title: String,
    /// Minimal reproduction code
    pub reproduction: String,
    /// Expected behavior
    pub expected: String,
    /// Actual behavior
    pub actual: String,
    /// Test file where bug was discovered
    pub test_file: Option<String>,
    /// Environment information
    pub environment: HashMap<String, String>,
}

impl BugReport {
    /// Create a new bug report
    ///
    /// # Arguments
    ///
    /// * `category` - Bug category classification
    /// * `severity` - Severity level
    /// * `title` - Bug title
    /// * `reproduction` - Minimal reproduction code
    /// * `expected` - Expected behavior
    /// * `actual` - Actual behavior
    ///
    /// # Example
    ///
    /// ```
    /// use ruchyruchy::bug_filing::{BugReport, Category, Severity};
    ///
    /// let bug = BugReport::new(
    ///     Category::Parser,
    ///     Severity::High,
    ///     "Parser fails on if-else".to_string(),
    ///     "if (true) 42 else 0".to_string(),
    ///     "Should parse".to_string(),
    ///     "Parse error".to_string(),
    /// );
    /// ```
    pub fn new(
        category: Category,
        severity: Severity,
        title: String,
        reproduction: String,
        expected: String,
        actual: String,
    ) -> Self {
        let mut environment = HashMap::new();
        environment.insert("ruchy_version".to_string(), "0.1.0".to_string());
        environment.insert("os".to_string(), "Linux".to_string());
        environment.insert("ruchyruchy_version".to_string(), "v1.10.0".to_string());

        Self {
            category,
            severity,
            confidence: 0.0, // Will be calculated
            title,
            reproduction,
            expected,
            actual,
            test_file: None,
            environment,
        }
    }

    /// Convert to GitHub issue markdown format
    ///
    /// Generates a complete GitHub issue body with all required sections.
    ///
    /// # Returns
    ///
    /// Markdown-formatted string ready for GitHub issue creation.
    pub fn to_github_markdown(&self) -> String {
        let mut markdown = String::new();

        // Header section
        markdown.push_str(&self.format_header());

        // Code reproduction section
        markdown.push_str(&self.format_reproduction());

        // Behavior sections
        markdown.push_str(&self.format_expected_behavior());
        markdown.push_str(&self.format_actual_behavior());

        // Environment and metadata
        markdown.push_str(&self.format_environment());
        if let Some(test_file) = &self.test_file {
            markdown.push_str(&format!("**Test File**: {}\n\n", test_file));
        }

        // Footer
        markdown.push_str(&Self::format_footer());

        markdown
    }

    /// Format bug report header with metadata
    fn format_header(&self) -> String {
        let category_str = format!("{:?}", self.category);
        let severity_str = format!("{:?}", self.severity);

        format!(
            "## Bug Report: {}\n\n\
             **Category**: {}\n\
             **Confidence**: {:.2}\n\
             **Severity**: {}\n\
             **Discovered By**: RuchyRuchy Interpreter v1.10.0\n\n",
            self.title, category_str, self.confidence, severity_str
        )
    }

    /// Format minimal reproduction code section
    fn format_reproduction(&self) -> String {
        format!(
            "### Minimal Reproduction\n\n\
             ```ruchy\n\
             {}\n\
             ```\n\n",
            self.reproduction
        )
    }

    /// Format expected behavior section
    fn format_expected_behavior(&self) -> String {
        format!("### Expected Behavior\n\n{}\n\n", self.expected)
    }

    /// Format actual behavior section
    fn format_actual_behavior(&self) -> String {
        format!("### Actual Behavior\n\n{}\n\n", self.actual)
    }

    /// Format environment information section
    fn format_environment(&self) -> String {
        let mut env_section = String::from("### Environment\n\n");
        for (key, value) in &self.environment {
            env_section.push_str(&format!("- **{}**: {}\n", key, value));
        }
        env_section.push('\n');
        env_section
    }

    /// Format issue footer with attribution
    fn format_footer() -> String {
        String::from(
            "---\n\
             **Filed by**: RuchyRuchy Automated Bug Reporter\n\
             **Ticket**: INTERP-034\n",
        )
    }

    /// Calculate bug fingerprint for deduplication
    ///
    /// Generates a hash-based fingerprint from reproduction code and error message.
    /// Used to detect duplicate bugs before filing.
    ///
    /// # Returns
    ///
    /// Hexadecimal string representing the bug fingerprint.
    pub fn fingerprint(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.reproduction.hash(&mut hasher);
        self.actual.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Builder pattern: Set confidence score
    ///
    /// # Arguments
    ///
    /// * `confidence` - Confidence score (0.0-1.0)
    ///
    /// # Example
    ///
    /// ```
    /// use ruchyruchy::bug_filing::{BugReport, Category, Severity};
    ///
    /// let bug = BugReport::new(
    ///     Category::Parser,
    ///     Severity::High,
    ///     "Bug".to_string(),
    ///     "code".to_string(),
    ///     "expected".to_string(),
    ///     "actual".to_string(),
    /// ).with_confidence(0.95);
    /// ```
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }

    /// Builder pattern: Set test file
    ///
    /// # Arguments
    ///
    /// * `test_file` - Path to test file where bug was discovered
    ///
    /// # Example
    ///
    /// ```
    /// use ruchyruchy::bug_filing::{BugReport, Category, Severity};
    ///
    /// let bug = BugReport::new(
    ///     Category::Parser,
    ///     Severity::High,
    ///     "Bug".to_string(),
    ///     "code".to_string(),
    ///     "expected".to_string(),
    ///     "actual".to_string(),
    /// ).with_test_file("tests/test_parser.rs".to_string());
    /// ```
    pub fn with_test_file(mut self, test_file: String) -> Self {
        self.test_file = Some(test_file);
        self
    }

    /// Builder pattern: Add environment variable
    ///
    /// # Arguments
    ///
    /// * `key` - Environment variable name
    /// * `value` - Environment variable value
    ///
    /// # Example
    ///
    /// ```
    /// use ruchyruchy::bug_filing::{BugReport, Category, Severity};
    ///
    /// let bug = BugReport::new(
    ///     Category::Parser,
    ///     Severity::High,
    ///     "Bug".to_string(),
    ///     "code".to_string(),
    ///     "expected".to_string(),
    ///     "actual".to_string(),
    /// ).with_env("custom_var", "custom_value");
    /// ```
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bug_report_creation() {
        let bug = BugReport::new(
            Category::Parser,
            Severity::High,
            "Test bug".to_string(),
            "code".to_string(),
            "expected".to_string(),
            "actual".to_string(),
        );

        assert_eq!(bug.category, Category::Parser);
        assert_eq!(bug.severity, Severity::High);
    }

    #[test]
    fn test_fingerprint_consistency() {
        let bug1 = BugReport::new(
            Category::Parser,
            Severity::High,
            "Bug 1".to_string(),
            "same code".to_string(),
            "expected".to_string(),
            "same error".to_string(),
        );

        let bug2 = BugReport::new(
            Category::Runtime,
            Severity::Low,
            "Bug 2".to_string(),
            "same code".to_string(),
            "expected".to_string(),
            "same error".to_string(),
        );

        // Same reproduction + error = same fingerprint
        assert_eq!(bug1.fingerprint(), bug2.fingerprint());
    }
}
