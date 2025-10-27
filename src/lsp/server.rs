// LSP Server
// Main LSP server implementation

use super::diagnostics::DiagnosticsProvider;
use super::protocol::{Diagnostic, TextDocumentItem};
use super::text_sync::TextDocumentManager;
use std::sync::{Arc, Mutex};

/// LSP Server state
pub struct LspServer {
    text_documents: Arc<Mutex<TextDocumentManager>>,
    diagnostics: DiagnosticsProvider,
    initialized: bool,
}

impl LspServer {
    /// Create a new LSP server
    pub fn new() -> Self {
        Self {
            text_documents: Arc::new(Mutex::new(TextDocumentManager::new())),
            diagnostics: DiagnosticsProvider::new(),
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
}
