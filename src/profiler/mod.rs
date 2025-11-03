//! Compiler Profiling Tool (PROFILER-001)
//!
//! Internal tool for Ruchy compiler developers to optimize performance.
//! Extends DEBUGGER-047 PerformanceProfiler with compiler-specific tracking.
//!
//! # Purpose
//!
//! This profiler helps Ruchy compiler developers identify optimization opportunities by:
//!
//! 1. **Phase Tracking**: Measure time in lexing, parsing, type checking, codegen
//! 2. **Type Observation**: Track runtime type signatures (Julia-inspired)
//! 3. **Hot Function Detection**: Identify functions consuming >1% of execution time
//! 4. **Optimization Discovery**: Find constant folding, inlining, TCO opportunities
//! 5. **Cross-Mode Comparison**: Compare AST/Bytecode/Transpiled/Compiled performance
//!
//! # Example
//!
//! ```rust
//! use ruchyruchy::profiler::CompilerProfiler;
//! use std::time::Duration;
//!
//! let mut profiler = CompilerProfiler::new();
//!
//! // Track compilation phases
//! profiler.start_phase("lexing");
//! // ... perform lexing ...
//! profiler.end_phase("lexing");
//!
//! // Get report
//! let report = profiler.phase_report();
//! assert!(report.contains_phase("lexing"));
//! ```
//!
//! # Implementation Status
//!
//! - **Phase 1** (Current): Infrastructure and phase tracking
//! - **Phase 2** (Planned): Type observation and hot function detection
//! - **Phase 3** (Planned): AST optimizer and optimization passes
//! - **Phase 4** (Planned): CLI and reporting

pub mod compiler_profiler;
pub mod types;

pub use compiler_profiler::CompilerProfiler;
pub use types::{
    ExecutionMode, FunctionProfile, OptKind, OptimizationOpportunity, PhaseReport, Stability,
    TypeSignature,
};
