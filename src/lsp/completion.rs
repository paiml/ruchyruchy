// Code Completion Provider
// Provides intelligent code suggestions

use super::protocol::{CompletionItem, CompletionItemKind, Position};

/// Completion provider
pub struct CompletionProvider;

impl CompletionProvider {
    /// Create a new completion provider
    pub fn new() -> Self {
        Self
    }

    /// Get completion items at a given position
    pub fn get_completions(
        &self,
        _document_text: &str,
        _position: Position,
    ) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        // Add keyword completions
        items.extend(self.get_keyword_completions());

        // Add type completions
        items.extend(self.get_type_completions());

        // Add function completions
        items.extend(self.get_function_completions());

        items
    }

    /// Get keyword completions
    fn get_keyword_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem::new("fun".to_string(), CompletionItemKind::Keyword)
                .with_detail("Function declaration".to_string())
                .with_documentation("Declares a new function".to_string())
                .with_insert_text("fun $1($2) {\n\t$0\n}".to_string()),
            CompletionItem::new("let".to_string(), CompletionItemKind::Keyword)
                .with_detail("Variable binding".to_string())
                .with_documentation("Declares a new variable binding".to_string())
                .with_insert_text("let $1 = $0".to_string()),
            CompletionItem::new("if".to_string(), CompletionItemKind::Keyword)
                .with_detail("Conditional expression".to_string())
                .with_documentation("Conditional branching".to_string())
                .with_insert_text("if $1 {\n\t$0\n}".to_string()),
            CompletionItem::new("else".to_string(), CompletionItemKind::Keyword)
                .with_detail("Else branch".to_string())
                .with_documentation("Alternative branch for if expression".to_string()),
            CompletionItem::new("match".to_string(), CompletionItemKind::Keyword)
                .with_detail("Pattern matching".to_string())
                .with_documentation("Pattern matching expression".to_string())
                .with_insert_text("match $1 {\n\t$0\n}".to_string()),
            CompletionItem::new("loop".to_string(), CompletionItemKind::Keyword)
                .with_detail("Infinite loop".to_string())
                .with_documentation("Creates an infinite loop".to_string())
                .with_insert_text("loop {\n\t$0\n}".to_string()),
            CompletionItem::new("while".to_string(), CompletionItemKind::Keyword)
                .with_detail("While loop".to_string())
                .with_documentation("Loop while condition is true".to_string())
                .with_insert_text("while $1 {\n\t$0\n}".to_string()),
            CompletionItem::new("for".to_string(), CompletionItemKind::Keyword)
                .with_detail("For loop".to_string())
                .with_documentation("Iterate over a collection".to_string())
                .with_insert_text("for $1 in $2 {\n\t$0\n}".to_string()),
            CompletionItem::new("return".to_string(), CompletionItemKind::Keyword)
                .with_detail("Return statement".to_string())
                .with_documentation("Returns a value from a function".to_string())
                .with_insert_text("return $0".to_string()),
            CompletionItem::new("break".to_string(), CompletionItemKind::Keyword)
                .with_detail("Break statement".to_string())
                .with_documentation("Exits a loop".to_string()),
            CompletionItem::new("continue".to_string(), CompletionItemKind::Keyword)
                .with_detail("Continue statement".to_string())
                .with_documentation("Skips to next loop iteration".to_string()),
            CompletionItem::new("type".to_string(), CompletionItemKind::Keyword)
                .with_detail("Type alias".to_string())
                .with_documentation("Declares a type alias".to_string())
                .with_insert_text("type $1 = $0".to_string()),
            CompletionItem::new("struct".to_string(), CompletionItemKind::Keyword)
                .with_detail("Struct definition".to_string())
                .with_documentation("Declares a new struct type".to_string())
                .with_insert_text("struct $1 {\n\t$0\n}".to_string()),
            CompletionItem::new("enum".to_string(), CompletionItemKind::Keyword)
                .with_detail("Enum definition".to_string())
                .with_documentation("Declares a new enum type".to_string())
                .with_insert_text("enum $1 {\n\t$0\n}".to_string()),
            CompletionItem::new("trait".to_string(), CompletionItemKind::Keyword)
                .with_detail("Trait definition".to_string())
                .with_documentation("Declares a new trait".to_string())
                .with_insert_text("trait $1 {\n\t$0\n}".to_string()),
            CompletionItem::new("impl".to_string(), CompletionItemKind::Keyword)
                .with_detail("Implementation".to_string())
                .with_documentation("Implements methods for a type".to_string())
                .with_insert_text("impl $1 {\n\t$0\n}".to_string()),
            CompletionItem::new("in".to_string(), CompletionItemKind::Keyword)
                .with_detail("In keyword".to_string())
                .with_documentation("Used in let-in expressions and for loops".to_string()),
            CompletionItem::new("true".to_string(), CompletionItemKind::Keyword)
                .with_detail("Boolean true".to_string())
                .with_documentation("Boolean literal true".to_string()),
            CompletionItem::new("false".to_string(), CompletionItemKind::Keyword)
                .with_detail("Boolean false".to_string())
                .with_documentation("Boolean literal false".to_string()),
        ]
    }

    /// Get type completions
    fn get_type_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem::new("i8".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("8-bit signed integer".to_string()),
            CompletionItem::new("i16".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("16-bit signed integer".to_string()),
            CompletionItem::new("i32".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("32-bit signed integer".to_string()),
            CompletionItem::new("i64".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("64-bit signed integer".to_string()),
            CompletionItem::new("u8".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("8-bit unsigned integer".to_string()),
            CompletionItem::new("u16".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("16-bit unsigned integer".to_string()),
            CompletionItem::new("u32".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("32-bit unsigned integer".to_string()),
            CompletionItem::new("u64".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("64-bit unsigned integer".to_string()),
            CompletionItem::new("f32".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("32-bit floating point".to_string()),
            CompletionItem::new("f64".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("64-bit floating point".to_string()),
            CompletionItem::new("bool".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("Boolean type".to_string()),
            CompletionItem::new("String".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("UTF-8 string type".to_string()),
            CompletionItem::new("str".to_string(), CompletionItemKind::TypeParameter)
                .with_detail("String slice type".to_string()),
        ]
    }

    /// Get function completions
    fn get_function_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem::new("println".to_string(), CompletionItemKind::Function)
                .with_detail("fn println(msg: String)".to_string())
                .with_documentation("Prints a message to stdout with newline".to_string())
                .with_insert_text("println($0)".to_string()),
            CompletionItem::new("print".to_string(), CompletionItemKind::Function)
                .with_detail("fn print(msg: String)".to_string())
                .with_documentation("Prints a message to stdout without newline".to_string())
                .with_insert_text("print($0)".to_string()),
        ]
    }
}

impl Default for CompletionProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_completions() {
        let provider = CompletionProvider::new();
        let completions = provider.get_completions("", Position::new(0, 0));

        // Should have keywords + types + functions
        assert!(completions.len() > 30);

        // Check for some expected completions
        assert!(completions.iter().any(|c| c.label == "fun"));
        assert!(completions.iter().any(|c| c.label == "let"));
        assert!(completions.iter().any(|c| c.label == "i32"));
        assert!(completions.iter().any(|c| c.label == "println"));
    }

    #[test]
    fn test_keyword_completions() {
        let provider = CompletionProvider::new();
        let keywords = provider.get_keyword_completions();

        assert!(keywords.len() >= 18);
        assert!(keywords.iter().any(|k| k.label == "fun"));
        assert!(keywords.iter().any(|k| k.label == "if"));
        assert!(keywords.iter().any(|k| k.label == "match"));
    }

    #[test]
    fn test_type_completions() {
        let provider = CompletionProvider::new();
        let types = provider.get_type_completions();

        assert!(types.len() >= 13);
        assert!(types.iter().any(|t| t.label == "i32"));
        assert!(types.iter().any(|t| t.label == "String"));
        assert!(types.iter().any(|t| t.label == "bool"));
    }

    #[test]
    fn test_function_completions() {
        let provider = CompletionProvider::new();
        let functions = provider.get_function_completions();

        assert!(functions.len() >= 2);
        assert!(functions.iter().any(|f| f.label == "println"));
        assert!(functions.iter().any(|f| f.label == "print"));
    }

    #[test]
    fn test_completion_has_details() {
        let provider = CompletionProvider::new();
        let completions = provider.get_completions("", Position::new(0, 0));

        // All completions should have details
        for completion in &completions {
            assert!(
                completion.detail.is_some(),
                "Completion '{}' missing detail",
                completion.label
            );
        }
    }
}
