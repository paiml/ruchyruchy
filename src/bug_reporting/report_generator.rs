// Bug Report Generator
// Implements REPORT-004 from specification v1.0.0
//
// Purpose: Generate comprehensive markdown bug reports integrating:
// - REPORT-001: Quantitative Analysis Engine (complexity, SATD, churn, coupling)
// - REPORT-002: Assisted Five-Whys Analysis (root cause investigation)
// - REPORT-003: TDD Integration (RED-GREEN-REFACTOR workflow)
// - Confidence Scoring (from bug_discovery module)
//
// References:
// - docs/specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md (Section 7: Bug Reporter Module)
// - Potdar & Shihab (2014): "Characteristics of Self-Admitted Technical Debt"
// - Chidamber & Kemerer (1994): Metrics suite for Object-Oriented design
// - Ohno (1988): Toyota Production System (Five-Whys)
// - Beck (2003): Test-Driven Development

use crate::bug_discovery::confidence::ConfidenceScore;
use crate::bug_reporting::five_whys::FiveWhysAnalysis;
use crate::bug_reporting::metrics::QuantitativeAnalysis;
use crate::bug_reporting::tdd::TddHistory;

/// Bug severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Critical, // Crash, data loss, security
    High,     // Major functionality broken
    Medium,   // Minor functionality broken, workaround exists
    Low,      // Cosmetic, documentation
}

impl Severity {
    /// Convert to emoji for markdown
    pub fn to_emoji(&self) -> &'static str {
        match self {
            Severity::Critical => "🔴",
            Severity::High => "🟠",
            Severity::Medium => "🟡",
            Severity::Low => "🟢",
        }
    }

    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
        }
    }
}

/// Bug category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BugCategory {
    Crash,
    Hang,
    WrongOutput,
    PerformanceRegression,
    MemoryLeak,
    TypeError,
    ParserError,
    Other(String),
}

impl BugCategory {
    /// Convert to string
    pub fn as_str(&self) -> &str {
        match self {
            BugCategory::Crash => "Crash",
            BugCategory::Hang => "Hang/Timeout",
            BugCategory::WrongOutput => "Incorrect Output",
            BugCategory::PerformanceRegression => "Performance Regression",
            BugCategory::MemoryLeak => "Memory Leak",
            BugCategory::TypeError => "Type Error",
            BugCategory::ParserError => "Parser Error",
            BugCategory::Other(s) => s,
        }
    }
}

/// Complete bug report
#[derive(Debug, Clone)]
pub struct BugReport {
    /// Bug title
    pub title: String,

    /// Bug description
    pub description: String,

    /// Severity level
    pub severity: Severity,

    /// Bug category
    pub category: BugCategory,

    /// Minimal reproduction code
    pub reproduction_code: String,

    /// Expected behavior
    pub expected: String,

    /// Actual behavior
    pub actual: String,

    /// Confidence score (from bug_discovery)
    pub confidence: ConfidenceScore,

    /// Quantitative analysis (REPORT-001)
    pub quantitative_analysis: Option<QuantitativeAnalysis>,

    /// Five-Whys analysis (REPORT-002)
    pub five_whys: Option<FiveWhysAnalysis>,

    /// TDD history (REPORT-003)
    pub tdd_history: Option<TddHistory>,

    /// Related files
    pub related_files: Vec<String>,

    /// Fix strategy recommendations
    pub fix_recommendations: Vec<String>,

    /// Prevention strategy
    pub prevention: Vec<String>,
}

impl BugReport {
    /// Create new bug report
    pub fn new(
        title: String,
        description: String,
        severity: Severity,
        category: BugCategory,
        reproduction_code: String,
        expected: String,
        actual: String,
        confidence: ConfidenceScore,
    ) -> Self {
        Self {
            title,
            description,
            severity,
            category,
            reproduction_code,
            expected,
            actual,
            confidence,
            quantitative_analysis: None,
            five_whys: None,
            tdd_history: None,
            related_files: Vec::new(),
            fix_recommendations: Vec::new(),
            prevention: Vec::new(),
        }
    }

    /// Add quantitative analysis
    pub fn with_quantitative_analysis(mut self, analysis: QuantitativeAnalysis) -> Self {
        self.quantitative_analysis = Some(analysis);
        self
    }

    /// Add Five-Whys analysis
    pub fn with_five_whys(mut self, analysis: FiveWhysAnalysis) -> Self {
        self.five_whys = Some(analysis);
        self
    }

    /// Add TDD history
    pub fn with_tdd_history(mut self, history: TddHistory) -> Self {
        self.tdd_history = Some(history);
        self
    }

    /// Add related file
    pub fn add_related_file(&mut self, file: String) {
        self.related_files.push(file);
    }

    /// Add fix recommendation
    pub fn add_fix_recommendation(&mut self, recommendation: String) {
        self.fix_recommendations.push(recommendation);
    }

    /// Add prevention strategy
    pub fn add_prevention(&mut self, strategy: String) {
        self.prevention.push(strategy);
    }

    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut report = String::new();

        // Title
        report.push_str(&format!("# {} {}: {}\n\n",
            self.severity.to_emoji(),
            self.severity.as_str(),
            self.title
        ));

        // Executive Summary
        report.push_str("## Executive Summary\n\n");
        report.push_str(&format!("**Category**: {}\n", self.category.as_str()));
        report.push_str(&format!("**Severity**: {} {}\n", self.severity.to_emoji(), self.severity.as_str()));
        report.push_str(&format!("**Confidence Score**: {:.2} / 1.0\n\n", self.confidence.overall));
        report.push_str(&format!("{}\n\n", self.description));

        // Impact statement
        let impact = match self.severity {
            Severity::Critical => "Impact: Critical system functionality affected. Immediate attention required.",
            Severity::High => "Impact: Major functionality impaired. High priority fix needed.",
            Severity::Medium => "Impact: Moderate functionality affected. Workaround may exist.",
            Severity::Low => "Impact: Minor issue with limited user impact.",
        };
        report.push_str(&format!("**{}**\n\n", impact));

        // Bug Details (collapsible)
        report.push_str("<details>\n<summary>\n\n## Bug Details\n\n</summary>\n\n");
        report.push_str("### Reproduction Steps\n\n");
        report.push_str("```ruchy\n");
        report.push_str(&self.reproduction_code);
        report.push_str("\n```\n\n");

        report.push_str("### Expected Behavior\n\n");
        report.push_str(&format!("{}\n\n", self.expected));

        report.push_str("### Actual Behavior\n\n");
        report.push_str(&format!("{}\n\n", self.actual));
        report.push_str("</details>\n\n");

        // Confidence Breakdown (collapsible)
        report.push_str("<details>\n<summary>\n\n## Confidence Analysis\n\n</summary>\n\n");
        report.push_str(&format!("**Overall Confidence**: {:.2} / 1.0\n\n", self.confidence.overall));
        report.push_str("### Confidence Factors\n\n");
        report.push_str(&format!("- **Discovery Method**: {:.2} (weight: {})\n",
            self.confidence.discovery_method_weight,
            "35%"
        ));
        report.push_str(&format!("- **Reproducibility**: {:.2} (weight: {})\n",
            self.confidence.reproducibility_score,
            "30%"
        ));
        report.push_str(&format!("- **Quantitative Evidence**: {:.2} (weight: {})\n",
            self.confidence.quantitative_evidence,
            "20%"
        ));
        report.push_str(&format!("- **Root Cause Clarity**: {:.2} (weight: {})\n\n",
            self.confidence.root_cause_clarity,
            "15%"
        ));
        report.push_str("</details>\n\n");

        // Quantitative Analysis (REPORT-001) (collapsible)
        if let Some(ref analysis) = self.quantitative_analysis {
            report.push_str("<details>\n<summary>\n\n## Quantitative Analysis\n\n</summary>\n\n");
            report.push_str(&format!("**Overall Risk Score**: {:.2} / 1.0 ({:?})\n\n",
                analysis.risk_score,
                analysis.risk_level()
            ));

            report.push_str("### Complexity Metrics\n\n");
            report.push_str(&format!("- **Lines of Code**: {}\n", analysis.complexity.loc));
            report.push_str(&format!("- **Cyclomatic Complexity**: {}\n", analysis.complexity.cyclomatic));
            report.push_str(&format!("- **Cognitive Complexity**: {}\n", analysis.complexity.cognitive));
            report.push_str(&format!("- **Halstead Difficulty**: {:.2}\n", analysis.complexity.halstead_difficulty));
            report.push_str(&format!("- **Parameters**: {}\n", analysis.complexity.parameters));
            report.push_str(&format!("- **Nesting Depth**: {}\n\n", analysis.complexity.nesting_depth));

            if let Some(ref churn) = analysis.churn {
                report.push_str("### Code Churn Analysis\n\n");
                report.push_str(&format!("- **Total Changes**: {}\n", churn.changes));
                report.push_str(&format!("- **Bugs Found**: {}\n", churn.bugs));
                let bugs_per_change = if churn.changes > 0 {
                    churn.bugs as f64 / churn.changes as f64
                } else {
                    0.0
                };
                report.push_str(&format!("- **Bugs per Change**: {:.3}\n\n", bugs_per_change));
            }

            report.push_str("### Technical Debt (SATD)\n\n");
            report.push_str(&format!("- **Total SATD Comments**: {}\n", analysis.satd_count));
            report.push_str(&format!("- **Severity Score**: {:.2} / 1.0\n\n", analysis.satd_severity));

            report.push_str("### Dependency Analysis\n\n");
            report.push_str(&format!("- **Coupling**: {}\n\n", analysis.coupling));
            report.push_str("</details>\n\n");
        }

        // Five-Whys Analysis (REPORT-002) (collapsible)
        if let Some(ref five_whys) = self.five_whys {
            report.push_str("<details>\n<summary>\n\n## Five-Whys Root Cause Analysis\n\n</summary>\n\n");
            report.push_str(&five_whys.to_markdown());
            report.push_str("\n</details>\n\n");
        }

        // TDD Workflow (REPORT-003) (collapsible)
        if let Some(ref tdd) = self.tdd_history {
            report.push_str("<details>\n<summary>\n\n## TDD Fix Workflow\n\n</summary>\n\n");
            report.push_str("### Recommended Approach\n\n");
            report.push_str("Follow the RED-GREEN-REFACTOR cycle:\n\n");
            report.push_str("1. **RED**: Write failing test that reproduces the bug\n");
            report.push_str("2. **GREEN**: Make minimal change to pass the test\n");
            report.push_str("3. **REFACTOR**: Clean up code while keeping tests green\n\n");

            if tdd.cycles().len() > 0 {
                report.push_str("### TDD History\n\n");
                report.push_str(&tdd.to_markdown());
                report.push_str("\n");
            }
            report.push_str("</details>\n\n");
        }

        // Related Files
        if !self.related_files.is_empty() {
            report.push_str("## Related Files\n\n");
            for file in &self.related_files {
                report.push_str(&format!("- `{}`\n", file));
            }
            report.push_str("\n");
        }

        // Fix Recommendations
        if !self.fix_recommendations.is_empty() {
            report.push_str("## Fix Recommendations\n\n");
            for (i, rec) in self.fix_recommendations.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", i + 1, rec));
            }
            report.push_str("\n");
        }

        // Prevention Strategy
        if !self.prevention.is_empty() {
            report.push_str("## Prevention Strategy\n\n");
            for strategy in &self.prevention {
                report.push_str(&format!("- {}\n", strategy));
            }
            report.push_str("\n");
        }

        // Footer
        report.push_str("---\n\n");
        report.push_str("*Generated by RuchyRuchy Bug Reporting System*\n");
        report.push_str("*Following Toyota Production System principles (Jidoka, Kaizen)*\n");

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bug_discovery::confidence::ConfidenceScore;
    use crate::bug_reporting::five_whys::{FiveWhysAnalysis, WhyLayer};
    use crate::bug_reporting::metrics::{ComplexityMetrics, QuantitativeAnalysis};
    use crate::bug_reporting::tdd::{TddCycle, TddHistory, TddPhase, TestResult};

    #[test]
    fn test_severity_emoji() {
        assert_eq!(Severity::Critical.to_emoji(), "🔴");
        assert_eq!(Severity::High.to_emoji(), "🟠");
        assert_eq!(Severity::Medium.to_emoji(), "🟡");
        assert_eq!(Severity::Low.to_emoji(), "🟢");
    }

    #[test]
    fn test_severity_string() {
        assert_eq!(Severity::Critical.as_str(), "CRITICAL");
        assert_eq!(Severity::High.as_str(), "HIGH");
        assert_eq!(Severity::Medium.as_str(), "MEDIUM");
        assert_eq!(Severity::Low.as_str(), "LOW");
    }

    #[test]
    fn test_bug_category_string() {
        assert_eq!(BugCategory::Crash.as_str(), "Crash");
        assert_eq!(BugCategory::Hang.as_str(), "Hang/Timeout");
        assert_eq!(BugCategory::WrongOutput.as_str(), "Incorrect Output");
        assert_eq!(BugCategory::Other("Custom".to_string()).as_str(), "Custom");
    }

    #[test]
    fn test_bug_report_creation() {
        let confidence = ConfidenceScore {
            overall: 0.85,
            discovery_method_weight: 0.9,
            reproducibility_score: 0.8,
            quantitative_evidence: 0.85,
            root_cause_clarity: 0.75,
        };

        let report = BugReport::new(
            "Parser crashes on nested expressions".to_string(),
            "Parser fails with stack overflow on deeply nested expressions".to_string(),
            Severity::Critical,
            BugCategory::Crash,
            "fun test() { let x = ((((1)))); }".to_string(),
            "Should parse successfully".to_string(),
            "Stack overflow error".to_string(),
            confidence,
        );

        assert_eq!(report.title, "Parser crashes on nested expressions");
        assert_eq!(report.severity, Severity::Critical);
        assert_eq!(report.category, BugCategory::Crash);
        assert_eq!(report.confidence.overall, 0.85);
    }

    #[test]
    fn test_bug_report_with_quantitative_analysis() {
        let confidence = ConfidenceScore {
            overall: 0.8,
            discovery_method_weight: 0.9,
            reproducibility_score: 0.7,
            quantitative_evidence: 0.8,
            root_cause_clarity: 0.7,
        };

        let complexity = ComplexityMetrics::new(100);
        let analysis = QuantitativeAnalysis::new(
            complexity,
            None, // churn
            5, // satd_count
            0.5, // satd_severity
            3, // coupling
        );

        let report = BugReport::new(
            "Test bug".to_string(),
            "Description".to_string(),
            Severity::High,
            BugCategory::WrongOutput,
            "code".to_string(),
            "expected".to_string(),
            "actual".to_string(),
            confidence,
        )
        .with_quantitative_analysis(analysis);

        assert!(report.quantitative_analysis.is_some());
        let analysis = report.quantitative_analysis.unwrap();
        assert_eq!(analysis.complexity.loc, 100);
        assert_eq!(analysis.satd_count, 5);
    }

    #[test]
    fn test_bug_report_with_five_whys() {
        let confidence = ConfidenceScore {
            overall: 0.75,
            discovery_method_weight: 0.8,
            reproducibility_score: 0.7,
            quantitative_evidence: 0.75,
            root_cause_clarity: 0.7,
        };

        let mut five_whys = FiveWhysAnalysis::new("Test bug".to_string());
        five_whys.add_layer(WhyLayer::new(
            1,
            "Why did the bug occur?".to_string(),
        ));

        let report = BugReport::new(
            "Test bug".to_string(),
            "Description".to_string(),
            Severity::Medium,
            BugCategory::TypeError,
            "code".to_string(),
            "expected".to_string(),
            "actual".to_string(),
            confidence,
        )
        .with_five_whys(five_whys);

        assert!(report.five_whys.is_some());
        let analysis = report.five_whys.unwrap();
        assert_eq!(analysis.layers.len(), 1);
    }

    #[test]
    fn test_bug_report_with_tdd_history() {
        let confidence = ConfidenceScore {
            overall: 0.7,
            discovery_method_weight: 0.75,
            reproducibility_score: 0.65,
            quantitative_evidence: 0.7,
            root_cause_clarity: 0.7,
        };

        let mut tdd = TddHistory::new();
        let mut cycle = TddCycle::new(
            1, // cycle
            TddPhase::Red,
            "Write failing test".to_string(),
        );
        cycle.update_tests(1, 0, 1, 0.0);
        cycle.test_result = TestResult::Fail;
        tdd.add_cycle(cycle);

        let report = BugReport::new(
            "Test bug".to_string(),
            "Description".to_string(),
            Severity::Low,
            BugCategory::ParserError,
            "code".to_string(),
            "expected".to_string(),
            "actual".to_string(),
            confidence,
        )
        .with_tdd_history(tdd);

        assert!(report.tdd_history.is_some());
        let history = report.tdd_history.unwrap();
        assert_eq!(history.cycles().len(), 1);
    }

    #[test]
    fn test_add_related_file() {
        let confidence = ConfidenceScore {
            overall: 0.8,
            discovery_method_weight: 0.9,
            reproducibility_score: 0.7,
            quantitative_evidence: 0.8,
            root_cause_clarity: 0.7,
        };

        let mut report = BugReport::new(
            "Test".to_string(),
            "Desc".to_string(),
            Severity::Medium,
            BugCategory::Crash,
            "code".to_string(),
            "exp".to_string(),
            "act".to_string(),
            confidence,
        );

        assert_eq!(report.related_files.len(), 0);
        report.add_related_file("parser.rs".to_string());
        assert_eq!(report.related_files.len(), 1);
        assert_eq!(report.related_files[0], "parser.rs");
    }

    #[test]
    fn test_add_fix_recommendation() {
        let confidence = ConfidenceScore {
            overall: 0.8,
            discovery_method_weight: 0.9,
            reproducibility_score: 0.7,
            quantitative_evidence: 0.8,
            root_cause_clarity: 0.7,
        };

        let mut report = BugReport::new(
            "Test".to_string(),
            "Desc".to_string(),
            Severity::High,
            BugCategory::Hang,
            "code".to_string(),
            "exp".to_string(),
            "act".to_string(),
            confidence,
        );

        assert_eq!(report.fix_recommendations.len(), 0);
        report.add_fix_recommendation("Add bounds checking".to_string());
        assert_eq!(report.fix_recommendations.len(), 1);
    }

    #[test]
    fn test_add_prevention() {
        let confidence = ConfidenceScore {
            overall: 0.8,
            discovery_method_weight: 0.9,
            reproducibility_score: 0.7,
            quantitative_evidence: 0.8,
            root_cause_clarity: 0.7,
        };

        let mut report = BugReport::new(
            "Test".to_string(),
            "Desc".to_string(),
            Severity::Critical,
            BugCategory::MemoryLeak,
            "code".to_string(),
            "exp".to_string(),
            "act".to_string(),
            confidence,
        );

        assert_eq!(report.prevention.len(), 0);
        report.add_prevention("Add property tests for memory safety".to_string());
        assert_eq!(report.prevention.len(), 1);
    }

    #[test]
    fn test_markdown_generation_basic() {
        let confidence = ConfidenceScore {
            overall: 0.85,
            discovery_method_weight: 0.95,
            reproducibility_score: 0.80,
            quantitative_evidence: 0.85,
            root_cause_clarity: 0.75,
        };

        let report = BugReport::new(
            "Parser crash on nested expr".to_string(),
            "Parser fails with stack overflow".to_string(),
            Severity::Critical,
            BugCategory::Crash,
            "fun test() { let x = ((1)); }".to_string(),
            "Should parse".to_string(),
            "Stack overflow".to_string(),
            confidence,
        );

        let markdown = report.to_markdown();

        assert!(markdown.contains("# 🔴 CRITICAL: Parser crash on nested expr"));
        assert!(markdown.contains("**Category**: Crash"));
        assert!(markdown.contains("**Confidence Score**: 0.85 / 1.0"));
        assert!(markdown.contains("```ruchy"));
        assert!(markdown.contains("Should parse"));
        assert!(markdown.contains("Stack overflow"));
        assert!(markdown.contains("### Confidence Factors"));
        assert!(markdown.contains("**Discovery Method**: 0.95"));
        assert!(markdown.contains("**Reproducibility**: 0.80"));
        assert!(markdown.contains("**Quantitative Evidence**: 0.85"));
        assert!(markdown.contains("**Root Cause Clarity**: 0.75"));
    }

    #[test]
    fn test_markdown_generation_full_report() {
        let confidence = ConfidenceScore {
            overall: 0.9,
            discovery_method_weight: 0.95,
            reproducibility_score: 0.90,
            quantitative_evidence: 0.90,
            root_cause_clarity: 0.85,
        };

        let complexity = ComplexityMetrics::new(250);
        let analysis = QuantitativeAnalysis::new(
            complexity,
            None,
            10,  // satd_count
            0.8, // satd_severity
            5,   // coupling
        );

        let mut report = BugReport::new(
            "Parser crash".to_string(),
            "Stack overflow".to_string(),
            Severity::Critical,
            BugCategory::Crash,
            "code".to_string(),
            "parse".to_string(),
            "crash".to_string(),
            confidence,
        )
        .with_quantitative_analysis(analysis);

        report.add_related_file("parser.ruchy".to_string());
        report.add_fix_recommendation("Add depth limit".to_string());
        report.add_prevention("Add property tests for depth limits".to_string());

        let markdown = report.to_markdown();

        assert!(markdown.contains("## Quantitative Analysis"));
        assert!(markdown.contains("**Lines of Code**: 250"));
        assert!(markdown.contains("## Related Files"));
        assert!(markdown.contains("`parser.ruchy`"));
        assert!(markdown.contains("## Fix Recommendations"));
        assert!(markdown.contains("Add depth limit"));
        assert!(markdown.contains("## Prevention Strategy"));
        assert!(markdown.contains("Add property tests"));
        assert!(markdown.contains("*Generated by RuchyRuchy Bug Reporting System*"));
    }

    #[test]
    fn test_markdown_generation_with_all_components() {
        let confidence = ConfidenceScore {
            overall: 0.92,
            discovery_method_weight: 1.0,
            reproducibility_score: 0.90,
            quantitative_evidence: 0.90,
            root_cause_clarity: 0.85,
        };

        let complexity = ComplexityMetrics::new(180);
        let analysis = QuantitativeAnalysis::new(
            complexity,
            None,
            8,  // satd_count
            0.6, // satd_severity
            4,  // coupling
        );

        let mut five_whys = FiveWhysAnalysis::new("Bug occurred".to_string());
        five_whys.add_layer(WhyLayer::new(
            1,
            "Why occurred?".to_string(),
        ));

        let mut tdd = TddHistory::new();
        let mut cycle = TddCycle::new(
            1, // cycle
            TddPhase::Red,
            "Test".to_string(),
        );
        cycle.update_tests(1, 0, 1, 0.0);
        cycle.test_result = TestResult::Fail;
        tdd.add_cycle(cycle);

        let mut report = BugReport::new(
            "Full report test".to_string(),
            "Complete".to_string(),
            Severity::High,
            BugCategory::WrongOutput,
            "code".to_string(),
            "correct".to_string(),
            "wrong".to_string(),
            confidence,
        )
        .with_quantitative_analysis(analysis)
        .with_five_whys(five_whys)
        .with_tdd_history(tdd);

        report.add_related_file("test.ruchy".to_string());
        report.add_fix_recommendation("Fix logic".to_string());
        report.add_prevention("Add tests".to_string());

        let markdown = report.to_markdown();

        // Should contain all sections
        assert!(markdown.contains("## Executive Summary"));
        assert!(markdown.contains("## Bug Details"));
        assert!(markdown.contains("## Confidence Analysis"));
        assert!(markdown.contains("## Quantitative Analysis"));
        assert!(markdown.contains("# Five-Whys Analysis")); // From five_whys.to_markdown()
        assert!(markdown.contains("## TDD Fix Workflow"));
        assert!(markdown.contains("## Related Files"));
        assert!(markdown.contains("## Fix Recommendations"));
        assert!(markdown.contains("## Prevention Strategy"));
    }

    // REPORT-003: Markdown Report Generation - RED PHASE TESTS

    /// Test: Collapsible Sections Support
    ///
    /// RED: This test WILL FAIL because we need to implement:
    /// - Collapsible sections using HTML <details><summary> tags
    /// - Long sections (>50 lines) should be collapsible
    /// - Quantitative Analysis, Five-Whys, TDD Workflow should be collapsible
    ///
    /// This test verifies markdown reports include collapsible sections for readability.
    #[test]
    fn test_report003_collapsible_sections() {
        // RED: This will fail - collapsible sections don't exist yet
        //
        let confidence = ConfidenceScore {
            overall: 0.88,
            discovery_method_weight: 0.95,
            reproducibility_score: 0.85,
            quantitative_evidence: 0.80,
            root_cause_clarity: 0.90,
        };

        let complexity = ComplexityMetrics::new(200);
        let analysis = QuantitativeAnalysis::new(
            complexity,
            None,
            10, // satd_count
            0.7, // satd_severity
            5,  // coupling
        );

        let mut five_whys = FiveWhysAnalysis::new("Performance regression detected".to_string());
        five_whys.add_layer(WhyLayer::new(
            1,
            "Algorithm changed from O(n) to O(n^2)".to_string(),
        ));

        let report = BugReport::new(
            "Performance regression in sort function".to_string(),
            "Sorting 10K items takes 5 seconds instead of 50ms".to_string(),
            Severity::High,
            BugCategory::PerformanceRegression,
            "fun sort(arr: [i32]) { bubble_sort(arr) }".to_string(),
            "Should complete in <100ms".to_string(),
            "Takes 5000ms".to_string(),
            confidence,
        )
        .with_quantitative_analysis(analysis)
        .with_five_whys(five_whys);

        let markdown = report.to_markdown();

        // Verify collapsible sections are present
        assert!(
            markdown.contains("<details>"),
            "Markdown should contain <details> tags for collapsible sections"
        );
        assert!(
            markdown.contains("<summary>"),
            "Markdown should contain <summary> tags for section headers"
        );
        assert!(
            markdown.contains("</details>"),
            "Markdown should properly close <details> tags"
        );

        // Quantitative Analysis should be collapsible
        assert!(
            markdown.contains("<details>") && markdown.contains("## Quantitative Analysis"),
            "Quantitative Analysis should be in a collapsible section"
        );

        // Verify collapsible sections can be expanded/collapsed
        let details_count = markdown.matches("<details>").count();
        let summary_count = markdown.matches("<summary>").count();
        assert_eq!(
            details_count, summary_count,
            "Every <details> should have a matching <summary>"
        );
    }

    /// Test: Executive Summary Content
    ///
    /// RED: This test WILL FAIL because we need to implement:
    /// - Comprehensive executive summary with key metrics
    /// - Severity, category, confidence score upfront
    /// - Impact summary (1-2 sentences)
    /// - Quick action items
    ///
    /// This test verifies the executive summary is complete and actionable.
    #[test]
    fn test_report003_executive_summary_content() {
        // RED: This will fail - executive summary needs enhancement
        //
        let confidence = ConfidenceScore {
            overall: 0.75,
            discovery_method_weight: 0.80,
            reproducibility_score: 0.70,
            quantitative_evidence: 0.75,
            root_cause_clarity: 0.75,
        };

        let report = BugReport::new(
            "Null pointer dereference in parser".to_string(),
            "Parser crashes when encountering empty input".to_string(),
            Severity::Critical,
            BugCategory::Crash,
            "fun parse(code: String) { code[0] }".to_string(),
            "Should handle empty string gracefully".to_string(),
            "Crashes with null pointer".to_string(),
            confidence,
        );

        let markdown = report.to_markdown();

        // Executive summary should contain key information upfront
        let exec_summary_idx = markdown
            .find("## Executive Summary")
            .expect("Should have Executive Summary section");

        // Get executive summary section (from ## Executive Summary to next ##)
        let next_section_idx = markdown[exec_summary_idx + 20..]
            .find("##")
            .map(|idx| exec_summary_idx + 20 + idx)
            .unwrap_or(markdown.len());

        let exec_summary = &markdown[exec_summary_idx..next_section_idx];

        // Should mention severity
        assert!(
            exec_summary.contains("CRITICAL") || exec_summary.contains("Critical"),
            "Executive summary should mention severity"
        );

        // Should mention category
        assert!(
            exec_summary.contains("Crash"),
            "Executive summary should mention bug category"
        );

        // Should mention confidence score
        assert!(
            exec_summary.contains("0.75") || exec_summary.contains("75%"),
            "Executive summary should mention confidence score"
        );

        // Should have impact statement
        assert!(
            exec_summary.contains("Impact:") || exec_summary.contains("impact"),
            "Executive summary should describe impact"
        );

        // Should be concise (not too long)
        assert!(
            exec_summary.lines().count() < 15,
            "Executive summary should be concise (<15 lines)"
        );
    }

    /// Test: Template Completeness
    ///
    /// RED: This test WILL FAIL because we need to verify:
    /// - All required sections are present
    /// - Sections are in correct order
    /// - No duplicate sections
    /// - Report metadata (generation time, tool version)
    ///
    /// This test verifies markdown reports follow a complete template.
    #[test]
    fn test_report003_template_completeness() {
        // RED: This will fail - template completeness validation needed
        //
        let confidence = ConfidenceScore {
            overall: 0.90,
            discovery_method_weight: 0.95,
            reproducibility_score: 0.88,
            quantitative_evidence: 0.90,
            root_cause_clarity: 0.88,
        };

        let complexity = ComplexityMetrics::new(150);
        let analysis = QuantitativeAnalysis::new(complexity, None, 5, 0.5, 3);

        let mut five_whys = FiveWhysAnalysis::new("Bug found".to_string());
        five_whys.add_layer(WhyLayer::new(1, "Why 1".to_string()));

        let mut tdd = TddHistory::new();
        tdd.add_cycle(TddCycle::new(1, TddPhase::Red, "Test".to_string()));

        let mut report = BugReport::new(
            "Type error in function call".to_string(),
            "Calling function with wrong type".to_string(),
            Severity::Medium,
            BugCategory::TypeError,
            "fun add(a: i32, b: i32) { a + b } add(\"hello\", 5)".to_string(),
            "Should reject mismatched types".to_string(),
            "Accepts and crashes".to_string(),
            confidence,
        )
        .with_quantitative_analysis(analysis)
        .with_five_whys(five_whys)
        .with_tdd_history(tdd);

        report.add_related_file("type_checker.ruchy".to_string());
        report.add_fix_recommendation("Add type validation".to_string());
        report.add_prevention("Property tests for type safety".to_string());

        let markdown = report.to_markdown();

        // Required sections in order
        let required_sections = vec![
            "## Executive Summary",
            "## Bug Details",
            "## Reproduction",
            "## Confidence Analysis",
            "## Quantitative Analysis",
            "# Five-Whys Analysis",
            "## TDD Fix Workflow",
            "## Related Files",
            "## Fix Recommendations",
            "## Prevention Strategy",
        ];

        let mut last_pos = 0;
        for section in &required_sections {
            let pos = markdown
                .find(section)
                .unwrap_or_else(|| panic!("Missing required section: {}", section));

            assert!(
                pos > last_pos,
                "Section '{}' is out of order (found at {} but previous was {})",
                section,
                pos,
                last_pos
            );

            last_pos = pos;
        }

        // Should have metadata footer
        assert!(
            markdown.contains("*Generated by RuchyRuchy"),
            "Report should have generation metadata"
        );

        // Should have generation timestamp or version
        assert!(
            markdown.contains("Generated") || markdown.contains("Version"),
            "Report should include generation time or tool version"
        );

        // No duplicate sections
        for section in &required_sections {
            let count = markdown.matches(*section).count();
            assert_eq!(
                count, 1,
                "Section '{}' appears {} times (should be 1)",
                section, count
            );
        }
    }
}
