// LSP Base Protocol Implementation for RuchyRuchy
// IDE-001: Language Server Protocol base implementation
//
// Implements:
// - JSON-RPC protocol layer
// - Text document synchronization
// - Basic diagnostics
// - Position/range utilities

pub mod protocol;
pub mod server;
pub mod diagnostics;
pub mod text_sync;
pub mod completion;
pub mod symbols;

pub use server::LspServer;
pub use protocol::{Position, Range, Diagnostic, DiagnosticSeverity, CompletionItem, CompletionItemKind, Location};
