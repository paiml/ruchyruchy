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

/// Location in a document
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

impl Location {
    pub fn new(uri: String, range: Range) -> Self {
        Self { uri, range }
    }
}

/// Completion item kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionItemKind {
    /// Plain text completion
    Text = 1,
    /// Method or member function
    Method = 2,
    /// Function or procedure
    Function = 3,
    /// Constructor
    Constructor = 4,
    /// Field or property of a class/struct
    Field = 5,
    /// Variable
    Variable = 6,
    /// Class
    Class = 7,
    /// Interface
    Interface = 8,
    /// Module
    Module = 9,
    /// Property
    Property = 10,
    /// Unit value
    Unit = 11,
    /// Value
    Value = 12,
    /// Enumeration
    Enum = 13,
    /// Language keyword
    Keyword = 14,
    /// Code snippet
    Snippet = 15,
    /// Color value
    Color = 16,
    /// File reference
    File = 17,
    /// Reference to another symbol
    Reference = 18,
    /// Folder or directory
    Folder = 19,
    /// Enumeration member
    EnumMember = 20,
    /// Constant value
    Constant = 21,
    /// Struct or record type
    Struct = 22,
    /// Event
    Event = 23,
    /// Operator
    Operator = 24,
    /// Type parameter or generic
    TypeParameter = 25,
}

/// Completion item
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionItem {
    /// The label of this completion item
    pub label: String,
    /// The kind of this completion item
    pub kind: Option<CompletionItemKind>,
    /// A human-readable string with additional information
    pub detail: Option<String>,
    /// A human-readable string that represents a doc-comment
    pub documentation: Option<String>,
    /// A string that should be inserted when selecting this completion
    pub insert_text: Option<String>,
}

impl CompletionItem {
    /// Create a new completion item
    pub fn new(label: String, kind: CompletionItemKind) -> Self {
        Self {
            label,
            kind: Some(kind),
            detail: None,
            documentation: None,
            insert_text: None,
        }
    }

    /// Add a detail string to this completion item
    pub fn with_detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }

    /// Add documentation to this completion item
    pub fn with_documentation(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Add insert text to this completion item
    pub fn with_insert_text(mut self, insert_text: String) -> Self {
        self.insert_text = Some(insert_text);
        self
    }
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

    #[test]
    fn test_completion_item_creation() {
        let item = CompletionItem::new("fun".to_string(), CompletionItemKind::Keyword);
        assert_eq!(item.label, "fun");
        assert_eq!(item.kind, Some(CompletionItemKind::Keyword));
        assert_eq!(item.detail, None);
    }

    #[test]
    fn test_completion_item_with_detail() {
        let item = CompletionItem::new("fun".to_string(), CompletionItemKind::Keyword)
            .with_detail("Function declaration keyword".to_string());
        assert_eq!(
            item.detail,
            Some("Function declaration keyword".to_string())
        );
    }

    #[test]
    fn test_completion_item_with_documentation() {
        let item = CompletionItem::new("fun".to_string(), CompletionItemKind::Keyword)
            .with_documentation("Declares a new function".to_string());
        assert_eq!(
            item.documentation,
            Some("Declares a new function".to_string())
        );
    }

    #[test]
    fn test_completion_item_builder() {
        let item = CompletionItem::new("println".to_string(), CompletionItemKind::Function)
            .with_detail("fn println(msg: String)".to_string())
            .with_documentation("Prints a message to stdout".to_string())
            .with_insert_text("println($0)".to_string());

        assert_eq!(item.label, "println");
        assert_eq!(item.kind, Some(CompletionItemKind::Function));
        assert!(item.detail.is_some());
        assert!(item.documentation.is_some());
        assert_eq!(item.insert_text, Some("println($0)".to_string()));
    }
}
