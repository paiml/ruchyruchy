//! Type definitions for compiler profiling
//!
//! This module provides types for tracking compilation phases, type observations,
//! and optimization opportunities. Inspired by Julia's JIT profiling architecture.
//!
//! # Overview
//!
//! The profiler types support three main capabilities:
//!
//! 1. **Phase Tracking**: Measure time spent in each compilation phase (lexing, parsing, type checking, codegen)
//! 2. **Type Observation**: Track runtime type signatures for function calls (Julia-inspired)
//! 3. **Optimization Discovery**: Identify optimization opportunities (constant folding, inlining, TCO)
//!
//! # Example
//!
//! ```rust
//! use ruchyruchy::profiler::{CompilerProfiler, TypeSignature, Stability};
//!
//! let profiler = CompilerProfiler::new();
//! profiler.start_phase("lexing");
//! // ... perform lexing ...
//! profiler.end_phase("lexing");
//!
//! let report = profiler.phase_report();
//! assert!(report.contains_phase("lexing"));
//! ```

use std::time::Duration;

/// Execution mode for cross-mode performance comparison
///
/// RuchyRuchy supports four execution modes with different performance characteristics:
///
/// - **AST**: Tree-walking interpreter (slowest, ~0.37x Python baseline)
/// - **Bytecode**: Virtual machine with bytecode (1.49x Python, high variance)
/// - **Transpiled**: TypeScript/Rust code generation (15.12x Python)
/// - **Compiled**: Native code via rustc (14.89x Python)
///
/// See `docs/specifications/performance-profiling-compiler-tooling.md` for benchmarking methodology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionMode {
    /// Tree-walking AST interpreter (slowest, debugging-friendly)
    AST,
    /// Bytecode virtual machine (moderate speed, high variance)
    Bytecode,
    /// Transpiled to TypeScript/Rust (fast, human-readable output)
    Transpiled,
    /// Compiled to native code via rustc (fastest, requires compilation time)
    Compiled,
}

/// Type signature for function calls (Julia-inspired)
///
/// Tracks the observed types for function parameters and return value.
/// Used for type stability analysis to detect polymorphic call sites.
///
/// # Example
///
/// ```rust
/// use ruchyruchy::profiler::TypeSignature;
///
/// let sig = TypeSignature::new(
///     vec!["Int32".to_string(), "Int32".to_string()],
///     "Int32".to_string()
/// );
///
/// assert_eq!(sig.param_types(), &["Int32", "Int32"]);
/// assert_eq!(sig.return_type(), "Int32");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeSignature {
    /// Parameter types observed at call site
    params: Vec<String>,
    /// Return type observed from function execution
    return_type: String,
}

impl TypeSignature {
    /// Create a new type signature
    ///
    /// # Arguments
    ///
    /// * `params` - Vector of parameter type names (e.g., ["Int32", "Float64"])
    /// * `return_type` - Return type name (e.g., "Int32")
    pub fn new(params: Vec<String>, return_type: String) -> Self {
        Self {
            params,
            return_type,
        }
    }

    /// Get parameter types as slice
    pub fn param_types(&self) -> &[String] {
        &self.params
    }

    /// Get return type as string slice
    pub fn return_type(&self) -> &str {
        &self.return_type
    }
}

/// Type stability analysis (monomorphic, polymorphic, megamorphic)
///
/// Classifies functions based on the number of distinct type signatures observed.
/// Inspired by Julia's inline caching and type inference engine.
///
/// - **Monomorphic**: Single type signature (best for optimization, can inline aggressively)
/// - **Polymorphic**: 2-3 type signatures (moderate optimization, use inline cache)
/// - **Megamorphic**: 4+ type signatures (poor optimization potential, generic dispatch)
///
/// See Julia's `jl_specializations_get_linfo_()` for similar classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability {
    /// Single type signature observed (1 unique signature)
    Monomorphic,
    /// Multiple type signatures observed (2-3 unique signatures)
    Polymorphic,
    /// Many type signatures observed (4+ unique signatures)
    Megamorphic,
}

/// Optimization opportunity kind
///
/// Identifies specific types of optimizations that can be applied to Ruchy code.
/// Each variant includes context needed to apply the optimization.
///
/// Based on analysis of benchmarking results showing:
/// - Constant folding: 10-20% speedup
/// - Inlining: 5-30% speedup (depends on call frequency)
/// - Tail-call optimization: 15-25% speedup for recursive functions
/// - Loop unrolling: 10-30% speedup for tight loops
#[derive(Debug, Clone, PartialEq)]
pub enum OptKind {
    /// Constant folding opportunity (e.g., `2 + 3 * 4` → `14`)
    ConstantFolding {
        /// Expression that can be folded
        expr: String,
        /// Computed constant value
        value: String,
    },
    /// Function inlining opportunity
    Inlining {
        /// Function name to inline
        function: String,
        /// Number of call sites (affects cost/benefit)
        call_count: usize,
    },
    /// Tail-call optimization opportunity
    TailCallOpt {
        /// Function with tail-recursive call
        function: String,
    },
    /// Loop unrolling opportunity
    LoopUnrolling {
        /// Loop identifier
        loop_id: usize,
        /// Number of iterations (if known at compile time)
        iterations: usize,
    },
}

/// Optimization opportunity with location and estimated impact
///
/// Combines the optimization kind with location information and performance estimates.
/// Used to prioritize optimization passes by expected ROI.
///
/// # Example
///
/// ```rust
/// use ruchyruchy::profiler::{OptimizationOpportunity, OptKind};
///
/// let opt = OptimizationOpportunity {
///     kind: OptKind::ConstantFolding {
///         expr: "2 + 3 * 4".to_string(),
///         value: "14".to_string(),
///     },
///     location: "test.ruchy:10:5".to_string(),
///     estimated_speedup: 1.15, // 15% speedup
///     confidence: 0.9,         // 90% confidence
/// };
/// ```
#[derive(Debug, Clone)]
pub struct OptimizationOpportunity {
    /// Type of optimization to apply
    pub kind: OptKind,
    /// Source location (file:line:col format)
    pub location: String,
    /// Estimated speedup multiplier (1.15 = 15% faster)
    pub estimated_speedup: f64,
    /// Confidence in estimate (0.0 to 1.0)
    pub confidence: f64,
}

/// Compilation phase timing report
///
/// Tracks time spent in each compilation phase (lexing, parsing, type checking, codegen).
/// Used to identify bottlenecks in the compilation pipeline.
///
/// # Example
///
/// ```rust
/// use ruchyruchy::profiler::PhaseReport;
/// use std::time::Duration;
///
/// let mut report = PhaseReport::new();
/// report.add_phase("lexing".to_string(), Duration::from_millis(10));
/// report.add_phase("parsing".to_string(), Duration::from_millis(20));
///
/// assert!(report.contains_phase("lexing"));
/// assert_eq!(report.phase_time("lexing").as_millis(), 10);
/// assert_eq!(report.total_time().as_millis(), 30);
/// ```
#[derive(Debug, Clone)]
pub struct PhaseReport {
    /// Map from phase name to duration
    phases: std::collections::HashMap<String, Duration>,
}

impl PhaseReport {
    /// Create a new empty phase report
    pub fn new() -> Self {
        Self {
            phases: std::collections::HashMap::new(),
        }
    }

    /// Add a phase timing to the report
    ///
    /// # Arguments
    ///
    /// * `name` - Phase name (e.g., "lexing", "parsing")
    /// * `duration` - Time spent in this phase
    pub fn add_phase(&mut self, name: String, duration: Duration) {
        self.phases.insert(name, duration);
    }

    /// Check if a phase exists in the report
    ///
    /// # Arguments
    ///
    /// * `name` - Phase name to check
    ///
    /// # Returns
    ///
    /// `true` if phase was tracked, `false` otherwise
    pub fn contains_phase(&self, name: &str) -> bool {
        self.phases.contains_key(name)
    }

    /// Get the time spent in a specific phase
    ///
    /// # Arguments
    ///
    /// * `name` - Phase name to query
    ///
    /// # Returns
    ///
    /// Duration for the phase, or `Duration::ZERO` if phase not found
    pub fn phase_time(&self, name: &str) -> Duration {
        self.phases.get(name).copied().unwrap_or(Duration::ZERO)
    }

    /// Get total time across all phases
    ///
    /// # Returns
    ///
    /// Sum of all phase durations
    pub fn total_time(&self) -> Duration {
        self.phases.values().sum()
    }
}

impl Default for PhaseReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Function-level profiling data for JIT compilation decisions (INTERP-049)
///
/// Tracks call count, execution time, and percentage of total time for a single function.
/// Used to identify hot functions that are good candidates for JIT compilation.
///
/// # JIT Decision Criteria
///
/// A function is a JIT candidate if:
/// - **High call count**: >1000 calls (indicates hot path)
/// - **High time percentage**: >30% of total execution time (Amdahl's Law)
///
/// # Example
///
/// ```rust
/// use ruchyruchy::profiler::FunctionProfile;
///
/// let profile = FunctionProfile {
///     name: "fibonacci".to_string(),
///     call_count: 15,
///     total_time_us: 1234.56,
/// };
///
/// assert_eq!(profile.name, "fibonacci");
/// assert_eq!(profile.call_count, 15);
/// assert!(profile.total_time_us > 1000.0);
/// ```
#[derive(Debug, Clone)]
pub struct FunctionProfile {
    /// Function name
    pub name: String,
    /// Number of times this function was called
    pub call_count: usize,
    /// Total execution time in microseconds
    pub total_time_us: f64,
}
