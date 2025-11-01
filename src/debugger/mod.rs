// DEBUGGER-046: Interactive REPL Debugger Module
//
// This module provides interactive debugging capabilities for Ruchy programs
// based on the bashrs REPL debugger pattern (matklad's debugger-as-REPL).

/// Interactive REPL debugger with time-travel capabilities
pub mod repl_debugger;

// Re-export main types for convenience
pub use repl_debugger::{DebugCommand, DebugSession, StepResult};
