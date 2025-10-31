// LSP Base Protocol Implementation for RuchyRuchy
// IDE-001: Language Server Protocol base implementation
//
// Implements:
// - JSON-RPC protocol layer
// - Text document synchronization
// - Basic diagnostics
// - Position/range utilities

/// Code completion support
pub mod completion;
/// Diagnostic reporting
pub mod diagnostics;
/// LSP protocol types and messages
pub mod protocol;
/// LSP server implementation
pub mod server;
/// Symbol resolution
pub mod symbols;
/// Text document synchronization
pub mod text_sync;

pub use protocol::{
    CompletionItem, CompletionItemKind, Diagnostic, DiagnosticSeverity, Location, Position, Range,
};
pub use server::LspServer;
