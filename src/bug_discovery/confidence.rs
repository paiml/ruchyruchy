// Confidence Scoring System (Jidoka Principle)
// Prevents alert fatigue by ranking automated findings by confidence level
//
// References:
// - Section 2.1 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md
// - Beller et al. (2015): Managing alert fatigue in automated systems

/// Overall confidence score for a bug report (0.0-1.0)
#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceScore {
    pub overall: f64,
    pub discovery_method_weight: f64,
    pub reproducibility_score: f64,
    pub quantitative_evidence: f64,
    pub root_cause_clarity: f64,
}

impl ConfidenceScore {
    /// Create a new confidence score from components
    pub fn new(
        discovery_method: f64,
        reproducibility: f64,
        evidence: f64,
        root_cause: f64,
    ) -> Self {
        let overall = ConfidenceScorer::calculate_overall(
            discovery_method,
            reproducibility,
            evidence,
            root_cause,
        );

        ConfidenceScore {
            overall,
            discovery_method_weight: discovery_method,
            reproducibility_score: reproducibility,
            quantitative_evidence: evidence,
            root_cause_clarity: root_cause,
        }
    }

    /// Get priority level based on confidence score
    pub fn priority(&self) -> Priority {
        match self.overall {
            x if x >= 0.85 => Priority::Critical,
            x if x >= 0.70 => Priority::High,
            x if x >= 0.50 => Priority::Medium,
            x if x >= 0.30 => Priority::Low,
            _ => Priority::Noise,
        }
    }

    /// Get recommended action based on priority
    pub fn recommended_action(&self) -> &'static str {
        match self.priority() {
            Priority::Critical => "File immediately, block release",
            Priority::High => "File within 24 hours, investigate",
            Priority::Medium => "File within 1 week, triage",
            Priority::Low => "Review manually before filing",
            Priority::Noise => "Suppress or flag for human review",
        }
    }
}

/// Priority levels based on confidence thresholds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Critical, // 0.85-1.0
    High,     // 0.70-0.84
    Medium,   // 0.50-0.69
    Low,      // 0.30-0.49
    Noise,    // <0.30
}

/// Discovery method types with their confidence weights
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiscoveryMethod {
    DifferentialTestVersionRegression, // 1.0
    DifferentialTestTargetMismatch,    // 0.9
    PropertyTestViolation,             // 0.95
    GrammarFuzzCrashHang,              // 0.85
    GrammarFuzzIncorrectOutput,        // 0.70
    MutationFuzzCrash,                 // 0.75
    CodeChurnHotSpot,                  // 0.60
}

impl DiscoveryMethod {
    /// Get confidence weight for this discovery method
    pub fn confidence_weight(&self) -> f64 {
        match self {
            DiscoveryMethod::DifferentialTestVersionRegression => 1.0,
            DiscoveryMethod::DifferentialTestTargetMismatch => 0.9,
            DiscoveryMethod::PropertyTestViolation => 0.95,
            DiscoveryMethod::GrammarFuzzCrashHang => 0.85,
            DiscoveryMethod::GrammarFuzzIncorrectOutput => 0.70,
            DiscoveryMethod::MutationFuzzCrash => 0.75,
            DiscoveryMethod::CodeChurnHotSpot => 0.60,
        }
    }
}

/// Reproducibility levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reproducibility {
    Always,              // 1.0
    AlwaysLargeTestCase, // 0.9
    IntermittentHigh,    // 0.7 (>50% failure rate)
    IntermittentLow,     // 0.5 (<50% failure rate)
    NonDeterministic,    // 0.3
}

impl Reproducibility {
    pub fn score(&self) -> f64 {
        match self {
            Reproducibility::Always => 1.0,
            Reproducibility::AlwaysLargeTestCase => 0.9,
            Reproducibility::IntermittentHigh => 0.7,
            Reproducibility::IntermittentLow => 0.5,
            Reproducibility::NonDeterministic => 0.3,
        }
    }
}

/// Quantitative evidence completeness
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvidenceLevel {
    Complete, // 1.0 - All metrics collected
    Partial,  // 0.8 - Missing 1-2 categories
    Limited,  // 0.6 - Only complexity or churn
    None,     // 0.4 - No quantitative data
}

impl EvidenceLevel {
    pub fn score(&self) -> f64 {
        match self {
            EvidenceLevel::Complete => 1.0,
            EvidenceLevel::Partial => 0.8,
            EvidenceLevel::Limited => 0.6,
            EvidenceLevel::None => 0.4,
        }
    }
}

/// Root cause clarity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RootCauseClarity {
    SingleObviousCause,   // 1.0
    PrimaryWithSecondary, // 0.8
    MultiplePlausible,    // 0.6
    UnclearHypothesis,    // 0.4
    NoRootCause,          // 0.2
}

impl RootCauseClarity {
    pub fn score(&self) -> f64 {
        match self {
            RootCauseClarity::SingleObviousCause => 1.0,
            RootCauseClarity::PrimaryWithSecondary => 0.8,
            RootCauseClarity::MultiplePlausible => 0.6,
            RootCauseClarity::UnclearHypothesis => 0.4,
            RootCauseClarity::NoRootCause => 0.2,
        }
    }
}

/// Confidence scorer
pub struct ConfidenceScorer;

impl ConfidenceScorer {
    /// Calculate overall confidence score using weighted formula
    ///
    /// Weights:
    /// - Discovery method: 35%
    /// - Reproducibility: 30%
    /// - Quantitative evidence: 20%
    /// - Root cause clarity: 15%
    pub fn calculate_overall(
        discovery_method: f64,
        reproducibility: f64,
        evidence: f64,
        root_cause: f64,
    ) -> f64 {
        0.35 * discovery_method + 0.30 * reproducibility + 0.20 * evidence + 0.15 * root_cause
    }

    /// Create confidence score from high-level components
    pub fn from_components(
        method: DiscoveryMethod,
        repro: Reproducibility,
        evidence: EvidenceLevel,
        root_cause: RootCauseClarity,
    ) -> ConfidenceScore {
        ConfidenceScore::new(
            method.confidence_weight(),
            repro.score(),
            evidence.score(),
            root_cause.score(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_confidence() {
        let score = ConfidenceScore::new(1.0, 1.0, 1.0, 1.0);
        assert!((score.overall - 1.0).abs() < 1e-10); // Floating point tolerance
        assert_eq!(score.priority(), Priority::Critical);
        assert!(score.recommended_action().contains("immediately"));
    }

    #[test]
    fn test_high_confidence() {
        let score = ConfidenceScore::new(0.9, 0.9, 0.8, 0.8);
        assert!(score.overall >= 0.85);
        assert_eq!(score.priority(), Priority::Critical);
    }

    #[test]
    fn test_medium_confidence() {
        let score = ConfidenceScore::new(0.7, 0.7, 0.6, 0.6);
        assert!(score.overall >= 0.50 && score.overall < 0.70);
        assert_eq!(score.priority(), Priority::Medium);
    }

    #[test]
    fn test_low_confidence() {
        let score = ConfidenceScore::new(0.4, 0.5, 0.4, 0.4);
        assert!(score.overall < 0.50);
    }

    #[test]
    fn test_from_components_version_regression() {
        let score = ConfidenceScorer::from_components(
            DiscoveryMethod::DifferentialTestVersionRegression,
            Reproducibility::Always,
            EvidenceLevel::Complete,
            RootCauseClarity::SingleObviousCause,
        );
        assert!((score.overall - 1.0).abs() < 1e-10); // Floating point tolerance
        assert_eq!(score.priority(), Priority::Critical);
    }

    #[test]
    fn test_from_components_churn_hotspot() {
        let score = ConfidenceScorer::from_components(
            DiscoveryMethod::CodeChurnHotSpot,
            Reproducibility::NonDeterministic,
            EvidenceLevel::Limited,
            RootCauseClarity::UnclearHypothesis,
        );
        // Should be lower confidence (predictive, not deterministic)
        assert!(score.overall < 0.60);
    }

    #[test]
    fn test_discovery_method_weights() {
        assert_eq!(
            DiscoveryMethod::DifferentialTestVersionRegression.confidence_weight(),
            1.0
        );
        assert_eq!(
            DiscoveryMethod::PropertyTestViolation.confidence_weight(),
            0.95
        );
        assert_eq!(DiscoveryMethod::CodeChurnHotSpot.confidence_weight(), 0.60);
    }

    #[test]
    fn test_reproducibility_scores() {
        assert_eq!(Reproducibility::Always.score(), 1.0);
        assert_eq!(Reproducibility::IntermittentHigh.score(), 0.7);
        assert_eq!(Reproducibility::NonDeterministic.score(), 0.3);
    }

    #[test]
    fn test_priority_thresholds() {
        let critical = ConfidenceScore::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(critical.priority(), Priority::Critical);

        let high = ConfidenceScore::new(0.9, 0.7, 0.7, 0.7);
        assert_eq!(high.priority(), Priority::High);

        let medium = ConfidenceScore::new(0.6, 0.6, 0.6, 0.6);
        assert_eq!(medium.priority(), Priority::Medium);

        let low = ConfidenceScore::new(0.4, 0.4, 0.4, 0.4);
        assert_eq!(low.priority(), Priority::Low);

        let noise = ConfidenceScore::new(0.2, 0.2, 0.2, 0.2);
        assert_eq!(noise.priority(), Priority::Noise);
    }
}
