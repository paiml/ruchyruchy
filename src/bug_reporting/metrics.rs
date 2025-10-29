// Quantitative Analysis Engine
// REPORT-001: Quantitative Analysis Engine Implementation
//
// References:
// - Chidamber & Kemerer (1994): Object-Oriented Metrics
// - Campbell (2018): Cognitive Complexity
// - Potdar & Shihab (2014): SATD Detection
// - Maldonado & Shihab (2015): Defect Prediction
// - Section 8.1 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md

use std::collections::HashMap;

/// Complexity metrics for a code unit
#[derive(Debug, Clone)]
pub struct ComplexityMetrics {
    /// Lines of code
    pub loc: usize,
    /// Cyclomatic complexity (McCabe)
    pub cyclomatic: u32,
    /// Cognitive complexity (SonarSource)
    pub cognitive: u32,
    /// Halstead difficulty
    pub halstead_difficulty: f64,
    /// Number of parameters
    pub parameters: usize,
    /// Nesting depth
    pub nesting_depth: u32,
}

impl ComplexityMetrics {
    /// Create new complexity metrics
    pub fn new(loc: usize) -> Self {
        ComplexityMetrics {
            loc,
            cyclomatic: 1, // Base complexity
            cognitive: 0,
            halstead_difficulty: 0.0,
            parameters: 0,
            nesting_depth: 0,
        }
    }

    /// Calculate complexity score (0.0-1.0, higher = more complex)
    pub fn complexity_score(&self) -> f64 {
        // Weighted combination of metrics
        // Thresholds based on industry standards:
        // - Cyclomatic > 10 = high complexity
        // - Cognitive > 15 = high complexity
        // - Halstead difficulty > 20 = high complexity
        // - Parameters > 5 = high complexity
        // - Nesting > 4 = high complexity

        let cyclomatic_score = (self.cyclomatic as f64 / 20.0).min(1.0);
        let cognitive_score = (self.cognitive as f64 / 30.0).min(1.0);
        let halstead_score = (self.halstead_difficulty / 40.0).min(1.0);
        let parameter_score = (self.parameters as f64 / 10.0).min(1.0);
        let nesting_score = (self.nesting_depth as f64 / 8.0).min(1.0);

        // Weighted average (cognitive complexity weighted highest)
        0.30 * cognitive_score
            + 0.25 * cyclomatic_score
            + 0.20 * halstead_score
            + 0.15 * parameter_score
            + 0.10 * nesting_score
    }

    /// Check if complexity exceeds thresholds
    pub fn is_complex(&self) -> bool {
        self.cyclomatic > 10
            || self.cognitive > 15
            || self.halstead_difficulty > 20.0
            || self.parameters > 5
            || self.nesting_depth > 4
    }
}

/// Code churn correlation with bugs
#[derive(Debug, Clone)]
pub struct ChurnCorrelation {
    /// File path
    pub file: String,
    /// Number of changes (commits)
    pub changes: usize,
    /// Number of bugs found
    pub bugs: usize,
    /// Correlation coefficient (-1.0 to 1.0)
    pub correlation: f64,
}

impl ChurnCorrelation {
    /// Create new churn correlation
    pub fn new(file: String, changes: usize, bugs: usize) -> Self {
        // Simple correlation: bugs per change
        let correlation = if changes > 0 {
            bugs as f64 / changes as f64
        } else {
            0.0
        };

        ChurnCorrelation {
            file,
            changes,
            bugs,
            correlation: correlation.min(1.0),
        }
    }

    /// Check if file is high-risk based on churn
    pub fn is_high_risk(&self) -> bool {
        // High risk if:
        // - Many changes (>20) AND high bug rate (>0.3 bugs/change)
        // OR very high bug rate (>0.5)
        (self.changes > 20 && self.correlation > 0.3) || self.correlation > 0.5
    }
}

/// Self-Admitted Technical Debt (SATD) type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatdType {
    /// TODO comment
    Todo,
    /// FIXME comment
    Fixme,
    /// HACK comment
    Hack,
    /// XXX comment
    Xxx,
    /// DEBT comment
    Debt,
}

impl SatdType {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            SatdType::Todo => "TODO",
            SatdType::Fixme => "FIXME",
            SatdType::Hack => "HACK",
            SatdType::Xxx => "XXX",
            SatdType::Debt => "DEBT",
        }
    }

    /// Get severity (0-10, higher = more severe)
    pub fn severity(&self) -> u8 {
        match self {
            SatdType::Todo => 3,
            SatdType::Fixme => 7,
            SatdType::Hack => 9,
            SatdType::Xxx => 8,
            SatdType::Debt => 6,
        }
    }
}

/// SATD detector
pub struct SatdDetector {
    /// Detected SATD instances
    satd_instances: Vec<(String, SatdType, String)>, // (file, type, comment)
}

impl SatdDetector {
    /// Create new SATD detector
    pub fn new() -> Self {
        SatdDetector {
            satd_instances: Vec::new(),
        }
    }

    /// Detect SATD in source code
    pub fn detect(&mut self, file: String, source: &str) {
        for (line_num, line) in source.lines().enumerate() {
            // Check for SATD markers
            if let Some(satd_type) = self.detect_satd_type(line) {
                let comment = self.extract_comment(line);
                let location = format!("{}:{}", file, line_num + 1);
                self.satd_instances
                    .push((location, satd_type, comment));
            }
        }
    }

    /// Detect SATD type from line
    fn detect_satd_type(&self, line: &str) -> Option<SatdType> {
        let line_upper = line.to_uppercase();

        if line_upper.contains("TODO") || line_upper.contains("TO DO") {
            Some(SatdType::Todo)
        } else if line_upper.contains("FIXME") || line_upper.contains("FIX ME") {
            Some(SatdType::Fixme)
        } else if line_upper.contains("HACK") {
            Some(SatdType::Hack)
        } else if line_upper.contains("XXX") {
            Some(SatdType::Xxx)
        } else if line_upper.contains("DEBT") || line_upper.contains("TECHNICAL DEBT") {
            Some(SatdType::Debt)
        } else {
            None
        }
    }

    /// Extract comment text
    fn extract_comment(&self, line: &str) -> String {
        // Find comment start
        if let Some(idx) = line.find("//") {
            line[idx + 2..].trim().to_string()
        } else if let Some(idx) = line.find("/*") {
            line[idx + 2..].trim().to_string()
        } else {
            line.trim().to_string()
        }
    }

    /// Get all SATD instances
    pub fn instances(&self) -> &[(String, SatdType, String)] {
        &self.satd_instances
    }

    /// Get SATD count by type
    pub fn count_by_type(&self, satd_type: SatdType) -> usize {
        self.satd_instances
            .iter()
            .filter(|(_, t, _)| *t == satd_type)
            .count()
    }

    /// Get total SATD count
    pub fn total_count(&self) -> usize {
        self.satd_instances.len()
    }

    /// Calculate SATD severity score (0.0-1.0)
    pub fn severity_score(&self) -> f64 {
        if self.satd_instances.is_empty() {
            return 0.0;
        }

        let total_severity: u32 = self
            .satd_instances
            .iter()
            .map(|(_, t, _)| t.severity() as u32)
            .sum();

        let max_severity = self.satd_instances.len() * 10;
        total_severity as f64 / max_severity as f64
    }
}

impl Default for SatdDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Dependency node in call graph
#[derive(Debug, Clone)]
pub struct DependencyNode {
    /// Function name
    pub name: String,
    /// Functions this depends on (calls)
    pub dependencies: Vec<String>,
    /// Functions that depend on this (callers)
    pub dependents: Vec<String>,
}

impl DependencyNode {
    /// Create new dependency node
    pub fn new(name: String) -> Self {
        DependencyNode {
            name,
            dependencies: Vec::new(),
            dependents: Vec::new(),
        }
    }

    /// Add a dependency
    pub fn add_dependency(&mut self, dependency: String) {
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }

    /// Add a dependent
    pub fn add_dependent(&mut self, dependent: String) {
        if !self.dependents.contains(&dependent) {
            self.dependents.push(dependent);
        }
    }

    /// Calculate fan-in (number of callers)
    pub fn fan_in(&self) -> usize {
        self.dependents.len()
    }

    /// Calculate fan-out (number of callees)
    pub fn fan_out(&self) -> usize {
        self.dependencies.len()
    }

    /// Calculate instability (fan-out / (fan-in + fan-out))
    pub fn instability(&self) -> f64 {
        let total = self.fan_in() + self.fan_out();
        if total == 0 {
            0.0
        } else {
            self.fan_out() as f64 / total as f64
        }
    }
}

/// Dependency analyzer
pub struct DependencyAnalyzer {
    /// Dependency graph
    nodes: HashMap<String, DependencyNode>,
}

impl DependencyAnalyzer {
    /// Create new dependency analyzer
    pub fn new() -> Self {
        DependencyAnalyzer {
            nodes: HashMap::new(),
        }
    }

    /// Add a node
    pub fn add_node(&mut self, name: String) {
        self.nodes
            .entry(name.clone())
            .or_insert_with(|| DependencyNode::new(name));
    }

    /// Add a dependency (caller -> callee)
    pub fn add_dependency(&mut self, caller: String, callee: String) {
        // Ensure both nodes exist
        self.add_node(caller.clone());
        self.add_node(callee.clone());

        // Add dependency
        if let Some(node) = self.nodes.get_mut(&caller) {
            node.add_dependency(callee.clone());
        }

        // Add dependent
        if let Some(node) = self.nodes.get_mut(&callee) {
            node.add_dependent(caller);
        }
    }

    /// Get a node
    pub fn get_node(&self, name: &str) -> Option<&DependencyNode> {
        self.nodes.get(name)
    }

    /// Get all nodes
    pub fn nodes(&self) -> Vec<&DependencyNode> {
        self.nodes.values().collect()
    }

    /// Find highly coupled nodes (fan-in + fan-out > threshold)
    pub fn find_highly_coupled(&self, threshold: usize) -> Vec<&DependencyNode> {
        self.nodes
            .values()
            .filter(|node| node.fan_in() + node.fan_out() > threshold)
            .collect()
    }

    /// Find unstable nodes (instability > threshold)
    pub fn find_unstable(&self, threshold: f64) -> Vec<&DependencyNode> {
        self.nodes
            .values()
            .filter(|node| node.instability() > threshold)
            .collect()
    }
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Quantitative analysis result
#[derive(Debug, Clone)]
pub struct QuantitativeAnalysis {
    /// Complexity metrics
    pub complexity: ComplexityMetrics,
    /// Churn correlation
    pub churn: Option<ChurnCorrelation>,
    /// SATD count
    pub satd_count: usize,
    /// SATD severity score
    pub satd_severity: f64,
    /// Dependency coupling
    pub coupling: usize,
    /// Overall risk score (0.0-1.0)
    pub risk_score: f64,
}

impl QuantitativeAnalysis {
    /// Create new quantitative analysis
    pub fn new(
        complexity: ComplexityMetrics,
        churn: Option<ChurnCorrelation>,
        satd_count: usize,
        satd_severity: f64,
        coupling: usize,
    ) -> Self {
        // Calculate overall risk score
        let complexity_risk = complexity.complexity_score();
        let churn_risk = churn
            .as_ref()
            .map(|c| c.correlation)
            .unwrap_or(0.0);
        let satd_risk = satd_severity;
        let coupling_risk = (coupling as f64 / 20.0).min(1.0);

        let risk_score = 0.35 * complexity_risk
            + 0.30 * churn_risk
            + 0.20 * satd_risk
            + 0.15 * coupling_risk;

        QuantitativeAnalysis {
            complexity,
            churn,
            satd_count,
            satd_severity,
            coupling,
            risk_score,
        }
    }

    /// Check if code unit is high risk
    pub fn is_high_risk(&self) -> bool {
        self.risk_score > 0.7
    }

    /// Get risk level as string
    pub fn risk_level(&self) -> &'static str {
        if self.risk_score >= 0.8 {
            "CRITICAL"
        } else if self.risk_score >= 0.6 {
            "HIGH"
        } else if self.risk_score >= 0.4 {
            "MEDIUM"
        } else if self.risk_score >= 0.2 {
            "LOW"
        } else {
            "MINIMAL"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complexity_metrics_creation() {
        let metrics = ComplexityMetrics::new(100);
        assert_eq!(metrics.loc, 100);
        assert_eq!(metrics.cyclomatic, 1);
        assert_eq!(metrics.cognitive, 0);
    }

    #[test]
    fn test_complexity_score_simple() {
        let metrics = ComplexityMetrics {
            loc: 50,
            cyclomatic: 5,
            cognitive: 8,
            halstead_difficulty: 10.0,
            parameters: 2,
            nesting_depth: 2,
        };

        let score = metrics.complexity_score();
        assert!(score > 0.0 && score < 0.5); // Should be low-medium complexity
    }

    #[test]
    fn test_complexity_is_complex() {
        let simple = ComplexityMetrics::new(50);
        assert!(!simple.is_complex());

        let complex = ComplexityMetrics {
            loc: 200,
            cyclomatic: 15,
            cognitive: 20,
            halstead_difficulty: 25.0,
            parameters: 7,
            nesting_depth: 5,
        };
        assert!(complex.is_complex());
    }

    #[test]
    fn test_churn_correlation_creation() {
        let churn = ChurnCorrelation::new("test.rs".to_string(), 10, 3);
        assert_eq!(churn.file, "test.rs");
        assert_eq!(churn.changes, 10);
        assert_eq!(churn.bugs, 3);
        assert!((churn.correlation - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_churn_high_risk() {
        let low_risk = ChurnCorrelation::new("safe.rs".to_string(), 5, 1);
        assert!(!low_risk.is_high_risk());

        let high_risk = ChurnCorrelation::new("risky.rs".to_string(), 30, 10);
        assert!(high_risk.is_high_risk());
    }

    #[test]
    fn test_satd_type_severity() {
        assert_eq!(SatdType::Todo.severity(), 3);
        assert_eq!(SatdType::Fixme.severity(), 7);
        assert_eq!(SatdType::Hack.severity(), 9);
    }

    #[test]
    fn test_satd_detector_creation() {
        let detector = SatdDetector::new();
        assert_eq!(detector.total_count(), 0);
    }

    #[test]
    fn test_satd_detector_detect_todo() {
        let mut detector = SatdDetector::new();
        let source = "// TODO: Fix this bug\nlet x = 1;";
        detector.detect("test.rs".to_string(), source);

        assert_eq!(detector.total_count(), 1);
        assert_eq!(detector.count_by_type(SatdType::Todo), 1);
    }

    #[test]
    fn test_satd_detector_detect_multiple() {
        let mut detector = SatdDetector::new();
        let source = "// TODO: Fix\n// FIXME: Broken\n// HACK: Temporary\nlet x = 1;";
        detector.detect("test.rs".to_string(), source);

        assert_eq!(detector.total_count(), 3);
        assert_eq!(detector.count_by_type(SatdType::Todo), 1);
        assert_eq!(detector.count_by_type(SatdType::Fixme), 1);
        assert_eq!(detector.count_by_type(SatdType::Hack), 1);
    }

    #[test]
    fn test_satd_severity_score() {
        let mut detector = SatdDetector::new();
        detector.detect("test.rs".to_string(), "// TODO: Minor issue");
        let score1 = detector.severity_score();

        let mut detector2 = SatdDetector::new();
        detector2.detect("test.rs".to_string(), "// HACK: Critical issue");
        let score2 = detector2.severity_score();

        // HACK should have higher severity than TODO
        assert!(score2 > score1);
    }

    #[test]
    fn test_dependency_node_creation() {
        let node = DependencyNode::new("test_func".to_string());
        assert_eq!(node.name, "test_func");
        assert_eq!(node.fan_in(), 0);
        assert_eq!(node.fan_out(), 0);
    }

    #[test]
    fn test_dependency_node_add() {
        let mut node = DependencyNode::new("caller".to_string());
        node.add_dependency("callee1".to_string());
        node.add_dependency("callee2".to_string());
        node.add_dependent("caller1".to_string());

        assert_eq!(node.fan_out(), 2);
        assert_eq!(node.fan_in(), 1);
    }

    #[test]
    fn test_dependency_node_instability() {
        let mut node = DependencyNode::new("func".to_string());
        node.add_dependency("dep1".to_string());
        node.add_dependency("dep2".to_string());
        node.add_dependent("caller".to_string());

        // Instability = fan-out / (fan-in + fan-out) = 2 / 3 = 0.666...
        assert!((node.instability() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_dependency_analyzer_creation() {
        let analyzer = DependencyAnalyzer::new();
        assert_eq!(analyzer.nodes().len(), 0);
    }

    #[test]
    fn test_dependency_analyzer_add_dependency() {
        let mut analyzer = DependencyAnalyzer::new();
        analyzer.add_dependency("main".to_string(), "helper".to_string());

        let main_node = analyzer.get_node("main").unwrap();
        assert_eq!(main_node.fan_out(), 1);

        let helper_node = analyzer.get_node("helper").unwrap();
        assert_eq!(helper_node.fan_in(), 1);
    }

    #[test]
    fn test_dependency_analyzer_highly_coupled() {
        let mut analyzer = DependencyAnalyzer::new();
        analyzer.add_dependency("main".to_string(), "func1".to_string());
        analyzer.add_dependency("main".to_string(), "func2".to_string());
        analyzer.add_dependency("main".to_string(), "func3".to_string());
        analyzer.add_dependency("func1".to_string(), "main".to_string());

        let coupled = analyzer.find_highly_coupled(2);
        assert!(!coupled.is_empty());
    }

    #[test]
    fn test_quantitative_analysis_creation() {
        let complexity = ComplexityMetrics::new(100);
        let churn = Some(ChurnCorrelation::new("test.rs".to_string(), 10, 2));

        let analysis = QuantitativeAnalysis::new(complexity, churn, 5, 0.3, 8);

        assert_eq!(analysis.satd_count, 5);
        assert!((analysis.satd_severity - 0.3).abs() < 0.01);
        assert!(analysis.risk_score >= 0.0 && analysis.risk_score <= 1.0);
    }

    #[test]
    fn test_quantitative_analysis_risk_level() {
        let complexity = ComplexityMetrics::new(50);
        let low_risk = QuantitativeAnalysis::new(complexity, None, 0, 0.0, 2);
        assert_eq!(low_risk.risk_level(), "MINIMAL");

        let complex = ComplexityMetrics {
            loc: 500,
            cyclomatic: 25,
            cognitive: 40,
            halstead_difficulty: 50.0,
            parameters: 12,
            nesting_depth: 10,
        };
        let churn = Some(ChurnCorrelation::new("test.rs".to_string(), 50, 20));
        let high_risk = QuantitativeAnalysis::new(complex, churn, 15, 0.9, 30);
        assert!(high_risk.is_high_risk());
    }
}
