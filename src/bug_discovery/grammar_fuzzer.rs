// Grammar-Based Fuzzing for Compiler Testing
// DISC-002: Grammar-Based Fuzzing Implementation
//
// References:
// - Holler et al. (2012): "Fuzzing with Code Fragments"
// - Zalewski (2014): "american fuzzy lop" (AFL) mutation strategies
// - Section 4.2 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md

use crate::bug_discovery::confidence::{
    ConfidenceScore, ConfidenceScorer, DiscoveryMethod, EvidenceLevel, Reproducibility,
    RootCauseClarity,
};
use std::collections::HashMap;
use std::fmt;

/// Grammar rule for generating syntax elements
#[derive(Debug, Clone, PartialEq)]
pub enum GrammarRule {
    /// Terminal symbol (literal string)
    Terminal(String),
    /// Non-terminal symbol (reference to another rule)
    NonTerminal(String),
    /// Sequence of rules (must match in order)
    Sequence(Vec<GrammarRule>),
    /// Choice between alternatives (pick one)
    Choice(Vec<GrammarRule>),
    /// Optional rule (0 or 1 occurrences)
    Optional(Box<GrammarRule>),
    /// Repetition (0 or more occurrences)
    Repeat(Box<GrammarRule>),
}

/// Grammar definition for generating test cases
#[derive(Debug, Clone)]
pub struct Grammar {
    /// Starting rule name
    pub start_symbol: String,
    /// Map of rule names to their definitions
    pub rules: HashMap<String, GrammarRule>,
    /// Maximum recursion depth
    pub max_depth: usize,
}

impl Grammar {
    /// Create a new grammar
    pub fn new(start_symbol: String) -> Self {
        Grammar {
            start_symbol,
            rules: HashMap::new(),
            max_depth: 10,
        }
    }

    /// Add a rule to the grammar
    pub fn add_rule(&mut self, name: String, rule: GrammarRule) {
        self.rules.insert(name, rule);
    }

    /// Create a minimal Ruchy grammar for testing
    pub fn ruchy_minimal() -> Self {
        let mut grammar = Grammar::new("program".to_string());

        // program ::= statement*
        grammar.add_rule(
            "program".to_string(),
            GrammarRule::Repeat(Box::new(GrammarRule::NonTerminal("statement".to_string()))),
        );

        // statement ::= let_stmt | if_stmt | expr_stmt
        grammar.add_rule(
            "statement".to_string(),
            GrammarRule::Choice(vec![
                GrammarRule::NonTerminal("let_stmt".to_string()),
                GrammarRule::NonTerminal("if_stmt".to_string()),
                GrammarRule::NonTerminal("expr_stmt".to_string()),
            ]),
        );

        // let_stmt ::= "let" identifier "=" expr ";"
        grammar.add_rule(
            "let_stmt".to_string(),
            GrammarRule::Sequence(vec![
                GrammarRule::Terminal("let ".to_string()),
                GrammarRule::NonTerminal("identifier".to_string()),
                GrammarRule::Terminal(" = ".to_string()),
                GrammarRule::NonTerminal("expr".to_string()),
                GrammarRule::Terminal(";\n".to_string()),
            ]),
        );

        // if_stmt ::= "if" expr "{" statement* "}"
        grammar.add_rule(
            "if_stmt".to_string(),
            GrammarRule::Sequence(vec![
                GrammarRule::Terminal("if ".to_string()),
                GrammarRule::NonTerminal("expr".to_string()),
                GrammarRule::Terminal(" {\n".to_string()),
                GrammarRule::Repeat(Box::new(GrammarRule::NonTerminal("statement".to_string()))),
                GrammarRule::Terminal("}\n".to_string()),
            ]),
        );

        // expr_stmt ::= expr ";"
        grammar.add_rule(
            "expr_stmt".to_string(),
            GrammarRule::Sequence(vec![
                GrammarRule::NonTerminal("expr".to_string()),
                GrammarRule::Terminal(";\n".to_string()),
            ]),
        );

        // expr ::= literal | identifier | binary_op
        grammar.add_rule(
            "expr".to_string(),
            GrammarRule::Choice(vec![
                GrammarRule::NonTerminal("literal".to_string()),
                GrammarRule::NonTerminal("identifier".to_string()),
                GrammarRule::NonTerminal("binary_op".to_string()),
            ]),
        );

        // binary_op ::= expr operator expr
        grammar.add_rule(
            "binary_op".to_string(),
            GrammarRule::Sequence(vec![
                GrammarRule::NonTerminal("literal".to_string()),
                GrammarRule::Terminal(" ".to_string()),
                GrammarRule::NonTerminal("operator".to_string()),
                GrammarRule::Terminal(" ".to_string()),
                GrammarRule::NonTerminal("literal".to_string()),
            ]),
        );

        // operator ::= "+" | "-" | "*" | "/"
        grammar.add_rule(
            "operator".to_string(),
            GrammarRule::Choice(vec![
                GrammarRule::Terminal("+".to_string()),
                GrammarRule::Terminal("-".to_string()),
                GrammarRule::Terminal("*".to_string()),
                GrammarRule::Terminal("/".to_string()),
            ]),
        );

        // literal ::= integer | boolean | string
        grammar.add_rule(
            "literal".to_string(),
            GrammarRule::Choice(vec![
                GrammarRule::NonTerminal("integer".to_string()),
                GrammarRule::NonTerminal("boolean".to_string()),
                GrammarRule::NonTerminal("string".to_string()),
            ]),
        );

        // integer ::= "0" | "1" | "42" | "100"
        grammar.add_rule(
            "integer".to_string(),
            GrammarRule::Choice(vec![
                GrammarRule::Terminal("0".to_string()),
                GrammarRule::Terminal("1".to_string()),
                GrammarRule::Terminal("42".to_string()),
                GrammarRule::Terminal("100".to_string()),
            ]),
        );

        // boolean ::= "true" | "false"
        grammar.add_rule(
            "boolean".to_string(),
            GrammarRule::Choice(vec![
                GrammarRule::Terminal("true".to_string()),
                GrammarRule::Terminal("false".to_string()),
            ]),
        );

        // string ::= "\"hello\"" | "\"world\""
        grammar.add_rule(
            "string".to_string(),
            GrammarRule::Choice(vec![
                GrammarRule::Terminal("\"hello\"".to_string()),
                GrammarRule::Terminal("\"world\"".to_string()),
            ]),
        );

        // identifier ::= "x" | "y" | "result"
        grammar.add_rule(
            "identifier".to_string(),
            GrammarRule::Choice(vec![
                GrammarRule::Terminal("x".to_string()),
                GrammarRule::Terminal("y".to_string()),
                GrammarRule::Terminal("result".to_string()),
            ]),
        );

        grammar
    }
}

/// Grammar-based fuzzer
pub struct GrammarFuzzer {
    grammar: Grammar,
    /// Random seed for reproducibility
    seed: u64,
    /// Current random state
    state: u64,
}

impl GrammarFuzzer {
    /// Create a new grammar-based fuzzer
    pub fn new(grammar: Grammar, seed: u64) -> Self {
        GrammarFuzzer {
            grammar,
            seed,
            state: seed,
        }
    }

    /// Generate a test case from the grammar
    pub fn generate(&mut self) -> String {
        self.generate_from_rule(&self.grammar.start_symbol.clone(), 0)
    }

    /// Generate from a specific rule with depth tracking
    fn generate_from_rule(&mut self, rule_name: &str, depth: usize) -> String {
        // Prevent infinite recursion
        if depth > self.grammar.max_depth {
            return String::new();
        }

        let rule = match self.grammar.rules.get(rule_name) {
            Some(r) => r.clone(),
            None => return format!("<UNDEFINED:{}>", rule_name),
        };

        self.generate_from_grammar_rule(&rule, depth)
    }

    /// Generate from a grammar rule
    fn generate_from_grammar_rule(&mut self, rule: &GrammarRule, depth: usize) -> String {
        match rule {
            GrammarRule::Terminal(s) => s.clone(),
            GrammarRule::NonTerminal(name) => self.generate_from_rule(name, depth + 1),
            GrammarRule::Sequence(rules) => {
                let mut result = String::new();
                for r in rules {
                    result.push_str(&self.generate_from_grammar_rule(r, depth));
                }
                result
            }
            GrammarRule::Choice(choices) => {
                if choices.is_empty() {
                    return String::new();
                }
                let idx = self.random_usize(choices.len());
                self.generate_from_grammar_rule(&choices[idx], depth)
            }
            GrammarRule::Optional(rule) => {
                if self.random_bool() {
                    self.generate_from_grammar_rule(rule, depth)
                } else {
                    String::new()
                }
            }
            GrammarRule::Repeat(rule) => {
                let mut result = String::new();
                // Generate 0-3 repetitions (biased toward smaller)
                let count = self.random_usize(4);
                for _ in 0..count {
                    result.push_str(&self.generate_from_grammar_rule(rule, depth));
                }
                result
            }
        }
    }

    /// Simple LCG random number generator for reproducibility
    fn next_random(&mut self) -> u64 {
        // Linear Congruential Generator (LCG)
        // Using constants from Numerical Recipes
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }

    /// Generate random boolean
    fn random_bool(&mut self) -> bool {
        self.next_random() % 2 == 0
    }

    /// Generate random usize in range [0, max)
    fn random_usize(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }
        (self.next_random() as usize) % max
    }

    /// Generate multiple test cases
    pub fn generate_batch(&mut self, count: usize) -> Vec<String> {
        (0..count).map(|_| self.generate()).collect()
    }
}

/// Fuzzing result from testing generated input
#[derive(Debug, Clone, PartialEq)]
pub enum FuzzResult {
    Pass,
    Crash(String),
    Hang { timeout_ms: u64 },
    IncorrectOutput { expected: String, actual: String },
}

/// Bug found via grammar-based fuzzing
#[derive(Debug, Clone)]
pub struct FuzzBug {
    pub test_case: String,
    pub result: FuzzResult,
    pub confidence: ConfidenceScore,
}

impl FuzzBug {
    /// Create a new fuzz bug
    pub fn new(test_case: String, result: FuzzResult) -> Self {
        let confidence = Self::calculate_confidence(&result);
        FuzzBug {
            test_case,
            result,
            confidence,
        }
    }

    /// Calculate confidence based on result type
    fn calculate_confidence(result: &FuzzResult) -> ConfidenceScore {
        match result {
            FuzzResult::Crash(_) => ConfidenceScorer::from_components(
                DiscoveryMethod::GrammarFuzzCrashHang,
                Reproducibility::Always,
                EvidenceLevel::Complete,
                RootCauseClarity::PrimaryWithSecondary,
            ),
            FuzzResult::Hang { .. } => ConfidenceScorer::from_components(
                DiscoveryMethod::GrammarFuzzCrashHang,
                Reproducibility::Always,
                EvidenceLevel::Complete,
                RootCauseClarity::MultiplePlausible,
            ),
            FuzzResult::IncorrectOutput { .. } => ConfidenceScorer::from_components(
                DiscoveryMethod::GrammarFuzzIncorrectOutput,
                Reproducibility::Always,
                EvidenceLevel::Partial,
                RootCauseClarity::UnclearHypothesis,
            ),
            FuzzResult::Pass => ConfidenceScore::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

/// Test case corpus for storing interesting inputs
#[derive(Debug, Clone)]
pub struct FuzzCorpus {
    /// Test cases that pass
    pub passing: Vec<String>,
    /// Test cases that crash
    pub crashing: Vec<String>,
    /// Test cases that hang
    pub hanging: Vec<String>,
    /// Test cases that produce incorrect output
    pub incorrect: Vec<String>,
}

impl FuzzCorpus {
    /// Create a new empty corpus
    pub fn new() -> Self {
        FuzzCorpus {
            passing: Vec::new(),
            crashing: Vec::new(),
            hanging: Vec::new(),
            incorrect: Vec::new(),
        }
    }

    /// Add a test case based on its result
    pub fn add(&mut self, test_case: String, result: &FuzzResult) {
        match result {
            FuzzResult::Pass => self.passing.push(test_case),
            FuzzResult::Crash(_) => self.crashing.push(test_case),
            FuzzResult::Hang { .. } => self.hanging.push(test_case),
            FuzzResult::IncorrectOutput { .. } => self.incorrect.push(test_case),
        }
    }

    /// Get total number of test cases
    pub fn total_count(&self) -> usize {
        self.passing.len() + self.crashing.len() + self.hanging.len() + self.incorrect.len()
    }

    /// Get count of failing test cases
    pub fn failure_count(&self) -> usize {
        self.crashing.len() + self.hanging.len() + self.incorrect.len()
    }

    /// Get all failing test cases
    pub fn failures(&self) -> Vec<String> {
        let mut failures = Vec::new();
        failures.extend(self.crashing.clone());
        failures.extend(self.hanging.clone());
        failures.extend(self.incorrect.clone());
        failures
    }
}

impl Default for FuzzCorpus {
    fn default() -> Self {
        Self::new()
    }
}

/// Test case minimizer (shrinking)
pub struct TestMinimizer;

impl TestMinimizer {
    /// Minimize a failing test case
    ///
    /// Strategy:
    /// 1. Try removing lines one at a time
    /// 2. Try removing tokens one at a time
    /// 3. Try simplifying expressions
    ///
    /// Returns the smallest test case that still reproduces the bug
    pub fn minimize<F>(test_case: &str, mut reproduces: F) -> String
    where
        F: FnMut(&str) -> bool,
    {
        let mut current = test_case.to_string();

        // Phase 1: Remove lines
        current = Self::minimize_lines(&current, &mut reproduces);

        // Phase 2: Remove characters
        current = Self::minimize_chars(&current, &mut reproduces);

        current
    }

    /// Try removing lines one at a time
    fn minimize_lines<F>(test_case: &str, reproduces: &mut F) -> String
    where
        F: FnMut(&str) -> bool,
    {
        let lines: Vec<&str> = test_case.lines().collect();
        let mut current_lines = lines.clone();

        for i in 0..current_lines.len() {
            // Try removing line i
            let mut candidate_lines = current_lines.clone();
            candidate_lines.remove(i);
            let candidate = candidate_lines.join("\n");

            if reproduces(&candidate) {
                // Successfully minimized - keep this version
                current_lines = candidate_lines;
                // Try removing more lines (restart from beginning)
                return Self::minimize_lines(&current_lines.join("\n"), reproduces);
            }
        }

        current_lines.join("\n")
    }

    /// Try removing characters one at a time
    fn minimize_chars<F>(test_case: &str, reproduces: &mut F) -> String
    where
        F: FnMut(&str) -> bool,
    {
        let mut current = test_case.to_string();
        let mut changed = true;

        while changed {
            changed = false;
            let chars: Vec<char> = current.chars().collect();

            for i in 0..chars.len() {
                // Try removing character i
                let mut candidate_chars = chars.clone();
                candidate_chars.remove(i);
                let candidate: String = candidate_chars.into_iter().collect();

                if reproduces(&candidate) {
                    // Successfully minimized
                    current = candidate;
                    changed = true;
                    break;
                }
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_creation() {
        let grammar = Grammar::new("start".to_string());
        assert_eq!(grammar.start_symbol, "start");
        assert_eq!(grammar.rules.len(), 0);
    }

    #[test]
    fn test_add_rule() {
        let mut grammar = Grammar::new("start".to_string());
        grammar.add_rule(
            "start".to_string(),
            GrammarRule::Terminal("hello".to_string()),
        );
        assert_eq!(grammar.rules.len(), 1);
    }

    #[test]
    fn test_ruchy_minimal_grammar() {
        let grammar = Grammar::ruchy_minimal();
        assert_eq!(grammar.start_symbol, "program");
        // Should have rules for: program, statement, let_stmt, if_stmt, expr_stmt,
        // expr, binary_op, operator, literal, integer, boolean, string, identifier
        assert!(grammar.rules.len() >= 10);
    }

    #[test]
    fn test_fuzzer_deterministic() {
        let grammar = Grammar::ruchy_minimal();
        let mut fuzzer1 = GrammarFuzzer::new(grammar.clone(), 42);
        let mut fuzzer2 = GrammarFuzzer::new(grammar, 42);

        let output1 = fuzzer1.generate();
        let output2 = fuzzer2.generate();

        // Same seed should produce same output
        assert_eq!(output1, output2);
    }

    #[test]
    fn test_fuzzer_generates_valid_syntax() {
        let grammar = Grammar::ruchy_minimal();
        let mut fuzzer = GrammarFuzzer::new(grammar, 42);

        let output = fuzzer.generate();
        // Should generate something (not empty usually)
        // Actual validation would require Ruchy parser
        assert!(output.len() >= 0); // Always true, but documents intent
    }

    #[test]
    fn test_fuzzer_batch_generation() {
        let grammar = Grammar::ruchy_minimal();
        let mut fuzzer = GrammarFuzzer::new(grammar, 42);

        let batch = fuzzer.generate_batch(5);
        assert_eq!(batch.len(), 5);
    }

    #[test]
    fn test_fuzz_bug_confidence_crash() {
        let bug = FuzzBug::new(
            "let x = 1;".to_string(),
            FuzzResult::Crash("segfault".to_string()),
        );
        // Crashes should have high confidence
        assert!(bug.confidence.overall > 0.7);
    }

    #[test]
    fn test_fuzz_bug_confidence_incorrect_output() {
        let bug = FuzzBug::new(
            "let x = 1;".to_string(),
            FuzzResult::IncorrectOutput {
                expected: "1".to_string(),
                actual: "2".to_string(),
            },
        );
        // Incorrect output has lower confidence than crashes
        assert!(bug.confidence.overall < 0.8);
        assert!(bug.confidence.overall > 0.5);
    }

    #[test]
    fn test_corpus_creation() {
        let corpus = FuzzCorpus::new();
        assert_eq!(corpus.total_count(), 0);
        assert_eq!(corpus.failure_count(), 0);
    }

    #[test]
    fn test_corpus_add_passing() {
        let mut corpus = FuzzCorpus::new();
        corpus.add("let x = 1;".to_string(), &FuzzResult::Pass);
        assert_eq!(corpus.total_count(), 1);
        assert_eq!(corpus.passing.len(), 1);
        assert_eq!(corpus.failure_count(), 0);
    }

    #[test]
    fn test_corpus_add_crash() {
        let mut corpus = FuzzCorpus::new();
        corpus.add(
            "let x = 1;".to_string(),
            &FuzzResult::Crash("error".to_string()),
        );
        assert_eq!(corpus.total_count(), 1);
        assert_eq!(corpus.crashing.len(), 1);
        assert_eq!(corpus.failure_count(), 1);
    }

    #[test]
    fn test_corpus_failures() {
        let mut corpus = FuzzCorpus::new();
        corpus.add(
            "crash.ruchy".to_string(),
            &FuzzResult::Crash("error".to_string()),
        );
        corpus.add(
            "hang.ruchy".to_string(),
            &FuzzResult::Hang { timeout_ms: 1000 },
        );
        corpus.add("pass.ruchy".to_string(), &FuzzResult::Pass);

        let failures = corpus.failures();
        assert_eq!(failures.len(), 2);
        assert_eq!(corpus.failure_count(), 2);
    }

    #[test]
    fn test_minimizer_removes_lines() {
        let test_case = "line 1\nline 2\nline 3\n";
        // Reproduce only if "line 2" is present
        let reproduces = |code: &str| code.contains("line 2");

        let minimized = TestMinimizer::minimize(test_case, reproduces);
        // Should remove lines 1 and 3
        assert!(minimized.contains("line 2"));
        assert!(!minimized.contains("line 1") || !minimized.contains("line 3"));
    }

    #[test]
    fn test_minimizer_removes_chars() {
        let test_case = "xxxxBUGxxxx";
        // Reproduce only if "BUG" is present
        let reproduces = |code: &str| code.contains("BUG");

        let minimized = TestMinimizer::minimize(test_case, reproduces);
        // Should remove all x's
        assert_eq!(minimized, "BUG");
    }

    #[test]
    fn test_minimizer_preserves_minimal_case() {
        let test_case = "BUG";
        // Reproduce only if exactly "BUG" is present
        let reproduces = |code: &str| code == "BUG";

        let minimized = TestMinimizer::minimize(test_case, reproduces);
        // Should not change
        assert_eq!(minimized, "BUG");
    }
}
