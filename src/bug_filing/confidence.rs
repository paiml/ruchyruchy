//! Confidence Calculation Module
//!
//! Implements 4-factor confidence scoring algorithm for bug reports.
//! Only bugs with confidence ≥0.9 are filed to upstream repository.

/// Confidence calculation factors
#[derive(Debug, Clone)]
pub struct ConfidenceFactors {
    /// Reproducibility level
    pub reproducibility: Reproducibility,
    /// Lines of code in reproduction
    pub lines_of_code: usize,
    /// Whether bug violates language specification
    pub spec_violation: bool,
    /// Impact level
    pub impact: Impact,
}

/// Reproducibility level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reproducibility {
    /// Always reproducible
    Always,
    /// Sometimes reproducible
    Sometimes,
    /// Never reproducible
    Never,
}

/// Impact level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Impact {
    /// Critical bugs (crashes, data loss)
    Critical,
    /// High impact bugs (incorrect behavior)
    High,
    /// Medium impact bugs (minor issues)
    Medium,
    /// Low impact bugs (cosmetic)
    Low,
}

/// Confidence calculator
///
/// Calculates confidence score (0.0-1.0) based on 4 weighted factors.
pub struct ConfidenceCalculator;

impl ConfidenceCalculator {
    /// Calculate confidence score (0.0-1.0)
    ///
    /// # Factors
    ///
    /// - **Reproducibility (40%)**:
    ///   - Always = 1.0
    ///   - Sometimes = 0.5
    ///   - Never = 0.0
    ///
    /// - **Minimality (30%)**:
    ///   - < 10 lines = 1.0
    ///   - 10-50 lines = 0.5
    ///   - > 50 lines = 0.0
    ///
    /// - **Spec Violation (20%)**:
    ///   - Clear spec violation = 1.0
    ///   - Undefined behavior = 0.5
    ///
    /// - **Impact (10%)**:
    ///   - Critical = 1.0
    ///   - High = 0.5
    ///   - Medium/Low = 0.0
    ///
    /// # Threshold
    ///
    /// Only bugs with confidence ≥0.9 are filed.
    ///
    /// # Example
    ///
    /// ```
    /// use ruchyruchy::bug_filing::{ConfidenceCalculator, ConfidenceFactors};
    /// use ruchyruchy::bug_filing::{Reproducibility, Impact};
    ///
    /// let factors = ConfidenceFactors {
    ///     reproducibility: Reproducibility::Always,
    ///     lines_of_code: 5,
    ///     spec_violation: true,
    ///     impact: Impact::Critical,
    /// };
    ///
    /// let confidence = ConfidenceCalculator::calculate(&factors);
    /// assert!(confidence >= 0.9);
    /// ```
    pub fn calculate(factors: &ConfidenceFactors) -> f64 {
        let repro_score = match factors.reproducibility {
            Reproducibility::Always => 1.0,
            Reproducibility::Sometimes => 0.5,
            Reproducibility::Never => 0.0,
        };

        let minimality_score = if factors.lines_of_code < 10 {
            1.0
        } else if factors.lines_of_code <= 50 {
            0.5
        } else {
            0.0
        };

        let spec_score = if factors.spec_violation { 1.0 } else { 0.5 };

        let impact_score = match factors.impact {
            Impact::Critical => 1.0,
            Impact::High => 0.5,
            Impact::Medium | Impact::Low => 0.0,
        };

        // Weighted average
        (repro_score * 0.4) + (minimality_score * 0.3) + (spec_score * 0.2) + (impact_score * 0.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_confidence() {
        let factors = ConfidenceFactors {
            reproducibility: Reproducibility::Always,
            lines_of_code: 5,
            spec_violation: true,
            impact: Impact::Critical,
        };

        let confidence = ConfidenceCalculator::calculate(&factors);
        assert!(confidence >= 0.9);
    }

    #[test]
    fn test_low_confidence() {
        let factors = ConfidenceFactors {
            reproducibility: Reproducibility::Never,
            lines_of_code: 100,
            spec_violation: false,
            impact: Impact::Low,
        };

        let confidence = ConfidenceCalculator::calculate(&factors);
        assert!(confidence < 0.9);
    }
}
