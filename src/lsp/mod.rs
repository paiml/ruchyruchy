// LSP Base Protocol Implementation for RuchyRuchy
// IDE-001: Language Server Protocol base implementation
//
// Implements:
// - JSON-RPC protocol layer
// - Text document synchronization
// - Basic diagnostics
// - Position/range utilities

pub mod completion;
pub mod diagnostics;
pub mod protocol;
pub mod server;
pub mod symbols;
pub mod text_sync;

pub use protocol::{
    CompletionItem, CompletionItemKind, Diagnostic, DiagnosticSeverity, Location, Position, Range,
};
pub use server::LspServer;
