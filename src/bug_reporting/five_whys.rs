// Assisted Five-Whys Analysis
// REPORT-002: Assisted Five-Whys Analysis Implementation
//
// References:
// - Ohno (1988): "Toyota Production System: Beyond Large-Scale Production"
// - Card (2017): "The Art of Agile Development"
// - Section 8.2 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md
//
// CRITICAL: This is ASSISTED, not AUTOMATED. System provides data-driven
// hypotheses with confidence levels. Human must validate causality.

use std::collections::HashMap;

/// Confidence level for hypotheses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceLevel {
    /// High confidence (>80% data correlation)
    High,
    /// Medium confidence (50-80% data correlation)
    Medium,
    /// Low confidence (<50% data correlation)
    Low,
}

impl ConfidenceLevel {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfidenceLevel::High => "HIGH",
            ConfidenceLevel::Medium => "MEDIUM",
            ConfidenceLevel::Low => "LOW",
        }
    }

    /// Get numeric score (0.0-1.0)
    pub fn score(&self) -> f64 {
        match self {
            ConfidenceLevel::High => 0.9,
            ConfidenceLevel::Medium => 0.65,
            ConfidenceLevel::Low => 0.35,
        }
    }
}

/// Data point for Five-Whys analysis
#[derive(Debug, Clone)]
pub struct DataPoint {
    /// Data point name/description
    pub name: String,
    /// Data value
    pub value: String,
    /// Source of data (e.g., "git log", "complexity analysis")
    pub source: String,
    /// Relevance to bug (0.0-1.0)
    pub relevance: f64,
}

impl DataPoint {
    /// Create a new data point
    pub fn new(name: String, value: String, source: String, relevance: f64) -> Self {
        DataPoint {
            name,
            value,
            source,
            relevance: relevance.clamp(0.0, 1.0),
        }
    }

    /// Check if data point is highly relevant
    pub fn is_highly_relevant(&self) -> bool {
        self.relevance > 0.7
    }
}

/// Hypothesis for root cause
#[derive(Debug, Clone)]
pub struct Hypothesis {
    /// Hypothesis statement
    pub statement: String,
    /// Supporting data points
    pub supporting_data: Vec<DataPoint>,
    /// Confidence level
    pub confidence: ConfidenceLevel,
    /// Requires human validation
    pub needs_validation: bool,
}

impl Hypothesis {
    /// Create a new hypothesis
    pub fn new(statement: String, confidence: ConfidenceLevel) -> Self {
        Hypothesis {
            statement,
            supporting_data: Vec::new(),
            confidence,
            needs_validation: true,
        }
    }

    /// Add supporting data
    pub fn add_data(mut self, data: DataPoint) -> Self {
        self.supporting_data.push(data);
        self
    }

    /// Calculate data strength (average relevance of supporting data)
    pub fn data_strength(&self) -> f64 {
        if self.supporting_data.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.supporting_data.iter().map(|d| d.relevance).sum();
        sum / self.supporting_data.len() as f64
    }

    /// Check if hypothesis is well-supported
    pub fn is_well_supported(&self) -> bool {
        self.supporting_data.len() >= 2 && self.data_strength() > 0.6
    }
}

/// Five-Whys layer
#[derive(Debug, Clone)]
pub struct WhyLayer {
    /// Layer number (1-5)
    pub layer: usize,
    /// Question asked
    pub question: String,
    /// Hypotheses for this layer
    pub hypotheses: Vec<Hypothesis>,
    /// Data points for this layer
    pub data_points: Vec<DataPoint>,
}

impl WhyLayer {
    /// Create a new Why layer
    pub fn new(layer: usize, question: String) -> Self {
        WhyLayer {
            layer,
            question,
            hypotheses: Vec::new(),
            data_points: Vec::new(),
        }
    }

    /// Add a hypothesis
    pub fn add_hypothesis(&mut self, hypothesis: Hypothesis) {
        self.hypotheses.push(hypothesis);
    }

    /// Add a data point
    pub fn add_data_point(&mut self, data: DataPoint) {
        self.data_points.push(data);
    }

    /// Get best hypothesis (highest confidence + data strength)
    pub fn best_hypothesis(&self) -> Option<&Hypothesis> {
        self.hypotheses.iter().max_by(|a, b| {
            let score_a = a.confidence.score() * a.data_strength();
            let score_b = b.confidence.score() * b.data_strength();
            score_a.partial_cmp(&score_b).unwrap()
        })
    }
}

/// Five-Whys analysis result
#[derive(Debug, Clone)]
pub struct FiveWhysAnalysis {
    /// Bug description
    pub bug_description: String,
    /// Five Why layers
    pub layers: Vec<WhyLayer>,
    /// Overall confidence in root cause
    pub overall_confidence: ConfidenceLevel,
    /// Needs human validation
    pub needs_validation: bool,
}

impl FiveWhysAnalysis {
    /// Create a new Five-Whys analysis
    pub fn new(bug_description: String) -> Self {
        FiveWhysAnalysis {
            bug_description,
            layers: Vec::new(),
            overall_confidence: ConfidenceLevel::Low,
            needs_validation: true,
        }
    }

    /// Add a layer
    pub fn add_layer(&mut self, layer: WhyLayer) {
        self.layers.push(layer);
    }

    /// Calculate overall confidence
    pub fn calculate_confidence(&mut self) {
        if self.layers.is_empty() {
            self.overall_confidence = ConfidenceLevel::Low;
            return;
        }

        // Average confidence across layers
        let avg_confidence: f64 = self
            .layers
            .iter()
            .filter_map(|l| l.best_hypothesis())
            .map(|h| h.confidence.score() * h.data_strength())
            .sum::<f64>()
            / self.layers.len() as f64;

        self.overall_confidence = if avg_confidence > 0.7 {
            ConfidenceLevel::High
        } else if avg_confidence > 0.5 {
            ConfidenceLevel::Medium
        } else {
            ConfidenceLevel::Low
        };
    }

    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Five-Whys Analysis (ASSISTED)\n\n");
        md.push_str("⚠️  **IMPORTANT**: This is an ASSISTED analysis. All hypotheses REQUIRE human validation.\n\n");

        md.push_str(&format!("**Bug**: {}\n\n", self.bug_description));
        md.push_str(&format!(
            "**Overall Confidence**: {} ({})\n\n",
            self.overall_confidence.as_str(),
            if self.needs_validation {
                "NEEDS VALIDATION"
            } else {
                "validated"
            }
        ));

        for layer in &self.layers {
            md.push_str(&format!("## Why #{}: {}\n\n", layer.layer, layer.question));

            if !layer.data_points.is_empty() {
                md.push_str("### Data Points\n\n");
                for data in &layer.data_points {
                    md.push_str(&format!(
                        "- **{}**: {} (source: {}, relevance: {:.0}%)\n",
                        data.name,
                        data.value,
                        data.source,
                        data.relevance * 100.0
                    ));
                }
                md.push('\n');
            }

            if !layer.hypotheses.is_empty() {
                md.push_str("### Hypotheses\n\n");
                for (i, hypothesis) in layer.hypotheses.iter().enumerate() {
                    md.push_str(&format!(
                        "{}. **[{}]** {}\n",
                        i + 1,
                        hypothesis.confidence.as_str(),
                        hypothesis.statement
                    ));

                    if !hypothesis.supporting_data.is_empty() {
                        md.push_str("   - Supporting data:\n");
                        for data in &hypothesis.supporting_data {
                            md.push_str(&format!(
                                "     - {}: {} ({:.0}% relevant)\n",
                                data.name,
                                data.value,
                                data.relevance * 100.0
                            ));
                        }
                    }

                    if hypothesis.needs_validation {
                        md.push_str("   - ⚠️  **Requires human validation**\n");
                    }

                    md.push('\n');
                }
            }
        }

        md.push_str("---\n\n");
        md.push_str("**Note**: This analysis is based on quantitative data. ");
        md.push_str("A human must validate the causality chain and confirm the root cause.\n");

        md
    }
}

/// Five-Whys analyzer
pub struct FiveWhysAnalyzer {
    /// Data points collected
    data_points: HashMap<String, Vec<DataPoint>>,
}

impl FiveWhysAnalyzer {
    /// Create a new Five-Whys analyzer
    pub fn new() -> Self {
        FiveWhysAnalyzer {
            data_points: HashMap::new(),
        }
    }

    /// Add a data point
    pub fn add_data_point(&mut self, category: String, data: DataPoint) {
        self.data_points.entry(category).or_default().push(data);
    }

    /// Generate hypotheses from data
    pub fn generate_hypotheses(&self, question: &str) -> Vec<Hypothesis> {
        let mut hypotheses = Vec::new();

        // Analyze complexity data
        if let Some(complexity_data) = self.data_points.get("complexity") {
            for data in complexity_data {
                if data.is_highly_relevant() {
                    let hypothesis = Hypothesis::new(
                        format!("High complexity contributed to the bug: {}", data.value),
                        ConfidenceLevel::Medium,
                    )
                    .add_data(data.clone());
                    hypotheses.push(hypothesis);
                }
            }
        }

        // Analyze churn data
        if let Some(churn_data) = self.data_points.get("churn") {
            for data in churn_data {
                if data.is_highly_relevant() {
                    let hypothesis = Hypothesis::new(
                        format!("Frequent changes indicate instability: {}", data.value),
                        ConfidenceLevel::High,
                    )
                    .add_data(data.clone());
                    hypotheses.push(hypothesis);
                }
            }
        }

        // Analyze SATD data
        if let Some(satd_data) = self.data_points.get("satd") {
            for data in satd_data {
                if data.is_highly_relevant() {
                    let hypothesis = Hypothesis::new(
                        format!(
                            "Technical debt marker indicates known issue: {}",
                            data.value
                        ),
                        ConfidenceLevel::Medium,
                    )
                    .add_data(data.clone());
                    hypotheses.push(hypothesis);
                }
            }
        }

        // If no specific hypotheses, generate generic one
        if hypotheses.is_empty() {
            hypotheses.push(Hypothesis::new(
                format!(
                    "Unknown cause - insufficient data for question: {}",
                    question
                ),
                ConfidenceLevel::Low,
            ));
        }

        hypotheses
    }

    /// Perform Five-Whys analysis
    pub fn analyze(&self, bug_description: String) -> FiveWhysAnalysis {
        let mut analysis = FiveWhysAnalysis::new(bug_description);

        // Generate 5 layers
        let questions = [
            "Why did the bug occur?",
            "Why was the system vulnerable to this issue?",
            "Why was this vulnerability not caught earlier?",
            "Why are our processes not preventing this?",
            "Why is the root cause not addressed?",
        ];

        for (i, question) in questions.iter().enumerate() {
            let mut layer = WhyLayer::new(i + 1, question.to_string());

            // Add relevant data points to layer
            for data_points in self.data_points.values() {
                for data in data_points {
                    if data.is_highly_relevant() {
                        layer.add_data_point(data.clone());
                    }
                }
            }

            // Generate hypotheses
            let hypotheses = self.generate_hypotheses(question);
            for hypothesis in hypotheses {
                layer.add_hypothesis(hypothesis);
            }

            analysis.add_layer(layer);
        }

        analysis.calculate_confidence();
        analysis
    }
}

impl Default for FiveWhysAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_level_values() {
        assert_eq!(ConfidenceLevel::High.as_str(), "HIGH");
        assert_eq!(ConfidenceLevel::Medium.as_str(), "MEDIUM");
        assert_eq!(ConfidenceLevel::Low.as_str(), "LOW");

        assert!((ConfidenceLevel::High.score() - 0.9).abs() < 0.01);
        assert!((ConfidenceLevel::Medium.score() - 0.65).abs() < 0.01);
        assert!((ConfidenceLevel::Low.score() - 0.35).abs() < 0.01);
    }

    #[test]
    fn test_data_point_creation() {
        let data = DataPoint::new(
            "Complexity".to_string(),
            "High".to_string(),
            "metrics".to_string(),
            0.85,
        );

        assert_eq!(data.name, "Complexity");
        assert_eq!(data.value, "High");
        assert!((data.relevance - 0.85).abs() < 0.01);
        assert!(data.is_highly_relevant());
    }

    #[test]
    fn test_data_point_relevance_clamping() {
        let too_low = DataPoint::new(
            "Test".to_string(),
            "val".to_string(),
            "src".to_string(),
            -0.5,
        );
        assert_eq!(too_low.relevance, 0.0);

        let too_high = DataPoint::new(
            "Test".to_string(),
            "val".to_string(),
            "src".to_string(),
            1.5,
        );
        assert_eq!(too_high.relevance, 1.0);
    }

    #[test]
    fn test_hypothesis_creation() {
        let hyp = Hypothesis::new("Test hypothesis".to_string(), ConfidenceLevel::High);

        assert_eq!(hyp.statement, "Test hypothesis");
        assert_eq!(hyp.confidence, ConfidenceLevel::High);
        assert!(hyp.needs_validation);
        assert_eq!(hyp.supporting_data.len(), 0);
    }

    #[test]
    fn test_hypothesis_with_data() {
        let data = DataPoint::new(
            "Test".to_string(),
            "Value".to_string(),
            "source".to_string(),
            0.9,
        );
        let hyp = Hypothesis::new("Test".to_string(), ConfidenceLevel::Medium).add_data(data);

        assert_eq!(hyp.supporting_data.len(), 1);
        assert!((hyp.data_strength() - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_hypothesis_well_supported() {
        let data1 = DataPoint::new("D1".to_string(), "V1".to_string(), "s1".to_string(), 0.7);
        let data2 = DataPoint::new("D2".to_string(), "V2".to_string(), "s2".to_string(), 0.8);

        let hyp = Hypothesis::new("Test".to_string(), ConfidenceLevel::High)
            .add_data(data1)
            .add_data(data2);

        assert!(hyp.is_well_supported());
    }

    #[test]
    fn test_why_layer_creation() {
        let layer = WhyLayer::new(1, "Why did this happen?".to_string());

        assert_eq!(layer.layer, 1);
        assert_eq!(layer.question, "Why did this happen?");
        assert_eq!(layer.hypotheses.len(), 0);
        assert_eq!(layer.data_points.len(), 0);
    }

    #[test]
    fn test_why_layer_best_hypothesis() {
        let mut layer = WhyLayer::new(1, "Test".to_string());

        let low = Hypothesis::new("Low confidence".to_string(), ConfidenceLevel::Low);
        let high = Hypothesis::new("High confidence".to_string(), ConfidenceLevel::High);

        layer.add_hypothesis(low);
        layer.add_hypothesis(high);

        let best = layer.best_hypothesis();
        assert!(best.is_some());
        assert_eq!(best.unwrap().confidence, ConfidenceLevel::High);
    }

    #[test]
    fn test_five_whys_analysis_creation() {
        let analysis = FiveWhysAnalysis::new("Test bug".to_string());

        assert_eq!(analysis.bug_description, "Test bug");
        assert_eq!(analysis.layers.len(), 0);
        assert!(analysis.needs_validation);
    }

    #[test]
    fn test_five_whys_confidence_calculation() {
        let mut analysis = FiveWhysAnalysis::new("Test".to_string());

        let mut layer = WhyLayer::new(1, "Why?".to_string());
        let data = DataPoint::new("D".to_string(), "V".to_string(), "S".to_string(), 0.9);
        let hyp = Hypothesis::new("Hypothesis".to_string(), ConfidenceLevel::High).add_data(data);
        layer.add_hypothesis(hyp);

        analysis.add_layer(layer);
        analysis.calculate_confidence();

        // High confidence hypothesis with high data strength = High overall
        assert_eq!(analysis.overall_confidence, ConfidenceLevel::High);
    }

    #[test]
    fn test_five_whys_markdown_generation() {
        let mut analysis = FiveWhysAnalysis::new("Test bug".to_string());

        let mut layer = WhyLayer::new(1, "Why did it happen?".to_string());
        let data = DataPoint::new(
            "Complexity".to_string(),
            "High".to_string(),
            "metrics".to_string(),
            0.8,
        );
        layer.add_data_point(data.clone());

        let hyp =
            Hypothesis::new("High complexity".to_string(), ConfidenceLevel::Medium).add_data(data);
        layer.add_hypothesis(hyp);

        analysis.add_layer(layer);
        analysis.calculate_confidence();

        let markdown = analysis.to_markdown();
        assert!(markdown.contains("# Five-Whys Analysis"));
        assert!(markdown.contains("ASSISTED"));
        assert!(markdown.contains("Test bug"));
        assert!(markdown.contains("Why #1"));
    }

    #[test]
    fn test_five_whys_analyzer_creation() {
        let analyzer = FiveWhysAnalyzer::new();
        assert_eq!(analyzer.data_points.len(), 0);
    }

    #[test]
    fn test_five_whys_analyzer_add_data() {
        let mut analyzer = FiveWhysAnalyzer::new();
        let data = DataPoint::new(
            "Test".to_string(),
            "Value".to_string(),
            "source".to_string(),
            0.7,
        );

        analyzer.add_data_point("complexity".to_string(), data);
        assert!(analyzer.data_points.contains_key("complexity"));
    }

    #[test]
    fn test_five_whys_analyzer_generate_hypotheses() {
        let mut analyzer = FiveWhysAnalyzer::new();

        let data = DataPoint::new(
            "High complexity".to_string(),
            "Cyclomatic: 25".to_string(),
            "metrics".to_string(),
            0.9,
        );

        analyzer.add_data_point("complexity".to_string(), data);

        let hypotheses = analyzer.generate_hypotheses("Why did the bug occur?");
        assert!(!hypotheses.is_empty());
        assert!(hypotheses[0].statement.contains("complexity"));
    }

    #[test]
    fn test_five_whys_analyzer_full_analysis() {
        let mut analyzer = FiveWhysAnalyzer::new();

        let complexity_data = DataPoint::new(
            "Complexity".to_string(),
            "Very high".to_string(),
            "metrics".to_string(),
            0.85,
        );

        let churn_data = DataPoint::new(
            "Churn".to_string(),
            "30 changes".to_string(),
            "git".to_string(),
            0.9,
        );

        analyzer.add_data_point("complexity".to_string(), complexity_data);
        analyzer.add_data_point("churn".to_string(), churn_data);

        let analysis = analyzer.analyze("Vec::new() hangs".to_string());

        assert_eq!(analysis.layers.len(), 5);
        assert!(analysis.needs_validation);
        assert!(!analysis.layers[0].hypotheses.is_empty());
    }
}
