// Interpreter Bug Discovery Integration
// Integrates runtime error detection with bug discovery infrastructure
//
// This module provides:
// - Runtime error categorization
// - Confidence scoring for interpreter errors
// - Bug report generation
// - Integration with existing bug discovery tools

use crate::bug_discovery::confidence::ConfidenceScore;
use crate::interpreter::evaluator::EvalError;
use std::fmt;

/// Bug report generated from interpreter runtime error
#[derive(Debug, Clone)]
pub struct InterpreterBugReport {
    /// The original error
    pub error: String,
    /// Confidence score for this bug
    pub confidence: ConfidenceScore,
    /// Bug category
    pub category: BugCategory,
    /// Minimal reproduction code (if delta debugging was applied)
    pub minimal_repro: Option<String>,
    /// Call stack at time of error
    pub call_stack: Vec<String>,
    /// Error location (if available)
    pub location: Option<ErrorLocation>,
}

/// Location where error occurred
#[derive(Debug, Clone)]
pub struct ErrorLocation {
    /// File path (if known)
    pub file: Option<String>,
    /// Line number (if known)
    pub line: Option<usize>,
    /// Column number (if known)
    pub column: Option<usize>,
}

/// Category of interpreter bug
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BugCategory {
    /// Division or modulo by zero
    DivisionByZero,
    /// Array index out of bounds
    IndexOutOfBounds,
    /// Undefined variable or function
    UndefinedIdentifier,
    /// Type mismatch
    TypeMismatch,
    /// Stack overflow from excessive recursion
    StackOverflow,
    /// No match arm matched in match expression
    NoMatchArm,
    /// Unsupported operation
    UnsupportedOperation,
    /// Other runtime error
    Other,
}

impl fmt::Display for BugCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BugCategory::DivisionByZero => write!(f, "Division by Zero"),
            BugCategory::IndexOutOfBounds => write!(f, "Index Out of Bounds"),
            BugCategory::UndefinedIdentifier => write!(f, "Undefined Identifier"),
            BugCategory::TypeMismatch => write!(f, "Type Mismatch"),
            BugCategory::StackOverflow => write!(f, "Stack Overflow"),
            BugCategory::NoMatchArm => write!(f, "No Match Arm"),
            BugCategory::UnsupportedOperation => write!(f, "Unsupported Operation"),
            BugCategory::Other => write!(f, "Other Runtime Error"),
        }
    }
}

/// Bug discovery analyzer for interpreter errors
pub struct BugDiscoveryAnalyzer {
    /// Track discovered bugs for deduplication
    discovered_bugs: Vec<BugFingerprint>,
}

/// Fingerprint for bug deduplication
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BugFingerprint {
    /// Bug category
    pub category: BugCategory,
    /// Error signature for matching
    pub error_signature: String,
}

impl BugDiscoveryAnalyzer {
    /// Create a new bug discovery analyzer
    pub fn new() -> Self {
        BugDiscoveryAnalyzer {
            discovered_bugs: Vec::new(),
        }
    }

    /// Analyze an interpreter error and generate bug report
    pub fn analyze_error(
        &mut self,
        error: &EvalError,
        source_code: &str,
    ) -> InterpreterBugReport {
        // Categorize the error
        let category = Self::categorize_error(error);

        // Extract call stack if available
        let call_stack = Self::extract_call_stack(error);

        // Calculate confidence score
        let confidence = Self::calculate_confidence(category, &call_stack);

        // Generate minimal reproduction (placeholder for delta debugging integration)
        let minimal_repro = Self::minimize_reproduction(source_code, error);

        // Create bug report
        let report = InterpreterBugReport {
            error: format!("{:?}", error),
            confidence,
            category,
            minimal_repro,
            call_stack,
            location: None, // TODO: Extract from error if available
        };

        // Track for deduplication
        let fingerprint = BugFingerprint {
            category,
            error_signature: Self::generate_signature(error),
        };

        if !self.is_duplicate(&fingerprint) {
            self.discovered_bugs.push(fingerprint);
        }

        report
    }

    /// Check if bug has been seen before (deduplication)
    pub fn is_duplicate(&self, fingerprint: &BugFingerprint) -> bool {
        self.discovered_bugs.contains(fingerprint)
    }

    /// Categorize error into bug category
    fn categorize_error(error: &EvalError) -> BugCategory {
        match error {
            EvalError::ValueError(v) => {
                let error_str = format!("{:?}", v);
                if error_str.contains("DivisionByZero") {
                    BugCategory::DivisionByZero
                } else if error_str.contains("TypeMismatch") {
                    BugCategory::TypeMismatch
                } else {
                    BugCategory::Other
                }
            }
            EvalError::UndefinedVariable { .. } | EvalError::UndefinedFunction { .. } => {
                BugCategory::UndefinedIdentifier
            }
            EvalError::StackOverflow => BugCategory::StackOverflow,
            EvalError::NoMatchArm => BugCategory::NoMatchArm,
            EvalError::UnsupportedOperation { .. } => BugCategory::UnsupportedOperation,
            _ => BugCategory::Other,
        }
    }

    /// Extract call stack from error
    fn extract_call_stack(error: &EvalError) -> Vec<String> {
        // Check if error has call stack information
        if let EvalError::WithCallStack { call_stack, .. } = error {
            call_stack.clone()
        } else {
            Vec::new()
        }
    }

    /// Calculate confidence score for interpreter error
    fn calculate_confidence(category: BugCategory, call_stack: &[String]) -> ConfidenceScore {
        // High confidence for clear runtime errors
        let discovery_method = match category {
            BugCategory::DivisionByZero => 0.95, // Very clear error
            BugCategory::IndexOutOfBounds => 0.90, // Clear error
            BugCategory::StackOverflow => 0.85,    // Clear but might be intentional test
            BugCategory::UndefinedIdentifier => 0.90, // Clear error
            BugCategory::TypeMismatch => 0.85,     // Clear but context-dependent
            BugCategory::NoMatchArm => 0.80,       // Might be incomplete code
            BugCategory::UnsupportedOperation => 0.75, // Might be unimplemented feature
            BugCategory::Other => 0.60,            // Unknown error type
        };

        // Reproducibility: 100% if we have the error
        let reproducibility = 1.0;

        // Evidence: Higher if we have call stack
        let evidence = if call_stack.is_empty() { 0.7 } else { 0.95 };

        // Root cause: Clear for most interpreter errors
        let root_cause = match category {
            BugCategory::DivisionByZero | BugCategory::IndexOutOfBounds => 1.0,
            BugCategory::UndefinedIdentifier => 0.95,
            BugCategory::StackOverflow => 0.90,
            BugCategory::TypeMismatch => 0.85,
            _ => 0.75,
        };

        ConfidenceScore::new(discovery_method, reproducibility, evidence, root_cause)
    }

    /// Generate bug signature for deduplication
    fn generate_signature(error: &EvalError) -> String {
        // Create a stable signature based on error type and key details
        match error {
            EvalError::ValueError(v) => format!("ValueError:{:?}", v),
            EvalError::UndefinedVariable { name } => format!("UndefinedVariable:{}", name),
            EvalError::UndefinedFunction { name } => format!("UndefinedFunction:{}", name),
            EvalError::StackOverflow => "StackOverflow".to_string(),
            EvalError::NoMatchArm => "NoMatchArm".to_string(),
            EvalError::UnsupportedOperation { operation } => {
                format!("UnsupportedOperation:{}", operation)
            }
            EvalError::WithCallStack { error, .. } => {
                // Recurse to get signature of underlying error
                Self::generate_signature(error)
            }
            _ => format!("{:?}", error),
        }
    }

    /// Minimize reproduction code using delta debugging
    /// (Placeholder - full delta debugging integration would go here)
    fn minimize_reproduction(_source: &str, _error: &EvalError) -> Option<String> {
        // TODO: Integrate with REPLIC-001 delta debugging
        // For now, return None (no minimization)
        None
    }

    /// Generate GitHub issue content for bug report
    pub fn generate_github_issue(&self, report: &InterpreterBugReport) -> String {
        let priority = report.confidence.priority();
        let recommended_action = report.confidence.recommended_action();

        format!(
            r#"## Bug Report: {} Runtime Error

**Priority**: {:?} (Confidence: {:.2})
**Category**: {}
**Recommended Action**: {}

### Error Details
```
{}
```

### Call Stack
{}

### Confidence Breakdown
- Discovery Method: {:.2}
- Reproducibility: {:.2}
- Evidence Quality: {:.2}
- Root Cause Clarity: {:.2}
- **Overall: {:.2}**

### Minimal Reproduction
{}

### Next Steps
{}

---
🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
"#,
            report.category,
            priority,
            report.confidence.overall,
            report.category,
            recommended_action,
            report.error,
            if report.call_stack.is_empty() {
                "(no call stack available)".to_string()
            } else {
                report
                    .call_stack
                    .iter()
                    .map(|f| format!("- {}", f))
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            report.confidence.discovery_method_weight,
            report.confidence.reproducibility_score,
            report.confidence.quantitative_evidence,
            report.confidence.root_cause_clarity,
            report.confidence.overall,
            report
                .minimal_repro
                .as_ref()
                .map(|code| format!("```ruchy\n{}\n```", code))
                .unwrap_or_else(|| "(minimization not yet implemented)".to_string()),
            recommended_action
        )
    }
}

impl Default for BugDiscoveryAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::value::ValueError;

    #[test]
    fn test_categorize_division_by_zero() {
        let error = EvalError::ValueError(ValueError::DivisionByZero);
        let category = BugDiscoveryAnalyzer::categorize_error(&error);
        assert_eq!(category, BugCategory::DivisionByZero);
    }

    #[test]
    fn test_categorize_undefined_variable() {
        let error = EvalError::UndefinedVariable {
            name: "x".to_string(),
        };
        let category = BugDiscoveryAnalyzer::categorize_error(&error);
        assert_eq!(category, BugCategory::UndefinedIdentifier);
    }

    #[test]
    fn test_confidence_score_high() {
        let category = BugCategory::DivisionByZero;
        let call_stack = vec!["main".to_string()];
        let confidence = BugDiscoveryAnalyzer::calculate_confidence(category, &call_stack);

        // Division by zero should have very high confidence
        assert!(confidence.overall > 0.9);
    }

    #[test]
    fn test_deduplication() {
        let mut analyzer = BugDiscoveryAnalyzer::new();
        let error = EvalError::ValueError(ValueError::DivisionByZero);

        // First analysis - not a duplicate
        let _report1 = analyzer.analyze_error(&error, "let x = 10 / 0;");
        assert_eq!(analyzer.discovered_bugs.len(), 1);

        // Second analysis - should be deduplicated
        let _report2 = analyzer.analyze_error(&error, "let y = 20 / 0;");
        assert_eq!(analyzer.discovered_bugs.len(), 1); // Still 1, not 2
    }
}
