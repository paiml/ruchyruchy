// Bug Reporting Module
// Implements REPORT-001 through REPORT-004 from specification v1.0.0
//
// References:
// - docs/specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md
// - REPORT-001: Quantitative Analysis Engine
// - REPORT-002: Assisted Five-Whys Analysis
// - REPORT-003: TDD Integration
// - REPORT-004: Markdown Report Generator

pub mod confidence;
pub mod five_whys;
pub mod github_integration;
pub mod issue_linking;
pub mod metrics;
pub mod report_generator;
pub mod tdd;
pub mod validation;

pub use confidence::{
    ConfidenceScore, ConfidenceScorer, DiscoveryMethod, Priority, QuantitativeEvidence,
    Reproducibility, RootCauseClarity,
};
pub use five_whys::{
    ConfidenceLevel, DataPoint, FiveWhysAnalysis, FiveWhysAnalyzer, Hypothesis, WhyLayer,
};
pub use github_integration::{
    BugReportConverter, CommentRequest, GitHubClient, GitHubResult, IssueRequest, IssueResponse,
};
pub use issue_linking::{
    BugIssue, DuplicateResult, IssueDeduplicator, RelatedIssue, SimilarityCalculator,
    SimilarityScore, DUPLICATE_THRESHOLD, RELATED_THRESHOLD,
};
pub use metrics::{
    ChurnCorrelation, ComplexityMetrics, DependencyAnalyzer, DependencyNode, QuantitativeAnalysis,
    SatdDetector, SatdType,
};
pub use report_generator::{BugCategory, BugReport, Severity};
pub use tdd::{QualityGate, QualityGates, TddCycle, TddHistory, TddPhase, TestResult};
pub use validation::{
    BugCategory as ValidationBugCategory, BugCorpusValidator, DetectionResult, HistoricalBug,
    ValidationMetrics, ValidationReport,
};
