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

pub mod bug_discovery;
pub mod evaluator;
pub mod parser;
pub mod scope;
pub mod value;

// Re-export main types for convenience
pub use bug_discovery::{BugDiscoveryAnalyzer, InterpreterBugReport};
pub use evaluator::{EvalError, Evaluator};
pub use parser::{Ast, AstNode, ParseError, Parser};
pub use scope::{Scope, ScopeError};
pub use value::{Value, ValueError};
