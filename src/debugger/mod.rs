// Debugger Module
//
// This module provides debugging capabilities for Ruchy programs including:
// - DEBUGGER-046: Interactive REPL debugger with time-travel (bashrs/matklad pattern)
// - DEBUGGER-047: Performance profiler with flame graphs (paiml-mcp-agent-toolkit pattern)

/// Interactive REPL debugger with time-travel capabilities
pub mod repl_debugger;

/// Performance profiler with flame graph generation
pub mod performance_profiler;

/// DEBUGGER-050: Tokenization debugging tools (GREEN phase)
pub mod tokenizer;

/// DEBUGGER-050: AST visualization tools (GREEN Phase Priority 2)
pub mod ast_viz;

/// DEBUGGER-052: JIT compiler debugger with Cranelift IR inspection (RED Phase)
pub mod jit;

// Re-export main types for convenience
pub use performance_profiler::{PerformanceProfiler, ProfileReport};
pub use repl_debugger::{DebugCommand, DebugSession, StepResult};

// DEBUGGER-050: Parser Debugger with Token Stream Inspection (GREEN Phase)
// Priority 1: Tokenization tools (GitHub issue #13)
// Toyota Way: Genchi Genbutsu - Every function addresses real debugging pain from PARSER-079

pub use tokenizer::TokenAnalysis;

/// Show detailed token stream with source locations (DEBUGGER-050 Priority 1)
pub fn tokenize(source: &str) -> String {
    tokenizer::tokenize(source)
}

/// Show token stream with error highlighting (DEBUGGER-050 Priority 1)
pub fn tokenize_with_errors(source: &str) -> String {
    tokenizer::tokenize_with_errors(source)
}

/// Analyze tokens for pattern conflicts (DEBUGGER-050 Priority 1)
pub fn tokenize_analyze(source: &str) -> TokenAnalysis {
    tokenizer::tokenize_analyze(source)
}

/// Side-by-side token comparison (DEBUGGER-050 Priority 1)
pub fn compare_tokens(working: &str, broken: &str) -> String {
    tokenizer::compare_tokens(working, broken)
}

/// Compare tokens with root cause hints (DEBUGGER-050 Priority 1)
pub fn compare_tokens_with_hints(working: &str, broken: &str) -> String {
    tokenizer::compare_tokens_with_hints(working, broken)
}

/// Show parser state at failure (DEBUGGER-050 Priority 1)
pub fn parser_trace(source: &str) -> String {
    tokenizer::parser_trace(source)
}

/// Parser trace with root cause analysis (DEBUGGER-050 Priority 1)
pub fn parser_trace_with_analysis(source: &str) -> String {
    tokenizer::parser_trace_with_analysis(source)
}

/// Show only failing portion of parse trace (DEBUGGER-050 Priority 1)
pub fn parser_trace_errors_only(source: &str) -> String {
    tokenizer::parser_trace_errors_only(source)
}

// Priority 2: AST visualization tools (GREEN Phase)
/// Generate AST as JSON (DEBUGGER-050 Priority 2)
pub fn visualize_ast(source: &str) -> String {
    ast_viz::visualize_ast(source)
}

/// Generate AST as Graphviz DOT format (DEBUGGER-050 Priority 2)
pub fn visualize_ast_graphviz(source: &str) -> String {
    ast_viz::visualize_ast_graphviz(source)
}

/// Visualize AST with source locations (DEBUGGER-050 Priority 2)
pub fn visualize_ast_with_locations(source: &str) -> String {
    ast_viz::visualize_ast_with_locations(source)
}

/// Show partial AST on parse error (DEBUGGER-050 Priority 2)
pub fn visualize_ast_partial(source: &str) -> Result<String, String> {
    ast_viz::visualize_ast_partial(source)
}

/// Compare ASTs from two code versions (DEBUGGER-050 Priority 2)
pub fn ast_diff(before: &str, after: &str) -> String {
    ast_viz::ast_diff(before, after)
}

/// Show AST construction step-by-step (DEBUGGER-050 Priority 2)
pub fn visualize_ast_steps(source: &str) -> Vec<String> {
    ast_viz::visualize_ast_steps(source)
}

/// Show AST with inferred types (DEBUGGER-050 Priority 2)
pub fn visualize_typed_ast(source: &str) -> String {
    ast_viz::visualize_typed_ast(source)
}
