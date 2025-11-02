//! CompilerProfiler implementation (PROFILER-001)
//!
//! Extends DEBUGGER-047 PerformanceProfiler with compiler-specific tracking.
//!
//! This module provides the core [`CompilerProfiler`] type for tracking compilation
//! phases, type observations, and optimization opportunities.

use super::types::{OptimizationOpportunity, PhaseReport, Stability, TypeSignature};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Compiler profiler for tracking compilation phases, types, and optimizations
#[derive(Debug, Clone)]
pub struct CompilerProfiler {
    data: Rc<RefCell<ProfilerData>>,
}

#[derive(Debug)]
struct ProfilerData {
    // Compilation phase tracking
    phases: HashMap<String, Duration>,
    current_phase: Option<(String, Instant)>,

    // Type observation (Julia-inspired)
    type_observations: HashMap<String, Vec<TypeSignature>>,

    // Hot function tracking (DEBUGGER-052)
    function_calls: HashMap<String, CallProfile>,
    // Total execution time across all tracked functions
    total_execution_time: Duration,
}

#[derive(Debug, Clone)]
struct CallProfile {
    count: usize,
    total_time: Duration,
}

impl CompilerProfiler {
    /// Create new compiler profiler
    pub fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(ProfilerData {
                phases: HashMap::new(),
                current_phase: None,
                type_observations: HashMap::new(),
                function_calls: HashMap::new(),
                total_execution_time: Duration::ZERO,
            })),
        }
    }

    /// Start timing a compilation phase
    pub fn start_phase(&self, name: &str) {
        let mut data = self.data.borrow_mut();
        data.current_phase = Some((name.to_string(), Instant::now()));
    }

    /// End timing a compilation phase
    pub fn end_phase(&self, name: &str) {
        let mut data = self.data.borrow_mut();
        if let Some((phase_name, start_time)) = data.current_phase.take() {
            assert_eq!(phase_name, name, "Phase mismatch");
            let duration = start_time.elapsed();
            data.phases.insert(phase_name, duration);
        }
    }

    /// Get phase timing report
    pub fn phase_report(&self) -> PhaseReport {
        let data = self.data.borrow();
        let mut report = PhaseReport::new();
        for (name, duration) in &data.phases {
            report.add_phase(name.clone(), *duration);
        }
        report
    }

    /// Observe type signature for a function call
    pub fn observe_type(&self, function: &str, sig: TypeSignature) {
        let mut data = self.data.borrow_mut();
        data.type_observations
            .entry(function.to_string())
            .or_default()
            .push(sig);
    }

    /// Get type observations for a function
    pub fn type_observations(&self, function: &str) -> Vec<TypeSignature> {
        let data = self.data.borrow();
        data.type_observations
            .get(function)
            .cloned()
            .unwrap_or_default()
    }

    /// Analyze type stability for a function
    pub fn type_stability(&self, function: &str) -> Stability {
        let observations = self.type_observations(function);
        let unique_types: std::collections::HashSet<_> = observations.iter().collect();

        match unique_types.len() {
            0 | 1 => Stability::Monomorphic,
            2 | 3 => Stability::Polymorphic,
            _ => Stability::Megamorphic,
        }
    }

    /// Analyze AST for optimization opportunities
    pub fn analyze_ast(
        &self,
        _ast: &crate::interpreter::parser::Ast,
    ) -> Vec<OptimizationOpportunity> {
        // Placeholder: will implement in follow-up
        vec![]
    }

    /// Record a function call with its execution time (DEBUGGER-052)
    ///
    /// Updates call count and total time for the function, and accumulates
    /// total execution time for percentage calculation.
    pub fn record_function_call(&self, function: &str, duration: Duration) {
        let mut data = self.data.borrow_mut();

        // Update function profile
        let profile = data
            .function_calls
            .entry(function.to_string())
            .or_insert(CallProfile {
                count: 0,
                total_time: Duration::ZERO,
            });

        profile.count += 1;
        profile.total_time += duration;

        // Update total execution time
        data.total_execution_time += duration;
    }

    /// Identify hot functions (>threshold% of total time)
    ///
    /// Returns functions consuming more than the threshold percentage of total
    /// execution time. Threshold is a fraction (0.01 = 1%).
    ///
    /// # Arguments
    ///
    /// * `threshold` - Minimum percentage (as fraction) to be considered "hot" (e.g., 0.01 for 1%)
    ///
    /// # Returns
    ///
    /// Vector of HotFunction sorted by percentage_of_total (descending)
    pub fn hot_functions(&self, threshold: f64) -> Vec<HotFunction> {
        let data = self.data.borrow();

        let total_time_micros = data.total_execution_time.as_micros() as f64;
        if total_time_micros == 0.0 {
            return vec![];
        }

        let mut hot_fns: Vec<HotFunction> = data
            .function_calls
            .iter()
            .map(|(name, profile)| {
                let func_time_micros = profile.total_time.as_micros() as f64;
                let percentage = (func_time_micros / total_time_micros) * 100.0;

                HotFunction {
                    name: name.clone(),
                    call_count: profile.count,
                    total_time: profile.total_time,
                    percentage_of_total: percentage,
                }
            })
            .filter(|f| f.percentage_of_total >= threshold * 100.0)
            .collect();

        // Sort by percentage descending (hottest first)
        hot_fns.sort_by(|a, b| {
            b.percentage_of_total
                .partial_cmp(&a.percentage_of_total)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        hot_fns
    }

    /// Profile code in specific execution mode (stub for Phase 2)
    #[allow(dead_code)]
    pub fn profile_mode(&self, _code: &str, _mode: super::ExecutionMode) -> Duration {
        Duration::ZERO
    }

    /// Get comparison report across modes (stub for Phase 2)
    #[allow(dead_code)]
    pub fn comparison_report(&self) -> ComparisonReport {
        ComparisonReport::new()
    }
}

/// Comparison report across execution modes (stub for Phase 2)
///
/// Will be used to compare performance across AST, Bytecode, Transpiled, and Compiled modes.
///
/// # Planned Features (Phase 2)
///
/// - Track execution time per mode
/// - Calculate speedup ratios
/// - Generate performance comparison reports
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ComparisonReport {}

impl Default for ComparisonReport {
    fn default() -> Self {
        Self::new()
    }
}

impl ComparisonReport {
    /// Create a new empty comparison report (stub)
    pub fn new() -> Self {
        Self {}
    }

    /// Check if a mode has been profiled (stub - always returns false)
    ///
    /// # Arguments
    ///
    /// * `_mode` - Execution mode to check
    pub fn has_mode(&self, _mode: super::ExecutionMode) -> bool {
        false
    }

    /// Calculate speedup between two modes (stub - always returns 1.0)
    ///
    /// # Arguments
    ///
    /// * `_mode1` - Baseline mode
    /// * `_mode2` - Comparison mode
    ///
    /// # Returns
    ///
    /// Speedup ratio (mode2_time / mode1_time)
    pub fn speedup(&self, _mode1: super::ExecutionMode, _mode2: super::ExecutionMode) -> f64 {
        1.0
    }
}

impl Default for CompilerProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Hot function profile
///
/// Identifies functions consuming significant execution time (>1% of total).
/// Used to prioritize optimization efforts.
///
/// # Example
///
/// ```rust
/// use ruchyruchy::profiler::compiler_profiler::HotFunction;
/// use std::time::Duration;
///
/// let hot_fn = HotFunction {
///     name: "fibonacci".to_string(),
///     call_count: 177,
///     total_time: Duration::from_millis(450),
///     percentage_of_total: 95.0,
/// };
///
/// assert!(hot_fn.percentage_of_total > 1.0);
/// ```
#[derive(Debug, Clone)]
pub struct HotFunction {
    /// Function name
    pub name: String,
    /// Number of times called
    pub call_count: usize,
    /// Total execution time across all calls
    pub total_time: Duration,
    /// Percentage of total program execution time
    pub percentage_of_total: f64,
}
