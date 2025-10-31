//! Bug Filing Module (INTERP-034)
//!
//! Provides infrastructure for filing discovered bugs to the upstream
//! Ruchy compiler repository at https://github.com/paiml/ruchy/issues.
//!
//! # Overview
//!
//! This module systematically files bugs discovered during RuchyRuchy
//! interpreter development (INTERP-001 through INTERP-033) to help
//! improve the upstream Ruchy compiler.
//!
//! # Architecture
//!
//! - **BugReport**: Bug representation with GitHub markdown generation
//! - **ConfidenceCalculator**: 4-factor scoring algorithm
//! - **GitHubBugFiler**: GitHub API integration
//!
//! # Confidence Scoring
//!
//! Only bugs with confidence ≥0.9 are filed. Confidence is calculated from:
//!
//! 1. **Reproducibility (40%)**: Always/Sometimes/Never
//! 2. **Minimality (30%)**: <10 / 10-50 / >50 lines
//! 3. **Spec Violation (20%)**: Clear / Undefined
//! 4. **Impact (10%)**: Critical / High / Medium / Low
//!
//! # Usage
//!
//! ```no_run
//! use ruchyruchy::bug_filing::{BugReport, Category, Severity};
//! use ruchyruchy::bug_filing::{GitHubBugFiler, ConfidenceCalculator};
//! use ruchyruchy::bug_filing::{ConfidenceFactors, Reproducibility, Impact};
//!
//! // Create bug report
//! let mut bug = BugReport::new(
//!     Category::Parser,
//!     Severity::High,
//!     "Parser fails on if-else".to_string(),
//!     "if (true) 42 else 0".to_string(),
//!     "Should parse successfully".to_string(),
//!     "Parse error: unexpected token".to_string(),
//! );
//!
//! // Calculate confidence
//! let factors = ConfidenceFactors {
//!     reproducibility: Reproducibility::Always,
//!     lines_of_code: 5,
//!     spec_violation: true,
//!     impact: Impact::Critical,
//! };
//! bug.confidence = ConfidenceCalculator::calculate(&factors);
//!
//! // File bug (dry-run mode for testing)
//! let mut filer = GitHubBugFiler::new("paiml/ruchy")
//!     .with_dry_run(true);
//! let result = filer.file_bug(&bug);
//! ```
//!
//! # Output Format
//!
//! GitHub issues are created with:
//! - Title with bug description
//! - Category and severity labels
//! - Confidence score
//! - Minimal reproduction code
//! - Expected vs actual behavior
//! - Environment information
//! - Test file reference

/// Bug report representation
pub mod bug_report;
/// Confidence calculation
pub mod confidence;
/// GitHub API client
pub mod github_client;

// Re-export main types
pub use bug_report::{BugReport, Category, Severity};
pub use confidence::{ConfidenceCalculator, ConfidenceFactors, Impact, Reproducibility};
pub use github_client::{FilingError, GitHubBugFiler, IssueNumber};
