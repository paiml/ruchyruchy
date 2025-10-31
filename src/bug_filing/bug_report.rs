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
        let category_str = format!("{:?}", self.category);
        let severity_str = format!("{:?}", self.severity);

        let mut markdown = String::new();
        markdown.push_str(&format!("## Bug Report: {}\n\n", self.title));
        markdown.push_str(&format!("**Category**: {}\n", category_str));
        markdown.push_str(&format!("**Confidence**: {:.2}\n", self.confidence));
        markdown.push_str(&format!("**Severity**: {}\n", severity_str));
        markdown.push_str("**Discovered By**: RuchyRuchy Interpreter v1.10.0\n\n");

        markdown.push_str("### Minimal Reproduction\n\n");
        markdown.push_str("```ruchy\n");
        markdown.push_str(&self.reproduction);
        markdown.push('\n');
        markdown.push_str("```\n\n");

        markdown.push_str("### Expected Behavior\n\n");
        markdown.push_str(&self.expected);
        markdown.push_str("\n\n");

        markdown.push_str("### Actual Behavior\n\n");
        markdown.push_str(&self.actual);
        markdown.push_str("\n\n");

        markdown.push_str("### Environment\n\n");
        for (key, value) in &self.environment {
            markdown.push_str(&format!("- **{}**: {}\n", key, value));
        }
        markdown.push('\n');

        if let Some(test_file) = &self.test_file {
            markdown.push_str(&format!("**Test File**: {}\n\n", test_file));
        }

        markdown.push_str("---\n");
        markdown.push_str("**Filed by**: RuchyRuchy Automated Bug Reporter\n");
        markdown.push_str("**Ticket**: INTERP-034\n");

        markdown
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
