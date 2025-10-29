//! RuchyRuchy - Educational Bootstrap Compiler Infrastructure
//!
//! This crate provides educational resources and debugging tools for the
//! Ruchy programming language ecosystem.
//!
//! # Components
//!
//! - **Bootstrap Compiler**: Educational implementation of compiler stages
//! - **Debugging Tools**: Validation and testing utilities
//! - **Performance Benchmarks**: Speed validation tools
//!
//! # Usage
//!
//! ```bash
//! # Install the debugging validation tool
//! cargo install ruchyruchy
//!
//! # Run debugging validation
//! ruchydbg validate
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

// Re-export the bootstrap pipeline components
/// Bootstrap pipeline module for compiler stages
pub mod bootstrap_pipeline;
/// Bootstrap showcase examples and demonstrations
pub mod bootstrap_showcase;
/// Performance benchmarking utilities
pub mod performance_benchmark;
/// Stage 3 code generation (real Rust codegen)
pub mod stage3_real_codegen;
/// Language Server Protocol (LSP) implementation
pub mod lsp;
/// Bug discovery, replication, and reporting system (DISC-001 through DISC-004)
pub mod bug_discovery;
/// Bug replication module (REPLIC-001 through REPLIC-003)
pub mod bug_replication;
/// Bug reporting module (REPORT-001 through REPORT-004)
pub mod bug_reporting;
/// Tracing infrastructure (DEBUGGER-014: Zero-cost compiler instrumentation)
pub mod tracing;
/// Statistical profiling (DEBUGGER-016: perf_event_open-based profiler)
pub mod profiling;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get the RuchyRuchy version
pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }
}
