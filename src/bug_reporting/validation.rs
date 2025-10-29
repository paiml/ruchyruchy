// Bug Discovery Validation Module
// Implements VALID-007 from specification v1.0.0
//
// Purpose: Validate bug discovery system against historical bugs
// - Measure detection rate (target: 95%+)
// - Analyze false positives (<5%)
// - Explain missed bugs
// - Generate validation report
//
// References:
// - Historical Ruchy bugs: https://github.com/paiml/ruchy/issues
// - Target: 79 historical bugs
// - Kim et al. (2013): "Classifying Software Changes: Clean or Buggy?"
// - D'Ambros et al. (2012): "Evaluating Defect Prediction Approaches"

use std::collections::HashMap;

/// Historical bug from GitHub
#[derive(Debug, Clone)]
pub struct HistoricalBug {
    /// Issue number
    pub issue_number: u64,

    /// Issue title
    pub title: String,

    /// Issue body
    pub body: String,

    /// Bug category
    pub category: BugCategory,

    /// Affected files
    pub files: Vec<String>,

    /// Error message (if any)
    pub error_message: Option<String>,

    /// Labels
    pub labels: Vec<String>,

    /// Was this bug critical?
    pub critical: bool,
}

impl HistoricalBug {
    /// Create new historical bug
    pub fn new(issue_number: u64, title: String, body: String, category: BugCategory) -> Self {
        Self {
            issue_number,
            title,
            body,
            category,
            files: Vec::new(),
            error_message: None,
            labels: Vec::new(),
            critical: false,
        }
    }

    /// Add affected file
    pub fn add_file(&mut self, file: String) {
        self.files.push(file);
    }

    /// Set error message
    pub fn set_error_message(&mut self, error: String) {
        self.error_message = Some(error);
    }

    /// Add label
    pub fn add_label(&mut self, label: String) {
        self.labels.push(label);
    }

    /// Mark as critical
    pub fn set_critical(&mut self, critical: bool) {
        self.critical = critical;
    }
}

/// Bug category for classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BugCategory {
    Crash,
    Hang,
    WrongOutput,
    TypeError,
    ParserError,
    CompilerError,
    RuntimeError,
    PerformanceRegression,
    MemoryLeak,
    Other(String),
}

impl BugCategory {
    /// Convert to string
    pub fn as_str(&self) -> &str {
        match self {
            BugCategory::Crash => "Crash",
            BugCategory::Hang => "Hang",
            BugCategory::WrongOutput => "Wrong Output",
            BugCategory::TypeError => "Type Error",
            BugCategory::ParserError => "Parser Error",
            BugCategory::CompilerError => "Compiler Error",
            BugCategory::RuntimeError => "Runtime Error",
            BugCategory::PerformanceRegression => "Performance Regression",
            BugCategory::MemoryLeak => "Memory Leak",
            BugCategory::Other(s) => s,
        }
    }
}

/// Detection result for a bug
#[derive(Debug, Clone)]
pub struct DetectionResult {
    /// Was the bug detected?
    pub detected: bool,

    /// Detection method (if detected)
    pub method: Option<String>,

    /// Confidence score (0.0-1.0)
    pub confidence: f64,

    /// Why was it missed? (if not detected)
    pub miss_reason: Option<String>,
}

impl DetectionResult {
    /// Create detected result
    pub fn detected(method: String, confidence: f64) -> Self {
        Self {
            detected: true,
            method: Some(method),
            confidence,
            miss_reason: None,
        }
    }

    /// Create missed result
    pub fn missed(reason: String) -> Self {
        Self {
            detected: false,
            method: None,
            confidence: 0.0,
            miss_reason: Some(reason),
        }
    }
}

/// Validation metrics
#[derive(Debug, Clone)]
pub struct ValidationMetrics {
    /// Total bugs in corpus
    pub total_bugs: usize,

    /// Bugs detected
    pub detected: usize,

    /// Bugs missed
    pub missed: usize,

    /// False positives
    pub false_positives: usize,

    /// Detection rate (0.0-1.0)
    pub detection_rate: f64,

    /// False positive rate (0.0-1.0)
    pub false_positive_rate: f64,

    /// Critical bugs detected
    pub critical_detected: usize,

    /// Critical bugs total
    pub critical_total: usize,
}

impl ValidationMetrics {
    /// Create new metrics
    pub fn new(total_bugs: usize, detected: usize, false_positives: usize, critical_detected: usize, critical_total: usize) -> Self {
        let missed = total_bugs - detected;
        let detection_rate = if total_bugs > 0 {
            detected as f64 / total_bugs as f64
        } else {
            0.0
        };
        let false_positive_rate = if detected > 0 {
            false_positives as f64 / detected as f64
        } else {
            0.0
        };

        Self {
            total_bugs,
            detected,
            missed,
            false_positives,
            detection_rate,
            false_positive_rate,
            critical_detected,
            critical_total,
        }
    }

    /// Check if meets target (95% detection, <5% false positives)
    pub fn meets_target(&self) -> bool {
        self.detection_rate >= 0.95 && self.false_positive_rate < 0.05
    }

    /// Generate summary report
    pub fn summary(&self) -> String {
        format!(
            "Detection Rate: {:.1}% ({}/{})\nFalse Positive Rate: {:.1}% ({}/{})\nCritical Detection: {:.1}% ({}/{})\nTarget Met: {}",
            self.detection_rate * 100.0,
            self.detected,
            self.total_bugs,
            self.false_positive_rate * 100.0,
            self.false_positives,
            self.detected,
            if self.critical_total > 0 { self.critical_detected as f64 / self.critical_total as f64 * 100.0 } else { 0.0 },
            self.critical_detected,
            self.critical_total,
            if self.meets_target() { "YES ✅" } else { "NO ❌" }
        )
    }
}

/// Validation report
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Validation metrics
    pub metrics: ValidationMetrics,

    /// Detection results by bug
    pub results: HashMap<u64, DetectionResult>,

    /// Missed bugs
    pub missed_bugs: Vec<u64>,

    /// False positives
    pub false_positive_details: Vec<String>,
}

impl ValidationReport {
    /// Create new validation report
    pub fn new(metrics: ValidationMetrics) -> Self {
        Self {
            metrics,
            results: HashMap::new(),
            missed_bugs: Vec::new(),
            false_positive_details: Vec::new(),
        }
    }

    /// Add detection result
    pub fn add_result(&mut self, bug_number: u64, result: DetectionResult) {
        if !result.detected {
            self.missed_bugs.push(bug_number);
        }
        self.results.insert(bug_number, result);
    }

    /// Add false positive
    pub fn add_false_positive(&mut self, details: String) {
        self.false_positive_details.push(details);
    }

    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Bug Discovery Validation Report\n\n");

        md.push_str("## Summary\n\n");
        md.push_str(&self.metrics.summary());
        md.push_str("\n\n");

        md.push_str("## Detection Rate Analysis\n\n");
        md.push_str(&format!("- **Total Historical Bugs**: {}\n", self.metrics.total_bugs));
        md.push_str(&format!("- **Bugs Detected**: {} ({:.1}%)\n",
            self.metrics.detected,
            self.metrics.detection_rate * 100.0
        ));
        md.push_str(&format!("- **Bugs Missed**: {} ({:.1}%)\n",
            self.metrics.missed,
            (self.metrics.missed as f64 / self.metrics.total_bugs as f64) * 100.0
        ));
        md.push_str(&format!("- **Target**: 95%+ detection rate\n"));
        md.push_str(&format!("- **Status**: {}\n\n",
            if self.metrics.detection_rate >= 0.95 { "✅ PASS" } else { "❌ FAIL" }
        ));

        md.push_str("## False Positive Analysis\n\n");
        md.push_str(&format!("- **False Positives**: {} ({:.1}%)\n",
            self.metrics.false_positives,
            self.metrics.false_positive_rate * 100.0
        ));
        md.push_str(&format!("- **Target**: <5% false positive rate\n"));
        md.push_str(&format!("- **Status**: {}\n\n",
            if self.metrics.false_positive_rate < 0.05 { "✅ PASS" } else { "❌ FAIL" }
        ));

        if !self.false_positive_details.is_empty() {
            md.push_str("### False Positive Details\n\n");
            for (i, fp) in self.false_positive_details.iter().enumerate() {
                md.push_str(&format!("{}. {}\n", i + 1, fp));
            }
            md.push_str("\n");
        }

        md.push_str("## Missed Bugs Analysis\n\n");
        if self.missed_bugs.is_empty() {
            md.push_str("✅ **No bugs missed!**\n\n");
        } else {
            md.push_str(&format!("❌ **{} bugs missed:**\n\n", self.missed_bugs.len()));
            for bug_number in &self.missed_bugs {
                if let Some(result) = self.results.get(bug_number) {
                    let reason = result.miss_reason.as_deref().unwrap_or("Unknown");
                    md.push_str(&format!("- Issue #{}: {}\n", bug_number, reason));
                }
            }
            md.push_str("\n");
        }

        md.push_str("## Critical Bugs\n\n");
        md.push_str(&format!("- **Critical Bugs Detected**: {} / {}\n",
            self.metrics.critical_detected,
            self.metrics.critical_total
        ));
        if self.metrics.critical_total > 0 {
            let critical_rate = self.metrics.critical_detected as f64 / self.metrics.critical_total as f64;
            md.push_str(&format!("- **Critical Detection Rate**: {:.1}%\n", critical_rate * 100.0));
        }
        md.push_str("\n");

        md.push_str("---\n\n");
        md.push_str("*Generated by RuchyRuchy Bug Discovery Validation System*\n");

        md
    }
}

/// Bug corpus validator
pub struct BugCorpusValidator {
    /// Historical bugs
    bugs: Vec<HistoricalBug>,
}

impl BugCorpusValidator {
    /// Create new validator
    pub fn new() -> Self {
        Self { bugs: Vec::new() }
    }

    /// Add historical bug
    pub fn add_bug(&mut self, bug: HistoricalBug) {
        self.bugs.push(bug);
    }

    /// Get total bug count
    pub fn bug_count(&self) -> usize {
        self.bugs.len()
    }

    /// Get bugs by category
    pub fn bugs_by_category(&self, category: &BugCategory) -> Vec<&HistoricalBug> {
        self.bugs.iter().filter(|b| &b.category == category).collect()
    }

    /// Get critical bugs
    pub fn critical_bugs(&self) -> Vec<&HistoricalBug> {
        self.bugs.iter().filter(|b| b.critical).collect()
    }

    /// Validate with detection function
    pub fn validate<F>(&self, mut detector: F) -> ValidationReport
    where
        F: FnMut(&HistoricalBug) -> DetectionResult,
    {
        let total_bugs = self.bugs.len();
        let mut detected = 0;
        let mut false_positives = 0; // Would need additional logic to determine
        let mut critical_detected = 0;
        let critical_total = self.critical_bugs().len();

        let mut report = ValidationReport::new(ValidationMetrics::new(
            total_bugs,
            0, // Will update
            false_positives,
            0, // Will update
            critical_total,
        ));

        for bug in &self.bugs {
            let result = detector(bug);

            if result.detected {
                detected += 1;
                if bug.critical {
                    critical_detected += 1;
                }
            }

            report.add_result(bug.issue_number, result);
        }

        // Update metrics with actual counts
        report.metrics = ValidationMetrics::new(
            total_bugs,
            detected,
            false_positives,
            critical_detected,
            critical_total,
        );

        report
    }
}

impl Default for BugCorpusValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_bug_creation() {
        let bug = HistoricalBug::new(
            123,
            "Parser crash".to_string(),
            "Stack overflow".to_string(),
            BugCategory::Crash,
        );

        assert_eq!(bug.issue_number, 123);
        assert_eq!(bug.title, "Parser crash");
        assert_eq!(bug.category, BugCategory::Crash);
        assert!(!bug.critical);
    }

    #[test]
    fn test_historical_bug_add_file() {
        let mut bug = HistoricalBug::new(1, "Test".to_string(), "Desc".to_string(), BugCategory::Crash);
        bug.add_file("parser.rs".to_string());

        assert_eq!(bug.files.len(), 1);
        assert_eq!(bug.files[0], "parser.rs");
    }

    #[test]
    fn test_historical_bug_set_critical() {
        let mut bug = HistoricalBug::new(1, "Test".to_string(), "Desc".to_string(), BugCategory::Crash);
        bug.set_critical(true);

        assert!(bug.critical);
    }

    #[test]
    fn test_bug_category_strings() {
        assert_eq!(BugCategory::Crash.as_str(), "Crash");
        assert_eq!(BugCategory::Hang.as_str(), "Hang");
        assert_eq!(BugCategory::WrongOutput.as_str(), "Wrong Output");
        assert_eq!(BugCategory::TypeError.as_str(), "Type Error");
    }

    #[test]
    fn test_detection_result_detected() {
        let result = DetectionResult::detected("Differential Testing".to_string(), 0.95);

        assert!(result.detected);
        assert_eq!(result.method, Some("Differential Testing".to_string()));
        assert_eq!(result.confidence, 0.95);
        assert!(result.miss_reason.is_none());
    }

    #[test]
    fn test_detection_result_missed() {
        let result = DetectionResult::missed("No test coverage".to_string());

        assert!(!result.detected);
        assert!(result.method.is_none());
        assert_eq!(result.confidence, 0.0);
        assert_eq!(result.miss_reason, Some("No test coverage".to_string()));
    }

    #[test]
    fn test_validation_metrics_creation() {
        let metrics = ValidationMetrics::new(100, 96, 2, 48, 50);

        assert_eq!(metrics.total_bugs, 100);
        assert_eq!(metrics.detected, 96);
        assert_eq!(metrics.missed, 4);
        assert_eq!(metrics.false_positives, 2);
        assert!((metrics.detection_rate - 0.96).abs() < 0.01);
        assert!((metrics.false_positive_rate - 0.0208).abs() < 0.01); // 2/96
    }

    #[test]
    fn test_validation_metrics_meets_target() {
        let good = ValidationMetrics::new(100, 96, 2, 48, 50);
        assert!(good.meets_target()); // 96% detection, 2% FP

        let bad_detection = ValidationMetrics::new(100, 90, 2, 45, 50);
        assert!(!bad_detection.meets_target()); // 90% detection < 95%

        let bad_fp = ValidationMetrics::new(100, 96, 10, 48, 50);
        assert!(!bad_fp.meets_target()); // 10/96 = 10.4% FP > 5%
    }

    #[test]
    fn test_validation_report_creation() {
        let metrics = ValidationMetrics::new(100, 95, 3, 47, 50);
        let report = ValidationReport::new(metrics);

        assert_eq!(report.metrics.total_bugs, 100);
        assert_eq!(report.metrics.detected, 95);
        assert_eq!(report.results.len(), 0);
        assert_eq!(report.missed_bugs.len(), 0);
    }

    #[test]
    fn test_validation_report_add_result() {
        let metrics = ValidationMetrics::new(2, 1, 0, 1, 1);
        let mut report = ValidationReport::new(metrics);

        report.add_result(1, DetectionResult::detected("Method 1".to_string(), 0.9));
        report.add_result(2, DetectionResult::missed("No coverage".to_string()));

        assert_eq!(report.results.len(), 2);
        assert_eq!(report.missed_bugs.len(), 1);
        assert_eq!(report.missed_bugs[0], 2);
    }

    #[test]
    fn test_validation_report_markdown() {
        let metrics = ValidationMetrics::new(10, 9, 0, 5, 5);
        let mut report = ValidationReport::new(metrics);

        report.add_result(1, DetectionResult::detected("Test".to_string(), 0.9));
        report.add_result(2, DetectionResult::missed("Reason".to_string()));

        let markdown = report.to_markdown();

        assert!(markdown.contains("# Bug Discovery Validation Report"));
        assert!(markdown.contains("Detection Rate: 90.0%"));
        assert!(markdown.contains("False Positive Rate: 0.0%"));
    }

    #[test]
    fn test_bug_corpus_validator_creation() {
        let validator = BugCorpusValidator::new();
        assert_eq!(validator.bug_count(), 0);
    }

    #[test]
    fn test_bug_corpus_validator_add_bug() {
        let mut validator = BugCorpusValidator::new();
        let bug = HistoricalBug::new(1, "Test".to_string(), "Desc".to_string(), BugCategory::Crash);

        validator.add_bug(bug);
        assert_eq!(validator.bug_count(), 1);
    }

    #[test]
    fn test_bug_corpus_validator_by_category() {
        let mut validator = BugCorpusValidator::new();

        let bug1 = HistoricalBug::new(1, "Crash".to_string(), "Desc".to_string(), BugCategory::Crash);
        let bug2 = HistoricalBug::new(2, "Hang".to_string(), "Desc".to_string(), BugCategory::Hang);
        let bug3 = HistoricalBug::new(3, "Another crash".to_string(), "Desc".to_string(), BugCategory::Crash);

        validator.add_bug(bug1);
        validator.add_bug(bug2);
        validator.add_bug(bug3);

        let crashes = validator.bugs_by_category(&BugCategory::Crash);
        assert_eq!(crashes.len(), 2);
    }

    #[test]
    fn test_bug_corpus_validator_critical_bugs() {
        let mut validator = BugCorpusValidator::new();

        let mut bug1 = HistoricalBug::new(1, "Critical".to_string(), "Desc".to_string(), BugCategory::Crash);
        bug1.set_critical(true);

        let bug2 = HistoricalBug::new(2, "Normal".to_string(), "Desc".to_string(), BugCategory::Hang);

        validator.add_bug(bug1);
        validator.add_bug(bug2);

        let critical = validator.critical_bugs();
        assert_eq!(critical.len(), 1);
        assert_eq!(critical[0].issue_number, 1);
    }

    #[test]
    fn test_bug_corpus_validator_validate() {
        let mut validator = BugCorpusValidator::new();

        let mut bug1 = HistoricalBug::new(1, "Crash".to_string(), "Desc".to_string(), BugCategory::Crash);
        bug1.set_critical(true);

        let bug2 = HistoricalBug::new(2, "Hang".to_string(), "Desc".to_string(), BugCategory::Hang);

        validator.add_bug(bug1);
        validator.add_bug(bug2);

        // Detector that detects all crashes
        let detector = |bug: &HistoricalBug| {
            if bug.category == BugCategory::Crash {
                DetectionResult::detected("Test detector".to_string(), 0.9)
            } else {
                DetectionResult::missed("Not a crash".to_string())
            }
        };

        let report = validator.validate(detector);

        assert_eq!(report.metrics.total_bugs, 2);
        assert_eq!(report.metrics.detected, 1);
        assert_eq!(report.metrics.missed, 1);
        assert_eq!(report.metrics.critical_detected, 1);
        assert_eq!(report.metrics.critical_total, 1);
    }

    #[test]
    fn test_validation_perfect_score() {
        let mut validator = BugCorpusValidator::new();

        for i in 1..=100 {
            validator.add_bug(HistoricalBug::new(
                i,
                format!("Bug {}", i),
                "Description".to_string(),
                BugCategory::Crash,
            ));
        }

        // Perfect detector
        let detector = |_: &HistoricalBug| {
            DetectionResult::detected("Perfect detector".to_string(), 1.0)
        };

        let report = validator.validate(detector);

        assert_eq!(report.metrics.detection_rate, 1.0);
        assert_eq!(report.metrics.missed, 0);
        assert!(report.metrics.meets_target());
    }
}
