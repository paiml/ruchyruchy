// Compiler module for RuchyRuchy
//
// Transpiles Ruchy AST to Rust source code
//
// Architecture:
// - codegen.rs: AST → Rust code generation
// - emit.rs: Code formatting and emission (future)
// - runtime.rs: Runtime library for compiled code (future)

/// Code generation module
pub mod codegen;

pub use codegen::CodeGenerator;

/// Compilation errors
#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    /// Parse error from input
    ParseError(String),
    /// Code generation error
    CodeGenError(String),
    /// Unsupported feature
    UnsupportedFeature(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CompileError::CodeGenError(msg) => write!(f, "Code generation error: {}", msg),
            CompileError::UnsupportedFeature(msg) => write!(f, "Unsupported feature: {}", msg),
        }
    }
}

impl std::error::Error for CompileError {}
