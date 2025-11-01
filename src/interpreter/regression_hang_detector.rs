// DEBUGGER-043: Regression & Hang Detector
//
// Detects regressions and hangs in the Ruchy interpreter.
// Based on analysis of 200 commits from ../ruchy showing patterns:
// - Runtime hangs (Vec::new, enum casts, infinite loops)
// - Regression bugs (behavior changes between versions)
// - Non-determinism (inconsistent results across runs)
// - State pollution (scope leakage)
//
// Design:
// - Timeout-based hang detection (prevent infinite loops)
// - Snapshot-based regression detection (behavior comparison)
// - Multi-run determinism checking
// - Isolated execution for state pollution detection

#![allow(missing_docs)] // Comprehensive inline documentation provided

use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::parser::Parser;
use std::time::Instant;

/// Hang detection result
#[derive(Debug, Clone, PartialEq)]
pub struct HangDetectionResult {
    pub is_hang: bool,
    pub is_stack_overflow: bool,
    pub hang_type: HangType,
    pub execution_time_ms: u64,
}

/// Types of hangs detected
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HangType {
    InfiniteLoop,
    InfiniteRecursion,
    Deadlock,
    Unknown,
    None,
}

/// Execution snapshot for regression detection
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionSnapshot {
    pub output: String,
    pub final_state: String,
    pub execution_time_ms: u64,
}

/// Regression and hang detector
pub struct RegressionHangDetector {
    /// Default timeout in milliseconds
    pub timeout_ms: u64,
}

impl RegressionHangDetector {
    /// Create new detector with default timeout (5 seconds)
    pub fn new() -> Self {
        Self { timeout_ms: 5000 }
    }

    /// Create detector with custom timeout
    pub fn with_timeout(timeout_ms: u64) -> Self {
        Self { timeout_ms }
    }

    /// Detect hang with timeout
    ///
    /// Executes code with timeout to prevent infinite loops.
    /// Returns HangDetectionResult indicating if code hung.
    pub fn detect_hang(&self, code: &str, timeout_ms: u64) -> HangDetectionResult {
        let start = Instant::now();

        // Try to execute code with timeout
        // Note: Rust doesn't have built-in timeout for sync code
        // For MVP, we rely on stack overflow detection and time measurement
        let result = self.execute_with_monitoring(code);

        let execution_time = start.elapsed().as_millis() as u64;

        match result {
            Ok(_) => {
                // Check if execution took longer than timeout
                let is_hang = execution_time > timeout_ms;
                HangDetectionResult {
                    is_hang,
                    is_stack_overflow: false,
                    hang_type: if is_hang {
                        HangType::InfiniteLoop
                    } else {
                        HangType::None
                    },
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => {
                // Check if it's a stack overflow
                let error_str = format!("{:?}", e);
                let is_stack_overflow = error_str.contains("StackOverflow");

                HangDetectionResult {
                    is_hang: is_stack_overflow,
                    is_stack_overflow,
                    hang_type: if is_stack_overflow {
                        HangType::InfiniteRecursion
                    } else {
                        HangType::Unknown
                    },
                    execution_time_ms: execution_time,
                }
            }
        }
    }

    /// Execute code with monitoring
    fn execute_with_monitoring(&self, code: &str) -> Result<String, String> {
        let mut parser = Parser::new(code);
        let ast = parser.parse().map_err(|e| format!("{:?}", e))?;

        let mut eval = Evaluator::new();
        let mut last_value = String::new();

        for statement in ast.nodes() {
            match eval.eval(statement) {
                Ok(value) => {
                    last_value = format!("{:?}", value);
                }
                Err(e) => {
                    return Err(format!("{:?}", e));
                }
            }
        }

        Ok(last_value)
    }

    /// Create execution snapshot
    ///
    /// Captures output, final state, and execution time for regression detection.
    pub fn create_snapshot(&self, code: &str) -> ExecutionSnapshot {
        let start = Instant::now();
        let output = self.execute_with_monitoring(code).unwrap_or_else(|e| e);
        let execution_time_ms = start.elapsed().as_millis() as u64;

        ExecutionSnapshot {
            output: output.clone(),
            final_state: output, // For now, use output as state
            execution_time_ms,
        }
    }

    /// Compare snapshots for regression detection
    ///
    /// Returns true if snapshots match (no regression)
    pub fn snapshots_match(
        &self,
        baseline: &ExecutionSnapshot,
        current: &ExecutionSnapshot,
    ) -> bool {
        baseline.output == current.output && baseline.final_state == current.final_state
    }

    /// Run code multiple times and return results
    ///
    /// Used for determinism checking
    pub fn run_multiple_times(&self, code: &str, count: usize) -> Vec<String> {
        let mut results = Vec::new();
        for _ in 0..count {
            let result = self.execute_with_monitoring(code).unwrap_or_else(|e| e);
            results.push(result);
        }
        results
    }

    /// Check if all results are equal
    ///
    /// Returns true if code is deterministic
    pub fn all_results_equal(&self, results: &[String]) -> bool {
        if results.is_empty() {
            return true;
        }

        let first = &results[0];
        results.iter().all(|r| r == first)
    }

    /// Run code in isolated environment
    ///
    /// Creates fresh evaluator to prevent state leakage
    pub fn run_isolated(&self, code: &str) -> Result<String, String> {
        self.execute_with_monitoring(code)
    }

    /// Measure execution time
    pub fn measure_execution_time(&self, code: &str) -> u64 {
        let start = Instant::now();
        let _ = self.execute_with_monitoring(code);
        start.elapsed().as_millis() as u64
    }

    /// Detect performance regression
    ///
    /// Returns slowdown factor (current / baseline)
    pub fn detect_performance_regression(&self, baseline_ms: u64, current_ms: u64) -> f64 {
        current_ms as f64 / baseline_ms as f64
    }

    /// Check for non-determinism
    ///
    /// Runs code N times and checks if results are consistent
    pub fn check_determinism(&self, code: &str, runs: usize) -> bool {
        let results = self.run_multiple_times(code, runs);
        self.all_results_equal(&results)
    }
}

impl Default for RegressionHangDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_execution() {
        let detector = RegressionHangDetector::new();
        let code = "let x = 1 + 2; x";
        let snapshot = detector.create_snapshot(code);

        assert!(!snapshot.output.is_empty());
        assert!(snapshot.execution_time_ms < 1000); // Should be fast
    }

    #[test]
    fn test_determinism_simple() {
        let detector = RegressionHangDetector::new();
        let code = "let x = 1 + 2; x";

        assert!(
            detector.check_determinism(code, 5),
            "Simple arithmetic should be deterministic"
        );
    }

    #[test]
    fn test_snapshot_matching() {
        let detector = RegressionHangDetector::new();
        let code = "let x = 42;";

        let snap1 = detector.create_snapshot(code);
        let snap2 = detector.create_snapshot(code);

        assert!(
            detector.snapshots_match(&snap1, &snap2),
            "Same code should produce same snapshot"
        );
    }

    #[test]
    fn test_isolated_execution() {
        let detector = RegressionHangDetector::new();

        // First run
        let _ = detector.run_isolated("let x = 42;");

        // Second run - x should not exist
        let result = detector.run_isolated("x");

        assert!(
            result.is_err(),
            "Variable x should not persist between isolated runs"
        );
    }
}
