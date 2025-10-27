// LSP Server
// Main LSP server implementation

use super::completion::CompletionProvider;
use super::diagnostics::DiagnosticsProvider;
use super::protocol::{CompletionItem, Diagnostic, Location, Position, TextDocumentItem};
use super::symbols::SymbolTable;
use super::text_sync::TextDocumentManager;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// LSP Server state
pub struct LspServer {
    text_documents: Arc<Mutex<TextDocumentManager>>,
    diagnostics: DiagnosticsProvider,
    completion: CompletionProvider,
    symbol_tables: Arc<Mutex<HashMap<String, SymbolTable>>>,
    initialized: bool,
}

impl LspServer {
    /// Create a new LSP server
    pub fn new() -> Self {
        Self {
            text_documents: Arc::new(Mutex::new(TextDocumentManager::new())),
            diagnostics: DiagnosticsProvider::new(),
            completion: CompletionProvider::new(),
            symbol_tables: Arc::new(Mutex::new(HashMap::new())),
            initialized: false,
        }
    }

    /// Initialize the LSP server
    pub fn initialize(&mut self) -> bool {
        self.initialized = true;
        true
    }

    /// Check if server is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Handle text document open
    pub fn text_document_did_open(&mut self, item: TextDocumentItem) -> Vec<Diagnostic> {
        if !self.initialized {
            return vec![];
        }

        // Store document
        let mut docs = self.text_documents.lock().unwrap();
        docs.open(item.uri.clone(), item.version, item.text.clone());
        drop(docs);

        // Parse symbols
        let mut tables = self.symbol_tables.lock().unwrap();
        let mut table = SymbolTable::new();
        table.parse_document(&item.uri, &item.text);
        tables.insert(item.uri.clone(), table);
        drop(tables);

        // Run diagnostics
        self.diagnostics.check_file(&item.uri, &item.text)
    }

    /// Handle text document change
    pub fn text_document_did_change(
        &mut self,
        uri: String,
        version: i32,
        text: String,
    ) -> Vec<Diagnostic> {
        if !self.initialized {
            return vec![];
        }

        // Update document
        let mut docs = self.text_documents.lock().unwrap();
        docs.change(&uri, version, text.clone());
        drop(docs);

        // Run diagnostics
        self.diagnostics.check_file(&uri, &text)
    }

    /// Handle text document close
    pub fn text_document_did_close(&mut self, uri: String) {
        if !self.initialized {
            return;
        }

        let mut docs = self.text_documents.lock().unwrap();
        docs.close(&uri);
    }

    /// Get document text
    pub fn get_document_text(&self, uri: &str) -> Option<String> {
        let docs = self.text_documents.lock().unwrap();
        docs.get_text(uri).map(|s| s.to_string())
    }

    /// Get completion items at a position
    pub fn get_completions(&self, uri: &str, position: Position) -> Vec<CompletionItem> {
        if !self.initialized {
            return vec![];
        }

        // Get document text
        let docs = self.text_documents.lock().unwrap();
        let text = match docs.get_text(uri) {
            Some(t) => t,
            None => return vec![],
        };

        // Get completions from provider
        self.completion.get_completions(text, position)
    }

    /// Go to definition
    pub fn goto_definition(&self, uri: &str, position: Position) -> Option<Location> {
        if !self.initialized {
            return None;
        }

        // Get document text
        let docs = self.text_documents.lock().unwrap();
        let text = docs.get_text(uri)?;

        // Get symbol table
        let tables = self.symbol_tables.lock().unwrap();
        let table = tables.get(uri)?;

        // Find symbol at cursor position
        let symbol_name = table.find_symbol_at_position(text, position)?;

        // Look up symbol definition
        let symbol = table.get_symbol(&symbol_name)?;

        Some(symbol.location.clone())
    }

    /// Find all references
    pub fn find_references(&self, uri: &str, position: Position) -> Vec<Location> {
        if !self.initialized {
            return vec![];
        }

        // Get document text (convert to owned String to avoid lifetime issues)
        let text = {
            let docs = self.text_documents.lock().unwrap();
            match docs.get_text(uri) {
                Some(t) => t.to_string(),
                None => return vec![],
            }
        };

        // Get symbol table
        let tables = self.symbol_tables.lock().unwrap();
        let table = match tables.get(uri) {
            Some(t) => t,
            None => return vec![],
        };

        // Find symbol at cursor position
        let symbol_name = match table.find_symbol_at_position(&text, position) {
            Some(name) => name,
            None => return vec![],
        };

        // Get all references
        let mut locations = table.get_references(&symbol_name);

        // Add the definition location as well
        if let Some(symbol) = table.get_symbol(&symbol_name) {
            locations.push(symbol.location.clone());
        }

        locations
    }

    /// Shutdown the server
    pub fn shutdown(&mut self) {
        self.initialized = false;
    }
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_initialize() {
        let mut server = LspServer::new();
        assert!(!server.is_initialized());

        let result = server.initialize();
        assert!(result);
        assert!(server.is_initialized());
    }

    #[test]
    fn test_text_document_open() {
        let mut server = LspServer::new();
        server.initialize();

        let item = TextDocumentItem {
            uri: "file:///test.ruchy".to_string(),
            language_id: "ruchy".to_string(),
            version: 1,
            text: "fun main() {}".to_string(),
        };

        let _diagnostics = server.text_document_did_open(item);

        let text = server.get_document_text("file:///test.ruchy");
        assert_eq!(text, Some("fun main() {}".to_string()));
    }

    #[test]
    fn test_text_document_change() {
        let mut server = LspServer::new();
        server.initialize();

        let item = TextDocumentItem {
            uri: "file:///test.ruchy".to_string(),
            language_id: "ruchy".to_string(),
            version: 1,
            text: "fun main() {}".to_string(),
        };

        server.text_document_did_open(item);

        let _diagnostics = server.text_document_did_change(
            "file:///test.ruchy".to_string(),
            2,
            "fun main() { println(\"hi\") }".to_string(),
        );

        let text = server.get_document_text("file:///test.ruchy");
        assert_eq!(text, Some("fun main() { println(\"hi\") }".to_string()));
    }

    #[test]
    fn test_text_document_close() {
        let mut server = LspServer::new();
        server.initialize();

        let item = TextDocumentItem {
            uri: "file:///test.ruchy".to_string(),
            language_id: "ruchy".to_string(),
            version: 1,
            text: "fun main() {}".to_string(),
        };

        server.text_document_did_open(item);
        server.text_document_did_close("file:///test.ruchy".to_string());

        let text = server.get_document_text("file:///test.ruchy");
        assert_eq!(text, None);
    }

    #[test]
    fn test_operations_before_initialize() {
        let mut server = LspServer::new();

        let item = TextDocumentItem {
            uri: "file:///test.ruchy".to_string(),
            language_id: "ruchy".to_string(),
            version: 1,
            text: "fun main() {}".to_string(),
        };

        let diagnostics = server.text_document_did_open(item);
        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_shutdown() {
        let mut server = LspServer::new();
        server.initialize();
        assert!(server.is_initialized());

        server.shutdown();
        assert!(!server.is_initialized());
    }

    #[test]
    fn test_get_completions() {
        let mut server = LspServer::new();
        server.initialize();

        let item = TextDocumentItem {
            uri: "file:///test.ruchy".to_string(),
            language_id: "ruchy".to_string(),
            version: 1,
            text: "fun main() {}".to_string(),
        };

        server.text_document_did_open(item);

        let completions = server.get_completions("file:///test.ruchy", Position::new(0, 0));

        // Should have keywords, types, and functions
        assert!(completions.len() > 30);
        assert!(completions.iter().any(|c| c.label == "fun"));
        assert!(completions.iter().any(|c| c.label == "let"));
        assert!(completions.iter().any(|c| c.label == "i32"));
    }

    #[test]
    fn test_get_completions_before_initialize() {
        let server = LspServer::new();
        let completions = server.get_completions("file:///test.ruchy", Position::new(0, 0));
        assert_eq!(completions.len(), 0);
    }

    #[test]
    fn test_get_completions_nonexistent_document() {
        let mut server = LspServer::new();
        server.initialize();

        let completions = server.get_completions("file:///nonexistent.ruchy", Position::new(0, 0));
        assert_eq!(completions.len(), 0);
    }

    #[test]
    fn test_goto_definition() {
        let mut server = LspServer::new();
        server.initialize();

        let item = TextDocumentItem {
            uri: "file:///test.ruchy".to_string(),
            language_id: "ruchy".to_string(),
            version: 1,
            text: "fun main() {\n    let x = 5\n}".to_string(),
        };

        server.text_document_did_open(item);

        // Test go-to-definition on "main" (position at character 5 on line 0)
        let location = server.goto_definition("file:///test.ruchy", Position::new(0, 5));
        assert!(location.is_some());
        let loc = location.unwrap();
        assert_eq!(loc.uri, "file:///test.ruchy");
        assert_eq!(loc.range.start.line, 0);

        // Test go-to-definition on "x" (position at character 8 on line 1)
        let location = server.goto_definition("file:///test.ruchy", Position::new(1, 8));
        assert!(location.is_some());
        let loc = location.unwrap();
        assert_eq!(loc.uri, "file:///test.ruchy");
        assert_eq!(loc.range.start.line, 1);
    }

    #[test]
    fn test_goto_definition_before_initialize() {
        let server = LspServer::new();
        let location = server.goto_definition("file:///test.ruchy", Position::new(0, 0));
        assert_eq!(location, None);
    }

    #[test]
    fn test_goto_definition_nonexistent_document() {
        let mut server = LspServer::new();
        server.initialize();

        let location = server.goto_definition("file:///nonexistent.ruchy", Position::new(0, 0));
        assert_eq!(location, None);
    }

    #[test]
    fn test_find_references() {
        let mut server = LspServer::new();
        server.initialize();

        let item = TextDocumentItem {
            uri: "file:///test.ruchy".to_string(),
            language_id: "ruchy".to_string(),
            version: 1,
            text: "fun main() {\n    let x = 5\n}".to_string(),
        };

        server.text_document_did_open(item);

        // Test find-references on "main" (should find definition)
        let refs = server.find_references("file:///test.ruchy", Position::new(0, 5));
        assert_eq!(refs.len(), 1); // Just the definition for now
        assert_eq!(refs[0].uri, "file:///test.ruchy");
    }

    #[test]
    fn test_find_references_before_initialize() {
        let server = LspServer::new();
        let refs = server.find_references("file:///test.ruchy", Position::new(0, 0));
        assert_eq!(refs.len(), 0);
    }

    #[test]
    fn test_find_references_nonexistent_document() {
        let mut server = LspServer::new();
        server.initialize();

        let refs = server.find_references("file:///nonexistent.ruchy", Position::new(0, 0));
        assert_eq!(refs.len(), 0);
    }
}
