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
pub mod bootstrap_pipeline;
pub mod bootstrap_showcase;
pub mod performance_benchmark;
pub mod stage3_real_codegen;

// Re-export key types and functions
pub use bootstrap_pipeline::*;
pub use stage3_real_codegen::*;

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
