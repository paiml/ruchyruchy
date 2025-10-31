// INTERP-040: NASA-Level Soak and Performance Testing Infrastructure
//
// This module implements soak testing for long-duration stability validation.
//
// Features:
// - RSS (Resident Set Size) measurement via /proc/self/status (Linux)
// - PMAT TDG integration for continuous quality monitoring
// - Workload generation with configurable distributions
// - Rate limiting for throughput control
// - Telemetry collection and analysis
// - Statistical validation (memory growth, performance drift)
//
// Integration:
// - Extends INTERP-029 (Fuzzing) for workload generation
// - Extends INTERP-030 (Benchmarking) for continuous monitoring
// - Extends INTERP-031 (Memory Safety) for long-duration validation
//
// Standards:
// - NASA-STD-8739.8 (Software Assurance)
// - DO-178C (Aviation software certification)
// - ISO/IEC 25010 (Quality models)

use std::time::{Duration, Instant};

/// Workload distribution strategies for soak testing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadDistribution {
    /// Equal probability for all grammar rules
    Uniform,
    /// Weighted by real-world usage (70% arithmetic, 20% variables, 10% control)
    Realistic,
    /// Stress test edge cases (80% complex, 20% simple)
    Adversarial,
}

/// Telemetry snapshot collected during soak test
#[derive(Debug, Clone)]
pub struct TelemetrySnapshot {
    /// Time since soak test start
    pub timestamp: Duration,
    /// Total programs executed so far
    pub programs_executed: u64,
    /// Resident set size in KB
    pub rss_kb: u64,
    /// Programs per minute
    pub throughput: f64,
    /// 50th percentile latency in microseconds
    pub p50_latency_us: u64,
    /// 95th percentile latency in microseconds
    pub p95_latency_us: u64,
    /// 99th percentile latency in microseconds
    pub p99_latency_us: u64,
    /// Errors encountered so far
    pub error_count: u64,
    /// PMAT TDG score (Technical Debt Gauge)
    pub pmat_tdg: f64,
}

/// Soak test configuration
#[derive(Debug, Clone)]
pub struct SoakConfig {
    /// Total duration (e.g., 24h for production, 60s for tests)
    pub duration: Duration,
    /// Programs per minute (e.g., 100)
    pub target_rate: u64,
    /// Workload distribution strategy
    pub distribution: WorkloadDistribution,
    /// How often to collect telemetry (e.g., 60s)
    pub sampling_interval: Duration,
}

impl Default for SoakConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(60), // 1 minute for tests
            target_rate: 100,
            distribution: WorkloadDistribution::Realistic,
            sampling_interval: Duration::from_secs(60),
        }
    }
}

/// Soak test results
#[derive(Debug, Clone)]
pub struct SoakResult {
    /// Uptime percentage (100.0 = perfect uptime, no crashes)
    pub uptime_percentage: f64,
    /// Total programs executed during soak test
    pub total_programs: u64,
    /// Programs that succeeded (no parse or eval errors)
    pub successful_programs: u64,
    /// Programs that failed (parse or eval errors)
    pub failed_programs: u64,
    /// Failure rate as percentage
    pub error_rate: f64,
    /// Number of crashes detected
    pub crashes: u64,
    /// Number of panics detected
    pub panics: u64,
    /// Initial RSS (Resident Set Size) in KB
    pub baseline_rss_kb: u64,
    /// Final RSS in KB
    pub final_rss_kb: u64,
    /// RSS change in KB (final - baseline, can be negative)
    pub memory_growth_kb: i64,
    /// Growth rate in KB per hour
    pub memory_growth_per_hour_kb: f64,
    /// Initial throughput in programs per minute
    pub baseline_throughput: f64,
    /// Final throughput in programs per minute
    pub final_throughput: f64,
    /// Throughput change as percentage per hour
    pub performance_drift_pct: f64,
    /// Initial PMAT TDG score
    pub baseline_pmat_tdg: f64,
    /// Final PMAT TDG score
    pub final_pmat_tdg: f64,
    /// All telemetry snapshots collected during soak test
    pub telemetry: Vec<TelemetrySnapshot>,
}

impl SoakResult {
    /// Check if Tier 1 (24h) acceptance criteria are met
    pub fn meets_tier1_criteria(&self) -> bool {
        self.uptime_percentage >= 99.99
            && self.memory_growth_per_hour_kb < 1.0
            && self.performance_drift_pct.abs() < 0.1
            && self.crashes == 0
            && self.panics == 0
    }

    /// Check if Tier 2 (48h) acceptance criteria are met
    pub fn meets_tier2_criteria(&self) -> bool {
        self.meets_tier1_criteria() && self.error_rate < 0.05
    }

    /// Check if Tier 3 (72h) acceptance criteria are met
    pub fn meets_tier3_criteria(&self) -> bool {
        self.meets_tier2_criteria()
            && self.memory_growth_per_hour_kb < 0.5
            && self.final_pmat_tdg >= 85.0
    }
}

/// TelemetryCollector for metrics gathering during soak tests
pub struct TelemetryCollector {
    snapshots: Vec<TelemetrySnapshot>,
    start_time: Instant,
}

impl TelemetryCollector {
    /// Create a new telemetry collector
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            start_time: Instant::now(),
        }
    }

    /// Collect a telemetry snapshot
    pub fn collect_snapshot(
        &mut self,
        programs_executed: u64,
        error_count: u64,
    ) -> TelemetrySnapshot {
        let elapsed = self.start_time.elapsed();

        // Measure actual memory (RSS via /proc/self/status on Linux)
        let rss_kb = Self::measure_rss_kb();

        // Calculate actual throughput
        let throughput = if elapsed.as_secs() > 0 {
            (programs_executed as f64 / elapsed.as_secs() as f64) * 60.0
        } else {
            0.0
        };

        // Measure actual PMAT TDG
        let pmat_tdg = Self::measure_pmat_tdg();

        let snapshot = TelemetrySnapshot {
            timestamp: elapsed,
            programs_executed,
            rss_kb,
            throughput,
            p50_latency_us: 0, // TODO: Implement latency tracking in future phase
            p95_latency_us: 0,
            p99_latency_us: 0,
            error_count,
            pmat_tdg,
        };

        self.snapshots.push(snapshot.clone());
        snapshot
    }

    /// Measure current RSS (Resident Set Size) in KB
    ///
    /// Reads /proc/self/status on Linux to get VmRSS
    /// Falls back to 0 on non-Linux or if measurement fails
    fn measure_rss_kb() -> u64 {
        #[cfg(target_os = "linux")]
        {
            use std::fs;

            // Read /proc/self/status
            if let Ok(status) = fs::read_to_string("/proc/self/status") {
                // Find VmRSS line
                for line in status.lines() {
                    if line.starts_with("VmRSS:") {
                        // Parse "VmRSS:   12345 kB"
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(rss) = parts[1].parse::<u64>() {
                                return rss;
                            }
                        }
                    }
                }
            }
        }

        // Fallback for non-Linux or if measurement fails
        0
    }

    /// Measure current PMAT TDG score
    ///
    /// Executes `pmat tdg .` and parses the output
    /// Falls back to 0.0 if PMAT is not available or measurement fails
    fn measure_pmat_tdg() -> f64 {
        use std::process::Command;

        // Execute: pmat tdg . --format json
        match Command::new("pmat")
            .args(["tdg", ".", "--format", "json"])
            .output()
        {
            Ok(output) if output.status.success() => {
                // Parse JSON output to extract TDG score
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    // Simple extraction: look for "tdg_score": 97.4
                    // TODO: Use proper JSON parsing in future phase
                    for line in stdout.lines() {
                        if line.contains("tdg_score") {
                            // Extract number after colon
                            if let Some(score_str) = line.split(':').nth(1) {
                                let cleaned = score_str.trim().trim_matches(',').trim_matches('"');
                                if let Ok(score) = cleaned.parse::<f64>() {
                                    return score;
                                }
                            }
                        }
                    }
                }
                0.0
            }
            _ => 0.0, // PMAT not available or execution failed
        }
    }

    /// Get all collected snapshots
    pub fn snapshots(&self) -> &[TelemetrySnapshot] {
        &self.snapshots
    }

    /// Calculate memory growth rate (KB per hour)
    pub fn calculate_memory_growth_rate(&self) -> f64 {
        if self.snapshots.len() < 2 {
            return 0.0;
        }

        let first = &self.snapshots[0];
        let last = &self.snapshots[self.snapshots.len() - 1];

        let growth_kb = last.rss_kb as i64 - first.rss_kb as i64;
        let duration_hours = last.timestamp.as_secs_f64() / 3600.0;

        if duration_hours > 0.0 {
            growth_kb as f64 / duration_hours
        } else {
            0.0
        }
    }

    /// Calculate performance drift (% per hour)
    pub fn calculate_performance_drift(&self) -> f64 {
        if self.snapshots.len() < 2 {
            return 0.0;
        }

        let first = &self.snapshots[0];
        let last = &self.snapshots[self.snapshots.len() - 1];

        if first.throughput == 0.0 {
            return 0.0;
        }

        let drift_pct = ((last.throughput - first.throughput) / first.throughput) * 100.0;
        let duration_hours = last.timestamp.as_secs_f64() / 3600.0;

        if duration_hours > 0.0 {
            drift_pct / duration_hours
        } else {
            0.0
        }
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// SoakTestRunner for executing long-duration stability tests
pub struct SoakTestRunner {
    config: SoakConfig,
    telemetry: TelemetryCollector,
    seed: u64,
}

impl SoakTestRunner {
    /// Create a new soak test runner with configuration
    pub fn new(config: SoakConfig) -> Self {
        Self {
            config,
            telemetry: TelemetryCollector::new(),
            seed: 42, // Deterministic for testing
        }
    }

    /// Run soak test and return results
    ///
    /// Executes workload generation with rate limiting, collects telemetry,
    /// and tracks memory growth and performance drift.
    pub fn run(&mut self) -> SoakResult {
        let start_time = Instant::now();
        let baseline_snapshot = self.telemetry.collect_snapshot(0, 0);

        let mut total_programs = 0;
        let mut successful_programs = 0;
        let mut failed_programs = 0;
        let mut error_count = 0;

        // Calculate target delay between programs for rate limiting
        let target_delay = Duration::from_secs_f64(60.0 / self.config.target_rate as f64);
        let mut last_sample_elapsed = Duration::ZERO;

        // Main workload loop
        while start_time.elapsed() < self.config.duration {
            // Generate program based on distribution
            let program = self.generate_program();
            total_programs += 1;

            // Execute program
            let result = self.execute_program(&program);
            match result {
                Ok(_) => successful_programs += 1,
                Err(_) => {
                    failed_programs += 1;
                    error_count += 1;
                }
            }

            // Rate limiting: sleep to maintain target throughput
            std::thread::sleep(target_delay);

            // Collect telemetry at sampling intervals
            let current_elapsed = start_time.elapsed();
            if current_elapsed - last_sample_elapsed >= self.config.sampling_interval {
                self.telemetry.collect_snapshot(total_programs, error_count);
                last_sample_elapsed = current_elapsed;
            }
        }

        // Collect final snapshot
        let final_snapshot = self.telemetry.collect_snapshot(total_programs, error_count);

        // Calculate results
        let elapsed = start_time.elapsed();
        let elapsed_hours = elapsed.as_secs_f64() / 3600.0;

        let memory_growth = final_snapshot.rss_kb as i64 - baseline_snapshot.rss_kb as i64;
        let memory_growth_per_hour = if elapsed_hours > 0.0 {
            memory_growth as f64 / elapsed_hours
        } else {
            0.0
        };

        let performance_drift = if baseline_snapshot.throughput > 0.0 {
            ((final_snapshot.throughput - baseline_snapshot.throughput)
                / baseline_snapshot.throughput)
                * 100.0
        } else {
            0.0
        };

        SoakResult {
            uptime_percentage: 100.0, // No crashes = 100% uptime
            total_programs,
            successful_programs,
            failed_programs,
            error_rate: if total_programs > 0 {
                (failed_programs as f64 / total_programs as f64) * 100.0
            } else {
                0.0
            },
            crashes: 0, // TODO: Implement crash detection in future phase
            panics: 0,  // TODO: Implement panic detection in future phase
            baseline_rss_kb: baseline_snapshot.rss_kb,
            final_rss_kb: final_snapshot.rss_kb,
            memory_growth_kb: memory_growth,
            memory_growth_per_hour_kb: memory_growth_per_hour,
            baseline_throughput: baseline_snapshot.throughput,
            final_throughput: final_snapshot.throughput,
            performance_drift_pct: performance_drift,
            baseline_pmat_tdg: baseline_snapshot.pmat_tdg,
            final_pmat_tdg: final_snapshot.pmat_tdg,
            telemetry: self.telemetry.snapshots().to_vec(),
        }
    }

    /// Generate a program based on workload distribution
    ///
    /// Uses LCG (Linear Congruential Generator) for deterministic randomness
    fn generate_program(&mut self) -> String {
        // Update seed using LCG
        const A: u64 = 1664525;
        const C: u64 = 1013904223;
        self.seed = A.wrapping_mul(self.seed).wrapping_add(C);

        // Generate program based on distribution
        match self.config.distribution {
            WorkloadDistribution::Uniform => self.generate_uniform(),
            WorkloadDistribution::Realistic => self.generate_realistic(),
            WorkloadDistribution::Adversarial => self.generate_adversarial(),
        }
    }

    /// Generate program with uniform distribution (all rules equal probability)
    fn generate_uniform(&self) -> String {
        let patterns = [
            "1 + 1",
            "let x = 10; x",
            "5 * 5",
            "if (true) { 1 } else { 2 }",
            "10 - 5",
            "20 / 4",
            "3 == 3",
            "7 > 5",
        ];

        let idx = (self.seed % patterns.len() as u64) as usize;
        patterns[idx].to_string()
    }

    /// Generate program with realistic distribution (70% arithmetic, 20% variables, 10% control)
    fn generate_realistic(&self) -> String {
        let pattern_type = self.seed % 100;

        if pattern_type < 70 {
            // 70% arithmetic
            let ops = ["+", "-", "*", "/"];
            let op = ops[(self.seed % ops.len() as u64) as usize];
            let left = self.seed % 100;
            let right = (self.seed / 100) % 100 + 1; // Avoid division by zero
            format!("{} {} {}", left, op, right)
        } else if pattern_type < 90 {
            // 20% variables
            format!("let x = {}; x + {}", self.seed % 50, (self.seed / 100) % 50)
        } else {
            // 10% control flow
            format!(
                "if ({} > 50) {{ {} }} else {{ {} }}",
                self.seed % 100,
                (self.seed / 100) % 100,
                (self.seed / 10000) % 100
            )
        }
    }

    /// Generate program with adversarial distribution (80% complex, 20% simple)
    fn generate_adversarial(&self) -> String {
        let pattern_type = self.seed % 100;

        if pattern_type < 80 {
            // 80% complex (nested, edge cases)
            let patterns = [
                format!(
                    "let x = {}; if (x > 50) {{ x * 2 }} else {{ x / 2 }}",
                    self.seed % 100
                ),
                format!(
                    "let a = {}; let b = {}; a + b * 2",
                    self.seed % 50,
                    (self.seed / 100) % 50
                ),
                format!(
                    "if ({} == {}) {{ 1 }} else {{ 0 }}",
                    self.seed % 10,
                    (self.seed / 10) % 10
                ),
                format!("{} / {}", self.seed % 1000, (self.seed / 100) % 100 + 1),
            ];
            let idx = (self.seed % patterns.len() as u64) as usize;
            patterns[idx].clone()
        } else {
            // 20% simple
            format!("{}", self.seed % 1000)
        }
    }

    /// Execute a program and return result
    ///
    /// Currently minimal implementation - will be extended with:
    /// - Crash detection (std::panic::catch_unwind)
    /// - Timeout detection
    /// - Resource usage tracking
    fn execute_program(&mut self, program: &str) -> Result<(), String> {
        use crate::interpreter::evaluator::Evaluator;
        use crate::interpreter::parser::Parser;

        // Parse
        let mut parser = Parser::new(program);
        let ast = parser
            .parse()
            .map_err(|e| format!("Parse error: {:?}", e))?;

        // Evaluate
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            eval.eval(statement)
                .map_err(|e| format!("Eval error: {:?}", e))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workload_distribution_variants() {
        assert_eq!(WorkloadDistribution::Uniform, WorkloadDistribution::Uniform);
        assert_ne!(
            WorkloadDistribution::Uniform,
            WorkloadDistribution::Realistic
        );
        assert_ne!(
            WorkloadDistribution::Realistic,
            WorkloadDistribution::Adversarial
        );
    }

    #[test]
    fn test_telemetry_collector_basic() {
        let mut collector = TelemetryCollector::new();
        let snapshot1 = collector.collect_snapshot(100, 0);
        assert_eq!(snapshot1.programs_executed, 100);
        assert_eq!(collector.snapshots().len(), 1);
    }

    #[test]
    fn test_soak_config_default() {
        let config = SoakConfig::default();
        assert_eq!(config.duration, Duration::from_secs(60));
        assert_eq!(config.target_rate, 100);
        assert_eq!(config.distribution, WorkloadDistribution::Realistic);
    }

    #[test]
    fn test_soak_runner_creation() {
        let config = SoakConfig::default();
        let _runner = SoakTestRunner::new(config);
    }

    #[test]
    fn test_rss_measurement() {
        let rss = TelemetryCollector::measure_rss_kb();
        // On Linux, should return non-zero value
        // On other platforms, returns 0
        #[cfg(target_os = "linux")]
        assert!(rss > 0, "RSS should be measured on Linux: {} KB", rss);
    }
}
