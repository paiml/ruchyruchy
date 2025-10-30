// Confidence Scoring System
// Implements Jidoka-based confidence scoring to prevent alert fatigue
//
// Formula: confidence = 0.30 * discovery + 0.30 * reproducibility
//                     + 0.25 * evidence + 0.15 * clarity
//
// Research grounding: Ohno (1988) Toyota Production System - Jidoka principle

/// Discovery method used to find the bug
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiscoveryMethod {
    /// Property-based testing (highest confidence)
    PropertyTesting,
    /// Fuzz testing (very high confidence)
    FuzzTesting,
    /// Manual testing (moderate confidence)
    ManualTesting,
    /// User report (lower confidence - may be imprecise)
    UserReport,
}

impl DiscoveryMethod {
    /// Get weight for this discovery method (0.0-1.0)
    pub fn weight(&self) -> f64 {
        match self {
            DiscoveryMethod::PropertyTesting => 0.9, // Mathematical properties verified
            DiscoveryMethod::FuzzTesting => 0.85,    // Large input space coverage
            DiscoveryMethod::ManualTesting => 0.7,   // Human-guided testing
            DiscoveryMethod::UserReport => 0.5,      // Real-world but potentially unclear
        }
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            DiscoveryMethod::PropertyTesting => "Property Testing",
            DiscoveryMethod::FuzzTesting => "Fuzz Testing",
            DiscoveryMethod::ManualTesting => "Manual Testing",
            DiscoveryMethod::UserReport => "User Report",
        }
    }
}

/// Reproducibility level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reproducibility {
    /// Always reproducible (100%)
    Always,
    /// Often reproducible (>75%)
    Often,
    /// Sometimes reproducible (25-75%)
    Sometimes,
    /// Rarely reproducible (<25%) - heisenbugs
    Rarely,
}

impl Reproducibility {
    /// Get weight for this reproducibility level (0.0-1.0)
    pub fn weight(&self) -> f64 {
        match self {
            Reproducibility::Always => 1.0,
            Reproducibility::Often => 0.8,
            Reproducibility::Sometimes => 0.5,
            Reproducibility::Rarely => 0.2,
        }
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Reproducibility::Always => "Always",
            Reproducibility::Often => "Often (>75%)",
            Reproducibility::Sometimes => "Sometimes (25-75%)",
            Reproducibility::Rarely => "Rarely (<25%)",
        }
    }
}

/// Quantitative evidence strength
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantitativeEvidence {
    /// Strong evidence (multiple metrics, clear patterns)
    Strong,
    /// Moderate evidence (some metrics, suggestive patterns)
    Moderate,
    /// Weak evidence (minimal metrics, unclear patterns)
    Weak,
}

impl QuantitativeEvidence {
    /// Get weight for this evidence level (0.0-1.0)
    pub fn weight(&self) -> f64 {
        match self {
            QuantitativeEvidence::Strong => 0.9,
            QuantitativeEvidence::Moderate => 0.6,
            QuantitativeEvidence::Weak => 0.3,
        }
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            QuantitativeEvidence::Strong => "Strong",
            QuantitativeEvidence::Moderate => "Moderate",
            QuantitativeEvidence::Weak => "Weak",
        }
    }
}

/// Root cause clarity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RootCauseClarity {
    /// Root cause confirmed (proven)
    Confirmed,
    /// Root cause likely (strong hypothesis)
    Likely,
    /// Root cause unclear (needs investigation)
    Unclear,
}

impl RootCauseClarity {
    /// Get weight for this clarity level (0.0-1.0)
    pub fn weight(&self) -> f64 {
        match self {
            RootCauseClarity::Confirmed => 1.0,
            RootCauseClarity::Likely => 0.7,
            RootCauseClarity::Unclear => 0.3,
        }
    }

    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            RootCauseClarity::Confirmed => "Confirmed",
            RootCauseClarity::Likely => "Likely",
            RootCauseClarity::Unclear => "Unclear",
        }
    }
}

/// Priority level based on confidence score
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    /// Critical: score > 0.8 (immediate action required)
    Critical,
    /// High: score > 0.6 (prioritize soon)
    High,
    /// Medium: score > 0.4 (address in normal flow)
    Medium,
    /// Low: score <= 0.4 (needs more investigation)
    Low,
}

impl Priority {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Critical => "CRITICAL",
            Priority::High => "HIGH",
            Priority::Medium => "MEDIUM",
            Priority::Low => "LOW",
        }
    }
}

/// Confidence score calculated from 4 factors
#[derive(Debug, Clone)]
pub struct ConfidenceScore {
    /// Discovery method used
    pub discovery_method: DiscoveryMethod,
    /// Reproducibility level
    pub reproducibility: Reproducibility,
    /// Quantitative evidence strength
    pub evidence: QuantitativeEvidence,
    /// Root cause clarity
    pub clarity: RootCauseClarity,
    /// Calculated confidence score (0.0-1.0)
    pub score: f64,
}

impl ConfidenceScore {
    /// Create a new confidence score
    pub fn new(
        discovery_method: DiscoveryMethod,
        reproducibility: Reproducibility,
        evidence: QuantitativeEvidence,
        clarity: RootCauseClarity,
    ) -> Self {
        let score = Self::calculate_score(discovery_method, reproducibility, evidence, clarity);

        ConfidenceScore {
            discovery_method,
            reproducibility,
            evidence,
            clarity,
            score,
        }
    }

    /// Calculate confidence score using 4-factor weighted formula
    ///
    /// Formula: confidence = 0.30 * discovery + 0.30 * reproducibility
    ///                     + 0.25 * evidence + 0.15 * clarity
    ///
    /// Weights sum to 1.0, ensuring score stays in [0.0, 1.0] range.
    fn calculate_score(
        discovery_method: DiscoveryMethod,
        reproducibility: Reproducibility,
        evidence: QuantitativeEvidence,
        clarity: RootCauseClarity,
    ) -> f64 {
        0.30 * discovery_method.weight()
            + 0.30 * reproducibility.weight()
            + 0.25 * evidence.weight()
            + 0.15 * clarity.weight()
    }

    /// Get calculated score
    pub fn calculate(&self) -> f64 {
        self.score
    }

    /// Determine priority based on score
    pub fn priority(&self) -> Priority {
        if self.score > 0.8 {
            Priority::Critical
        } else if self.score > 0.6 {
            Priority::High
        } else if self.score > 0.4 {
            Priority::Medium
        } else {
            Priority::Low
        }
    }

    /// Generate human-readable explanation
    pub fn explain(&self) -> String {
        format!(
            "Confidence Score: {:.2} ({})\n\
             - Discovery Method: {} (weight: {})\n\
             - Reproducibility: {} (weight: {})\n\
             - Evidence: {} (weight: {})\n\
             - Root Cause: {} (weight: {})\n\
             \n\
             Formula: 0.30*{} + 0.30*{} + 0.25*{} + 0.15*{} = {:.3}",
            self.score,
            self.priority().as_str(),
            self.discovery_method.as_str(),
            self.discovery_method.weight(),
            self.reproducibility.as_str(),
            self.reproducibility.weight(),
            self.evidence.as_str(),
            self.evidence.weight(),
            self.clarity.as_str(),
            self.clarity.weight(),
            self.discovery_method.weight(),
            self.reproducibility.weight(),
            self.evidence.weight(),
            self.clarity.weight(),
            self.score,
        )
    }
}

/// Confidence scorer for bug reports
pub struct ConfidenceScorer {
    // Future: Add configuration for custom weights
}

impl ConfidenceScorer {
    /// Create a new confidence scorer
    pub fn new() -> Self {
        ConfidenceScorer {}
    }

    /// Score a bug report using the 4-factor formula
    pub fn score(
        &self,
        discovery_method: DiscoveryMethod,
        reproducibility: Reproducibility,
        evidence: QuantitativeEvidence,
        clarity: RootCauseClarity,
    ) -> ConfidenceScore {
        ConfidenceScore::new(discovery_method, reproducibility, evidence, clarity)
    }
}

impl Default for ConfidenceScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_score_bounds() {
        // Minimum score
        let min = ConfidenceScore::new(
            DiscoveryMethod::UserReport,
            Reproducibility::Rarely,
            QuantitativeEvidence::Weak,
            RootCauseClarity::Unclear,
        );
        assert!(min.score >= 0.0 && min.score <= 1.0);

        // Maximum score
        let max = ConfidenceScore::new(
            DiscoveryMethod::PropertyTesting,
            Reproducibility::Always,
            QuantitativeEvidence::Strong,
            RootCauseClarity::Confirmed,
        );
        assert!(max.score >= 0.0 && max.score <= 1.0);
        assert!(max.score > min.score);
    }

    #[test]
    fn test_priority_thresholds() {
        let critical = ConfidenceScore::new(
            DiscoveryMethod::PropertyTesting,
            Reproducibility::Always,
            QuantitativeEvidence::Strong,
            RootCauseClarity::Confirmed,
        );
        assert_eq!(critical.priority(), Priority::Critical);

        let low = ConfidenceScore::new(
            DiscoveryMethod::UserReport,
            Reproducibility::Rarely,
            QuantitativeEvidence::Weak,
            RootCauseClarity::Unclear,
        );
        assert_eq!(low.priority(), Priority::Low);
    }
}
