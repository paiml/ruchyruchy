// Text Document Synchronization
// Manages text document state and changes

use std::collections::HashMap;

/// Text document manager
pub struct TextDocumentManager {
    documents: HashMap<String, TextDocument>,
}

/// Text document state
pub struct TextDocument {
    pub uri: String,
    pub version: i32,
    pub text: String,
}

impl TextDocumentManager {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    /// Open a text document
    pub fn open(&mut self, uri: String, version: i32, text: String) {
        self.documents
            .insert(uri.clone(), TextDocument { uri, version, text });
    }

    /// Change a text document (full content replacement)
    pub fn change(&mut self, uri: &str, version: i32, text: String) -> bool {
        if let Some(doc) = self.documents.get_mut(uri) {
            doc.version = version;
            doc.text = text;
            true
        } else {
            false
        }
    }

    /// Close a text document
    pub fn close(&mut self, uri: &str) -> bool {
        self.documents.remove(uri).is_some()
    }

    /// Get a text document
    pub fn get(&self, uri: &str) -> Option<&TextDocument> {
        self.documents.get(uri)
    }

    /// Get document text
    pub fn get_text(&self, uri: &str) -> Option<&str> {
        self.documents.get(uri).map(|doc| doc.text.as_str())
    }
}

impl Default for TextDocumentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_document() {
        let mut manager = TextDocumentManager::new();
        manager.open(
            "file:///test.ruchy".to_string(),
            1,
            "fun main() {}".to_string(),
        );

        let doc = manager.get("file:///test.ruchy");
        assert!(doc.is_some());
        assert_eq!(doc.unwrap().version, 1);
        assert_eq!(doc.unwrap().text, "fun main() {}");
    }

    #[test]
    fn test_change_document() {
        let mut manager = TextDocumentManager::new();
        manager.open(
            "file:///test.ruchy".to_string(),
            1,
            "fun main() {}".to_string(),
        );

        let changed = manager.change(
            "file:///test.ruchy",
            2,
            "fun main() { println(\"hi\") }".to_string(),
        );
        assert!(changed);

        let doc = manager.get("file:///test.ruchy");
        assert_eq!(doc.unwrap().version, 2);
        assert_eq!(doc.unwrap().text, "fun main() { println(\"hi\") }");
    }

    #[test]
    fn test_close_document() {
        let mut manager = TextDocumentManager::new();
        manager.open(
            "file:///test.ruchy".to_string(),
            1,
            "fun main() {}".to_string(),
        );

        let closed = manager.close("file:///test.ruchy");
        assert!(closed);

        let doc = manager.get("file:///test.ruchy");
        assert!(doc.is_none());
    }

    #[test]
    fn test_get_text() {
        let mut manager = TextDocumentManager::new();
        manager.open(
            "file:///test.ruchy".to_string(),
            1,
            "fun main() {}".to_string(),
        );

        let text = manager.get_text("file:///test.ruchy");
        assert_eq!(text, Some("fun main() {}"));
    }

    #[test]
    fn test_change_nonexistent() {
        let mut manager = TextDocumentManager::new();
        let changed = manager.change("file:///nonexistent.ruchy", 1, "text".to_string());
        assert!(!changed);
    }
}
