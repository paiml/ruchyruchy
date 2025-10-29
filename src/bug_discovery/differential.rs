// Differential Testing with Statistical Analysis
// DISC-001: Differential Testing Implementation
//
// References:
// - McKeeman (1998): "Differential testing for software"
// - Kalibera & Jones (2013): Statistical performance analysis
// - Section 5.1 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md

use crate::bug_discovery::confidence::{
    ConfidenceScore, ConfidenceScorer, DiscoveryMethod, EvidenceLevel, Reproducibility,
    RootCauseClarity,
};
use crate::bug_discovery::statistics::{cohens_d, mean, welchs_t_test, PerformanceRegression};
use std::time::{Duration, Instant};

/// Test status for differential testing
#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    Pass,
    Hang(Duration),
    Crash(String),
    WrongOutput(String),
    PerfRegression {
        slowdown_factor: f64,
        p_value: f64,
    },
}

/// Result from running a single test
#[derive(Debug, Clone)]
pub struct TestResult {
    pub status: TestStatus,
    pub execution_time_ms: Option<f64>,
    pub memory_usage_mb: Option<f64>,
    pub output: Option<String>,
}

/// Compiler version for differential testing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompilerVersion {
    pub version: String,
    pub target: CompilationTarget,
}

/// Compilation target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompilationTarget {
    Debug,
    Release,
    Wasm,
}

impl std::fmt::Display for CompilerVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", self.version, self.target)
    }
}

/// Bug discovered via differential testing
#[derive(Debug, Clone)]
pub struct RegressionBug {
    pub test_case: String,
    pub working_version: CompilerVersion,
    pub broken_version: CompilerVersion,
    pub failure_mode: FailureMode,
    pub confidence: ConfidenceScore,
}

/// Failure mode classification
#[derive(Debug, Clone)]
pub enum FailureMode {
    Hang { timeout_ms: u64 },
    Crash { error_message: String },
    WrongOutput { expected: String, actual: String },
    PerformanceRegression { regression: PerformanceRegression },
}

/// Differential tester
pub struct DifferentialTester {
    versions: Vec<CompilerVersion>,
    statistical_samples: usize,
    significance_level: f64,
    min_slowdown: f64,
}

impl DifferentialTester {
    /// Create a new differential tester
    pub fn new(versions: Vec<CompilerVersion>) -> Self {
        DifferentialTester {
            versions,
            statistical_samples: 30,
            significance_level: 0.05,
            min_slowdown: 1.2, // 20% slowdown threshold
        }
    }

    /// Configure statistical analysis parameters
    pub fn with_statistical_params(
        mut self,
        samples: usize,
        significance: f64,
        min_slowdown: f64,
    ) -> Self {
        self.statistical_samples = samples;
        self.significance_level = significance;
        self.min_slowdown = min_slowdown;
        self
    }

    /// Find regressions across versions
    pub fn find_regressions(&self, test_cases: &[String]) -> Vec<RegressionBug> {
        let mut bugs = vec![];

        for test_case in test_cases {
            // Run across all versions
            let results = self.run_across_versions(test_case);

            // Detect functional regressions
            if let Some(bug) = self.detect_functional_regression(test_case, &results) {
                bugs.push(bug);
            }

            // Detect performance regressions
            if let Some(bug) = self.detect_performance_regression(test_case, &results) {
                bugs.push(bug);
            }
        }

        bugs
    }

    /// Run test case across all versions
    fn run_across_versions(&self, test_case: &str) -> Vec<(CompilerVersion, Vec<TestResult>)> {
        self.versions
            .iter()
            .map(|version| {
                let results = self.run_multiple_times(version, test_case, self.statistical_samples);
                (version.clone(), results)
            })
            .collect()
    }

    /// Run test multiple times for statistical analysis
    fn run_multiple_times(
        &self,
        version: &CompilerVersion,
        test_case: &str,
        count: usize,
    ) -> Vec<TestResult> {
        (0..count)
            .map(|_| self.run_single_test(version, test_case))
            .collect()
    }

    /// Run a single test (placeholder - would actually run compiler)
    fn run_single_test(&self, _version: &CompilerVersion, _test_case: &str) -> TestResult {
        // Placeholder: In production, this would:
        // 1. Compile test_case with the specified version
        // 2. Execute the compiled code
        // 3. Measure execution time, memory, output
        // 4. Return TestResult

        let start = Instant::now();
        // Simulate execution
        std::thread::sleep(Duration::from_millis(10));
        let execution_time = start.elapsed().as_secs_f64() * 1000.0;

        TestResult {
            status: TestStatus::Pass,
            execution_time_ms: Some(execution_time),
            memory_usage_mb: Some(10.0),
            output: Some("success".to_string()),
        }
    }

    /// Detect functional regressions (crashes, hangs, wrong output)
    fn detect_functional_regression(
        &self,
        test_case: &str,
        results: &[(CompilerVersion, Vec<TestResult>)],
    ) -> Option<RegressionBug> {
        for i in 0..results.len() - 1 {
            let (working_ver, working_results) = &results[i];
            let (broken_ver, broken_results) = &results[i + 1];

            // Check if previously passing test now fails
            let working_passes = working_results
                .iter()
                .all(|r| matches!(r.status, TestStatus::Pass));
            let broken_fails = broken_results
                .iter()
                .any(|r| !matches!(r.status, TestStatus::Pass));

            if working_passes && broken_fails {
                // Determine failure mode from first failure
                let failure_mode = broken_results
                    .iter()
                    .find(|r| !matches!(r.status, TestStatus::Pass))
                    .map(|r| match &r.status {
                        TestStatus::Hang(duration) => FailureMode::Hang {
                            timeout_ms: duration.as_millis() as u64,
                        },
                        TestStatus::Crash(msg) => FailureMode::Crash {
                            error_message: msg.clone(),
                        },
                        TestStatus::WrongOutput(diff) => FailureMode::WrongOutput {
                            expected: "...".to_string(),
                            actual: diff.clone(),
                        },
                        _ => unreachable!(),
                    })?;

                // Calculate confidence
                let confidence = ConfidenceScorer::from_components(
                    DiscoveryMethod::DifferentialTestVersionRegression,
                    Reproducibility::Always,
                    EvidenceLevel::Complete,
                    RootCauseClarity::PrimaryWithSecondary,
                );

                return Some(RegressionBug {
                    test_case: test_case.to_string(),
                    working_version: working_ver.clone(),
                    broken_version: broken_ver.clone(),
                    failure_mode,
                    confidence,
                });
            }
        }

        None
    }

    /// Detect performance regressions using statistical analysis
    fn detect_performance_regression(
        &self,
        test_case: &str,
        results: &[(CompilerVersion, Vec<TestResult>)],
    ) -> Option<RegressionBug> {
        for i in 0..results.len() - 1 {
            let (baseline_ver, baseline_results) = &results[i];
            let (current_ver, current_results) = &results[i + 1];

            // Extract execution times
            let baseline_times: Vec<f64> = baseline_results
                .iter()
                .filter_map(|r| r.execution_time_ms)
                .collect();

            let current_times: Vec<f64> = current_results
                .iter()
                .filter_map(|r| r.execution_time_ms)
                .collect();

            if baseline_times.len() < self.statistical_samples
                || current_times.len() < self.statistical_samples
            {
                continue;
            }

            // Perform Welch's t-test
            let (_t_stat, p_value) = welchs_t_test(&baseline_times, &current_times);
            let slowdown = mean(&current_times) / mean(&baseline_times);
            let effect_size = cohens_d(&baseline_times, &current_times);

            // Check for significant regression
            if p_value < self.significance_level && slowdown > self.min_slowdown {
                let regression = PerformanceRegression {
                    baseline_version: baseline_ver.to_string(),
                    regressed_version: current_ver.to_string(),
                    slowdown_factor: slowdown,
                    p_value,
                    baseline_mean_ms: mean(&baseline_times),
                    regressed_mean_ms: mean(&current_times),
                    effect_size,
                };

                // Calculate confidence based on statistical strength
                let confidence = self.calculate_perf_regression_confidence(&regression);

                return Some(RegressionBug {
                    test_case: test_case.to_string(),
                    working_version: baseline_ver.clone(),
                    broken_version: current_ver.clone(),
                    failure_mode: FailureMode::PerformanceRegression { regression },
                    confidence,
                });
            }
        }

        None
    }

    /// Calculate confidence for performance regressions
    fn calculate_perf_regression_confidence(&self, regression: &PerformanceRegression) -> ConfidenceScore {
        // Higher confidence for:
        // - Lower p-values (more significant)
        // - Larger effect sizes (more noticeable)
        // - Larger slowdowns (more impactful)

        let p_score = (self.significance_level - regression.p_value) / self.significance_level;
        let effect_score = (regression.effect_size.abs() / 2.0).min(1.0);
        let slowdown_score = ((regression.slowdown_factor - 1.0) / 2.0).min(1.0);

        let overall = 0.5 * p_score + 0.3 * effect_score + 0.2 * slowdown_score;

        ConfidenceScore::new(overall, 0.9, 0.8, 0.7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_versions() -> Vec<CompilerVersion> {
        vec![
            CompilerVersion {
                version: "v3.146".to_string(),
                target: CompilationTarget::Debug,
            },
            CompilerVersion {
                version: "v3.147".to_string(),
                target: CompilationTarget::Debug,
            },
            CompilerVersion {
                version: "v3.148".to_string(),
                target: CompilationTarget::Debug,
            },
        ]
    }

    #[test]
    fn test_differential_tester_creation() {
        let versions = create_test_versions();
        let tester = DifferentialTester::new(versions.clone());
        assert_eq!(tester.versions.len(), 3);
        assert_eq!(tester.statistical_samples, 30);
    }

    #[test]
    fn test_statistical_params_configuration() {
        let versions = create_test_versions();
        let tester = DifferentialTester::new(versions)
            .with_statistical_params(50, 0.01, 1.5);

        assert_eq!(tester.statistical_samples, 50);
        assert_eq!(tester.significance_level, 0.01);
        assert_eq!(tester.min_slowdown, 1.5);
    }

    #[test]
    fn test_compiler_version_display() {
        let version = CompilerVersion {
            version: "v3.146".to_string(),
            target: CompilationTarget::Debug,
        };
        assert_eq!(version.to_string(), "v3.146 (Debug)");
    }

    #[test]
    fn test_run_single_test() {
        let versions = create_test_versions();
        let tester = DifferentialTester::new(versions.clone());
        let result = tester.run_single_test(&versions[0], "test code");

        assert!(matches!(result.status, TestStatus::Pass));
        assert!(result.execution_time_ms.is_some());
    }

    #[test]
    fn test_run_multiple_times() {
        let versions = create_test_versions();
        let tester = DifferentialTester::new(versions.clone());
        let results = tester.run_multiple_times(&versions[0], "test code", 5);

        assert_eq!(results.len(), 5);
        assert!(results.iter().all(|r| r.execution_time_ms.is_some()));
    }
}
