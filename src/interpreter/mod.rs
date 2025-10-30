// INTERP-001: Interpreter Module
// Phase 3: Runtime Bug Discovery via Example Execution
//
// This module implements a tree-walking interpreter for Ruchy
// to systematically discover runtime bugs by executing all
// examples from ruchy-book (212 examples across 17 chapters)
//
// Research Foundation:
// - Aho et al. (2006): Compiler Design
// - Ierusalimschy et al. (2007): Lua Implementation
// - Cadar et al. (2008): KLEE Symbolic Execution
//
// Architecture: Tree-walking AST interpreter
// Goal: Discover 50+ runtime bugs, >90% path coverage

pub mod parser;

// Re-export main types for convenience
pub use parser::{Parser, ParseError, Ast, AstNode};
