// LSP Protocol Types
// Based on Language Server Protocol 3.17 specification

use serde::{Deserialize, Serialize};

/// Position in a text document (zero-based)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// Line position (zero-based)
    pub line: u32,
    /// Character offset on a line (zero-based, UTF-16 code units)
    pub character: u32,
}

impl Position {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

/// Range in a text document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    /// Start position (inclusive)
    pub start: Position,
    /// End position (exclusive)
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Diagnostic severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// Diagnostic (error, warning, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Range where the diagnostic applies
    pub range: Range,
    /// Diagnostic severity
    pub severity: Option<DiagnosticSeverity>,
    /// Diagnostic code
    pub code: Option<String>,
    /// Source of the diagnostic (e.g., "ruchy")
    pub source: Option<String>,
    /// Diagnostic message
    pub message: String,
}

impl Diagnostic {
    pub fn error(range: Range, message: String) -> Self {
        Self {
            range,
            severity: Some(DiagnosticSeverity::Error),
            code: None,
            source: Some("ruchy".to_string()),
            message,
        }
    }

    pub fn warning(range: Range, message: String) -> Self {
        Self {
            range,
            severity: Some(DiagnosticSeverity::Warning),
            code: None,
            source: Some("ruchy".to_string()),
            message,
        }
    }
}

/// Text document identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

/// Versioned text document identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: String,
    pub version: i32,
}

/// Text document item (full document content)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextDocumentItem {
    pub uri: String,
    pub language_id: String,
    pub version: i32,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.character, 10);
    }

    #[test]
    fn test_range_creation() {
        let start = Position::new(0, 0);
        let end = Position::new(0, 5);
        let range = Range::new(start, end);
        assert_eq!(range.start, start);
        assert_eq!(range.end, end);
    }

    #[test]
    fn test_diagnostic_error() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 5));
        let diag = Diagnostic::error(range, "Test error".to_string());
        assert_eq!(diag.severity, Some(DiagnosticSeverity::Error));
        assert_eq!(diag.message, "Test error");
        assert_eq!(diag.source, Some("ruchy".to_string()));
    }

    #[test]
    fn test_diagnostic_warning() {
        let range = Range::new(Position::new(1, 0), Position::new(1, 10));
        let diag = Diagnostic::warning(range, "Test warning".to_string());
        assert_eq!(diag.severity, Some(DiagnosticSeverity::Warning));
        assert_eq!(diag.message, "Test warning");
    }
}
