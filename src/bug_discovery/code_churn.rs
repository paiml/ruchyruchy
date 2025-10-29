// Code Churn Analysis for Bug Prediction
// DISC-004: Code Churn Analysis Implementation
//
// References:
// - Nagappan & Ball (2005): "Use of relative code churn measures to predict system defect density"
// - Hassan (2009): "Predicting faults using the complexity of code changes"
// - Section 6.1 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md

use crate::bug_discovery::confidence::{
    ConfidenceScore, ConfidenceScorer, DiscoveryMethod, EvidenceLevel, Reproducibility,
    RootCauseClarity,
};
use std::collections::HashMap;

/// File change information from version control
#[derive(Debug, Clone, PartialEq)]
pub struct FileChange {
    pub file_path: String,
    pub commit_hash: String,
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub author: String,
    pub timestamp: u64, // Unix timestamp
}

impl FileChange {
    /// Create a new file change
    pub fn new(
        file_path: String,
        commit_hash: String,
        lines_added: usize,
        lines_deleted: usize,
        author: String,
        timestamp: u64,
    ) -> Self {
        FileChange {
            file_path,
            commit_hash,
            lines_added,
            lines_deleted,
            author,
            timestamp,
        }
    }

    /// Total churn for this change
    pub fn total_churn(&self) -> usize {
        self.lines_added + self.lines_deleted
    }
}

/// Code churn metrics for a file
#[derive(Debug, Clone)]
pub struct ChurnMetrics {
    pub file_path: String,
    pub total_changes: usize,
    pub total_lines_added: usize,
    pub total_lines_deleted: usize,
    pub total_churn: usize,
    pub unique_authors: usize,
    pub change_frequency: f64, // Changes per day
    pub churn_rate: f64,       // Lines changed per change
}

impl ChurnMetrics {
    /// Calculate metrics from file changes
    pub fn from_changes(file_path: String, changes: &[FileChange], days: u64) -> Self {
        let total_changes = changes.len();
        let total_lines_added: usize = changes.iter().map(|c| c.lines_added).sum();
        let total_lines_deleted: usize = changes.iter().map(|c| c.lines_deleted).sum();
        let total_churn = total_lines_added + total_lines_deleted;

        let unique_authors: std::collections::HashSet<String> =
            changes.iter().map(|c| c.author.clone()).collect();

        let change_frequency = if days > 0 {
            total_changes as f64 / days as f64
        } else {
            0.0
        };

        let churn_rate = if total_changes > 0 {
            total_churn as f64 / total_changes as f64
        } else {
            0.0
        };

        ChurnMetrics {
            file_path,
            total_changes,
            total_lines_added,
            total_lines_deleted,
            total_churn,
            unique_authors: unique_authors.len(),
            change_frequency,
            churn_rate,
        }
    }

    /// Calculate risk score (0.0-1.0)
    pub fn risk_score(&self) -> f64 {
        // Higher churn = higher risk
        // More authors = higher risk (coordination overhead)
        // More frequent changes = higher risk

        let churn_score = (self.total_churn as f64 / 1000.0).min(1.0);
        let author_score = (self.unique_authors as f64 / 5.0).min(1.0);
        let frequency_score = (self.change_frequency * 10.0).min(1.0);

        // Weighted average
        0.5 * churn_score + 0.3 * author_score + 0.2 * frequency_score
    }
}

/// Code churn hotspot (high-risk file)
#[derive(Debug, Clone)]
pub struct ChurnHotspot {
    pub metrics: ChurnMetrics,
    pub risk_level: RiskLevel,
    pub confidence: ConfidenceScore,
}

/// Risk level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Critical, // Risk score >= 0.8
    High,     // Risk score >= 0.6
    Medium,   // Risk score >= 0.4
    Low,      // Risk score >= 0.2
    Minimal,  // Risk score < 0.2
}

impl RiskLevel {
    /// Classify risk based on score
    pub fn from_score(score: f64) -> Self {
        if score >= 0.8 {
            RiskLevel::Critical
        } else if score >= 0.6 {
            RiskLevel::High
        } else if score >= 0.4 {
            RiskLevel::Medium
        } else if score >= 0.2 {
            RiskLevel::Low
        } else {
            RiskLevel::Minimal
        }
    }
}

impl ChurnHotspot {
    /// Create a new churn hotspot
    pub fn new(metrics: ChurnMetrics) -> Self {
        let risk_score = metrics.risk_score();
        let risk_level = RiskLevel::from_score(risk_score);

        // Confidence scoring based on evidence strength
        let reproducibility = if metrics.total_changes > 10 {
            Reproducibility::Always
        } else if metrics.total_changes > 5 {
            Reproducibility::IntermittentHigh
        } else {
            Reproducibility::IntermittentLow
        };

        let evidence = if metrics.total_changes > 10 && metrics.unique_authors > 3 {
            EvidenceLevel::Complete
        } else if metrics.total_changes > 5 {
            EvidenceLevel::Partial
        } else {
            EvidenceLevel::Limited
        };

        let confidence = ConfidenceScorer::from_components(
            DiscoveryMethod::CodeChurnHotSpot,
            reproducibility,
            evidence,
            RootCauseClarity::UnclearHypothesis,
        );

        ChurnHotspot {
            metrics,
            risk_level,
            confidence,
        }
    }
}

/// Code churn analyzer
pub struct ChurnAnalyzer {
    changes: Vec<FileChange>,
    analysis_window_days: u64,
}

impl ChurnAnalyzer {
    /// Create a new churn analyzer
    pub fn new() -> Self {
        ChurnAnalyzer {
            changes: Vec::new(),
            analysis_window_days: 90, // Default: 90 days
        }
    }

    /// Set analysis window in days
    pub fn with_window_days(mut self, days: u64) -> Self {
        self.analysis_window_days = days;
        self
    }

    /// Add a file change
    pub fn add_change(&mut self, change: FileChange) {
        self.changes.push(change);
    }

    /// Add multiple changes
    pub fn add_changes(&mut self, changes: Vec<FileChange>) {
        self.changes.extend(changes);
    }

    /// Calculate metrics for all files
    pub fn calculate_metrics(&self) -> Vec<ChurnMetrics> {
        let mut file_changes: HashMap<String, Vec<FileChange>> = HashMap::new();

        // Group changes by file
        for change in &self.changes {
            file_changes
                .entry(change.file_path.clone())
                .or_insert_with(Vec::new)
                .push(change.clone());
        }

        // Calculate metrics for each file
        file_changes
            .into_iter()
            .map(|(file_path, changes)| {
                ChurnMetrics::from_changes(file_path, &changes, self.analysis_window_days)
            })
            .collect()
    }

    /// Identify hotspots (high-risk files)
    pub fn identify_hotspots(&self, min_risk: f64) -> Vec<ChurnHotspot> {
        self.calculate_metrics()
            .into_iter()
            .filter(|metrics| metrics.risk_score() >= min_risk)
            .map(ChurnHotspot::new)
            .collect()
    }

    /// Get top N hotspots by risk
    pub fn top_hotspots(&self, n: usize) -> Vec<ChurnHotspot> {
        let mut hotspots: Vec<ChurnHotspot> = self
            .calculate_metrics()
            .into_iter()
            .map(ChurnHotspot::new)
            .collect();

        // Sort by risk score (descending)
        hotspots.sort_by(|a, b| {
            b.metrics
                .risk_score()
                .partial_cmp(&a.metrics.risk_score())
                .unwrap()
        });

        hotspots.into_iter().take(n).collect()
    }
}

impl Default for ChurnAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_change_creation() {
        let change = FileChange::new(
            "src/main.rs".to_string(),
            "abc123".to_string(),
            10,
            5,
            "alice".to_string(),
            1234567890,
        );
        assert_eq!(change.file_path, "src/main.rs");
        assert_eq!(change.total_churn(), 15);
    }

    #[test]
    fn test_churn_metrics_calculation() {
        let changes = vec![
            FileChange::new(
                "src/main.rs".to_string(),
                "abc123".to_string(),
                10,
                5,
                "alice".to_string(),
                1000,
            ),
            FileChange::new(
                "src/main.rs".to_string(),
                "def456".to_string(),
                20,
                10,
                "bob".to_string(),
                2000,
            ),
        ];

        let metrics = ChurnMetrics::from_changes("src/main.rs".to_string(), &changes, 30);

        assert_eq!(metrics.total_changes, 2);
        assert_eq!(metrics.total_lines_added, 30);
        assert_eq!(metrics.total_lines_deleted, 15);
        assert_eq!(metrics.total_churn, 45);
        assert_eq!(metrics.unique_authors, 2);
    }

    #[test]
    fn test_risk_score_calculation() {
        let changes = vec![
            FileChange::new(
                "risky.rs".to_string(),
                "abc123".to_string(),
                100,
                50,
                "alice".to_string(),
                1000,
            ),
            FileChange::new(
                "risky.rs".to_string(),
                "def456".to_string(),
                200,
                100,
                "bob".to_string(),
                2000,
            ),
            FileChange::new(
                "risky.rs".to_string(),
                "ghi789".to_string(),
                150,
                75,
                "charlie".to_string(),
                3000,
            ),
        ];

        let metrics = ChurnMetrics::from_changes("risky.rs".to_string(), &changes, 10);
        let risk_score = metrics.risk_score();

        // High churn + multiple authors + frequent changes = high risk
        assert!(risk_score > 0.3);
    }

    #[test]
    fn test_risk_level_classification() {
        assert_eq!(RiskLevel::from_score(0.9), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(0.7), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(0.5), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(0.3), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(0.1), RiskLevel::Minimal);
    }

    #[test]
    fn test_hotspot_creation() {
        let changes = vec![FileChange::new(
            "hotspot.rs".to_string(),
            "abc123".to_string(),
            100,
            50,
            "alice".to_string(),
            1000,
        )];

        let metrics = ChurnMetrics::from_changes("hotspot.rs".to_string(), &changes, 30);
        let hotspot = ChurnHotspot::new(metrics);

        // Should have valid confidence score
        assert!(hotspot.confidence.overall > 0.0);
        assert!(hotspot.confidence.overall <= 1.0);
    }

    #[test]
    fn test_churn_analyzer_creation() {
        let analyzer = ChurnAnalyzer::new();
        assert_eq!(analyzer.analysis_window_days, 90);
        assert_eq!(analyzer.changes.len(), 0);
    }

    #[test]
    fn test_churn_analyzer_add_change() {
        let mut analyzer = ChurnAnalyzer::new();
        let change = FileChange::new(
            "src/main.rs".to_string(),
            "abc123".to_string(),
            10,
            5,
            "alice".to_string(),
            1000,
        );

        analyzer.add_change(change);
        assert_eq!(analyzer.changes.len(), 1);
    }

    #[test]
    fn test_churn_analyzer_calculate_metrics() {
        let mut analyzer = ChurnAnalyzer::new();
        analyzer.add_change(FileChange::new(
            "src/a.rs".to_string(),
            "abc123".to_string(),
            10,
            5,
            "alice".to_string(),
            1000,
        ));
        analyzer.add_change(FileChange::new(
            "src/b.rs".to_string(),
            "def456".to_string(),
            20,
            10,
            "bob".to_string(),
            2000,
        ));

        let metrics = analyzer.calculate_metrics();
        assert_eq!(metrics.len(), 2);
    }

    #[test]
    fn test_churn_analyzer_identify_hotspots() {
        let mut analyzer = ChurnAnalyzer::new().with_window_days(10);

        // Add high-churn file
        for i in 0..5 {
            analyzer.add_change(FileChange::new(
                "risky.rs".to_string(),
                format!("commit{}", i),
                100,
                50,
                format!("author{}", i),
                i as u64 * 1000,
            ));
        }

        // Add low-churn file
        analyzer.add_change(FileChange::new(
            "safe.rs".to_string(),
            "abc123".to_string(),
            5,
            2,
            "alice".to_string(),
            1000,
        ));

        let hotspots = analyzer.identify_hotspots(0.3);
        // Should identify risky.rs as hotspot
        assert!(hotspots.len() >= 1);
        assert!(hotspots.iter().any(|h| h.metrics.file_path == "risky.rs"));
    }

    #[test]
    fn test_churn_analyzer_top_hotspots() {
        let mut analyzer = ChurnAnalyzer::new();

        // Add multiple files with different risk levels
        for i in 0..3 {
            analyzer.add_change(FileChange::new(
                "high_risk.rs".to_string(),
                format!("h{}", i),
                100,
                50,
                format!("author{}", i),
                i as u64 * 1000,
            ));
        }

        for i in 0..2 {
            analyzer.add_change(FileChange::new(
                "medium_risk.rs".to_string(),
                format!("m{}", i),
                50,
                25,
                format!("author{}", i),
                i as u64 * 1000,
            ));
        }

        analyzer.add_change(FileChange::new(
            "low_risk.rs".to_string(),
            "l1".to_string(),
            10,
            5,
            "alice".to_string(),
            1000,
        ));

        let top2 = analyzer.top_hotspots(2);
        assert_eq!(top2.len(), 2);
        // First should be highest risk
        assert!(top2[0].metrics.risk_score() >= top2[1].metrics.risk_score());
    }
}
