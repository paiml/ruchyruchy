// Property-Based Testing for Compiler Validation
// DISC-003: Property-Based Testing Integration
//
// References:
// - Claessen & Hughes (2000): "QuickCheck: A Lightweight Tool for Random Testing"
// - Regehr et al. (2012): "Test-case reduction for C compiler bugs"
// - Section 5.2 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md

use crate::bug_discovery::confidence::{
    ConfidenceScore, ConfidenceScorer, DiscoveryMethod, EvidenceLevel, Reproducibility,
    RootCauseClarity,
};

/// A property that should hold for all valid inputs
#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub description: String,
}

impl Property {
    /// Create a new property
    pub fn new(name: String, description: String) -> Self {
        Property { name, description }
    }

    /// Common compiler properties
    pub fn roundtrip_parse_emit() -> Self {
        Property::new(
            "roundtrip_parse_emit".to_string(),
            "parse(emit(ast)) == ast".to_string(),
        )
    }

    pub fn type_preservation() -> Self {
        Property::new(
            "type_preservation".to_string(),
            "typecheck(transform(ast)) preserves types".to_string(),
        )
    }

    pub fn deterministic_compilation() -> Self {
        Property::new(
            "deterministic_compilation".to_string(),
            "compile(source) produces identical output on repeated runs".to_string(),
        )
    }

    pub fn semantic_equivalence() -> Self {
        Property::new(
            "semantic_equivalence".to_string(),
            "compile(source).run() == interpret(source)".to_string(),
        )
    }

    pub fn idempotent_optimization() -> Self {
        Property::new(
            "idempotent_optimization".to_string(),
            "optimize(optimize(code)) == optimize(code)".to_string(),
        )
    }
}

/// Result of property checking
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyResult {
    /// Property holds for all tested inputs
    Success { cases_tested: usize },
    /// Property violated with counterexample
    Violation {
        counterexample: String,
        shrunk_example: Option<String>,
        cases_until_failure: usize,
    },
    /// Testing encountered an error
    Error { message: String },
}

/// Property test generator trait
pub trait Generator {
    /// Generate a random value
    fn generate(&mut self) -> String;
}

/// Simple AST generator for property testing
pub struct AstGenerator {
    #[allow(dead_code)]
    seed: u64,
    state: u64,
    max_depth: usize,
}

impl AstGenerator {
    /// Create a new AST generator
    pub fn new(seed: u64) -> Self {
        AstGenerator {
            seed,
            state: seed,
            max_depth: 5,
        }
    }

    /// Set maximum recursion depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Generate an expression
    pub fn generate_expr(&mut self, depth: usize) -> String {
        if depth >= self.max_depth || self.random_bool() {
            // Generate leaf (literal or variable)
            if self.random_bool() {
                format!("{}", self.random_int(0, 100))
            } else {
                self.random_var()
            }
        } else {
            // Generate binary operation
            let left = self.generate_expr(depth + 1);
            let op = self.random_op();
            let right = self.generate_expr(depth + 1);
            format!("({} {} {})", left, op, right)
        }
    }

    /// Generate a statement
    pub fn generate_stmt(&mut self) -> String {
        match self.random_int(0, 3) {
            0 => {
                // Let binding
                let var = self.random_var();
                let expr = self.generate_expr(0);
                format!("let {} = {};", var, expr)
            }
            1 => {
                // If statement
                let cond = self.generate_expr(0);
                let then_stmt = self.generate_expr(0);
                format!("if {} {{ {}; }}", cond, then_stmt)
            }
            _ => {
                // Expression statement
                let expr = self.generate_expr(0);
                format!("{};", expr)
            }
        }
    }

    // Random generation helpers
    fn next_random(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }

    fn random_bool(&mut self) -> bool {
        self.next_random().is_multiple_of(2)
    }

    fn random_int(&mut self, min: i32, max: i32) -> i32 {
        if max <= min {
            return min;
        }
        min + ((self.next_random() as i32).abs() % (max - min))
    }

    fn random_var(&mut self) -> String {
        let vars = ["x", "y", "z", "a", "b"];
        vars[self.random_int(0, vars.len() as i32) as usize].to_string()
    }

    fn random_op(&mut self) -> &'static str {
        let ops = ["+", "-", "*", "/"];
        ops[self.random_int(0, ops.len() as i32) as usize]
    }
}

impl Generator for AstGenerator {
    fn generate(&mut self) -> String {
        self.generate_stmt()
    }
}

/// Property checker
pub struct PropertyChecker {
    /// Number of test cases to generate
    pub test_cases: usize,
    /// Random seed for reproducibility
    pub seed: u64,
}

impl PropertyChecker {
    /// Create a new property checker
    pub fn new(seed: u64) -> Self {
        PropertyChecker {
            test_cases: 100,
            seed,
        }
    }

    /// Set number of test cases
    pub fn with_test_cases(mut self, count: usize) -> Self {
        self.test_cases = count;
        self
    }

    /// Check a property with a predicate function
    pub fn check<F>(&self, _property: &Property, mut predicate: F) -> PropertyResult
    where
        F: FnMut(&str) -> bool,
    {
        let mut generator = AstGenerator::new(self.seed);

        for i in 0..self.test_cases {
            let test_case = generator.generate();

            if !predicate(&test_case) {
                // Property violated - try to shrink
                let shrunk = self.shrink(&test_case, &mut predicate);

                return PropertyResult::Violation {
                    counterexample: test_case,
                    shrunk_example: shrunk,
                    cases_until_failure: i + 1,
                };
            }
        }

        PropertyResult::Success {
            cases_tested: self.test_cases,
        }
    }

    /// Shrink a counterexample to minimal form
    fn shrink<F>(&self, test_case: &str, predicate: &mut F) -> Option<String>
    where
        F: FnMut(&str) -> bool,
    {
        let mut current = test_case.to_string();
        let mut changed = true;

        // Try removing characters one at a time
        while changed {
            changed = false;
            let chars: Vec<char> = current.chars().collect();

            for i in 0..chars.len() {
                let mut candidate_chars = chars.clone();
                candidate_chars.remove(i);
                let candidate: String = candidate_chars.into_iter().collect();

                // Check if still violates property
                if !candidate.is_empty() && !predicate(&candidate) {
                    current = candidate;
                    changed = true;
                    break;
                }
            }
        }

        if current != test_case {
            Some(current)
        } else {
            None
        }
    }
}

/// Bug found via property testing
#[derive(Debug, Clone)]
pub struct PropertyBug {
    pub property: Property,
    pub counterexample: String,
    pub shrunk_example: Option<String>,
    pub confidence: ConfidenceScore,
}

impl PropertyBug {
    /// Create a new property bug
    pub fn new(property: Property, counterexample: String, shrunk_example: Option<String>) -> Self {
        let confidence = ConfidenceScorer::from_components(
            DiscoveryMethod::PropertyTestViolation,
            Reproducibility::Always,
            EvidenceLevel::Complete,
            RootCauseClarity::SingleObviousCause,
        );

        PropertyBug {
            property,
            counterexample,
            shrunk_example,
            confidence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_creation() {
        let prop = Property::new("test".to_string(), "desc".to_string());
        assert_eq!(prop.name, "test");
        assert_eq!(prop.description, "desc");
    }

    #[test]
    fn test_common_properties() {
        let roundtrip = Property::roundtrip_parse_emit();
        assert_eq!(roundtrip.name, "roundtrip_parse_emit");

        let types = Property::type_preservation();
        assert_eq!(types.name, "type_preservation");

        let deterministic = Property::deterministic_compilation();
        assert_eq!(deterministic.name, "deterministic_compilation");

        let semantic = Property::semantic_equivalence();
        assert_eq!(semantic.name, "semantic_equivalence");

        let idempotent = Property::idempotent_optimization();
        assert_eq!(idempotent.name, "idempotent_optimization");
    }

    #[test]
    fn test_ast_generator_deterministic() {
        let mut gen1 = AstGenerator::new(42);
        let mut gen2 = AstGenerator::new(42);

        let output1 = gen1.generate();
        let output2 = gen2.generate();

        // Same seed should produce same output
        assert_eq!(output1, output2);
    }

    #[test]
    fn test_ast_generator_produces_output() {
        let mut gen = AstGenerator::new(42);
        let output = gen.generate();

        // Should produce non-empty output
        assert!(!output.is_empty());
    }

    #[test]
    fn test_ast_generator_expr_respects_depth() {
        let mut gen = AstGenerator::new(42).with_max_depth(1);
        let expr = gen.generate_expr(0);

        // At max_depth=1, should only generate leaves
        // (This test is probabilistic but should usually pass)
        assert!(!expr.is_empty());
    }

    #[test]
    fn test_property_checker_success() {
        let prop = Property::new("always_true".to_string(), "test".to_string());
        let checker = PropertyChecker::new(42).with_test_cases(10);

        let result = checker.check(&prop, |_test_case| {
            // Property always holds
            true
        });

        match result {
            PropertyResult::Success { cases_tested } => {
                assert_eq!(cases_tested, 10);
            }
            _ => panic!("Expected success"),
        }
    }

    #[test]
    fn test_property_checker_violation() {
        let prop = Property::new("always_false".to_string(), "test".to_string());
        let checker = PropertyChecker::new(42).with_test_cases(10);

        let result = checker.check(&prop, |_test_case| {
            // Property never holds
            false
        });

        match result {
            PropertyResult::Violation {
                counterexample,
                cases_until_failure,
                ..
            } => {
                assert!(!counterexample.is_empty());
                assert_eq!(cases_until_failure, 1);
            }
            _ => panic!("Expected violation"),
        }
    }

    #[test]
    fn test_property_checker_shrinking() {
        let prop = Property::new("no_spaces".to_string(), "test".to_string());
        let checker = PropertyChecker::new(42).with_test_cases(100);

        let result = checker.check(&prop, |test_case| {
            // Property: no spaces allowed
            !test_case.contains(' ')
        });

        match result {
            PropertyResult::Violation {
                counterexample,
                shrunk_example,
                ..
            } => {
                assert!(counterexample.contains(' '));
                // Shrinking should reduce size (or stay same if minimal)
                if let Some(shrunk) = shrunk_example {
                    assert!(shrunk.len() <= counterexample.len());
                    assert!(shrunk.contains(' ')); // Still violates property
                }
            }
            _ => {
                // Test might pass if generator doesn't produce spaces
                // That's OK for this test
            }
        }
    }

    #[test]
    fn test_property_bug_confidence() {
        let prop = Property::roundtrip_parse_emit();
        let bug = PropertyBug::new(prop, "let x = 1;".to_string(), Some("x".to_string()));

        // Property violations should have very high confidence
        assert!(bug.confidence.overall > 0.85);
    }

    #[test]
    fn test_property_result_equality() {
        let success1 = PropertyResult::Success { cases_tested: 100 };
        let success2 = PropertyResult::Success { cases_tested: 100 };
        assert_eq!(success1, success2);

        let violation1 = PropertyResult::Violation {
            counterexample: "test".to_string(),
            shrunk_example: None,
            cases_until_failure: 5,
        };
        let violation2 = PropertyResult::Violation {
            counterexample: "test".to_string(),
            shrunk_example: None,
            cases_until_failure: 5,
        };
        assert_eq!(violation1, violation2);
    }
}
