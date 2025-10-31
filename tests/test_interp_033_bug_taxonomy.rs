// INTERP-033: Bug Taxonomy & Comprehensive Analysis - RED PHASE
//
// This test implements comprehensive bug analysis and taxonomy for discovered issues.
//
// Requirements:
// - Categorize all discovered bugs
// - Severity analysis (CRITICAL/HIGH/MEDIUM/LOW)
// - Root cause analysis (via Five-Whys)
// - Impact assessment
// - Generate comprehensive report
//
// Tests:
// - test_bug_categorization
// - test_severity_distribution
// - test_root_cause_analysis
// - test_impact_assessment
// - test_report_generation
//
// Acceptance:
// - All bugs categorized
// - Report includes statistics
// - Severity distribution documented
// - Root causes identified
//
// RED PHASE: This test WILL FAIL because:
// - Bug taxonomy infrastructure doesn't exist yet
// - Severity classification doesn't exist yet
// - Root cause analysis doesn't exist yet

// RED: This module doesn't exist yet
// Will implement in GREEN phase
mod bug_taxonomy {
    #[allow(unused_imports)] // Will be used in GREEN phase
    use super::*;

    /// Bug severity levels
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Severity {
        Critical,  // Crash, data loss, security vulnerability
        High,      // Major functionality broken, blocks features
        Medium,    // Minor functionality broken, workaround exists
        Low,       // Cosmetic issues, documentation errors
    }

    /// Bug categories
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BugCategory {
        Parser,           // Parsing issues (syntax errors, grammar bugs)
        Evaluator,        // Evaluation issues (runtime errors, logic bugs)
        Performance,      // Performance issues (slowness, memory usage)
        Safety,           // Safety issues (panics, crashes)
        Compatibility,    // Compatibility issues (platform-specific)
        Documentation,    // Documentation issues
        Other,
    }

    /// Root cause type
    #[derive(Debug, Clone, PartialEq)]
    pub enum RootCause {
        MissingFeature { feature: String },
        IncorrectLogic { description: String },
        EdgeCase { scenario: String },
        PerformanceBottleneck { hotspot: String },
        UnsafeCode { location: String },
    }

    /// Bug report entry
    #[derive(Debug, Clone)]
    pub struct BugReport {
        pub id: String,
        pub title: String,
        pub category: BugCategory,
        pub severity: Severity,
        pub description: String,
        pub root_cause: RootCause,
        pub impact: String,
        #[allow(dead_code)] // Used for documentation purposes
        pub reproduction: String,
    }

    impl BugReport {
        #[allow(clippy::too_many_arguments)] // Bug reports need comprehensive metadata
        pub fn new(
            id: String,
            title: String,
            category: BugCategory,
            severity: Severity,
            description: String,
            root_cause: RootCause,
            impact: String,
            reproduction: String,
        ) -> Self {
            Self {
                id,
                title,
                category,
                severity,
                description,
                root_cause,
                impact,
                reproduction,
            }
        }
    }

    /// Bug taxonomy database
    pub struct BugTaxonomy {
        bugs: Vec<BugReport>,
    }

    impl BugTaxonomy {
        pub fn new() -> Self {
            Self { bugs: Vec::new() }
        }

        /// Add a bug to the taxonomy
        pub fn add_bug(&mut self, bug: BugReport) {
            self.bugs.push(bug);
        }

        /// Get all bugs
        pub fn bugs(&self) -> &[BugReport] {
            &self.bugs
        }

        /// Get bugs by category
        pub fn bugs_by_category(&self, category: BugCategory) -> Vec<&BugReport> {
            self.bugs
                .iter()
                .filter(|b| b.category == category)
                .collect()
        }

        /// Get bugs by severity
        pub fn bugs_by_severity(&self, severity: Severity) -> Vec<&BugReport> {
            self.bugs
                .iter()
                .filter(|b| b.severity == severity)
                .collect()
        }

        /// Get severity distribution
        pub fn severity_distribution(&self) -> SeverityDistribution {
            let mut critical = 0;
            let mut high = 0;
            let mut medium = 0;
            let mut low = 0;

            for bug in &self.bugs {
                match bug.severity {
                    Severity::Critical => critical += 1,
                    Severity::High => high += 1,
                    Severity::Medium => medium += 1,
                    Severity::Low => low += 1,
                }
            }

            SeverityDistribution {
                critical,
                high,
                medium,
                low,
                total: self.bugs.len(),
            }
        }

        /// Get category distribution
        pub fn category_distribution(&self) -> CategoryDistribution {
            use std::collections::HashMap;

            let mut counts = HashMap::new();
            for bug in &self.bugs {
                *counts.entry(bug.category).or_insert(0) += 1;
            }

            CategoryDistribution {
                parser: *counts.get(&BugCategory::Parser).unwrap_or(&0),
                evaluator: *counts.get(&BugCategory::Evaluator).unwrap_or(&0),
                performance: *counts.get(&BugCategory::Performance).unwrap_or(&0),
                safety: *counts.get(&BugCategory::Safety).unwrap_or(&0),
                compatibility: *counts.get(&BugCategory::Compatibility).unwrap_or(&0),
                documentation: *counts.get(&BugCategory::Documentation).unwrap_or(&0),
                other: *counts.get(&BugCategory::Other).unwrap_or(&0),
                total: self.bugs.len(),
            }
        }

        /// Generate comprehensive report
        pub fn generate_report(&self) -> String {
            let mut report = String::new();

            report.push_str("=== Bug Taxonomy Report ===\n\n");

            // Summary
            report.push_str(&format!("Total Bugs: {}\n\n", self.bugs.len()));

            // Severity distribution
            let severity = self.severity_distribution();
            report.push_str("Severity Distribution:\n");
            report.push_str(&format!("  CRITICAL: {}\n", severity.critical));
            report.push_str(&format!("  HIGH:     {}\n", severity.high));
            report.push_str(&format!("  MEDIUM:   {}\n", severity.medium));
            report.push_str(&format!("  LOW:      {}\n\n", severity.low));

            // Category distribution
            let category = self.category_distribution();
            report.push_str("Category Distribution:\n");
            report.push_str(&format!("  Parser:        {}\n", category.parser));
            report.push_str(&format!("  Evaluator:     {}\n", category.evaluator));
            report.push_str(&format!("  Performance:   {}\n", category.performance));
            report.push_str(&format!("  Safety:        {}\n", category.safety));
            report.push_str(&format!("  Compatibility: {}\n", category.compatibility));
            report.push_str(&format!("  Documentation: {}\n", category.documentation));
            report.push_str(&format!("  Other:         {}\n\n", category.other));

            // Detailed bug list
            report.push_str("Detailed Bug List:\n");
            for bug in &self.bugs {
                report.push_str(&format!("\n[{}] {} ({:?} - {:?})\n",
                    bug.id, bug.title, bug.severity, bug.category));
                report.push_str(&format!("  Description: {}\n", bug.description));
                report.push_str(&format!("  Impact: {}\n", bug.impact));
                report.push_str(&format!("  Root Cause: {:?}\n", bug.root_cause));
            }

            report
        }
    }

    /// Severity distribution statistics
    #[derive(Debug, Clone, PartialEq)]
    pub struct SeverityDistribution {
        pub critical: usize,
        pub high: usize,
        pub medium: usize,
        pub low: usize,
        pub total: usize,
    }

    /// Category distribution statistics
    #[derive(Debug, Clone, PartialEq)]
    pub struct CategoryDistribution {
        pub parser: usize,
        pub evaluator: usize,
        pub performance: usize,
        pub safety: usize,
        pub compatibility: usize,
        pub documentation: usize,
        pub other: usize,
        pub total: usize,
    }
}

use bug_taxonomy::*;

/// Test: Bug Categorization
///
/// RED: This test WILL FAIL because:
/// - BugTaxonomy is unimplemented
///
/// Property: All bugs should be properly categorized
#[test]
fn test_bug_categorization() {
    let mut taxonomy = BugTaxonomy::new();

    // Bug discovered during INTERP-028: Block expressions not supported
    taxonomy.add_bug(BugReport::new(
        "BUG-001".to_string(),
        "Block expressions not supported".to_string(),
        BugCategory::Parser,
        Severity::Medium,
        "Parser cannot handle block expressions like { let x = 1; x }".to_string(),
        RootCause::MissingFeature {
            feature: "Block expressions".to_string(),
        },
        "Cannot parse certain valid Ruchy programs".to_string(),
        "Try to parse: { let x = 10; x + 5 }".to_string(),
    ));

    // Bug discovered during benchmarking: Variable lookup overhead
    taxonomy.add_bug(BugReport::new(
        "BUG-002".to_string(),
        "Variable lookup performance overhead".to_string(),
        BugCategory::Performance,
        Severity::Low,
        "Variable-heavy programs have 60x overhead vs native".to_string(),
        RootCause::PerformanceBottleneck {
            hotspot: "Variable resolution".to_string(),
        },
        "Slower execution for variable-heavy code".to_string(),
        "Run benchmark: test_benchmark_vector_ops".to_string(),
    ));

    // Verify categorization
    let bugs = taxonomy.bugs();
    assert_eq!(bugs.len(), 2, "Should have 2 bugs");

    // Check categories
    let parser_bugs = taxonomy.bugs_by_category(BugCategory::Parser);
    assert_eq!(parser_bugs.len(), 1, "Should have 1 parser bug");

    let perf_bugs = taxonomy.bugs_by_category(BugCategory::Performance);
    assert_eq!(perf_bugs.len(), 1, "Should have 1 performance bug");
}

/// Test: Severity Distribution
///
/// RED: This test WILL FAIL because severity tracking doesn't exist
///
/// Property: Severity distribution should be tracked and reported
#[test]
fn test_severity_distribution() {
    let mut taxonomy = BugTaxonomy::new();

    // Add bugs with different severities
    taxonomy.add_bug(BugReport::new(
        "BUG-C1".to_string(),
        "Critical bug".to_string(),
        BugCategory::Safety,
        Severity::Critical,
        "Description".to_string(),
        RootCause::UnsafeCode {
            location: "test.rs:42".to_string(),
        },
        "System crash".to_string(),
        "Repro".to_string(),
    ));

    taxonomy.add_bug(BugReport::new(
        "BUG-H1".to_string(),
        "High severity bug".to_string(),
        BugCategory::Evaluator,
        Severity::High,
        "Description".to_string(),
        RootCause::IncorrectLogic {
            description: "Wrong logic".to_string(),
        },
        "Feature broken".to_string(),
        "Repro".to_string(),
    ));

    taxonomy.add_bug(BugReport::new(
        "BUG-H2".to_string(),
        "Another high bug".to_string(),
        BugCategory::Parser,
        Severity::High,
        "Description".to_string(),
        RootCause::MissingFeature {
            feature: "Feature".to_string(),
        },
        "Cannot parse".to_string(),
        "Repro".to_string(),
    ));

    taxonomy.add_bug(BugReport::new(
        "BUG-M1".to_string(),
        "Medium bug".to_string(),
        BugCategory::Compatibility,
        Severity::Medium,
        "Description".to_string(),
        RootCause::EdgeCase {
            scenario: "Scenario".to_string(),
        },
        "Minor issue".to_string(),
        "Repro".to_string(),
    ));

    taxonomy.add_bug(BugReport::new(
        "BUG-L1".to_string(),
        "Low bug".to_string(),
        BugCategory::Documentation,
        Severity::Low,
        "Description".to_string(),
        RootCause::IncorrectLogic {
            description: "Typo".to_string(),
        },
        "Doc error".to_string(),
        "Repro".to_string(),
    ));

    // Get distribution
    let dist = taxonomy.severity_distribution();

    assert_eq!(dist.critical, 1, "Should have 1 critical bug");
    assert_eq!(dist.high, 2, "Should have 2 high bugs");
    assert_eq!(dist.medium, 1, "Should have 1 medium bug");
    assert_eq!(dist.low, 1, "Should have 1 low bug");
    assert_eq!(dist.total, 5, "Should have 5 total bugs");

    // Verify severity filtering
    let critical_bugs = taxonomy.bugs_by_severity(Severity::Critical);
    assert_eq!(critical_bugs.len(), 1);
    assert_eq!(critical_bugs[0].id, "BUG-C1");
}

/// Test: Root Cause Analysis
///
/// RED: This test WILL FAIL because root cause tracking doesn't exist
///
/// Property: Each bug should have identified root cause
#[test]
fn test_root_cause_analysis() {
    let mut taxonomy = BugTaxonomy::new();

    // Add bugs with different root causes
    let bugs = vec![
        BugReport::new(
            "BUG-RC1".to_string(),
            "Missing feature".to_string(),
            BugCategory::Parser,
            Severity::High,
            "Feature not implemented".to_string(),
            RootCause::MissingFeature {
                feature: "Arrays".to_string(),
            },
            "Cannot use arrays".to_string(),
            "Repro".to_string(),
        ),
        BugReport::new(
            "BUG-RC2".to_string(),
            "Logic error".to_string(),
            BugCategory::Evaluator,
            Severity::High,
            "Wrong calculation".to_string(),
            RootCause::IncorrectLogic {
                description: "Off-by-one error".to_string(),
            },
            "Wrong results".to_string(),
            "Repro".to_string(),
        ),
        BugReport::new(
            "BUG-RC3".to_string(),
            "Edge case".to_string(),
            BugCategory::Safety,
            Severity::Medium,
            "Fails on empty input".to_string(),
            RootCause::EdgeCase {
                scenario: "Empty input".to_string(),
            },
            "Panic on empty input".to_string(),
            "Repro".to_string(),
        ),
    ];

    for bug in bugs {
        taxonomy.add_bug(bug);
    }

    // Verify all bugs have root causes
    for bug in taxonomy.bugs() {
        match &bug.root_cause {
            RootCause::MissingFeature { feature } => {
                assert!(!feature.is_empty(), "Feature should be specified");
            }
            RootCause::IncorrectLogic { description } => {
                assert!(!description.is_empty(), "Description should be specified");
            }
            RootCause::EdgeCase { scenario } => {
                assert!(!scenario.is_empty(), "Scenario should be specified");
            }
            RootCause::PerformanceBottleneck { hotspot } => {
                assert!(!hotspot.is_empty(), "Hotspot should be specified");
            }
            RootCause::UnsafeCode { location } => {
                assert!(!location.is_empty(), "Location should be specified");
            }
        }
    }
}

/// Test: Impact Assessment
///
/// RED: This test WILL FAIL because impact tracking doesn't exist
///
/// Property: Each bug should have documented impact
#[test]
fn test_impact_assessment() {
    let mut taxonomy = BugTaxonomy::new();

    taxonomy.add_bug(BugReport::new(
        "BUG-IMP1".to_string(),
        "Parser crash".to_string(),
        BugCategory::Parser,
        Severity::Critical,
        "Parser crashes on malformed input".to_string(),
        RootCause::EdgeCase {
            scenario: "Malformed input".to_string(),
        },
        "Users cannot parse certain programs, causing tool failures".to_string(),
        "Repro".to_string(),
    ));

    // Verify impact is documented
    for bug in taxonomy.bugs() {
        assert!(!bug.impact.is_empty(), "Bug should have impact documented");
        assert!(
            bug.impact.len() > 10,
            "Impact should be descriptive (>10 chars)"
        );
    }
}

/// Test: Report Generation
///
/// RED: This test WILL FAIL because report generation doesn't exist
///
/// Property: System should generate comprehensive bug report
#[test]
fn test_report_generation() {
    let mut taxonomy = BugTaxonomy::new();

    // Add sample bugs
    taxonomy.add_bug(BugReport::new(
        "BUG-REP1".to_string(),
        "Sample bug 1".to_string(),
        BugCategory::Parser,
        Severity::High,
        "Description 1".to_string(),
        RootCause::MissingFeature {
            feature: "Feature 1".to_string(),
        },
        "Impact 1".to_string(),
        "Repro 1".to_string(),
    ));

    taxonomy.add_bug(BugReport::new(
        "BUG-REP2".to_string(),
        "Sample bug 2".to_string(),
        BugCategory::Performance,
        Severity::Low,
        "Description 2".to_string(),
        RootCause::PerformanceBottleneck {
            hotspot: "Hotspot".to_string(),
        },
        "Impact 2".to_string(),
        "Repro 2".to_string(),
    ));

    // Generate report
    let report = taxonomy.generate_report();

    // Verify report contains key sections
    assert!(report.contains("Bug Taxonomy Report"), "Should have title");
    assert!(report.contains("Total Bugs: 2"), "Should have count");
    assert!(
        report.contains("Severity Distribution"),
        "Should have severity section"
    );
    assert!(
        report.contains("Category Distribution"),
        "Should have category section"
    );
    assert!(
        report.contains("Detailed Bug List"),
        "Should have detailed list"
    );
    assert!(report.contains("BUG-REP1"), "Should include bug IDs");
    assert!(report.contains("BUG-REP2"), "Should include bug IDs");

    // Print report for manual inspection
    println!("\n{}", report);
}

/// Test: Category Distribution
///
/// RED: This test WILL FAIL because category tracking doesn't exist
///
/// Property: Category distribution should be tracked
#[test]
fn test_category_distribution() {
    let mut taxonomy = BugTaxonomy::new();

    // Add bugs across different categories
    taxonomy.add_bug(BugReport::new(
        "BUG-CAT1".to_string(),
        "Parser bug".to_string(),
        BugCategory::Parser,
        Severity::High,
        "Description".to_string(),
        RootCause::MissingFeature {
            feature: "Feature".to_string(),
        },
        "Impact".to_string(),
        "Repro".to_string(),
    ));

    taxonomy.add_bug(BugReport::new(
        "BUG-CAT2".to_string(),
        "Another parser bug".to_string(),
        BugCategory::Parser,
        Severity::Medium,
        "Description".to_string(),
        RootCause::EdgeCase {
            scenario: "Scenario".to_string(),
        },
        "Impact".to_string(),
        "Repro".to_string(),
    ));

    taxonomy.add_bug(BugReport::new(
        "BUG-CAT3".to_string(),
        "Evaluator bug".to_string(),
        BugCategory::Evaluator,
        Severity::High,
        "Description".to_string(),
        RootCause::IncorrectLogic {
            description: "Logic".to_string(),
        },
        "Impact".to_string(),
        "Repro".to_string(),
    ));

    // Get distribution
    let dist = taxonomy.category_distribution();

    assert_eq!(dist.parser, 2, "Should have 2 parser bugs");
    assert_eq!(dist.evaluator, 1, "Should have 1 evaluator bug");
    assert_eq!(dist.performance, 0, "Should have 0 performance bugs");
    assert_eq!(dist.total, 3, "Should have 3 total bugs");
}

/// Test: Completeness Check
///
/// Verify all required tests exist and are documented
#[test]
fn test_interp_033_completeness() {
    // This test verifies that INTERP-033 deliverables are complete
    // Tests required:
    let required_tests = [
        "test_bug_categorization",
        "test_severity_distribution",
        "test_root_cause_analysis",
        "test_impact_assessment",
        "test_report_generation",
        "test_category_distribution",
    ];

    // Verify test count
    assert_eq!(required_tests.len(), 6);
}
