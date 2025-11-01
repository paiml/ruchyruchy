// DEBUGGER-042: Pathological Input Detector
//
// Detects inputs that cause extreme performance degradation (>10x slowdown vs expected).
// Complements fuzzing (which finds crashes) with performance cliff detection.

#![allow(missing_docs)] // Comprehensive inline documentation provided

use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::parser::Parser;
use std::collections::HashMap;
use std::time::Instant;

/// Category of pathological input
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)] // All stress categories - intentional naming
pub enum PathologicalCategory {
    ParserStress,    // Deeply nested expressions, long identifier chains
    EvaluatorStress, // Quadratic variable lookup, deep call stacks
    MemoryStress,    // Allocation bombs, large data structures
}

/// Pathological input detection result
#[derive(Debug, Clone)]
pub struct PathologicalDetection {
    pub input: String,
    pub category: PathologicalCategory,
    pub slowdown_factor: f64, // e.g., 15.5x
    pub baseline_time_us: f64,
    pub actual_time_us: f64,
    pub is_pathological: bool, // true if slowdown > threshold
}

/// Baseline performance database
/// Maps operation types to expected execution time (in microseconds)
#[derive(Debug, Clone)]
pub struct PerformanceBaseline {
    baselines: HashMap<String, f64>,
}

impl PerformanceBaseline {
    pub fn new() -> Self {
        let mut baselines = HashMap::new();

        // Initialize baselines from INTERP-030 benchmarking results
        // Simple arithmetic: 28x overhead vs 200ns native = ~5,600ns = 5.6µs
        baselines.insert("simple_arithmetic".to_string(), 5.6);

        // Variable operations: 60x overhead vs 200ns = ~12,000ns = 12µs
        baselines.insert("variable_ops".to_string(), 12.0);

        // Function call: Estimated 20µs per call (includes setup/teardown)
        baselines.insert("function_call".to_string(), 20.0);

        Self { baselines }
    }

    pub fn get(&self, operation: &str) -> Option<f64> {
        self.baselines.get(operation).copied()
    }

    #[allow(dead_code)] // Future enhancement: dynamic baseline updating
    pub fn add(&mut self, operation: String, time_us: f64) {
        self.baselines.insert(operation, time_us);
    }
}

impl Default for PerformanceBaseline {
    fn default() -> Self {
        Self::new()
    }
}

/// Pathological input detector
pub struct PathologicalDetector {
    baseline: PerformanceBaseline,
    pub threshold: f64, // Slowdown threshold (default: 10.0x)
}

impl PathologicalDetector {
    /// Create new detector with default threshold (10x)
    pub fn new() -> Self {
        Self {
            baseline: PerformanceBaseline::new(),
            threshold: 10.0,
        }
    }

    /// Create detector with custom threshold
    pub fn with_threshold(threshold: f64) -> Self {
        Self {
            baseline: PerformanceBaseline::new(),
            threshold,
        }
    }

    /// Detect pathological input by comparing against baseline
    ///
    /// Executes the input program and measures performance against expected baseline
    pub fn detect(&self, input: &str, category: PathologicalCategory) -> PathologicalDetection {
        // Get baseline for this category
        let baseline_key = match category {
            PathologicalCategory::ParserStress => "simple_arithmetic",
            PathologicalCategory::EvaluatorStress => "variable_ops",
            PathologicalCategory::MemoryStress => "variable_ops",
        };

        let baseline_time_us = self.baseline.get(baseline_key).unwrap_or(10.0); // Default 10µs

        // Measure actual execution time
        let start = Instant::now();
        let mut parser = Parser::new(input);
        if let Ok(ast) = parser.parse() {
            let mut eval = Evaluator::new();
            for statement in ast.nodes() {
                // Ignore errors - we're just measuring performance
                let _ = eval.eval(statement);
            }
        }
        let duration = start.elapsed();
        let actual_time_us = duration.as_micros() as f64;

        // Calculate slowdown factor
        let slowdown_factor = actual_time_us / baseline_time_us;

        // Check if pathological
        let is_pathological = slowdown_factor > self.threshold;

        PathologicalDetection {
            input: input.to_string(),
            category,
            slowdown_factor,
            baseline_time_us,
            actual_time_us,
            is_pathological,
        }
    }

    /// Generate deeply nested expressions for parser stress testing
    ///
    /// Example: ((((1 + 2) + 3) + 4) + 5) + ... + N
    pub fn generate_nested_expression(depth: usize) -> String {
        let mut expr = "1".to_string();
        for i in 2..=depth {
            expr = format!("({} + {})", expr, i);
        }
        expr
    }

    /// Generate quadratic variable lookup pattern
    ///
    /// Example:
    /// let a = 1;
    /// let b = a;
    /// let c = b;
    /// ... (N variables)
    /// c (lookup requires scanning all N variables)
    pub fn generate_quadratic_lookup(var_count: usize) -> String {
        let mut code = String::new();
        code.push_str("let a = 1;\n");

        for i in 1..var_count {
            let prev = (b'a' + (i - 1) as u8) as char;
            let curr = (b'a' + i as u8) as char;
            code.push_str(&format!("let {} = {};\n", curr, prev));
        }

        // Final lookup (worst case - scans all variables)
        let last = (b'a' + (var_count - 1) as u8) as char;
        code.push_str(&format!("{}", last));

        code
    }
}

impl Default for PathologicalDetector {
    fn default() -> Self {
        Self::new()
    }
}
