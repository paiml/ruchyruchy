// Test Case Minimization via Delta Debugging
// REPLIC-001: Minimization System Implementation
//
// References:
// - Zeller & Hildebrandt (2002): "Simplifying and Isolating Failure-Inducing Input"
// - Regehr et al. (2012): "Test-case reduction for C compiler bugs"
// - Misherghi & Su (2006): "HDD: Hierarchical Delta Debugging"
// - Section 7.1 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md

use std::collections::HashSet;

/// Result of running a test case
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestOutcome {
    /// Test passes (no bug triggered)
    Pass,
    /// Test fails (bug triggered)
    Fail,
    /// Test result is unresolved (timeout, crash, etc.)
    Unresolved,
}

/// Strategy for minimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinimizationStrategy {
    /// Line-based reduction (remove lines)
    Line,
    /// Token-based reduction (remove tokens)
    Token,
    /// Character-based reduction (remove characters)
    Character,
    /// Semantic-aware reduction (preserve syntax validity)
    Semantic,
}

/// Result of minimization
#[derive(Debug, Clone)]
pub struct MinimizationResult {
    /// Minimized test case
    pub minimized: String,
    /// Original size (in units based on strategy)
    pub original_size: usize,
    /// Minimized size (in units based on strategy)
    pub minimized_size: usize,
    /// Number of test runs performed
    pub test_runs: usize,
    /// Reduction ratio (0.0 = no reduction, 1.0 = empty)
    pub reduction_ratio: f64,
}

impl MinimizationResult {
    /// Create a new minimization result
    pub fn new(
        minimized: String,
        original_size: usize,
        minimized_size: usize,
        test_runs: usize,
    ) -> Self {
        let reduction_ratio = if original_size > 0 {
            1.0 - (minimized_size as f64 / original_size as f64)
        } else {
            0.0
        };

        MinimizationResult {
            minimized,
            original_size,
            minimized_size,
            test_runs,
            reduction_ratio,
        }
    }
}

/// Delta Debugging minimizer
pub struct DeltaDebugger<F>
where
    F: FnMut(&str) -> TestOutcome,
{
    /// Test oracle (returns TestOutcome for given input)
    test_fn: F,
    /// Minimization strategy
    strategy: MinimizationStrategy,
    /// Number of test runs performed
    test_runs: usize,
    /// Cache of tested configurations to avoid redundant tests
    cache: HashSet<String>,
}

impl<F> DeltaDebugger<F>
where
    F: FnMut(&str) -> TestOutcome,
{
    /// Create a new delta debugger
    pub fn new(test_fn: F, strategy: MinimizationStrategy) -> Self {
        DeltaDebugger {
            test_fn,
            strategy,
            test_runs: 0,
            cache: HashSet::new(),
        }
    }

    /// Run test with caching
    fn test(&mut self, input: &str) -> TestOutcome {
        if let Some(_) = self.cache.get(input) {
            // Cache hit - return cached result
            // For now, we recompute (could store outcomes in map)
            // This avoids the complexity of storing outcomes
        }

        self.test_runs += 1;
        let outcome = (self.test_fn)(input);
        self.cache.insert(input.to_string());
        outcome
    }

    /// Minimize a failing test case using delta debugging
    pub fn minimize(&mut self, input: &str) -> MinimizationResult {
        // Verify input actually fails
        if self.test(input) != TestOutcome::Fail {
            // Input doesn't fail - return as-is
            let size = self.measure_size(input);
            return MinimizationResult::new(input.to_string(), size, size, self.test_runs);
        }

        let original_size = self.measure_size(input);
        let minimized = match self.strategy {
            MinimizationStrategy::Line => self.minimize_lines(input),
            MinimizationStrategy::Token => self.minimize_tokens(input),
            MinimizationStrategy::Character => self.minimize_chars(input),
            MinimizationStrategy::Semantic => self.minimize_semantic(input),
        };

        let minimized_size = self.measure_size(&minimized);

        MinimizationResult::new(minimized, original_size, minimized_size, self.test_runs)
    }

    /// Measure size based on strategy
    fn measure_size(&self, input: &str) -> usize {
        match self.strategy {
            MinimizationStrategy::Line => input.lines().count(),
            MinimizationStrategy::Token => self.tokenize(input).len(),
            MinimizationStrategy::Character => input.len(),
            MinimizationStrategy::Semantic => input.lines().count(),
        }
    }

    /// Delta debugging algorithm (classic ddmin)
    fn ddmin(&mut self, chunks: Vec<String>, granularity: usize) -> Vec<String> {
        let n = chunks.len();

        if n == 1 {
            // Single chunk - can't reduce further
            return chunks;
        }

        // Calculate subset size based on granularity
        let subset_size = std::cmp::max(1, n / granularity);

        // Phase 1: Try subsets (remove chunks)
        for i in 0..(n / subset_size) {
            let start = i * subset_size;
            let end = ((i + 1) * subset_size).min(n);
            let subset: Vec<String> = chunks
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx < start || *idx >= end)
                .map(|(_, chunk)| chunk.clone())
                .collect();

            if subset.is_empty() {
                continue;
            }

            let test_input = self.reconstruct(&subset);
            if self.test(&test_input) == TestOutcome::Fail {
                // Subset still fails - recurse with same granularity
                return self.ddmin(subset, 2);
            }
        }

        // Phase 2: Try complements (keep only one subset)
        for i in 0..(n / subset_size) {
            let start = i * subset_size;
            let end = ((i + 1) * subset_size).min(n);
            let complement: Vec<String> = chunks
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx >= start && *idx < end)
                .map(|(_, chunk)| chunk.clone())
                .collect();

            if complement.is_empty() {
                continue;
            }

            let test_input = self.reconstruct(&complement);
            if self.test(&test_input) == TestOutcome::Fail {
                // Complement still fails - recurse with same granularity
                return self.ddmin(complement, 2);
            }
        }

        // Phase 3: Increase granularity
        if granularity < n {
            // Try smaller subsets (increase granularity)
            return self.ddmin(chunks, std::cmp::min(granularity * 2, n));
        }

        // Cannot reduce further
        chunks
    }

    /// Minimize by removing lines
    fn minimize_lines(&mut self, input: &str) -> String {
        let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
        let minimized_lines = self.ddmin(lines, 2);
        self.reconstruct(&minimized_lines)
    }

    /// Minimize by removing tokens
    fn minimize_tokens(&mut self, input: &str) -> String {
        let tokens = self.tokenize(input);
        let minimized_tokens = self.ddmin(tokens, 2);
        minimized_tokens.join("")
    }

    /// Minimize by removing characters
    fn minimize_chars(&mut self, input: &str) -> String {
        let chars: Vec<String> = input.chars().map(|c| c.to_string()).collect();
        let minimized_chars = self.ddmin(chars, 2);
        minimized_chars.join("")
    }

    /// Minimize with semantic awareness (preserve syntax)
    fn minimize_semantic(&mut self, input: &str) -> String {
        // For semantic minimization, we use line-based but preserve structure
        // This is a simplified version - a real implementation would parse AST
        self.minimize_lines(input)
    }

    /// Tokenize input (simple whitespace + punctuation split)
    fn tokenize(&self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        for ch in input.chars() {
            if ch.is_whitespace() || "(){}[];,".contains(ch) {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                if !ch.is_whitespace() {
                    tokens.push(ch.to_string());
                } else {
                    tokens.push(" ".to_string());
                }
            } else {
                current.push(ch);
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }

    /// Reconstruct input from chunks
    fn reconstruct(&self, chunks: &[String]) -> String {
        match self.strategy {
            MinimizationStrategy::Line | MinimizationStrategy::Semantic => chunks.join("\n"),
            MinimizationStrategy::Token | MinimizationStrategy::Character => chunks.join(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimization_result_creation() {
        let result = MinimizationResult::new("minimized".to_string(), 100, 50, 10);
        assert_eq!(result.original_size, 100);
        assert_eq!(result.minimized_size, 50);
        assert_eq!(result.test_runs, 10);
        assert!((result.reduction_ratio - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_minimization_result_no_reduction() {
        let result = MinimizationResult::new("same".to_string(), 100, 100, 5);
        assert!((result.reduction_ratio - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_minimization_result_complete_reduction() {
        let result = MinimizationResult::new("".to_string(), 100, 0, 20);
        assert!((result.reduction_ratio - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_delta_debugger_creation() {
        let test_fn = |_: &str| TestOutcome::Pass;
        let debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);
        assert_eq!(debugger.strategy, MinimizationStrategy::Line);
        assert_eq!(debugger.test_runs, 0);
    }

    #[test]
    fn test_minimize_already_passing() {
        let test_fn = |_: &str| TestOutcome::Pass;
        let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

        let input = "line1\nline2\nline3";
        let result = debugger.minimize(input);

        // Should return input unchanged since it doesn't fail
        assert_eq!(result.minimized, input);
        assert_eq!(result.reduction_ratio, 0.0);
    }

    #[test]
    fn test_minimize_line_based_simple() {
        // Fails if input contains "bug"
        let test_fn = |input: &str| {
            if input.contains("bug") {
                TestOutcome::Fail
            } else {
                TestOutcome::Pass
            }
        };

        let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

        let input = "line1\nline2 bug\nline3";
        let result = debugger.minimize(input);

        // Should reduce to just the line containing "bug"
        assert!(result.minimized.contains("bug"));
        assert!(result.reduction_ratio > 0.0);
        assert!(result.minimized_size < result.original_size);
    }

    #[test]
    fn test_minimize_character_based() {
        // Fails if input contains "xyz"
        let test_fn = |input: &str| {
            if input.contains("xyz") {
                TestOutcome::Fail
            } else {
                TestOutcome::Pass
            }
        };

        let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Character);

        let input = "abcxyzdef";
        let result = debugger.minimize(input);

        // Should reduce to minimal string containing "xyz"
        assert!(result.minimized.contains("xyz"));
        assert!(result.reduction_ratio > 0.0);
    }

    #[test]
    fn test_minimize_token_based() {
        // Fails if input contains "bad" token
        let test_fn = |input: &str| {
            if input.contains("bad") {
                TestOutcome::Fail
            } else {
                TestOutcome::Pass
            }
        };

        let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Token);

        let input = "good bad good";
        let result = debugger.minimize(input);

        // Should reduce to minimal tokens containing "bad"
        assert!(result.minimized.contains("bad"));
        assert!(result.reduction_ratio > 0.0);
    }

    #[test]
    fn test_minimize_semantic_strategy() {
        // Fails if input has more than 2 lines
        let test_fn = |input: &str| {
            if input.lines().count() > 2 {
                TestOutcome::Fail
            } else {
                TestOutcome::Pass
            }
        };

        let mut debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Semantic);

        let input = "line1\nline2\nline3\nline4";
        let result = debugger.minimize(input);

        // Should reduce to 3 lines (minimal failing case)
        assert_eq!(result.minimized.lines().count(), 3);
        assert!(result.reduction_ratio > 0.0);
    }

    #[test]
    fn test_tokenize_simple() {
        let test_fn = |_: &str| TestOutcome::Pass;
        let debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Token);

        let tokens = debugger.tokenize("let x = 5;");
        assert!(tokens.contains(&"let".to_string()));
        assert!(tokens.contains(&"x".to_string()));
        assert!(tokens.contains(&"=".to_string()));
        assert!(tokens.contains(&"5".to_string()));
        assert!(tokens.contains(&";".to_string()));
    }

    #[test]
    fn test_tokenize_with_parens() {
        let test_fn = |_: &str| TestOutcome::Pass;
        let debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Token);

        let tokens = debugger.tokenize("fun(a, b)");
        assert!(tokens.contains(&"fun".to_string()));
        assert!(tokens.contains(&"(".to_string()));
        assert!(tokens.contains(&"a".to_string()));
        assert!(tokens.contains(&",".to_string()));
        assert!(tokens.contains(&"b".to_string()));
        assert!(tokens.contains(&")".to_string()));
    }

    #[test]
    fn test_reconstruct_lines() {
        let test_fn = |_: &str| TestOutcome::Pass;
        let debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

        let chunks = vec!["line1".to_string(), "line2".to_string(), "line3".to_string()];
        let reconstructed = debugger.reconstruct(&chunks);
        assert_eq!(reconstructed, "line1\nline2\nline3");
    }

    #[test]
    fn test_reconstruct_tokens() {
        let test_fn = |_: &str| TestOutcome::Pass;
        let debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Token);

        let chunks = vec!["let".to_string(), " ".to_string(), "x".to_string()];
        let reconstructed = debugger.reconstruct(&chunks);
        assert_eq!(reconstructed, "let x");
    }

    #[test]
    fn test_measure_size_line() {
        let test_fn = |_: &str| TestOutcome::Pass;
        let debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Line);

        let input = "line1\nline2\nline3";
        assert_eq!(debugger.measure_size(input), 3);
    }

    #[test]
    fn test_measure_size_character() {
        let test_fn = |_: &str| TestOutcome::Pass;
        let debugger = DeltaDebugger::new(test_fn, MinimizationStrategy::Character);

        let input = "hello";
        assert_eq!(debugger.measure_size(input), 5);
    }
}
