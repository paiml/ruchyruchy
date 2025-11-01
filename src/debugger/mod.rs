// Debugger Module
//
// This module provides debugging capabilities for Ruchy programs including:
// - DEBUGGER-046: Interactive REPL debugger with time-travel (bashrs/matklad pattern)
// - DEBUGGER-047: Performance profiler with flame graphs (paiml-mcp-agent-toolkit pattern)

/// Interactive REPL debugger with time-travel capabilities
pub mod repl_debugger;

/// Performance profiler with flame graph generation
pub mod performance_profiler;

// Re-export main types for convenience
pub use performance_profiler::{PerformanceProfiler, ProfileReport};
pub use repl_debugger::{DebugCommand, DebugSession, StepResult};
